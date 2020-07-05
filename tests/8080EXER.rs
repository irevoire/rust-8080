#![allow(non_snake_case)]

mod init;

#[test]
fn main() {
    let file = "tests/bin/8080EXER.COM";
    let mut cpu = init::cpu(file);
    let mut instructions = 0;

    while !unsafe { init::FINISHED } {
        instructions += 1;
        cpu.cycle();
    }
    println!("in {}", instructions);
    panic!("NO!");
}
