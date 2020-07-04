use super::*;

impl Cpu {
    /// Halt processor
    pub fn halt(&mut self) {
        panic!("CPU HALTED");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_halt() {
        let mut cpu = Cpu::from_raw(vec![0b01110110]);
        cpu.cycle();
    }
}
