use super::*;

impl Cpu {
    /// Push register pair from the stack
    /// RP=11 refers to PSW for PUSH (cannot push SP).
    /// see the [push_psw](#method.push_psw) method
    /// ```rust
    /// use rust_8080::*;
    ///
    /// let mut cpu = Cpu::from_raw(vec![0b11010101, 0x00, 0xff, 0xaa]);
    /// cpu.pc = 0; // push the content of 01 (de) to sp
    /// cpu.sp = 0; // make sp point to 0xff, 0xaa
    /// cpu.reg.de_set(0x9911);
    /// cpu.cycle();
    /// assert_eq!(cpu.sp, 2);
    /// assert_eq!(cpu.pc, 1);
    /// assert_eq!(cpu.ram[2], 0x99);
    /// assert_eq!(cpu.ram[3], 0x11);
    /// ```
    pub fn push(&mut self, rp: u8) {
        let rp = match rp {
            0x00 => self.reg.bc(),
            0x01 => self.reg.de(),
            0x02 => self.reg.hl(),
            a => panic!("PUSH called with invalid register pair: {:x}", a),
        };
        self.sp += 2;
        self.internal_push(rp);
        self.pc += 1;
    }

    /// Push PSW from the stack
    /// see the function [push](#method.push) for other registers
    /// ```rust
    /// use rust_8080::*;
    ///
    /// let mut cpu = Cpu::from_raw(vec![0b11110101, 0x00, 0xff, 0xaa]);
    /// cpu.pc = 0; // push the content of 01 (de) to sp
    /// cpu.sp = 0; // make sp point to 0xff, 0xaa
    /// cpu.reg.a = 0x99;
    /// cpu.cycle();
    /// assert_eq!(cpu.sp, 2);
    /// assert_eq!(cpu.pc, 1);
    /// assert_eq!(cpu.ram[2], 0x02); // the bit 1 of flags is always set
    /// assert_eq!(cpu.ram[3], 0x99);
    /// ```
    pub fn push_psw(&mut self) {
        self.sp += 2;
        self.internal_push(self.reg.psw());
        self.pc += 1;
    }
}
