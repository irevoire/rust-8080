use crate::Error;
use std::io::Read;

pub struct Memory {
    vec: Vec<u8>,
}

impl<T: std::slice::SliceIndex<[u8]>> std::ops::Index<T> for Memory {
    type Output = T::Output;

    fn index(&self, idx: T) -> &Self::Output {
        &self.vec[idx]
    }
}

impl<T: std::slice::SliceIndex<[u8]>> std::ops::IndexMut<T> for Memory {
    fn index_mut(&mut self, idx: T) -> &mut Self::Output {
        &mut self.vec[idx]
    }
}

impl Memory {
    pub fn from(vec: Vec<u8>) -> Self {
        Self { vec }
    }

    pub fn from_file(file: &str) -> Result<Self, Error> {
        let mut f = std::fs::File::open(file)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        Ok(Self { vec: buffer })
    }

    pub fn dword(&self, idx: usize) -> &u16 {
        unsafe { std::mem::transmute::<&u8, &u16>(&self.vec[idx]) }
    }

    pub fn dword_mut(&mut self, idx: usize) -> &mut u16 {
        unsafe { std::mem::transmute::<&mut u8, &mut u16>(&mut self.vec[idx]) }
    }
}
