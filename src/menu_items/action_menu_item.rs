use crate::keyboard::{FunctionKey, KeyboardKey};
use crate::menu_items::menu_item::{MenuItem, PressResult, LABEL_BYTES};
use core::str::FromStr;
use heapless::{String, Vec};

pub struct ActionMenuItem<'a, const CHAR_HEIGHT_CONST: usize, const LINE_BYTES_SIZE_CONST: usize> {
    label: &'a str,
    on_pressed: &'a mut dyn FnMut() -> bool,
}

impl<'a, const CHAR_HEIGHT_CONST: usize, const LINE_BYTES_SIZE_CONST: usize>
    ActionMenuItem<'a, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>
{
    pub fn new(
        label: &'a str,
        on_pressed: &'a mut dyn FnMut() -> bool,
    ) -> ActionMenuItem<'a, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST> {
        ActionMenuItem { label, on_pressed }
    }
}

impl<'a, const CHAR_HEIGHT_CONST: usize, const LINE_BYTES_SIZE_CONST: usize>
    MenuItem<'a, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>
    for ActionMenuItem<'a, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>
{
    fn get_label(&self, _is_focused: bool) -> String<{ LABEL_BYTES }> {
        String::from_str(self.label).unwrap()
    }

    fn press(&mut self, key: &KeyboardKey, _is_focused: bool) -> PressResult {
        let handled = if let Some(function_key) = &key.function_key {
            if *function_key == FunctionKey::ENTER {
                (self.on_pressed)();
                true
            } else {
                false
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
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn can_create_a_menu_item() {
        let clicked_count = Rc::new(RefCell::new(0));
        let clicked_count_clone = Rc::clone(&clicked_count);
        let mut on_click = move || {
            *clicked_count_clone.borrow_mut() += 1;
            true
        };
        let mut item: ActionMenuItem<2, { 16 * BYTES_PER_CHAR }> =
            ActionMenuItem::new("label", &mut on_click);
        assert_eq!(item.get_label(false), "label");
        assert_eq!(*clicked_count.borrow(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::ENTER), None), false),
            PressResult {
                focus: false,
                handled: true
            }
        );
        assert_eq!(*clicked_count.borrow(), 1);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::ENTER), None), false),
            PressResult {
                focus: false,
                handled: true
            }
        );
        assert_eq!(*clicked_count.borrow(), 2);

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
