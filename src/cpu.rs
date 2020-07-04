use crate::*;

pub struct Cpu {
    pub reg: Registers,
    /// stack pointer
    pub sp: u16,
    /// program counter
    pub pc: usize,
    pub ram: Memory,
    pub flags: Flags,
}

/// merge the byte 2 and 3 of the opcode to create a 16 bits number
fn d16(opcode: &[u8]) -> u16 {
    if opcode.len() < 3 {
        panic!("Malformed opcode");
    }
    ((opcode[2] as u16) << 8) | opcode[1] as u16
}

/// merge the byte 2 and 3 of the opcode to create a usize
fn addr(opcode: &[u8]) -> usize {
    if opcode.len() < 3 {
        panic!("Malformed opcode");
    }
    ((opcode[2] as usize) << 8) | opcode[1] as usize
}

impl Cpu {
    pub fn from_filename_at(file: &str, starting_addr: usize) -> Result<Self, Error> {
        Ok(Self {
            reg: Registers::new(),

            sp: 0,
            pc: starting_addr,

            ram: Memory::from_file_at(file, starting_addr)?,
            flags: Flags::new(),
        })
    }

    pub fn from_bytes(from: Vec<u8>) -> Self {
        Self {
            reg: Registers::new(),

            sp: 0,
            pc: 0,

            ram: Memory::from(from),
            flags: Flags::new(),
        }
    }

    #[bitmatch]
    pub fn cycle(&mut self) {
        let opcode = &self.ram[self.pc..];
        println!("{:04x}\t{}", self.pc, decompiler::instr(opcode));

        #[bitmatch]
        match opcode[0] {
            "0000_0000" => self.nop(),
            "1100_0011" => self.jmp(addr(opcode)),
            "11cc_c010" => self.cond_jmp(c, addr(opcode)),
            "1100_1101" => self.call(addr(opcode)),
            "1100_1001" => self.ret(),
            // register
            "00rr_r101" => self.dcr(r.into()),
            "00rr_r100" => self.inr(r.into()),
            "1011_1sss" => self.cmp(s.into()),
            "1111_1110" => self.cpi(opcode[1]),
            "00aa_a110" => self.mvi(a.into(), opcode[1]),
            // register pair
            "11111001" => self.sphl(),
            "00101010 " => self.lhld(d16(opcode)),
            "00rr_0001" => self.lxi(r, d16(opcode)),
            "00rr_1010" => self.ldax(r),
            "00rr_1011" => self.dcx(r),
            "00rr_0011" => self.inx(r),
            "1111_0001" => self.pop_psw(),
            "11rr_0001" => self.pop(r),
            "1111_0101" => self.push_psw(),
            "11rr_0101" => self.push(r),
            // other
            "0111_0110" => self.halt(), // overlap with the mov instruction
            "01aa_abbb" => self.mov(a.into(), b.into()),
            "aaaa_aaaa" => panic!("Instruction {0:#010b} {0:#04x} is not implemented", a),
        }

        println!("sp: {0} {0:#x}", self.sp);
        println!("registers: {:?}", self.reg);
        println!("flags: {:?}", self.flags);
    }

    fn nop(&mut self) {
        self.pc += 1;
    }

    /// Unconditionnal jump
    fn jmp(&mut self, addr: usize) {
        self.pc = addr;
    }

    /// Conditionnal jump
    fn cond_jmp(&mut self, cond: u8, addr: usize) {
        let cond = match cond {
            0b000 => !self.flags.zero,
            0b001 => self.flags.zero,
            0b010 => !self.flags.carry,
            0b011 => self.flags.carry,
            0b100 => !self.flags.parity,
            0b101 => self.flags.parity,
            0b110 => !self.flags.sign,
            0b111 => self.flags.sign,
            c => panic!("cond_jmp called with invalid value: {:b}", c),
        };
        if cond {
            self.pc = addr;
        } else {
            self.pc += 2;
        }
    }

    /// Unconditionnal subroutine call
    fn call(&mut self, addr: usize) {
        let ret_addr = self.pc + 3;
        let stack = self.ram.dword_mut((self.sp) as usize);
        *stack = ret_addr as u16;
        self.sp += 2;
        self.pc = addr;
    }

