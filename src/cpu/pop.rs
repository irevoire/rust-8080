use super::*;

impl Cpu {
    /// Pop register pair from the stack
    /// RP=11 refers to PSW for POP (cannot pop SP).
    /// see the [pop_psw](#method.pop_psw) method
    /// ```rust
    /// use rust_8080::*;
    ///
    /// let mut cpu = Cpu::from_raw(vec![0b11010001, 0x00, 0xff, 0xaa]);
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
    pub fn pop(&mut self, rp: u8) {
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
    /// let mut cpu = Cpu::from_raw(vec![0b11110001, 0x00, 0xff, 0xaa]);
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
    pub fn pop_psw(&mut self) {
        self.sp -= 1;
        let res = self.ram[self.sp as usize];
        self.flags
            .update((res, false), &[Zero, Sign, Parity, Carry, AuxCarry]);
        self.pc += 1;
        self.reg.a = self.ram[self.sp as usize];
        self.sp -= 1;
    }
}
