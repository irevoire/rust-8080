#[test]
fn main() {
    let file = "tests/hex/8080EXER.HEX";
    let mut cpu = rust_8080::Cpu::from_filename(&file).unwrap();

    loop {
        cpu.cycle();
    }
}
