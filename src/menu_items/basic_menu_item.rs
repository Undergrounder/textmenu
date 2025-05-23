use crate::keyboard::KeyboardKey;
use crate::menu_items::menu_item::{MenuItem, PressResult};
use crate::menu_items::menu_item_kind::MenuItemKind;

pub struct BasicMenuItem {
    label: String,
}

impl BasicMenuItem {
    pub fn new(label: String) -> BasicMenuItem {
        BasicMenuItem { label }
    }
}

impl MenuItem for BasicMenuItem {
    fn get_label(&self, _is_focused: bool) -> String {
        self.label.clone()
    }

    fn press(&mut self, _key: &KeyboardKey, _is_focused: bool) -> PressResult {
        PressResult {
            handled: false,
            focus: false,
        }
    }

    fn kind(&self) -> MenuItemKind {
        MenuItemKind::BasicMenuItem(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyboard::FunctionKey;

    #[test]
    fn can_create_a_basic_menu_item() {
        let mut item: BasicMenuItem = BasicMenuItem::new(String::from("label"));
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
