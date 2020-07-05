use anyhow::Result;
use std::io::Read;

#[derive(Clone, Debug)]
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
    pub fn from_raw(vec: Vec<u8>) -> Self {
        Self { vec }
    }

    pub fn from_file_at(file: &str, starting_addr: usize) -> Result<Self> {
        let mut f = std::fs::File::open(file)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        let mut vec = vec![0; starting_addr];
        vec.append(&mut buffer);

        Ok(Self { vec })
    }

    pub fn dword(&self, idx: impl Into<usize>) -> u16 {
        let dword = *unsafe { std::mem::transmute::<&u8, &u16>(&self.vec[idx.into()]) };
        u16::from_be(dword)
    }

    pub fn dword_set(&mut self, idx: impl Into<usize>, value: u16) {
        let dword = unsafe { std::mem::transmute::<&mut u8, &mut u16>(&mut self.vec[idx.into()]) };
        *dword = u16::to_be(value);
    }
}
