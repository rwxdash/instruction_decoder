use crate::decoder::constants::{EffectiveAddressCalculation, Op};
use std::{iter::Peekable, slice::Iter};

pub(crate) fn immediate_to_accumulator(
    operation: Op,
    contents_iterator: &mut Peekable<Iter<'_, u8>>,
    instruction: &u8,
    output: &mut String,
) {
    let word_byte_field = instruction & 0b1;

    if word_byte_field == 0b1 {
        let data_field_first = *contents_iterator.next().unwrap();
        let data_field_second = *contents_iterator.next().unwrap();
        let data = i16::from_le_bytes([data_field_first, data_field_second]);
        output.push_str(
            format!(
                "{} {}, {}\n",
                operation,
                EffectiveAddressCalculation::AX,
                data
            )
            .as_str(),
        );
    } else {
        let data_field_first = *contents_iterator.next().unwrap();
        let data = i8::from_le_bytes([data_field_first]);
        output.push_str(
            format!(
                "{} {}, {}\n",
                operation,
                EffectiveAddressCalculation::AL,
                data
            )
            .as_str(),
        );
    }

    ()
}
