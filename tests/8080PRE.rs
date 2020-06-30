#![allow(non_snake_case)]

#[test]
fn main() {
    let file = "tests/bin/8080PRE.COM";
    let mut cpu = rust_8080::Cpu::from_filename_at(&file, 0x100).unwrap();

    let mut cpt = 0;

    loop {
        cpt += 1;
        cpu.cycle();

        if cpt == 1061 {
            panic!();
        }
    }
}
