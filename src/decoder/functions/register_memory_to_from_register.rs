use crate::decoder::{constants::Op, utils::*};
use std::{iter::Peekable, slice::Iter};

pub(crate) fn register_memory_to_from_register(
    operation: Op,
    contents_iterator: &mut Peekable<Iter<'_, u8>>,
    instruction: &u8,
    output: &mut String,
) -> () {
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

    let rm_address_calculation = which_address(&mode_field, &word_byte_field, &rm_field, &false);

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
                let displacement = i16::from_le_bytes([displacement_low, displacement_high]);

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
        0 | 1 | 2 => Some(format!("{} {}, {}\n", operation, destination, source)),
        _ => None,
    };

    if line.is_some() {
        output.push_str(line.unwrap().as_str());
    };

    ()
}
