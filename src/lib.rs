#![allow(mutable_borrow_reservation_conflict)]
#![allow(dead_code)]

mod decompiler;
mod flags;
mod memory;
mod registers;

use bitmatch::bitmatch;
use flags::*;
use memory::Memory;
use registers::Registers;

type Error = Box<dyn std::error::Error>;

pub struct Cpu {
    reg: Registers,
    /// stack pointer
    sp: u16,
    /// program counter
    pc: usize,
    ram: Memory,
    flags: Flags,
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
    pub fn from_filename(file: &str) -> Result<Self, Error> {
        Ok(Self {
            reg: Registers::new(),

            sp: 0,
            pc: 0,

            ram: Memory::from_file(file)?,
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
        println!("{:#04x}\t{}", self.pc, decompiler::instr(opcode));

        #[bitmatch]
        match opcode[0] {
            "0000_0000" => self.nop(),
            "1100_0011" => self.jmp(addr(opcode)),
            "1100_1101" => self.call(addr(opcode)),
            // register
            "00rr_r101" => self.dcr(r.into()),
            "00rr_r100" => self.inr(r.into()),
            "00aa_a110" => self.mvi(a.into(), opcode[1]),
            // register pair
            "00rr_0001" => self.lxi(r, d16(opcode)),
            "00rr_1010" => self.ldax(r),
            "00rr_1011" => self.dcx(r),
            "00rr_0011" => self.inx(r),
            // opther
            "0111_0110" => self.halt(), // overlap with the mov instruction
            "01aa_abbb" => self.mov(a.into(), b.into()),
            "aaaa_aaaa" => panic!("Instruction {0:#08b} {0:#04x} is not implemented", a),
        }
    }

    fn nop(&mut self) {
        self.pc += 1;
    }

    /// Unconditionnal jump
    fn jmp(&mut self, addr: usize) {
        self.pc = addr;
    }

    /// Unconditionnal subroutine call
    fn call(&mut self, addr: usize) {
        let ret_addr = self.pc + 2;
        let stack = self.ram.dword_mut((self.sp - 1) as usize);
        *stack = ret_addr as u16;
        self.sp -= 2;
        self.pc = addr;
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
        let d16 = u16::from_be(d16);
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
            a => panic!("INX called with invalid register pair: {:x}", a),
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
