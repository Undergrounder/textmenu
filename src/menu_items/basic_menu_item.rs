use crate::keyboard::KeyboardKey;
use crate::menu_items::menu_item::{MenuItem, PressResult, LABEL_BYTES};
use core::str::FromStr;
use heapless::String;
use crate::menu_items::menu_item_kind::MenuItemKind;

pub struct BasicMenuItem<'a> {
    label: &'a str,
}

impl<'a> BasicMenuItem<'a> {
    pub fn new(label: &str) -> BasicMenuItem {
        BasicMenuItem { label }
    }
}

impl<'a> MenuItem<'a> for BasicMenuItem<'a> {
    fn get_label(&self, _is_focused: bool) -> String<{ LABEL_BYTES }> {
        String::from_str(self.label).unwrap()
    }

    fn press(&mut self, _key: &KeyboardKey, _is_focused: bool) -> PressResult {
        PressResult {
            handled: false,
            focus: false,
        }
    }

    fn kind(&'a self) -> MenuItemKind<'a> {
        MenuItemKind::BasicMenuItem(&self)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyboard::FunctionKey;

    #[test]
    fn can_create_a_basic_menu_item() {
        let mut item: BasicMenuItem = BasicMenuItem::new("label");
        assert_eq!(item.get_label(false), "label");
        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::LEFT), None), false),
            PressResult {
                focus: false,
                handled: false
            }
        );
        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::RIGHT), None), false),
            PressResult {
                focus: false,
                handled: false
            }
        );
        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::BACK), None), false),
            PressResult {
                focus: false,
                handled: false
            }
        );
    }
}