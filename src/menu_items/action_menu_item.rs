use crate::keyboard::{FunctionKey, KeyboardKey};
use crate::menu_items::menu_item::{MenuItem, PressResult};
use crate::menu_items::menu_item_kind::MenuItemKind;

pub struct ActionMenuItem {
    label: String,
    on_pressed: Box<dyn FnMut() -> bool>,
}

impl ActionMenuItem {
    pub fn new(label: String, on_pressed: Box<dyn FnMut() -> bool>) -> ActionMenuItem {
        ActionMenuItem { label, on_pressed }
    }
}

impl MenuItem for ActionMenuItem {
    fn get_label(&self, _is_focused: bool) -> String {
        self.label.clone() // TODO avoid clone
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

    fn kind(&self) -> MenuItemKind {
        MenuItemKind::ActionMenuItem(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn can_create_a_menu_item() {
        let clicked_count = Rc::new(RefCell::new(0));
        let clicked_count_clone = Rc::clone(&clicked_count);
        let on_click = move || {
            *clicked_count_clone.borrow_mut() += 1;
            true
        };
        let mut item: ActionMenuItem =
            ActionMenuItem::new(String::from("label"), Box::new(on_click));
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