    /// Return from a subroutine call
    fn ret(&mut self) {
        let ret_addr = usize::from_le(*self.ram.dword(self.sp as usize) as usize);
        self.pc = ret_addr;
        self.sp -= 2;
    }

    /// Set SP to content of H:L
    /// ```rust
    /// use rust_8080::*;
    ///
    /// let mut cpu = Cpu::from_bytes(vec![0b11111001]);
    /// cpu.pc = 0;
    /// cpu.sp = 0;
    /// *cpu.reg.hl_mut() = 42;
    /// cpu.cycle();
    /// assert_eq!(cpu.sp, 42);
    /// assert_eq!(cpu.pc, 1);
    /// ```
    fn sphl(&mut self) {
        self.sp = *self.reg.hl();
        self.pc += 1;
    }

    /// Load H:L from memory
    fn lhld(&mut self, a: u16) {
        let dword = *self.ram.dword(a as usize);
        *self.reg.hl_mut() = (dword << 8) | (dword >> 8);
        self.pc += 3;
    }

    /// Load indirect through BC or DE
    fn ldax(&mut self, rp: u8) {
        self.reg.a = match rp {
            0x00 => self.ram[*self.reg.bc() as usize],
            0x01 => self.ram[*self.reg.de() as usize],
            a => panic!("LDAX called with invalid register pair: {:x}", a),
        };
        self.pc += 1;
    }

    /// Load register pair immediate
    fn lxi(&mut self, rp: u8, d16: u16) {
        match rp {
            0x00 => *self.reg.bc_mut() = d16,
            0x01 => *self.reg.de_mut() = d16,
            0x02 => *self.reg.hl_mut() = d16,
            0x03 => self.sp = d16,
            a => panic!("LXI called with invalid register pair: {:x}", a),
        }
        self.pc += 3;
    }

    /// Decrement register pair
    /// Do not update any flags
    fn dcx(&mut self, rp: u8) {
        let rp = match rp {
            0x00 => self.reg.bc_mut(),
            0x01 => self.reg.de_mut(),
            0x02 => self.reg.hl_mut(),
            0x03 => &mut self.sp,
            a => panic!("DNX called with invalid register pair: {:x}", a),
        };
        *rp = rp.wrapping_sub(1);
        self.pc += 1;
    }

    /// Increment register pair
    /// Do not update any flags
    fn inx(&mut self, rp: u8) {
        let rp = match rp {
            0x00 => self.reg.bc_mut(),
            0x01 => self.reg.de_mut(),
            0x02 => self.reg.hl_mut(),
            0x03 => &mut self.sp,
            a => panic!("INX called with invalid register pair: {:x}", a),
        };
        *rp = rp.wrapping_add(1);
        self.pc += 1;
    }

    /// Push register pair from the stack
    /// RP=11 refers to PSW for PUSH (cannot push SP).
    /// see the [push_psw](#method.push_psw) method
    /// ```rust
    /// use rust_8080::*;
    ///
    /// let mut cpu = Cpu::from_bytes(vec![0b11010101, 0x00, 0xff, 0xaa]);
    /// cpu.pc = 0; // push the content of 01 (de) to sp
    /// cpu.sp = 0; // make sp point to 0xff, 0xaa
    /// *cpu.reg.de_mut() = 0x9911;
    /// cpu.cycle();
    /// assert_eq!(cpu.sp, 2);
    /// assert_eq!(cpu.pc, 1);
    /// assert_eq!(cpu.ram[2], 0x11);
    /// assert_eq!(cpu.ram[3], 0x99);
    /// ```
    fn push(&mut self, rp: u8) {
        let rp = match rp {
            0x00 => self.reg.bc(),
            0x01 => self.reg.de(),
            0x02 => self.reg.hl(),
            a => panic!("POP called with invalid register pair: {:x}", a),
        };
        self.sp += 2;
        *self.ram.dword_mut(self.sp as usize) = *rp;
        self.pc += 1;
    }

