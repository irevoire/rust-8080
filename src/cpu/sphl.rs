use super::*;

impl Cpu {
    /// Set SP to content of H:L
    /// ```rust
    /// use rust_8080::*;
    ///
    /// let mut cpu = Cpu::from_raw(vec![0b11111001]);
    /// cpu.pc = 0;
    /// cpu.sp = 0;
    /// *cpu.reg.hl_mut() = 42;
    /// cpu.cycle();
    /// assert_eq!(cpu.sp, 42);
    /// assert_eq!(cpu.pc, 1);
    /// ```
    pub fn sphl(&mut self) {
        self.sp = self.reg.hl();
        self.pc += 1;
    }
}
