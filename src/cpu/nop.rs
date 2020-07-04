use super::*;

impl Cpu {
    /// No operation
    /// ```rust
    /// use rust_8080::*;
    ///
    /// let mut cpu = Cpu::from_raw(vec![0b00000000]);
    /// cpu.pc = 0;
    /// cpu.cycle();
    /// assert_eq!(cpu.pc, 1);
    /// ```
    pub fn nop(&mut self) {
        self.pc += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop() {
        let mut cpu = Cpu::from_raw(vec![0]);
        cpu.cycle();

        assert_eq!(cpu.pc, 1);
    }
}
