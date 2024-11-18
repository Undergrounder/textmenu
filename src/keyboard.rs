use crate::consts::BYTES_PER_CHAR;
use heapless::Vec;

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
    pub char_input: Option<Vec<u8, BYTES_PER_CHAR>>,
}

impl KeyboardKey {
    pub fn new(
        function_key: Option<FunctionKey>,
        char_input: Option<Vec<u8, BYTES_PER_CHAR>>,
    ) -> KeyboardKey {
        KeyboardKey {
            function_key,
            char_input,
        }
    }
}
