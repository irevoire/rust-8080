mod dcr;
mod halt;
mod mov;
mod mvi;
mod nop;
mod pop;
mod push;
mod sphl;

use crate::*;
use anyhow::Result;
use Flags::*;

pub struct Cpu {
    pub reg: Registers,
    /// stack pointer
    pub sp: u16,
    /// program counter
    pub pc: usize,
    pub ram: Memory,
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
    pub fn from_filename_at(file: &str, starting_addr: usize) -> Result<Self> {
        Ok(Self {
            reg: Registers::new(),

            sp: 0,
            pc: starting_addr,

            ram: Memory::from_file_at(file, starting_addr)?,
        })
    }

    pub fn from_raw(from: Vec<u8>) -> Self {
        Self {
            reg: Registers::new(),

            sp: 0,
            pc: 0,

            ram: Memory::from_raw(from),
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
    }

    /// Unconditionnal jump
    fn jmp(&mut self, addr: usize) {
        self.pc = addr;
    }

    /// Conditionnal jump
    fn cond_jmp(&mut self, cond: u8, addr: usize) {
        let cond = match cond {
            0b000 => !self.reg.zero(),
            0b001 => self.reg.zero(),
            0b010 => !self.reg.carry(),
            0b011 => self.reg.carry(),
            0b100 => !self.reg.parity(),
            0b101 => self.reg.parity(),
            0b110 => !self.reg.sign(),
            0b111 => self.reg.sign(),
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

    /// Load H:L from memory
    fn lhld(&mut self, a: u16) {
        let dword = *self.ram.dword(a as usize);
        *self.reg.hl_mut() = (dword << 8) | (dword >> 8);
        self.pc += 3;
    }

    /// Load indirect through BC or DE
    fn ldax(&mut self, rp: u8) {
        self.reg.a = match rp {
            0x00 => self.ram[self.reg.bc() as usize],
            0x01 => self.ram[self.reg.de() as usize],
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

    /// Compare register with A
    fn cmp(&mut self, r: usize) {
        let res = self.reg.a.overflowing_sub(self.reg[r]);
        self.reg
            .update_flags(res, &[Zero, Sign, Parity, Carry, AuxCarry]);
        self.pc += 1;
    }

    /// Compare immediate with A
    fn cpi(&mut self, val: u8) {
        let res = self.reg.a.overflowing_sub(val);
        self.reg
            .update_flags(res, &[Zero, Sign, Parity, Carry, AuxCarry]);
        self.pc += 2;
    }

    /// Increment register
    /// update the flags: Zero, Sign, Parity, AuxiliaryCarry
    fn inr(&mut self, r: usize) {
        let r = match r {
            0x06 => &mut self.ram[self.reg.hl() as usize],
            r => &mut self.reg[r],
        };
        let res = r.overflowing_add(1);
        *r = res.0;
        self.reg.update_flags(res, &[Zero, Sign, Parity, AuxCarry]);
        self.pc += 1;
    }
}
