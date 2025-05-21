use crate::keyboard::KeyboardKey;
use crate::menu_items::menu_item_kind::MenuItemKind;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct PressResult {
    pub handled: bool,
    pub focus: bool,
}

pub trait MenuItem {
    fn get_label(&self, is_focused: bool) -> String;
    fn press(&mut self, key: &KeyboardKey, is_focused: bool) -> PressResult;
    fn kind(&self) -> MenuItemKind;
}
