use crate::decoder::constants::*;

pub(crate) fn which_instruction(byte: &u8) -> (Op, Instruction) {
    match byte {
        // MOV
        0b10001000..=0b10001011 => (Op::Mov, Instruction::MovRegisterMemoryToFromRegister),
        0b11000110..=0b11000111 => (Op::Mov, Instruction::MovImmediateToRegisterMemory),
        0b10110000..=0b10111111 => (Op::Mov, Instruction::MovImmediateToRegister),
        0b10100000..=0b10100001 => (Op::Mov, Instruction::MovMemoryToAccumulator),
        0b10100010..=0b10100011 => (Op::Mov, Instruction::MovAccumulatorToMemory),
        0b10001110 => (Op::Mov, Instruction::MovRegisterMemoryToSegmentRegister),
        0b10001100 => (Op::Mov, Instruction::MovSegmentRegisterToRegisterMemory),

        // ADD
        0b00000000..=0b00000011 => (Op::Add, Instruction::AddRegisterMemoryWithRegisterToEither),
        0b00000100..=0b00000101 => (Op::Add, Instruction::AddImmediateToAccumulator),

        // SUB
        0b00101000..=0b00101011 => (Op::Sub, Instruction::SubRegisterMemoryAndRegisterToEither),
        0b00101100..=0b00101101 => (Op::Sub, Instruction::SubImmediateFromAccumulator),

        // CMP
        0b00111000..=0b00111011 => (Op::Cmp, Instruction::CmpRegisterMemoryAndRegister),
        0b00111100..=0b00111101 => (Op::Cmp, Instruction::CmpImmediateWithAccumulator),

        _ => (Op::Invalid, Instruction::Invalid),
    }
}

pub(crate) fn which_address(
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

pub(crate) fn which_displacement(rm_field: &u8, mode_field: &u8) -> u8 {
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
