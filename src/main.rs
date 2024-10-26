use std::env;
use std::fmt;
use std::fs;

enum Opcode {
    Mov = 0b100010,
    Invalid,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Opcode::Mov => "mov",
            Opcode::Invalid => "invalid opcode",
        };

        write!(f, "{}", value)
    }
}

enum Register {
    AL, // 000
    CL, // 001
    DL, // 010
    BL, // 011
    AH, // 100
    CH, // 101
    DH, // 110
    BH, // 111
    AX, // 000
    CX, // 001
    DX, // 010
    BX, // 011
    SP, // 100
    BP, // 101
    SI, // 110
    DI, // 111
    Invalid,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Register::AL => "al", // 000
            Register::CL => "cl", // 001
            Register::DL => "dl", // 010
            Register::BL => "bl", // 011
            Register::AH => "ah", // 100
            Register::CH => "ch", // 101
            Register::DH => "dh", // 110
            Register::BH => "bh", // 111
            Register::AX => "ax", // 000
            Register::CX => "cx", // 001
            Register::DX => "dx", // 010
            Register::BX => "bx", // 011
            Register::SP => "sp", // 100
            Register::BP => "bp", // 101
            Register::SI => "si", // 110
            Register::DI => "di", // 111
            Register::Invalid => "invalid register",
        };

        write!(f, "{}", value)
    }
}

fn which_reg(w: u8, reg: u8) -> Register {
    match (w, reg) {
        (0, 0b000) => Register::AL, // 000
        (0, 0b001) => Register::CL, // 001
        (0, 0b010) => Register::DL, // 010
        (0, 0b011) => Register::BL, // 011
        (0, 0b100) => Register::AH, // 100
        (0, 0b101) => Register::CH, // 101
        (0, 0b110) => Register::DH, // 110
        (0, 0b111) => Register::BH, // 111
        (1, 0b000) => Register::AX, // 000
        (1, 0b001) => Register::CX, // 001
        (1, 0b010) => Register::DX, // 010
        (1, 0b011) => Register::BX, // 011
        (1, 0b100) => Register::SP, // 100
        (1, 0b101) => Register::BP, // 101
        (1, 0b110) => Register::SI, // 110
        (1, 0b111) => Register::DI, // 111
        (_, _) => Register::Invalid,
    }
}

fn which_op(opcode: u8) -> Opcode {
    match opcode {
        0b100010 => Opcode::Mov,
        _ => Opcode::Invalid,
    }
}

/*
 * 137  -> 10001001  -> 100010|d|w  -> 100010 (mov) | d = 0     | w = 1
 * 217  -> 11011001  -> mod|reg|r/m -> mod = 11     | reg = 011 | r/m = 001
 */
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Expected 1 argument, found {}", args.len() - 1);
        return;
    }

    let file_path: &str = &args[1].as_str();
    println!("; Read from the binary\n");

    let contents: Vec<u8> = fs::read(&file_path).expect("Error reading file");
    let processed: String = process(contents);

    println!("{}", processed);
}

fn process(contents: Vec<u8>) -> String {
    let mut output: String = String::from("bits 16\n\n");

    for instruction in contents.chunks(2) {
        let opcode_field = (instruction[0] >> 2) & 0b111111;
        let direction_field = (instruction[0] >> 1) & 0b1;
        let word_byte_field = instruction[0] & 0b1;
        let _mode_field = (instruction[1] >> 6) & 0b11;
        let register_field = (instruction[1] >> 3) & 0b111;
        let rm_field = instruction[1] & 0b111;

        let op = which_op(opcode_field);
        let reg = which_reg(word_byte_field, register_field);
        let rm = which_reg(word_byte_field, rm_field);

        let line: String = match direction_field {
            1 => format!("{} {}, {}\n", op, reg, rm),
            0 => format!("{} {}, {}\n", op, rm, reg),
            _ => format!("; invalid instruction\n"),
        };

        output.push_str(line.as_str());
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize_asm(content: &str) -> String {
        content
            .split("\n")
            .filter(|line| !line.starts_with(";"))
            .filter(|line| !line.chars().all(|c| c.is_whitespace()))
            .collect::<Vec<&str>>()
            .join("\n")
    }

    fn compare(asm_file: &str, bin_file: &str) {
        let original_asm_content: String =
            fs::read_to_string(asm_file).expect("Error reading file");
        let original_asm_content_normalized: String = normalize_asm(&original_asm_content);

        let bin_to_asm_content: String = process(fs::read(bin_file).expect("Error reading file"));
        let bin_to_asm_content_normalized: String = normalize_asm(&bin_to_asm_content);

        assert_eq!(
            original_asm_content_normalized,
            bin_to_asm_content_normalized
        );
    }

    #[test]
    fn listing_0037_single_register_mov() {
        const ASM_FILE_PATH: &str = "./vendor/listing_0037_single_register_mov.asm";
        const BIN_FILE_PATH: &str = "./vendor/listing_0037_single_register_mov";

        compare(ASM_FILE_PATH, BIN_FILE_PATH)
    }

    #[test]
    fn listing_0038_single_register_mov() {
        const ASM_FILE_PATH: &str = "./vendor/listing_0038_many_register_mov.asm";
        const BIN_FILE_PATH: &str = "./vendor/listing_0038_many_register_mov";

        compare(ASM_FILE_PATH, BIN_FILE_PATH)
    }
}
