use super::*;

impl Cpu {
    /// Move immediate to register
    pub fn mvi(&mut self, a: usize, val: u8) {
        let a = match a {
            0x06 => &mut self.ram[*self.reg.hl() as usize],
            a => &mut self.reg[a],
        };
        *a = val;
        self.pc += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mvi() {
        //                                 MVI  A <- D8
        let mut cpu = Cpu::from_raw(vec![0b00_111_110, 42]);
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
}
