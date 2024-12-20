use crate::keyboard::{FunctionKey, KeyboardKey};
use crate::menu_items::menu_item::{MenuItem, PressResult, LABEL_BYTES};
use core::fmt::Write;
use heapless::{String, Vec};

pub struct ToggleMenuItem<'a, const CHAR_HEIGHT_CONST: usize, const LINE_BYTES_SIZE_CONST: usize> {
    label: &'a str,
    text_true: &'a str,
    text_false: &'a str,
    value: bool,
}

impl<'a, const CHAR_HEIGHT_CONST: usize, const LINE_BYTES_SIZE_CONST: usize>
    ToggleMenuItem<'a, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>
{
    pub fn new(label: &str) -> ToggleMenuItem<CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST> {
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

impl<'a, const CHAR_HEIGHT_CONST: usize, const LINE_BYTES_SIZE_CONST: usize>
    MenuItem<'a, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>
    for ToggleMenuItem<'a, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>
{
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

    fn generate_lines_to_render(
        &self,
    ) -> Option<Vec<String<LINE_BYTES_SIZE_CONST>, CHAR_HEIGHT_CONST>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::BYTES_PER_CHAR;

    #[test]
    fn item_is_usable() {
        let mut item: ToggleMenuItem<2, { 16 * BYTES_PER_CHAR }> = ToggleMenuItem::new("label");
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
