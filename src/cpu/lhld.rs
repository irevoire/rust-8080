use super::*;

impl Cpu {
    /// Load H:L from memory
    /// Write the content of mem[d16] to hl
    /// ```rust
    /// use rust_8080::*;
    ///
    /// let mut cpu = Cpu::from_raw(vec![0x00, 0xc9]);
    /// assert_eq!(cpu.reg.h, 0x00);
    /// assert_eq!(cpu.reg.l, 0x00);
    /// cpu.lhld(0);
    /// assert_eq!(cpu.reg.hl(), 0xc900);
    /// ```
    pub fn lhld(&mut self, d16: u16) {
        self.reg.hl_set(self.ram.dword(d16));
        self.pc += 3;
    }
}
