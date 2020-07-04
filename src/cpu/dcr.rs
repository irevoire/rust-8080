use super::*;

impl Cpu {
    /// Decrement register
    /// update the flags: Zero, Sign, Parity, AuxiliaryCarry
    pub fn dcr(&mut self, r: usize) {
        let r = match r {
            0x06 => &mut self.ram[*self.reg.hl() as usize],
            r => &mut self.reg[r],
        };
        let res = r.overflowing_sub(1);
        *r = res.0;
        self.flags.update(res, &[Zero, Sign, Parity, AuxCarry]);
        self.pc += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dcr() {
        let mut cpu = Cpu::from_raw(vec![0]);
        cpu.dcr(0);
        assert_eq!(cpu.flags.sign, true);
        assert_eq!(cpu.flags.carry, false);
    }
}
