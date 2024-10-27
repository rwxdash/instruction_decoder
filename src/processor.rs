use std::fmt;

enum Instruction {
    RegisterMemoryToRegister,
    ImmediateToRegisterMemory,
    ImmediateToRegister,
    MemoryToAccumulator,
    AccumulatorToMemory,
    RegisterMemoryToSegmentRegister,
    SegmentRegisterToRegisterMemory,
    Invalid,
}

enum EffectiveAddressCalculation {
    AL,   // 000
    CL,   // 001
    DL,   // 010
    BL,   // 011
    AH,   // 100
    CH,   // 101
    DH,   // 110
    BH,   // 111
    AX,   // 000
    CX,   // 001
    DX,   // 010
    BX,   // 011
    SP,   // 100
    BP,   // 101
    SI,   // 110
    DI,   // 111
    BxSi, // 000
    BxDi, // 001
    BpSi, // 010
    BpDi, // 011
    Si,   // 100
    Di,   // 101
    Bp,   // 110
    Bx,   // 111
}

impl fmt::Display for EffectiveAddressCalculation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: String = match self {
            EffectiveAddressCalculation::AL => Register::AL.to_string(),
            EffectiveAddressCalculation::CL => Register::CL.to_string(),
            EffectiveAddressCalculation::DL => Register::DL.to_string(),
            EffectiveAddressCalculation::BL => Register::BL.to_string(),
            EffectiveAddressCalculation::AH => Register::AH.to_string(),
            EffectiveAddressCalculation::CH => Register::CH.to_string(),
            EffectiveAddressCalculation::DH => Register::DH.to_string(),
            EffectiveAddressCalculation::BH => Register::BH.to_string(),
            EffectiveAddressCalculation::AX => Register::AX.to_string(),
            EffectiveAddressCalculation::CX => Register::CX.to_string(),
            EffectiveAddressCalculation::DX => Register::DX.to_string(),
            EffectiveAddressCalculation::BX => Register::BX.to_string(),
            EffectiveAddressCalculation::SP => Register::SP.to_string(),
            EffectiveAddressCalculation::BP => Register::BP.to_string(),
            EffectiveAddressCalculation::SI => Register::SI.to_string(),
            EffectiveAddressCalculation::DI => Register::DI.to_string(),
            EffectiveAddressCalculation::BxSi => format!("bx + si"),
            EffectiveAddressCalculation::BxDi => format!("bx + di"),
            EffectiveAddressCalculation::BpSi => format!("bp + si"),
            EffectiveAddressCalculation::BpDi => format!("bp + di"),
            EffectiveAddressCalculation::Si => format!("si"),
            EffectiveAddressCalculation::Di => format!("di"),
            EffectiveAddressCalculation::Bp => format!("bp"),
            EffectiveAddressCalculation::Bx => format!("bx"),
        };

        write!(f, "{}", value)
    }
}

enum Op {
    Mov,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Op::Mov => "mov",
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
        };

        write!(f, "{}", value)
    }
}

fn which_instruction(byte: &u8) -> Instruction {
    match byte {
        0b10001000..=0b10001011 => Instruction::RegisterMemoryToRegister,
        0b11000110..=0b11000111 => Instruction::ImmediateToRegisterMemory,
        0b10110000..=0b10111111 => Instruction::ImmediateToRegister,
        0b10100000..=0b10100001 => Instruction::MemoryToAccumulator,
        0b10100010..=0b10100011 => Instruction::AccumulatorToMemory,
        0b10001110 => Instruction::RegisterMemoryToSegmentRegister,
        0b10001100 => Instruction::SegmentRegisterToRegisterMemory,
        _ => Instruction::Invalid,
    }
}

