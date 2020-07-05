use super::*;

impl Cpu {
    /// Set SP to content of H:L
    /// ```rust
    /// use rust_8080::*;
    ///
    /// let mut cpu = Cpu::from_raw(vec![0b11111001]);
    /// cpu.pc = 0;
    /// cpu.sp = 0;
    /// cpu.reg.h = 0xc9;
    /// cpu.reg.l = 0;
    /// cpu.cycle();
    /// assert_eq!(cpu.sp, 0xc900);
    /// assert_eq!(cpu.pc, 1);
    /// ```
    pub fn sphl(&mut self) {
        self.sp = self.reg.hl();
        self.pc += 1;
    }
}
