use crate::decoder::{constants::Op, utils::*};
use std::{iter::Peekable, slice::Iter};

pub(crate) fn immediate_to_register_memory(
    operation: Op,
    contents_iterator: &mut Peekable<Iter<'_, u8>>,
    instruction: &u8,
    output: &mut String,
) {
    let rm: String;

    let next_instruction = *contents_iterator.next().unwrap();
    let sign_field = (instruction >> 1) & 0b1;
    let word_byte_field = instruction & 0b1;
    let mode_field = (next_instruction >> 6) & 0b111;
    let _register_field = (next_instruction >> 3) & 0b111;
    let rm_field = next_instruction & 0b111;

    let displacement = which_displacement(&rm_field, &mode_field);

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

    match operation {
        Op::Mov => {
            if word_byte_field == 0b1 {
                let data_field_first = *contents_iterator.next().unwrap();
                let data_field_second = *contents_iterator.next().unwrap();
                let data = i16::from_le_bytes([data_field_first, data_field_second]);
                output.push_str(format!("{} {}, word {}\n", operation, rm, data).as_str());
            } else {
                let data_field_first = *contents_iterator.next().unwrap();
                let data = i8::from_le_bytes([data_field_first]);
                output.push_str(format!("{} {}, byte {}\n", operation, rm, data).as_str());
            }
        }
        Op::Add => {
            if sign_field == 0b0 && word_byte_field == 0b1 {
                let data_field_first = *contents_iterator.next().unwrap();
                let data_field_second = *contents_iterator.next().unwrap();
                let data = i16::from_le_bytes([data_field_first, data_field_second]);
                output.push_str(format!("{} word {}, {}\n", operation, rm, data).as_str());
            } else if sign_field == 0b1 && word_byte_field == 0b1 {
                let data_field_first = *contents_iterator.next().unwrap();
                let data = i8::from_le_bytes([data_field_first]);
                output.push_str(format!("{} word {}, {}\n", operation, rm, data).as_str());
            } else {
                let data_field_first = *contents_iterator.next().unwrap();
                let data = i8::from_le_bytes([data_field_first]);
                output.push_str(format!("{} byte {}, {}\n", operation, rm, data).as_str());
            }
        }
        Op::Sub => {
            if sign_field == 0b0 && word_byte_field == 0b1 {
                let data_field_first = *contents_iterator.next().unwrap();
                let data_field_second = *contents_iterator.next().unwrap();
                let data = i16::from_le_bytes([data_field_first, data_field_second]);
                output.push_str(format!("{} word {}, {}\n", operation, rm, data).as_str());
            } else if sign_field == 0b1 && word_byte_field == 0b1 {
                let data_field_first = *contents_iterator.next().unwrap();
                let data = i8::from_le_bytes([data_field_first]);
                output.push_str(format!("{} word {}, {}\n", operation, rm, data).as_str());
            } else {
                let data_field_first = *contents_iterator.next().unwrap();
                let data = i8::from_le_bytes([data_field_first]);
                output.push_str(format!("{} byte {}, {}\n", operation, rm, data).as_str());
            }
        }
        Op::Cmp => {
            if sign_field == 0b0 && word_byte_field == 0b1 {
                let data_field_first = *contents_iterator.next().unwrap();
                let data_field_second = *contents_iterator.next().unwrap();
                let data = i16::from_le_bytes([data_field_first, data_field_second]);
                output.push_str(format!("{} word {}, {}\n", operation, rm, data).as_str());
            } else if sign_field == 0b1 && word_byte_field == 0b1 {
                let data_field_first = *contents_iterator.next().unwrap();
                let data = i8::from_le_bytes([data_field_first]);
                output.push_str(format!("{} word {}, {}\n", operation, rm, data).as_str());
            } else {
                let data_field_first = *contents_iterator.next().unwrap();
                let data = i8::from_le_bytes([data_field_first]);
                output.push_str(format!("{} byte {}, {}\n", operation, rm, data).as_str());
            }
        }
        _ => {}
    }

    ()
}