fn which_address(
    mode_field: &u8,
    word_byte_field: &u8,
    rm_field: &u8,
) -> Option<EffectiveAddressCalculation> {
    let address: Option<EffectiveAddressCalculation> = match (mode_field, word_byte_field, rm_field)
    {
        (0b11, 0b0, 0b000) => Some(EffectiveAddressCalculation::AL),
        (0b11, 0b0, 0b001) => Some(EffectiveAddressCalculation::CL),
        (0b11, 0b0, 0b010) => Some(EffectiveAddressCalculation::DL),
        (0b11, 0b0, 0b011) => Some(EffectiveAddressCalculation::BL),
        (0b11, 0b0, 0b100) => Some(EffectiveAddressCalculation::AH),
        (0b11, 0b0, 0b101) => Some(EffectiveAddressCalculation::CH),
        (0b11, 0b0, 0b110) => Some(EffectiveAddressCalculation::DH),
        (0b11, 0b0, 0b111) => Some(EffectiveAddressCalculation::BH),
        (0b11, 0b1, 0b000) => Some(EffectiveAddressCalculation::AX),
        (0b11, 0b1, 0b001) => Some(EffectiveAddressCalculation::CX),
        (0b11, 0b1, 0b010) => Some(EffectiveAddressCalculation::DX),
        (0b11, 0b1, 0b011) => Some(EffectiveAddressCalculation::BX),
        (0b11, 0b1, 0b100) => Some(EffectiveAddressCalculation::SP),
        (0b11, 0b1, 0b101) => Some(EffectiveAddressCalculation::BP),
        (0b11, 0b1, 0b110) => Some(EffectiveAddressCalculation::SI),
        (0b11, 0b1, 0b111) => Some(EffectiveAddressCalculation::DI),
        (_, _, 0b000) => Some(EffectiveAddressCalculation::BxSi),
        (_, _, 0b001) => Some(EffectiveAddressCalculation::BxDi),
        (_, _, 0b010) => Some(EffectiveAddressCalculation::BpSi),
        (_, _, 0b011) => Some(EffectiveAddressCalculation::BpDi),
        (_, _, 0b100) => Some(EffectiveAddressCalculation::Si),
        (_, _, 0b101) => Some(EffectiveAddressCalculation::Di),
        (_, _, 0b110) => {
            if *mode_field == 0b00 {
                None // Direct Address
            } else {
                Some(EffectiveAddressCalculation::Bp)
            }
        }
        (_, _, 0b111) => Some(EffectiveAddressCalculation::Bx),
        _ => panic!("Invalid rm_field: {:b}", rm_field),
    };

    address
}

fn which_displacement(rm_field: &u8, mode_field: &u8) -> u8 {
    let displacement: u8 = match mode_field {
        0b00 => {
            if *rm_field == 0b110 {
                2
            } else {
                0
            }
        }
        0b01 => 1,
        0b10 => 2,
        0b11 => 0,
        _ => panic!("Invalid mode_field: {:b}", mode_field),
    };

    displacement
}

pub(crate) fn process_bin(contents: &Vec<u8>) -> String {
    let mut output: String = String::from("bits 16\n\n");
    let mut contents_iterator = contents.iter();
    // for instruction in contents.chunks(2) {
    while let Some(instruction) = contents_iterator.next() {
        let opcode_field: Instruction = which_instruction(instruction);

        match opcode_field {
            Instruction::RegisterMemoryToRegister => {
                let rm: String;

                let next_instruction = contents_iterator.next().unwrap();
                let direction_field = (instruction >> 1) & 0b1;
                let word_byte_field = instruction & 0b1;
                let mode_field = (next_instruction >> 6) & 0b11;
                let register_field = (next_instruction >> 3) & 0b111;
                let rm_field = next_instruction & 0b111;
                // let _address = which_address(&mode_field, &word_byte_field, &rm_field);
                let displacement = which_displacement(&rm_field, &mode_field);

                let reg = which_address(&mode_field, &word_byte_field, &register_field)
                    .unwrap()
                    .to_string();

                let rm_address_calculation =
                    which_address(&mode_field, &word_byte_field, &rm_field);

                if mode_field != 0b11 {
                    rm = match displacement {
                        1 => {
                            let displacement_low = *contents_iterator.next().unwrap();
                            let displacement = u8::from_le_bytes([displacement_low]);

                            if let Some(rm_address_calculation) = rm_address_calculation {
                                format!("[{} + {}]", rm_address_calculation, displacement)
                            } else {
                                format!("[{}]", displacement)
                            }
                        }
                        2 => {
                            let displacement_low = *contents_iterator.next().unwrap();
                            let displacement_high = *contents_iterator.next().unwrap();
                            let displacement =
                                u16::from_le_bytes([displacement_low, displacement_high]);

                            format!("[{} + {}]", rm_address_calculation.unwrap(), displacement)
                        }
                        _ => format!("[{}]", rm_address_calculation.unwrap()),
                    };
                } else {
                    rm = format!("{}", rm_address_calculation.unwrap());
                }

                let (destination, source) = if direction_field == 1 {
                    (reg, rm)
                } else {
                    (rm, reg)
                };

                let line: Option<String> = match displacement {
                    0 | 1 | 2 => Some(format!("{} {}, {}\n", Op::Mov, destination, source)),
                    _ => None,
                };

                if line.is_some() {
                    output.push_str(line.unwrap().as_str());
                };

                ()
            }
            Instruction::ImmediateToRegisterMemory => todo!(),
            Instruction::ImmediateToRegister => todo!(),
            Instruction::MemoryToAccumulator => todo!(),
            Instruction::AccumulatorToMemory => todo!(),
            Instruction::RegisterMemoryToSegmentRegister => todo!(),
            Instruction::SegmentRegisterToRegisterMemory => todo!(),
            Instruction::Invalid => (), // panic!("Invalid instruction byte: {:b}", byte),
        }
    }

    output
}
