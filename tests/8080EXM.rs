#[test]
fn main() {
    let file = "tests/bin/8080EXM.COM";
    let mut cpu = rust_8080::Cpu::from_filename(&file).unwrap();

    loop {
        cpu.cycle();
    }
}
