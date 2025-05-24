use crate::keyboard::KeyboardKey;
use std::any::Any;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct PressResult {
    pub handled: bool,
    pub focus: bool,
}

pub trait MenuItem: Any {
    fn get_label(&self, is_focused: bool) -> String;
    fn press(&mut self, key: &KeyboardKey, is_focused: bool) -> PressResult;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
