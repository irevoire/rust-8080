#[derive(Debug, Default)]
#[repr(C, packed)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,

    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Default::default()
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
