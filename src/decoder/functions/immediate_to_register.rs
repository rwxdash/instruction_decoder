use crate::decoder::{constants::Op, utils::*};
use std::{iter::Peekable, slice::Iter};

pub(crate) fn immediate_to_register(
    operation: Op,
    contents_iterator: &mut Peekable<Iter<'_, u8>>,
    instruction: &u8,
    output: &mut String,
) {
    let word_byte_field = (instruction >> 3) & 0b1;
    let register_field = instruction & 0b111;
    if word_byte_field == 0b1 {
        let data_field_first = *contents_iterator.next().unwrap();
        let data_field_second = *contents_iterator.next().unwrap();
        let data = i16::from_le_bytes([data_field_first, data_field_second]);
        let reg = which_address(&0, &word_byte_field, &register_field, &true)
            .unwrap()
            .to_string();
        output.push_str(format!("{} {}, {}\n", operation, reg, data).as_str());
    } else {
        let data_field_first = *contents_iterator.next().unwrap();
        let data = i8::from_le_bytes([data_field_first]);
        let reg = which_address(&0, &word_byte_field, &register_field, &true)
            .unwrap()
            .to_string();
        output.push_str(format!("{} {}, {}\n", operation, reg, data).as_str());
    }

    ()
}
