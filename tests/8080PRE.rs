#![allow(non_snake_case)]

#[test]
fn main() {
    let file = "tests/bin/8080PRE.COM";
    let mut cpu = rust_8080::Cpu::from_filename(&file).unwrap();

    loop {
        cpu.cycle();
    }
}
