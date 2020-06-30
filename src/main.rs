fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .expect("Provide a file to load");
    let mut cpu = rust_8080::Cpu::from_filename(&file).unwrap();

    loop {
        cpu.cycle();
    }
}
