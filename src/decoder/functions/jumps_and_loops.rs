use crate::decoder::constants::Op;
use std::{iter::Peekable, slice::Iter};

pub(crate) fn handle_jumps(
    operation: Op,
    contents_iterator: &mut Peekable<Iter<'_, u8>>,
    output: &mut String,
) {
    let data_field_first = *contents_iterator.next().unwrap();
    let data = i8::from_le_bytes([data_field_first]);

    output.push_str(format!("{} $+{}\n", operation, data).as_str());
}

pub(crate) fn handle_loops(
    operation: Op,
    contents_iterator: &mut Peekable<Iter<'_, u8>>,
    output: &mut String,
) {
    let data_field_first = *contents_iterator.next().unwrap();
    let data = i8::from_le_bytes([data_field_first]);

    output.push_str(format!("{} $+{}\n", operation, data).as_str());
}
