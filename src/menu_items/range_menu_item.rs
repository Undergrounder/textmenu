use crate::keyboard::{FunctionKey, KeyboardKey};
use crate::menu_items::menu_item::{MenuItem, PressResult, LABEL_BYTES};
use core::fmt::Write;
use heapless::{String};
use crate::menu_items::menu_item_kind::MenuItemKind;

pub struct RangeMenuItem<'a> {
    label: &'a str,
    value: u32,
    focused_value: u32,
    max_value: u32,
    min_value: u32,
    step_size: u32,
}

impl<'a> RangeMenuItem<'a> {
    pub fn new(
        label: &'a str,
        min_value: u32,
        max_value: u32,
        step_size: u32,
    ) -> Result<RangeMenuItem<'a>, &'static str> {
        if min_value == max_value {
            Err("Min and max value can't be equal")
        } else if min_value > max_value {
            Err("Max value must be bigger than min value")
        } else {
            let value_difference = max_value - min_value;
            let reminder = value_difference % step_size;
            if reminder == 0 {
                Ok(RangeMenuItem {
                    label,
                    value: min_value,
                    focused_value: min_value,
                    min_value,
                    max_value,
                    step_size,
                })
            } else {
                Err("The step size doesn't allow traversing min_value to max_value without any reminder")
            }
        }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn select_next_value(&mut self) {
        let mut new_value = self.value + self.step_size;
        if new_value > self.max_value {
            new_value = self.min_value;
        }
        self.value = new_value;
        self.focused_value = new_value;
    }

    pub fn select_prev_value(&mut self) {
        let new_value = if self.value == self.min_value {
            self.max_value
        } else {
            self.value - self.step_size
        };
        self.value = new_value;
        self.focused_value = new_value;
    }

    pub fn select_focused_next_value(&mut self) {
        let mut new_value = self.focused_value + self.step_size;
        if new_value > self.max_value {
            new_value = self.min_value;
        }
        self.focused_value = new_value;
    }

    pub fn select_focused_prev_value(&mut self) {
        let new_value = if self.focused_value == self.min_value {
            self.max_value
        } else {
            self.focused_value - self.step_size
        };
        self.focused_value = new_value;
    }

    fn enter(&mut self, is_focused: bool) -> PressResult {
        if is_focused {
            self.value = self.focused_value;
        } else {
            self.focused_value = self.value;
        }
        PressResult {
            focus: !is_focused,
            handled: true,
        }
    }

    fn back(&mut self) -> PressResult {
        self.focused_value = self.value;
        PressResult {
            handled: true,
            focus: false,
        }
    }

    fn left(&mut self) -> PressResult {
        self.select_focused_prev_value();
        PressResult {
            focus: true,
            handled: true,
        }
    }

    fn right(&mut self) -> PressResult {
        self.select_focused_next_value();
        PressResult {
            focus: true,
            handled: true,
        }
    }
}

impl<'a> MenuItem<'a> for RangeMenuItem<'a> {
    fn get_label(&self, is_focused: bool) -> String<{ LABEL_BYTES }> {
        let value = if is_focused {
            &self.focused_value
        } else {
            &self.value
        };

        let mut label_str: String<{ LABEL_BYTES }> = String::new();
        write!(label_str, "{}: {}", self.label, value).unwrap();
        label_str
    }

    fn press(&mut self, key: &KeyboardKey, is_focused: bool) -> PressResult {
        if let Some(function_key) = &key.function_key {
            match function_key {
                FunctionKey::ENTER => self.enter(is_focused),
                FunctionKey::BACK => self.back(),
                FunctionKey::LEFT => self.left(),
                FunctionKey::RIGHT => self.right(),
                _ => PressResult {
                    focus: false,
                    handled: false,
                },
            }
        } else {
            PressResult {
                focus: false,
                handled: false,
            }
        }
    }

    fn kind(&'a self) -> MenuItemKind<'a> {
        MenuItemKind::RangeMenuItem(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_new_error(expected_error_msg: &str, min_value: u32, max_value: u32, step_size: u32) {
        let range_menu_item_result: Result<RangeMenuItem, &str> =
            RangeMenuItem::new("label", min_value, max_value, step_size);
        if let Err(error_msg) = range_menu_item_result {
            assert_eq!(error_msg, expected_error_msg);
        } else {
            panic!("new should return an error");
        }
    }

    #[test]
    fn new_fails_invalid_step_size() {
        assert_new_error(
            "The step size doesn't allow traversing min_value to max_value without any reminder",
            0,
            100,
            9,
        );
    }

    #[test]
    fn new_fails_invalid_min_max() {
        assert_new_error("Min and max value can't be equal", 50, 50, 1);
    }

    #[test]
    fn new_fails_min_bigger_max() {
        assert_new_error("Max value must be bigger than min value", 50, 20, 1);
    }

    #[test]
    fn item_works_as_expected() {
        let mut item: RangeMenuItem =
            RangeMenuItem::new("label", 0, 100, 20).unwrap();

        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 0");
        assert_eq!(item.get_value(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::ENTER), None), false),
            PressResult {
                focus: true,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 0");
        assert_eq!(item.get_value(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::RIGHT), None), true),
            PressResult {
                focus: true,
                handled: true
            }
        );
        assert_eq!(item.get_label(true), "label: 20");
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_value(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::BACK), None), true),
            PressResult {
                focus: false,
                handled: true
            }
        );
        assert_eq!(item.get_label(true), "label: 0");
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_value(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::ENTER), None), false),
            PressResult {
                focus: true,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 0");
        assert_eq!(item.get_value(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::RIGHT), None), true),
            PressResult {
                focus: true,
                handled: true
            }
        );
        assert_eq!(item.get_label(true), "label: 20");
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_value(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::ENTER), None), true),
            PressResult {
                focus: false,
                handled: true
            }
        );
        assert_eq!(item.get_label(true), "label: 20");
        assert_eq!(item.get_label(false), "label: 20");
        assert_eq!(item.get_value(), 20);
    }

    #[test]
    fn left_should_overflow_to_max() {
        let mut item: RangeMenuItem =
            RangeMenuItem::new("label", 0, 100, 20).unwrap();
        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::ENTER), None), false),
            PressResult {
                focus: true,
                handled: true
            }
        );

        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 0");
        assert_eq!(item.get_value(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::LEFT), None), true),
            PressResult {
                focus: true,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 100");
        assert_eq!(item.get_value(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::LEFT), None), true),
            PressResult {
                focus: true,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 80");
        assert_eq!(item.get_value(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::ENTER), None), true),
            PressResult {
                focus: false,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: 80");
        assert_eq!(item.get_label(true), "label: 80");
        assert_eq!(item.get_value(), 80);
    }

    #[test]
    fn left_should_overflow_to_min() {
        let mut item: RangeMenuItem =
            RangeMenuItem::new("label", 0, 40, 20).unwrap();
        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::ENTER), None), false),
            PressResult {
                focus: true,
                handled: true
            }
        );

        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 0");
        assert_eq!(item.get_value(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::RIGHT), None), true),
            PressResult {
                focus: true,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 20");
        assert_eq!(item.get_value(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::RIGHT), None), true),
            PressResult {
                focus: true,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 40");
        assert_eq!(item.get_value(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::RIGHT), None), true),
            PressResult {
                focus: true,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 0");
        assert_eq!(item.get_value(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::RIGHT), None), true),
            PressResult {
                focus: true,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 20");
        assert_eq!(item.get_value(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::ENTER), None), true),
            PressResult {
                focus: false,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: 20");
        assert_eq!(item.get_label(true), "label: 20");
        assert_eq!(item.get_value(), 20);
    }
}