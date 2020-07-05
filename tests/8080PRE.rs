#![allow(non_snake_case)]

mod init;

#[test]
fn main() {
    let file = "tests/bin/8080PRE.COM";
    let mut cpu = init::cpu(file);

    let mut cpt = 0;

    loop {
        cpt += 1;
        cpu.cycle();

        if cpt == 1061 {
            panic!();
        }
    }
}
