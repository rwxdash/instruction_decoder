pub(crate) mod accumulator_to_memory;
pub(crate) mod immediate_to_accumulator;
pub(crate) mod immediate_to_register;
pub(crate) mod immediate_to_register_memory;
pub(crate) mod jumps_and_loops;
pub(crate) mod memory_to_accumulator;
pub(crate) mod register_memory_to_from_register;

pub(crate) use crate::decoder::functions::accumulator_to_memory::accumulator_to_memory;
pub(crate) use crate::decoder::functions::immediate_to_accumulator::immediate_to_accumulator;
pub(crate) use crate::decoder::functions::immediate_to_register::immediate_to_register;
pub(crate) use crate::decoder::functions::immediate_to_register_memory::immediate_to_register_memory;
pub(crate) use crate::decoder::functions::jumps_and_loops::{handle_jumps, handle_loops};
pub(crate) use crate::decoder::functions::memory_to_accumulator::memory_to_accumulator;
pub(crate) use crate::decoder::functions::register_memory_to_from_register::register_memory_to_from_register;
