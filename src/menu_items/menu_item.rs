use crate::keyboard::KeyboardKey;
use heapless::{String, Vec};

#[cfg(all(feature = "max_label_length_200"))]
pub const MAX_LABEL_LENGTH: usize = 200;
#[cfg(all(
    not(feature = "max_label_length_200"),
    feature = "max_label_length_100"
))]
pub const MAX_LABEL_LENGTH: usize = 100;

#[cfg(all(
    not(feature = "max_label_length_200"),
    not(feature = "max_label_length_100"),
    feature = "max_label_length_50"
))]
pub const MAX_LABEL_LENGTH: usize = 50;
pub const BYTES_PER_CHAR: usize = 4;
pub const LABEL_BYTES: usize = MAX_LABEL_LENGTH * BYTES_PER_CHAR;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct PressResult {
    pub handled: bool,
    pub focus: bool,
}

pub trait MenuItem<'a> {
    fn get_label(&self, is_focused: bool) -> String<LABEL_BYTES>;
    fn press(&mut self, key: &KeyboardKey, is_focused: bool) -> PressResult;
}
