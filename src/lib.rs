#![allow(mutable_borrow_reservation_conflict)]
#![allow(dead_code)]

mod cpu;
pub mod decompiler;
mod flags;
mod memory;
mod registers;

use bitmatch::bitmatch;
pub use cpu::Cpu;
pub use flags::*;
pub use memory::Memory;
pub use registers::Registers;

type Error = Box<dyn std::error::Error>;
