fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .expect("Provide a file to load");
    let mut cpu = rust_8080::Cpu::from_filename_at(&file, 0x100).unwrap();

    loop {
        cpu.cycle();
    }
}
