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

fn main() {
    println!("; Read from the binary\n");
    println!("bits 16\n");

    /*
     * 137  -> 10001001  -> 100010|d|w  -> 100010 (mov) | d = 0     | w = 1
     * 217  -> 11011001  -> mod|reg|r/m -> mod = 11     | reg = 011 | r/m = 001
     */
    let file_path: &str =
        "/home/oz/workspace/rust/instruction_decoder/vendor/listing_0038_many_register_mov";
    let contents: Vec<u8> = fs::read(&file_path).expect("Error reading file");
    // println!("{:b} {:b}", &contents[0], &contents[1]);

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

        let output = match direction_field {
            1 => format!("{} {}, {}", op, reg, rm),
            0 => format!("{} {}, {}", op, rm, reg),
            _ => format!("; invalid instruction"),
        };

        println!("{}", output);
    }
}
