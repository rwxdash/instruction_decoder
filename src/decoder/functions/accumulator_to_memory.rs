use crate::decoder::{constants::EffectiveAddressCalculation, constants::Op};
use std::{iter::Peekable, slice::Iter};

pub(crate) fn accumulator_to_memory(
    operation: Op,
    contents_iterator: &mut Peekable<Iter<'_, u8>>,
    output: &mut String,
) {
    let data_field_first = *contents_iterator.next().unwrap();
    let data_field_second = *contents_iterator.next().unwrap();
    let data = i16::from_le_bytes([data_field_first, data_field_second]);
    output.push_str(
        format!(
            "{} [{}], {}\n",
            operation,
            data,
            EffectiveAddressCalculation::AX
        )
        .as_str(),
    );

    ()
}
