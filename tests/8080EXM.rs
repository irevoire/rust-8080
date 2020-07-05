#![allow(non_snake_case)]

mod init;

#[test]
fn main() {
    let file = "tests/bin/8080EXM.COM";
    let mut cpu = init::cpu(file);

    loop {
        cpu.cycle();
    }
}
