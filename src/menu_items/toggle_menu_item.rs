use crate::keyboard::{FunctionKey, KeyboardKey};
use crate::menu_items::menu_item::{MenuItem, PressResult};
use core::fmt::Write;
use std::any::Any;

pub struct ToggleMenuItem {
    label: String,
    text_true: String,
    text_false: String,
    value: bool,
}

impl ToggleMenuItem {
    pub fn new(label: String) -> ToggleMenuItem {
        ToggleMenuItem {
            label,
            text_true: String::from("ON"),
            text_false: String::from("OFF"),
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

impl MenuItem for ToggleMenuItem {
    fn get_label(&self, _is_focused: bool) -> String {
        let value_text = if self.value {
            &self.text_true
        } else {
            &self.text_false
        };
        let mut label_str: String = String::new();
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_is_usable() {
        let mut item: ToggleMenuItem = ToggleMenuItem::new(String::from("label"));
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
