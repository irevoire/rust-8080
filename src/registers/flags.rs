use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Flags {
    Sign,
    Zero,
    Parity,
    Carry,
    AuxCarry,
}

impl Registers {
    /// set the part we should not use of the flags register to their correct value
    pub(super) fn fix_flags(&mut self) {
        self.flags = _fix_flags(self.flags);
    }

    /// return true if the sign flag is set
    pub fn sign(&self) -> bool {
        self.flags.get_bit(7)
    }

    /// set the sign flag
    pub fn set_sign(&mut self, value: bool) {
        self.flags.set_bit(7, value);
    }

    /// return true if the zero flag is set
    pub fn zero(&self) -> bool {
        self.flags.get_bit(6)
    }

    /// set the zero flag
    pub fn set_zero(&mut self, value: bool) {
        self.flags.set_bit(6, value);
    }

    /// return true if the half-carry flag is set
    pub fn half_carry(&self) -> bool {
        self.flags.get_bit(4)
    }

    /// set the half-carry flag
    pub fn set_half_carry(&mut self, value: bool) {
        self.flags.set_bit(4, value);
    }

    /// return true if the parity flag is set
    pub fn parity(&self) -> bool {
        self.flags.get_bit(2)
    }

    /// set the parity flag
    pub fn set_parity(&mut self, value: bool) {
        self.flags.set_bit(2, value);
    }

    /// return true if the carry flag is set
    pub fn carry(&self) -> bool {
        self.flags.get_bit(0)
    }

    /// set the carry flag
    pub fn set_carry(&mut self, value: bool) {
        self.flags.set_bit(0, value);
    }

    /// updates the flags with the result of an checked_xxx call (result, carry)
    pub fn update_flags(&mut self, res: (u8, bool), updates: &[Flags]) {
        use Flags::*;

        let (val, overflow) = res;

        for f in updates {
            match f {
                Sign => self.set_sign(val & 0x80 != 0),
                Zero => self.set_zero(val == 0),
                Parity => self.set_parity(val.count_zeros() % 2 == 0),
                Carry => self.set_carry(overflow),
                AuxCarry => (),
            }
        }
    }
}

/// set the part we should not use of the flags register to their correct value
pub(super) fn _fix_flags(mut f: u8) -> u8 {
    // bit 3 and 5 are always 0
    f.set_bit(3, false);
    f.set_bit(5, false);
    // bit 1 is always 1
    f.set_bit(1, true);
    f
}
