pub static mut FINISHED: bool = false;

pub fn cpu(file: &str) -> rust_8080::Cpu {
    let mut cpu = rust_8080::Cpu::from_filename_at(&file, 0x100).unwrap();
    // inject "out 1, a at 0x0000 (signal to stop the test)
    cpu.ram[0] = 0xD3;
    cpu.ram[1] = 0x00;

    // inject "in a,0" at 0x0005 (signal to output some characters)
    cpu.ram[5] = 0xDB;
    cpu.ram[6] = 0x00;
    cpu.ram[7] = 0xC9;

    cpu.port_in = Some(Box::new(|cpu, _| {
        let op = cpu.reg.c;

        if op == 2 {
            // print char stored in e
            eprint!("{}", cpu.reg.e as char);
        } else if op == 9 {
            // print from de until '$'
            let mut addr = cpu.reg.de() as usize;
            while cpu.ram[addr] != '$' as u8 {
                eprint!("{}", cpu.ram[addr] as char);
                addr += 1;
            }
        }

        0xff
    }));
    cpu.port_out = Some(Box::new(|_, _, _| unsafe { FINISHED = true }));
    cpu
}
