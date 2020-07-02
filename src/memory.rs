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

    pub fn from_file_at(file: &str, starting_addr: usize) -> Result<Self, Error> {
        let mut f = std::fs::File::open(file)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        let mut vec = vec![0; starting_addr];
        vec.append(&mut buffer);
        // inject "out 1, a at 0x0000 (signal to stop the test)
        vec[0] = 0xD3;
        vec[1] = 0x00;

        // inject "in a,0" at 0x0005 (signal to output some characters)
        vec[5] = 0xDB;
        vec[6] = 0x00;
        vec[7] = 0xC9;

        Ok(Self { vec })
    }

    pub fn dword(&self, idx: usize) -> &u16 {
        unsafe { std::mem::transmute::<&u8, &u16>(&self.vec[idx]) }
    }

    pub fn dword_mut(&mut self, idx: usize) -> &mut u16 {
        unsafe { std::mem::transmute::<&mut u8, &mut u16>(&mut self.vec[idx]) }
    }
}