    /// Push PSW from the stack
    /// see the function [push](#method.push) for other registers
    /// ```rust
    /// use rust_8080::*;
    ///
    /// let mut cpu = Cpu::from_bytes(vec![0b11110101, 0x00, 0xff, 0xaa]);
    /// cpu.pc = 0; // push the content of 01 (de) to sp
    /// cpu.sp = 1; // make sp point to 0xff, 0xaa
    /// cpu.reg.a = 0x99;
    /// cpu.cycle();
    /// assert_eq!(cpu.sp, 3);
    /// assert_eq!(cpu.pc, 1);
    /// assert_eq!(cpu.ram[2], 0x99);
    /// // assert_eq!(cpu.ram[3], 0x??); I have no idea of what it should be
    /// ```
    fn push_psw(&mut self) {
        self.sp += 1;
        self.ram[self.sp as usize] = self.reg.a;
        self.sp += 1;
        self.ram[self.sp as usize] = self.flags.as_byte();
        self.pc += 1;
    }

    /// Pop register pair from the stack
    /// RP=11 refers to PSW for POP (cannot pop SP).
    /// see the [pop_psw](#method.pop_psw) method
    /// ```rust
    /// use rust_8080::*;
    ///
    /// let mut cpu = Cpu::from_bytes(vec![0b11010001, 0x00, 0xff, 0xaa]);
    /// cpu.pc = 0; // pop the content of sp to 01 (de)
    /// cpu.sp = 4; // make sp point to 0xff, 0xff
    /// *cpu.reg.de_mut() = 0;
    /// cpu.cycle();
    /// assert_eq!(cpu.sp, 2);
    /// assert_eq!(cpu.pc, 1);
    /// assert_eq!(cpu.reg.d, 0xaa);
    /// assert_eq!(cpu.reg.e, 0xff);
    /// assert_eq!(*cpu.reg.de(), 0xffaa);
    /// ```
    fn pop(&mut self, rp: u8) {
        let rp = match rp {
            0x00 => self.reg.bc_mut(),
            0x01 => self.reg.de_mut(),
            0x02 => self.reg.hl_mut(),
            a => panic!("POP called with invalid register pair: {:x}", a),
        };
        self.sp -= 2;
        let tmp = *self.ram.dword(self.sp as usize);
        *rp = ((tmp & 0xff) << 8) | (tmp >> 8);
        self.pc += 1;
    }

    /// Pop PSW from the stack
    /// When PSW is POP'd, ALL flags are affected.
    /// see the function [pop](#method.pop) for other registers
    /// ```rust
    /// use rust_8080::*;
    ///
    /// let mut cpu = Cpu::from_bytes(vec![0b11110001, 0x00, 0xff, 0xaa]);
    /// cpu.pc = 0; // pop the content of sp to 11 (a + flags)
    /// cpu.sp = 4; // make sp point to 0xff, 0xff
    /// cpu.reg.a = 0;
    /// cpu.cycle();
    /// assert_eq!(cpu.sp, 2);
    /// assert_eq!(cpu.pc, 1);
    /// assert_eq!(cpu.reg.a, 0xaa);
    /// assert_eq!(cpu.flags.sign, true);
    /// assert_eq!(cpu.flags.zero, false);
    /// assert_eq!(cpu.flags.parity, true);
    /// assert_eq!(cpu.flags.carry, false);
    /// assert_eq!(cpu.flags.aux_carry, false);
    /// ```
    fn pop_psw(&mut self) {
        self.sp -= 1;
        let res = self.ram[self.sp as usize];
        self.flags
            .update((res, false), &[Zero, Sign, Parity, Carry, AuxCarry]);
        self.pc += 1;
        self.reg.a = self.ram[self.sp as usize];
        self.sp -= 1;
    }

    /// Compare register with A
    fn cmp(&mut self, r: usize) {
        let res = self.reg.a.overflowing_sub(self.reg[r]);
        self.flags
            .update(res, &[Zero, Sign, Parity, Carry, AuxCarry]);
        self.pc += 1;
    }

    /// Compare immediate with A
    fn cpi(&mut self, val: u8) {
        let res = self.reg.a.overflowing_sub(val);
        self.flags
            .update(res, &[Zero, Sign, Parity, Carry, AuxCarry]);
        self.pc += 2;
    }

