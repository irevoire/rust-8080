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
    /// cpu.sp = 2; // make sp point to 0xff, 0xff
    /// cpu.reg.de_set(0);
    /// cpu.cycle();
    /// assert_eq!(cpu.sp, 0);
    /// assert_eq!(cpu.pc, 1);
    /// assert_eq!(cpu.reg.d, 0xff);
    /// assert_eq!(cpu.reg.e, 0xaa);
    /// assert_eq!(cpu.reg.de(), 0xffaa);
    /// ```
    pub fn pop(&mut self, rp: u8) {
        let tmp = self.internal_pop();

        match rp {
            0x00 => self.reg.bc_set(tmp),
            0x01 => self.reg.de_set(tmp),
            0x02 => self.reg.hl_set(tmp),
            a => panic!("POP called with invalid register pair: {:x}", a),
        }
        self.sp -= 2;
        self.pc += 1;
    }

    /// Pop PSW from the stack
    /// When PSW is POP'd, ALL flags are affected.
    /// see the function [pop](#method.pop) for other registers
    /// ```rust
    /// use rust_8080::*;
    ///
    /// let mut cpu = Cpu::from_raw(vec![0b11110001, 0x00, 0xaa, 0xff]);
    /// cpu.pc = 0; // pop the content of sp to 11 (a + flags)
    /// cpu.sp = 2; // make sp point to 0xff, 0xff
    /// cpu.reg.a = 0;
    /// cpu.cycle();
    /// assert_eq!(cpu.sp, 0);
    /// assert_eq!(cpu.pc, 1);
    /// assert_eq!(cpu.reg.a, 0xaa);
    /// assert_eq!(cpu.reg.sign(), true);
    /// assert_eq!(cpu.reg.zero(), true);
    /// assert_eq!(cpu.reg.parity(), true);
    /// assert_eq!(cpu.reg.carry(), true);
    /// assert_eq!(cpu.reg.half_carry(), true);
    /// ```
    pub fn pop_psw(&mut self) {
        let tmp = self.internal_pop();

        self.sp -= 2;
        self.reg.set_psw(tmp);
        self.pc += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut cpu = Cpu::from_raw(vec![0x00, 0x01, 0x02, 0x03]);
        cpu.pc = 0;
        cpu.sp = 0; // make sp point to 0x00, 0x01
        cpu.reg.bc_set(0x4235);
        cpu.push(0x00); // bc
        assert_eq!(cpu.sp, 2);
        assert_eq!(cpu.pc, 1);
        assert_eq!(cpu.reg.bc(), 0x4235); // this could be wrong on arm?
        assert_eq!(cpu.reg.de(), 0x0000);

        cpu.pop(0x01); // de
        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.pc, 2);
        assert_eq!(cpu.reg.bc(), cpu.reg.de());
    }
}
