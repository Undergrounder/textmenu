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