    /// Move immediate to register
    fn mvi(&mut self, a: usize, val: u8) {
        let a = match a {
            0x06 => &mut self.ram[*self.reg.hl() as usize],
            a => &mut self.reg[a],
        };
        *a = val;
        self.pc += 2;
    }

    /// Decrement register
    /// update the flags: Zero, Sign, Parity, AuxiliaryCarry
    fn dcr(&mut self, r: usize) {
        let r = match r {
            0x06 => &mut self.ram[*self.reg.hl() as usize],
            r => &mut self.reg[r],
        };
        let res = r.overflowing_sub(1);
        *r = res.0;
        self.flags.update(res, &[Zero, Sign, Parity, AuxCarry]);
        self.pc += 1;
    }

    /// Increment register
    /// update the flags: Zero, Sign, Parity, AuxiliaryCarry
    fn inr(&mut self, r: usize) {
        let r = match r {
            0x06 => &mut self.ram[*self.reg.hl() as usize],
            r => &mut self.reg[r],
        };
        let res = r.overflowing_add(1);
        *r = res.0;
        self.flags.update(res, &[Zero, Sign, Parity, AuxCarry]);
        self.pc += 1;
    }

    /// Move register nb a to register nb b
    fn mov(&mut self, a: usize, b: usize) {
        let b = match b {
            0x06 => self.ram[*self.reg.hl() as usize],
            b => self.reg[b],
        };
        let a = match a {
            0x06 => &mut self.ram[*self.reg.hl() as usize],
            a => &mut self.reg[a],
        };
        *a = b;
        self.pc += 1;
    }

    /// Halt processor
    fn halt(&mut self) {
        panic!("CPU HALTED");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop() {
        let mut cpu = Cpu::from_bytes(vec![0]);
        cpu.cycle();

        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_mvi() {
        //                                 MVI  A <- D8
        let mut cpu = Cpu::from_bytes(vec![0b00_111_110, 42]);
        cpu.mvi(0, 1);
        assert_eq!(cpu.reg.b, 1);
        cpu.mvi(1, 2);
        assert_eq!(cpu.reg.c, 2);
        cpu.mvi(2, 3);
        assert_eq!(cpu.reg.d, 3);
        cpu.mvi(3, 4);
        assert_eq!(cpu.reg.e, 4);
        cpu.mvi(4, 5);
        assert_eq!(cpu.reg.h, 5);
        cpu.mvi(5, 6);
        assert_eq!(cpu.reg.l, 6);
        cpu.mvi(7, 7);
        assert_eq!(cpu.reg.a, 7);

        cpu.pc = 0;
        cpu.cycle(); //execute MVI  A <- 42
        assert_eq!(cpu.reg.a, 42);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn test_mov() {
        //                                 MOV  A <- D   MOV  M <- D
        let mut cpu = Cpu::from_bytes(vec![0b01_111_010, 0b01_110_010]);
        cpu.reg.b = 12;
        cpu.reg.c = 2;
        cpu.reg.d = 42;
        cpu.mov(0, 1);

        assert_eq!(cpu.reg.b, 2);
        assert_eq!(cpu.reg.c, 2);

        cpu.pc = 0;
        cpu.cycle(); // execute the mov A <- D
        assert_eq!(cpu.reg.a, 42);
        assert_eq!(cpu.reg.d, 42);

        *cpu.reg.hl_mut() = 0; // we want to modify the address 0
        cpu.pc = 1;
        cpu.cycle(); // execute the mov M <- D
        assert_eq!(cpu.ram[0], 42);

        assert_eq!(cpu.pc, 2);
    }

    #[test]
    #[should_panic]
    fn test_halt() {
        let mut cpu = Cpu::from_bytes(vec![0b01110110]);
        cpu.cycle();
    }

    #[test]
    fn test_dcr() {
        let mut cpu = Cpu::from_bytes(vec![0]);
        cpu.dcr(0);
        assert_eq!(cpu.flags.sign, true);
        assert_eq!(cpu.flags.carry, false);
    }
}
