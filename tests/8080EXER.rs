#![allow(non_snake_case)]

#[test]
fn main() {
    let file = "tests/bin/8080EXER.COM";
    let mut cpu = rust_8080::Cpu::from_filename_at(&file, 0x100).unwrap();

    loop {
        cpu.cycle();
    }
}
