use crate::decoder::constants::*;
use crate::decoder::functions::*;
use crate::decoder::utils::*;

pub(crate) fn process_bin(contents: &Vec<u8>) -> String {
    let mut output: String = String::from("bits 16\n\n");
    let mut contents_iterator = contents.iter().peekable();
    // for instruction in contents.chunks(2) {
    while let Some(instruction) = contents_iterator.next() {
        let (opcode, opcode_instruction, _, _): (Op, Instruction, Option<u8>, Option<u8>) =
            match instruction {
                0b10000000..=0b10000011 => {
                    let next_instruction = contents_iterator.peek().unwrap();
                    let mode_field = (*next_instruction >> 6) & 0b11;
                    let opcode_field = (*next_instruction >> 3) & 0b111;
                    let rm_field = *next_instruction & 0b111;

                    match opcode_field {
                        0b000 => (
                            Op::Add,
                            Instruction::AddImmediateToRegisterMemory,
                            Some(mode_field),
                            Some(rm_field),
                        ),
                        0b101 => (
                            Op::Sub,
                            Instruction::SubImmediateFromRegisterMemory,
                            Some(mode_field),
                            Some(rm_field),
                        ),
                        0b111 => (
                            Op::Cmp,
                            Instruction::CmpImmediateWithRegisterMemory,
                            Some(mode_field),
                            Some(rm_field),
                        ),
                        _ => (Op::Invalid, Instruction::Invalid, None, None),
                    }
                }
                0b01110100 => (Op::Je, Instruction::JumpOnEqual, None, None),
                0b01111100 => (Op::Jl, Instruction::JumpOnLess, None, None),
                0b01111110 => (Op::Jle, Instruction::JumpOnLessOrEqual, None, None),
                0b01110010 => (Op::Jb, Instruction::JumpOnBelow, None, None),
                0b01110110 => (Op::Jbe, Instruction::JumpOnBelowOrEqual, None, None),
                0b01111010 => (Op::Jp, Instruction::JumpOnParity, None, None),
                0b01110000 => (Op::Jo, Instruction::JumpOnOverflow, None, None),
                0b01111000 => (Op::Js, Instruction::JumpOnSign, None, None),
                0b01110101 => (Op::Jne, Instruction::JumpOnNotEqual, None, None),
                0b01111101 => (Op::Jnl, Instruction::JumpOnNotLess, None, None),
                0b01111111 => (Op::Jg, Instruction::JumpOnGreater, None, None),
                0b01110011 => (Op::Jnb, Instruction::JumpOnNotBelow, None, None),
                0b01110111 => (Op::Ja, Instruction::JumpOnAbove, None, None),
                0b01111011 => (Op::Jnp, Instruction::JumpOnNotPar, None, None),
                0b01110001 => (Op::Jno, Instruction::JumpOnNotOverflow, None, None),
                0b01111001 => (Op::Jns, Instruction::JumpOnNotSign, None, None),
                0b11100010 => (Op::Loop, Instruction::LoopCxTimes, None, None),
                0b11100001 => (Op::Loopz, Instruction::LoopWhileZero, None, None),
                0b11100000 => (Op::Loopnz, Instruction::LoopWhileNotZero, None, None),
                0b11100011 => (Op::Jcxz, Instruction::JumpOnCxZero, None, None),
                _ => {
                    let (opcode, opcode_instruction) = which_instruction(instruction);

                    (opcode, opcode_instruction, None, None)
                }
            };

        match (opcode, opcode_instruction) {
            (Op::Mov, Instruction::MovRegisterMemoryToFromRegister) => {
                register_memory_to_from_register(
                    Op::Mov,
                    &mut contents_iterator,
                    instruction,
                    &mut output,
                );
            }
            (Op::Mov, Instruction::MovImmediateToRegisterMemory) => {
                immediate_to_register_memory(
                    Op::Mov,
                    &mut contents_iterator,
                    instruction,
                    &mut output,
                );
            }
            (Op::Mov, Instruction::MovImmediateToRegister) => {
                immediate_to_register(Op::Mov, &mut contents_iterator, instruction, &mut output);
            }
            (Op::Mov, Instruction::MovMemoryToAccumulator) => {
                memory_to_accumulator(Op::Mov, &mut contents_iterator, &mut output);
            }
            (Op::Mov, Instruction::MovAccumulatorToMemory) => {
                accumulator_to_memory(Op::Mov, &mut contents_iterator, &mut output);
            }
            (Op::Mov, Instruction::MovRegisterMemoryToSegmentRegister) => todo!(),
            (Op::Mov, Instruction::MovSegmentRegisterToRegisterMemory) => todo!(),
            (Op::Add, Instruction::AddRegisterMemoryWithRegisterToEither) => {
                register_memory_to_from_register(
                    Op::Add,
                    &mut contents_iterator,
                    instruction,
                    &mut output,
                );
            }
            (Op::Add, Instruction::AddImmediateToRegisterMemory) => {
                immediate_to_register_memory(
                    Op::Add,
                    &mut contents_iterator,
                    instruction,
                    &mut output,
                );
            }
            (Op::Add, Instruction::AddImmediateToAccumulator) => {
                immediate_to_accumulator(Op::Add, &mut contents_iterator, instruction, &mut output);
            }
            (Op::Sub, Instruction::SubRegisterMemoryAndRegisterToEither) => {
                register_memory_to_from_register(
                    Op::Sub,
                    &mut contents_iterator,
                    instruction,
                    &mut output,
                );
            }
            (Op::Sub, Instruction::SubImmediateFromRegisterMemory) => {
                immediate_to_register_memory(
                    Op::Sub,
                    &mut contents_iterator,
                    instruction,
                    &mut output,
                );
            }
            (Op::Sub, Instruction::SubImmediateFromAccumulator) => {
                immediate_to_accumulator(Op::Sub, &mut contents_iterator, instruction, &mut output);
            }
            (Op::Cmp, Instruction::CmpRegisterMemoryAndRegister) => {
                register_memory_to_from_register(
                    Op::Cmp,
                    &mut contents_iterator,
                    instruction,
                    &mut output,
                );
            }
            (Op::Cmp, Instruction::CmpImmediateWithRegisterMemory) => {
                immediate_to_register_memory(
                    Op::Cmp,
                    &mut contents_iterator,
                    instruction,
                    &mut output,
                );
            }
            (Op::Cmp, Instruction::CmpImmediateWithAccumulator) => {
                immediate_to_accumulator(Op::Cmp, &mut contents_iterator, instruction, &mut output);
            }
            (Op::Je, Instruction::JumpOnEqual) => {
                handle_jumps(Op::Je, &mut contents_iterator, &mut output);
            }
            (Op::Jl, Instruction::JumpOnLess) => {
                handle_jumps(Op::Jl, &mut contents_iterator, &mut output);
            }
            (Op::Jle, Instruction::JumpOnLessOrEqual) => {
                handle_jumps(Op::Jle, &mut contents_iterator, &mut output);
            }
            (Op::Jb, Instruction::JumpOnBelow) => {
                handle_jumps(Op::Jb, &mut contents_iterator, &mut output);
            }
            (Op::Jbe, Instruction::JumpOnBelowOrEqual) => {
                handle_jumps(Op::Jbe, &mut contents_iterator, &mut output);
            }
            (Op::Jp, Instruction::JumpOnParity) => {
                handle_jumps(Op::Jp, &mut contents_iterator, &mut output);
            }
            (Op::Jo, Instruction::JumpOnOverflow) => {
                handle_jumps(Op::Jo, &mut contents_iterator, &mut output);
            }
            (Op::Js, Instruction::JumpOnSign) => {
                handle_jumps(Op::Js, &mut contents_iterator, &mut output);
            }
            (Op::Jne, Instruction::JumpOnNotEqual) => {
                handle_jumps(Op::Jne, &mut contents_iterator, &mut output);
            }
            (Op::Jnl, Instruction::JumpOnNotLess) => {
                handle_jumps(Op::Jnl, &mut contents_iterator, &mut output);
            }
            (Op::Jg, Instruction::JumpOnGreater) => {
                handle_jumps(Op::Jg, &mut contents_iterator, &mut output);
            }
            (Op::Jnb, Instruction::JumpOnNotBelow) => {
                handle_jumps(Op::Jnb, &mut contents_iterator, &mut output);
            }
            (Op::Ja, Instruction::JumpOnAbove) => {
                handle_jumps(Op::Ja, &mut contents_iterator, &mut output);
            }
            (Op::Jnp, Instruction::JumpOnNotPar) => {
                handle_jumps(Op::Jnp, &mut contents_iterator, &mut output);
            }
            (Op::Jno, Instruction::JumpOnNotOverflow) => {
                handle_jumps(Op::Jno, &mut contents_iterator, &mut output);
            }
            (Op::Jns, Instruction::JumpOnNotSign) => {
                handle_jumps(Op::Jns, &mut contents_iterator, &mut output);
            }
            (Op::Loop, Instruction::LoopCxTimes) => {
                handle_loops(Op::Loop, &mut contents_iterator, &mut output);
            }
            (Op::Loopz, Instruction::LoopWhileZero) => {
                handle_loops(Op::Loopz, &mut contents_iterator, &mut output);
            }
            (Op::Loopnz, Instruction::LoopWhileNotZero) => {
                handle_loops(Op::Loopnz, &mut contents_iterator, &mut output);
            }
            (Op::Jcxz, Instruction::JumpOnCxZero) => {
                handle_loops(Op::Jcxz, &mut contents_iterator, &mut output);
            }
            (_, _) => (),
        }
    }

    output
}
