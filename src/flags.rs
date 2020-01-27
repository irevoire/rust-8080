#[derive(Default)]
pub struct Flags {
    sign: bool,
    zero: bool,
    parity: bool,
    carry: bool,
    aux_carry: bool,
}

impl Flags {
    pub fn new() -> Self {
        Default::default()
    }
}
