use crate::keyboard::{FunctionKey, KeyboardKey};
use crate::menu_items::menu_item::{MenuItem, PressResult};
use crate::menu_items::menu_item_kind::MenuItemKind;

pub struct SubmenuMenuItem {
    label: String,
    items: Vec<Box<dyn MenuItem>>,
    // View state
    selected_item_idx: usize,
    is_focused: bool,
}

impl SubmenuMenuItem {
    pub fn new(label: String, items: Vec<Box<dyn MenuItem>>) -> SubmenuMenuItem {
        // TODO panic if items length === 0
        SubmenuMenuItem {
            label,
            items,
            selected_item_idx: 0,
            is_focused: false,
        }
    }

    pub fn get_selected_item(&self) -> &dyn MenuItem {
        &*self.items[self.selected_item_idx]
    }

    pub fn get_mut_selected_item(&mut self) -> &mut dyn MenuItem {
        &mut *self.items[self.selected_item_idx]
    }

    fn up(&mut self) -> bool {
        if self.is_focused {
            false
        } else if let Some(new_selected_item_idx) = self.selected_item_idx.checked_sub(1) {
            self.selected_item_idx = new_selected_item_idx;
            true
        } else {
            false
        }
    }

    pub fn down(&mut self) -> bool {
        if self.is_focused {
            false
        } else if let Some(new_selected_item_idx) = self.selected_item_idx.checked_add(1) {
            if new_selected_item_idx < self.items.len() {
                self.selected_item_idx = new_selected_item_idx;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    #[inline]
    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    #[inline]
    pub fn get_selected_item_idx(&self) -> usize {
        self.selected_item_idx
    }

    pub fn get_item(&self, idx: usize) -> Option<&dyn MenuItem> {
        self.items.get(idx).map(|v| &**v)
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }
}

impl MenuItem for SubmenuMenuItem {
    fn get_label(&self, _is_focused: bool) -> String {
        self.label.clone() // TODO avoid clone
    }

    fn press(&mut self, key: &KeyboardKey, is_focused: bool) -> PressResult {
        if is_focused {
            let item_press_result = {
                let is_focused = self.is_focused;
                let selected_item = self.get_mut_selected_item();
                selected_item.press(key, is_focused)
            };
            self.is_focused = item_press_result.focus;
            if item_press_result.handled {
                PressResult {
                    handled: true,
                    focus: true,
                }
            } else {
                if let Some(function_key) = &key.function_key {
                    match function_key {
                        FunctionKey::BACK => PressResult {
                            focus: false,
                            handled: true,
                        },
                        FunctionKey::UP => {
                            let handled = self.up();
                            PressResult {
                                focus: true,
                                handled,
                            }
                        }
                        FunctionKey::DOWN => {
                            let handled = self.down();
                            PressResult {
                                focus: true,
                                handled,
                            }
                        }
                        _ => PressResult {
                            handled: false,
                            focus: true,
                        },
                    }
                } else {
                    PressResult {
                        handled: false,
                        focus: true,
                    }
                }
            }
        } else {
            match key.function_key {
                Some(FunctionKey::ENTER) => PressResult {
                    handled: true,
                    focus: true,
                },
                _ => PressResult {
                    handled: false,
                    focus: false,
                },
            }
        }
    }

    fn kind(&self) -> MenuItemKind {
        MenuItemKind::SubmenuMenuItem(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menu_items::basic_menu_item::BasicMenuItem;

    #[test]
    fn can_create_a_menu_item() {
        let items: Vec<Box<dyn MenuItem>> =
            vec![Box::new(BasicMenuItem::new(String::from("Item1")))];
        let item: SubmenuMenuItem = SubmenuMenuItem::new(String::from("label"), items);

        assert_eq!(item.get_label(false), "label");
    }
}
