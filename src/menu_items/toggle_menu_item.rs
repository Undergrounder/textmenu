use crate::keyboard::{FunctionKey, KeyboardKey};
use crate::menu_items::menu_item::{MenuItem, PressResult, LABEL_BYTES};
use core::fmt::Write;
use heapless::String;
use crate::menu_items::menu_item_kind::MenuItemKind;

pub struct ToggleMenuItem<'a> {
    label: &'a str,
    text_true: &'a str,
    text_false: &'a str,
    value: bool,
}

impl<'a> ToggleMenuItem<'a> {
    pub fn new(label: &str) -> ToggleMenuItem {
        ToggleMenuItem {
            label,
            text_true: "ON",
            text_false: "OFF",
            value: false,
        }
    }

    pub fn get_value(&self) -> bool {
        self.value
    }

    fn enter(&mut self, _is_focused: bool) -> bool {
        self.value = !self.value;
        true
    }
}

impl<'a> MenuItem<'a> for ToggleMenuItem<'a> {
    fn get_label(&self, _is_focused: bool) -> String<{ LABEL_BYTES }> {
        let value_text = if self.value {
            self.text_true
        } else {
            self.text_false
        };
        let mut label_str: String<{ LABEL_BYTES }> = String::new();
        write!(label_str, "{}: {}", self.label, &value_text).unwrap();
        label_str
    }

    fn press(&mut self, key: &KeyboardKey, is_focused: bool) -> PressResult {
        let handled = if let Some(function_key) = &key.function_key {
            match function_key {
                FunctionKey::ENTER => self.enter(is_focused),
                _ => false,
            }
        } else {
            false
        };
        PressResult {
            handled,
            focus: false,
        }
    }

    fn kind(&'a self) -> MenuItemKind<'a> {
        MenuItemKind::ToggleMenuItem(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_is_usable() {
        let mut item: ToggleMenuItem = ToggleMenuItem::new("label");
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

        assert_eq!(item.get_label(false), "label: OFF");
        assert_eq!(item.get_value(), false);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::ENTER), None), false),
            PressResult {
                focus: false,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: ON");
        assert_eq!(item.get_value(), true);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::ENTER), None), false),
            PressResult {
                focus: false,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: OFF");
        assert_eq!(item.get_value(), false);
    }
}
