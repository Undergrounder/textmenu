use core::option::Option;

extern crate alloc;
use alloc::vec::Vec;

use core::cmp::PartialEq;
use core::prelude::rust_2024::derive;

#[derive(PartialEq)]
pub enum FunctionKey {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    ENTER,
    BACK,
}

pub struct KeyboardKey {
    pub function_key: Option<FunctionKey>,
    pub char_input: Option<Vec<u8>>,
}

impl KeyboardKey {
    pub fn new(function_key: Option<FunctionKey>, char_input: Option<Vec<u8>>) -> KeyboardKey {
        KeyboardKey {
            function_key,
            char_input,
        }
    }
}
