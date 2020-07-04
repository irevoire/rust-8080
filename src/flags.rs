#[derive(Debug, Default)]
pub struct Flags {
    pub sign: bool,
    pub zero: bool,
    pub parity: bool,
    pub carry: bool,
    pub aux_carry: bool,
}

impl Flags {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn as_byte(&self) -> u8 {
        0 | ((self.sign as u8) << 7)
            | ((self.zero as u8) << 6)
            | ((self.parity as u8) << 5)
            | ((self.carry as u8) << 4)
            | ((self.aux_carry as u8) << 3)
    }
}
