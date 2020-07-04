use bit_field::BitField;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[repr(C, packed)]
pub struct Registers {
    pub flags: u8,

    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,

    pub h: u8,
    pub l: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Flags {
    Sign,
    Zero,
    Parity,
    Carry,
    AuxCarry,
}

impl Registers {
    pub fn new() -> Self {
        Default::default()
    }

    /// give access to a merge of the registers flags and a
    pub fn psw(&self) -> u16 {
        self.a as u16 | ((Self::_fix_flags(self.flags) as u16) << 8)
    }

    /// give access to a merge of the register b and c
    pub fn bc(&self) -> u16 {
        unsafe { *std::mem::transmute::<&u8, &u16>(&self.b) }
    }

    /// give access to a merge of the register d and e
    pub fn de(&self) -> u16 {
        unsafe { *std::mem::transmute::<&u8, &u16>(&self.d) }
    }

    /// give access to a merge of the register h and l
    pub fn hl(&self) -> u16 {
        unsafe { *std::mem::transmute::<&u8, &u16>(&self.h) }
    }

    /// give access to a merge of the registers flags and a
    pub fn psw_mut(&mut self) -> &mut u16 {
        self.fix_flags();
        unsafe { std::mem::transmute::<&mut u8, &mut u16>(&mut self.flags) }
    }

    /// give access to a merge of the register b and c
    pub fn bc_mut(&mut self) -> &mut u16 {
        unsafe { std::mem::transmute::<&mut u8, &mut u16>(&mut self.b) }
    }

    /// give access to a merge of the register d and e
    pub fn de_mut(&mut self) -> &mut u16 {
        unsafe { std::mem::transmute::<&mut u8, &mut u16>(&mut self.d) }
    }

    /// give access to a merge of the register h and l
    pub fn hl_mut(&mut self) -> &mut u16 {
        unsafe { std::mem::transmute::<&mut u8, &mut u16>(&mut self.h) }
    }

    /// set the part we should not use of the flags register to their correct value
    fn _fix_flags(mut f: u8) -> u8 {
        // bit 3 and 5 are always 0
        f.set_bit(3, false);
        f.set_bit(5, false);
        // bit 1 is always 1
        f.set_bit(1, true);
        f
    }

    /// set the part we should not use of the flags register to their correct value
    fn fix_flags(&mut self) {
        self.flags = Self::_fix_flags(self.flags);
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

/// access to the registers by indexes
/// If the indey 0x06 is supplied this function will panic.
/// The index 0x06 mean you should use the registers hl as an index in the memory
impl std::ops::Index<usize> for Registers {
    type Output = u8;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0x00 => &self.b,
            0x01 => &self.c,
            0x02 => &self.d,
            0x03 => &self.e,
            0x04 => &self.h,
            0x05 => &self.l,
            // access into memory
            0x07 => &self.a,
            _ => panic!("Access to the undefined register number: {}", idx),
        }
    }
}

/// access to the registers by indexes
/// If the indey 0x06 is supplied this function will panic.
/// The index 0x06 mean you should use the registers hl as an index in the memory
impl std::ops::IndexMut<usize> for Registers {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0x00 => &mut self.b,
            0x01 => &mut self.c,
            0x02 => &mut self.d,
            0x03 => &mut self.e,
            0x04 => &mut self.h,
            0x05 => &mut self.l,
            // access into memory
            0x07 => &mut self.a,
            _ => panic!("Access to the undefined register number: {}", idx),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_of_registers() {
        let mut registers = Registers::new();
        let b = 0x03; // 0b0000_0011
        let c = 0xc0; // 0b1100_0000
        registers.b = b;
        registers.c = c;
        assert_eq!(*registers.bc(), 0xC003);
    }
}
