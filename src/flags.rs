#[derive(Debug, Default)]
pub struct Flags {
    pub sign: bool,
    pub zero: bool,
    pub parity: bool,
    pub carry: bool,
    pub aux_carry: bool,
}

pub use FlagsName::*;
pub enum FlagsName {
    Sign,
    Zero,
    Parity,
    Carry,
    AuxCarry,
}

impl Flags {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self, res: (u8, bool), updates: &[FlagsName]) {
        let (val, overflow) = res;

        for f in updates {
            match f {
                Sign => self.sign = val & 0x80 != 0,
                Zero => self.zero = val == 0,
                Parity => self.parity = val.count_zeros() % 2 == 0,
                Carry => self.carry = overflow,
                AuxCarry => (),
            }
        }
    }

    pub fn as_byte(&self) -> u8 {
        0 | ((self.sign as u8) << 7)
            | ((self.zero as u8) << 6)
            | ((self.parity as u8) << 5)
            | ((self.carry as u8) << 4)
            | ((self.aux_carry as u8) << 3)
    }
}
