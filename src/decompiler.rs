use bitmatch::bitmatch;

#[bitmatch]
pub fn instr(opcode: &[u8]) -> String {
    #[bitmatch]
    match opcode[0] {
        "0000_0000" => "NOP".to_string(),
        "1100_0011" => format!("JMP\t{}", addr(opcode)),
        "11cc_c010" => format!("{}\t{}", cond_jmp(c), addr(opcode)),
        "1100_1101" => format!("CALL\t{}", addr(opcode)),
        "1100_1001" => "RET".to_string(),
        // ports
        "1101_1011" => format!("IN\t{}", opcode[1]),
        "1101_0011" => format!("OUT\t{}", opcode[1]),
        // register
        "00rr_r101" => format!("DCR\t{}", reg(r)),
        "00rr_r100" => format!("INR\t{}", reg(r)),
        "1111_1110" => format!("CPI\t{:#04x}", opcode[1]),
        "00rr_r110" => format!("MVI\t{}\t{:#04x}", reg(r), opcode[1]),
        // register pair
        "1111_1001" => format!("SPHL"),
        "00rr_0001" => format!("LXI\t{}\t{}", regpair(r), d16(opcode)),
        "0011_1010" => format!("LDA\t{}", addr(opcode)),
        "0011_0010" => format!("STA\t{}", addr(opcode)),
        "0010_1010" => format!("LHLD\t{}", addr(opcode)),
        "0010_0010" => format!("SHLD\t{}", addr(opcode)),
        "00rr_1010" => format!("LDAX\t{}", regpair(r)),
        "00rr_0010" => format!("STAX\t{}", regpair(r)),
        "00rr_1011" => format!("DCR\t{}", regpair(r)),
        "00rr_0011" => format!("INX\t{}", regpair(r)),
        "11rr_0101" => format!("PUSH\t{}", regpair(r)),
        "11rr_0001" => format!("POP\t{}", regpair(r)),
        // other
        "0111_0110" => format!("HALT"),
        "01aa_abbb" => format!("MOV\t{}\t{}", reg(a), reg(b)),
        "aaaa_aaaa" => panic!("Instruction {0:#010b} {0:#04x} is not implemented", a),
    }
}

fn cond_jmp(cond: u8) -> String {
    let res = match cond {
        0b000 => "JNZ",
        0b001 => "JZ",
        0b010 => "JNC",
        0b011 => "JC",
        0b100 => "JPO",
        0b101 => "JPE",
        0b110 => "JP",
        0b111 => "JM",
        c => panic!("cond_jmp called with invalid value: {:b}", c),
    };
    res.to_string()
}

fn addr(opcode: &[u8]) -> String {
    format!("\x1B[1;35m{:#04x}{:02x}\x1B[m", opcode[2], opcode[1])
}

fn d16(opcode: &[u8]) -> String {
    format!("\x1B[1;33m#${:#04x}{:02x}\x1B[m", opcode[2], opcode[1])
}

fn reg(r: u8) -> String {
    let r = match r {
        0x00 => "%B",
        0x01 => "%C",
        0x02 => "%D",
        0x03 => "%E",
        0x04 => "%H",
        0x05 => "%L",
        0x06 => "%M",
        0x07 => "%A",
        _ => panic!("Failed to decompile register {}", r),
    };
    format!("\x1B[1;32m{}\x1B[m", r)
}

fn regpair(r: u8) -> String {
    let r = match r {
        0x00 => "%%BC",
        0x01 => "%%DE",
        0x02 => "%%HL",
        0x03 => "SP",
        _ => panic!("Failed to decompile register pair {}", r),
    };
    format!("\x1B[1;36m{}\x1B[m", r)
}
