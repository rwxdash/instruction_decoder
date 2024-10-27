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
        let value = match self {
            EffectiveAddressCalculation::AL => "al",
            EffectiveAddressCalculation::CL => "cl",
            EffectiveAddressCalculation::DL => "dl",
            EffectiveAddressCalculation::BL => "bl",
            EffectiveAddressCalculation::AH => "ah",
            EffectiveAddressCalculation::CH => "ch",
            EffectiveAddressCalculation::DH => "dh",
            EffectiveAddressCalculation::BH => "bh",
            EffectiveAddressCalculation::AX => "ax",
            EffectiveAddressCalculation::CX => "cx",
            EffectiveAddressCalculation::DX => "dx",
            EffectiveAddressCalculation::BX => "bx",
            EffectiveAddressCalculation::SP => "sp",
            EffectiveAddressCalculation::BP => "bp",
            EffectiveAddressCalculation::SI => "si",
            EffectiveAddressCalculation::DI => "di",
            EffectiveAddressCalculation::BxSi => "bx + si",
            EffectiveAddressCalculation::BxDi => "bx + di",
            EffectiveAddressCalculation::BpSi => "bp + si",
            EffectiveAddressCalculation::BpDi => "bp + di",
            EffectiveAddressCalculation::Si => "si",
            EffectiveAddressCalculation::Di => "di",
            EffectiveAddressCalculation::Bp => "bp",
            EffectiveAddressCalculation::Bx => "bx",
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
    register_only: &bool,
) -> Option<EffectiveAddressCalculation> {
    let address: Option<EffectiveAddressCalculation> =
        match (mode_field, word_byte_field, rm_field, register_only) {
            // In Instruction::ImmediateToRegister, mode_field is not present.
            // So, Register (not R/Ms) arms should support the pattern where
            // mode_field is unavailable (ie. not 0b11) but expected results is a register.
            (_, 0b0, 0b000, true) | (0b11, 0b0, 0b000, _) => Some(EffectiveAddressCalculation::AL),
            (_, 0b0, 0b001, true) | (0b11, 0b0, 0b001, _) => Some(EffectiveAddressCalculation::CL),
            (_, 0b0, 0b010, true) | (0b11, 0b0, 0b010, _) => Some(EffectiveAddressCalculation::DL),
            (_, 0b0, 0b011, true) | (0b11, 0b0, 0b011, _) => Some(EffectiveAddressCalculation::BL),
            (_, 0b0, 0b100, true) | (0b11, 0b0, 0b100, _) => Some(EffectiveAddressCalculation::AH),
            (_, 0b0, 0b101, true) | (0b11, 0b0, 0b101, _) => Some(EffectiveAddressCalculation::CH),
            (_, 0b0, 0b110, true) | (0b11, 0b0, 0b110, _) => Some(EffectiveAddressCalculation::DH),
            (_, 0b0, 0b111, true) | (0b11, 0b0, 0b111, _) => Some(EffectiveAddressCalculation::BH),
            (_, 0b1, 0b000, true) | (0b11, 0b1, 0b000, _) => Some(EffectiveAddressCalculation::AX),
            (_, 0b1, 0b001, true) | (0b11, 0b1, 0b001, _) => Some(EffectiveAddressCalculation::CX),
            (_, 0b1, 0b010, true) | (0b11, 0b1, 0b010, _) => Some(EffectiveAddressCalculation::DX),
            (_, 0b1, 0b011, true) | (0b11, 0b1, 0b011, _) => Some(EffectiveAddressCalculation::BX),
            (_, 0b1, 0b100, true) | (0b11, 0b1, 0b100, _) => Some(EffectiveAddressCalculation::SP),
            (_, 0b1, 0b101, true) | (0b11, 0b1, 0b101, _) => Some(EffectiveAddressCalculation::BP),
            (_, 0b1, 0b110, true) | (0b11, 0b1, 0b110, _) => Some(EffectiveAddressCalculation::SI),
            (_, 0b1, 0b111, true) | (0b11, 0b1, 0b111, _) => Some(EffectiveAddressCalculation::DI),
            (_, _, 0b000, false) => Some(EffectiveAddressCalculation::BxSi),
            (_, _, 0b001, false) => Some(EffectiveAddressCalculation::BxDi),
            (_, _, 0b010, false) => Some(EffectiveAddressCalculation::BpSi),
            (_, _, 0b011, false) => Some(EffectiveAddressCalculation::BpDi),
            (_, _, 0b100, false) => Some(EffectiveAddressCalculation::Si),
            (_, _, 0b101, false) => Some(EffectiveAddressCalculation::Di),
            (_, _, 0b110, false) => {
                if *mode_field == 0b00 {
                    None // Direct Address
                } else {
                    Some(EffectiveAddressCalculation::Bp)
                }
            }
            (_, _, 0b111, false) => Some(EffectiveAddressCalculation::Bx),
            _ => panic!(
                "Invalid {:08b}, {:08b}, {:08b}, {:}",
                mode_field, word_byte_field, rm_field, register_only
            ),
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
                let displacement = which_displacement(&rm_field, &mode_field);

                let reg = which_address(&mode_field, &word_byte_field, &register_field, &true)
                    .unwrap()
                    .to_string();

                let rm_address_calculation =
                    which_address(&mode_field, &word_byte_field, &rm_field, &false);

                if mode_field != 0b11 {
                    rm = match displacement {
                        1 => {
                            let displacement_low = *contents_iterator.next().unwrap();
                            let displacement = i8::from_le_bytes([displacement_low]);

                            if let Some(rm_address_calculation) = rm_address_calculation {
                                if displacement.is_negative() {
                                    format!("[{} - {}]", rm_address_calculation, -displacement)
                                } else if displacement.is_positive() {
                                    format!("[{} + {}]", rm_address_calculation, displacement)
                                } else {
                                    format!("[{}]", rm_address_calculation)
                                }
                            } else {
                                format!("[{}]", displacement)
                            }
                        }
                        2 => {
                            let displacement_low = *contents_iterator.next().unwrap();
                            let displacement_high = *contents_iterator.next().unwrap();
                            let displacement =
                                i16::from_le_bytes([displacement_low, displacement_high]);

                            if let Some(rm_address_calculation) = rm_address_calculation {
                                if displacement.is_negative() {
                                    format!("[{} - {}]", rm_address_calculation, -displacement)
                                } else {
                                    format!("[{} + {}]", rm_address_calculation, displacement)
                                }
                            } else {
                                format!("[{}]", displacement)
                            }
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
            Instruction::ImmediateToRegisterMemory => {
                let rm: String;

                let next_instruction = *contents_iterator.next().unwrap();
                let word_byte_field = instruction & 0b1;
                let mode_field = (next_instruction >> 6) & 0b111;
                let rm_field = next_instruction & 0b111;
                let _register_field: u8 = 0b000;

                let displacement = which_displacement(&rm_field, &mode_field);

                let rm_address_calculation =
                    which_address(&mode_field, &word_byte_field, &rm_field, &false);

                if mode_field != 0b11 {
                    rm = match displacement {
                        1 => {
                            let displacement_low = *contents_iterator.next().unwrap();
                            let displacement = i8::from_le_bytes([displacement_low]);

                            println!("{}", displacement);
                            if let Some(rm_address_calculation) = rm_address_calculation {
                                if displacement.is_negative() {
                                    format!("[{} - {}]", rm_address_calculation, -displacement)
                                } else if displacement.is_positive() {
                                    format!("[{} + {}]", rm_address_calculation, displacement)
                                } else {
                                    format!("[{}]", rm_address_calculation)
                                }
                            } else {
                                format!("[{}]", displacement)
                            }
                        }
                        2 => {
                            let displacement_low = *contents_iterator.next().unwrap();
                            let displacement_high = *contents_iterator.next().unwrap();
                            let displacement =
                                i16::from_le_bytes([displacement_low, displacement_high]);

                            if let Some(rm_address_calculation) = rm_address_calculation {
                                if displacement.is_negative() {
                                    format!("[{} - {}]", rm_address_calculation, -displacement)
                                } else {
                                    format!("[{} + {}]", rm_address_calculation, displacement)
                                }
                            } else {
                                format!("[{}]", displacement)
                            }
                        }
                        _ => format!("[{}]", rm_address_calculation.unwrap()),
                    };
                } else {
                    rm = format!("{}", rm_address_calculation.unwrap());
                }

                if word_byte_field == 0b1 {
                    let data_field_first = *contents_iterator.next().unwrap();
                    let data_field_second = *contents_iterator.next().unwrap();
                    let data = i16::from_le_bytes([data_field_first, data_field_second]);
                    output.push_str(format!("{} {}, word {}\n", Op::Mov, rm, data).as_str())
                } else {
                    let data_field_first = *contents_iterator.next().unwrap();
                    let data = i8::from_le_bytes([data_field_first]);
                    output.push_str(format!("{} {}, byte {}\n", Op::Mov, rm, data).as_str())
                }
            }
            Instruction::ImmediateToRegister => {
                let word_byte_field = (instruction >> 3) & 0b1;
                let register_field = instruction & 0b111;
                if word_byte_field == 0b1 {
                    let data_field_first = *contents_iterator.next().unwrap();
                    let data_field_second = *contents_iterator.next().unwrap();
                    let data = i16::from_le_bytes([data_field_first, data_field_second]);
                    let reg = which_address(&0, &word_byte_field, &register_field, &true)
                        .unwrap()
                        .to_string();
                    output.push_str(format!("{} {}, {}\n", Op::Mov, reg, data).as_str())
                } else {
                    let data_field_first = *contents_iterator.next().unwrap();
                    let data = i8::from_le_bytes([data_field_first]);
                    let reg = which_address(&0, &word_byte_field, &register_field, &true)
                        .unwrap()
                        .to_string();
                    output.push_str(format!("{} {}, {}\n", Op::Mov, reg, data).as_str())
                }
            }
            Instruction::MemoryToAccumulator => {
                let data_field_first = *contents_iterator.next().unwrap();
                let data_field_second = *contents_iterator.next().unwrap();
                let data = i16::from_le_bytes([data_field_first, data_field_second]);
                output.push_str(
                    format!(
                        "{} {}, [{}]\n",
                        Op::Mov,
                        EffectiveAddressCalculation::AX,
                        data
                    )
                    .as_str(),
                )
            }
            Instruction::AccumulatorToMemory => {
                let data_field_first = *contents_iterator.next().unwrap();
                let data_field_second = *contents_iterator.next().unwrap();
                let data = i16::from_le_bytes([data_field_first, data_field_second]);
                output.push_str(
                    format!(
                        "{} [{}], {}\n",
                        Op::Mov,
                        data,
                        EffectiveAddressCalculation::AX
                    )
                    .as_str(),
                )
            }
            Instruction::RegisterMemoryToSegmentRegister => todo!(),
            Instruction::SegmentRegisterToRegisterMemory => todo!(),
            Instruction::Invalid => (), // panic!("Invalid instruction byte: {:b}", byte),
        }
    }

    output
}
