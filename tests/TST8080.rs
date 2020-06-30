#[test]
fn main() {
    let file = "tests/bin/TST8080.COM";
    let mut cpu = rust_8080::Cpu::from_filename(&file).unwrap();

    loop {
        cpu.cycle();
    }
}
