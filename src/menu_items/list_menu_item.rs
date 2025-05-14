use crate::keyboard::{FunctionKey, KeyboardKey};
use crate::menu_items::menu_item::{MenuItem, PressResult, LABEL_BYTES};
use core::fmt::Write;
use heapless::String;

pub struct ListMenuItem<'a> {
    label: &'a str,
    entries: &'a [&'a str],
    selected_entry_idx: usize,
    focus_selected_entry_idx: usize,
}

impl<'a> ListMenuItem<'a> {
    pub fn new(label: &'a str, entries: &'a [&'a str]) -> Result<ListMenuItem<'a>, &'static str> {
        if entries.is_empty() {
            Err("At least one entry required")
        } else {
            let menu_item = ListMenuItem {
                label,
                entries,
                selected_entry_idx: 0,
                focus_selected_entry_idx: 0,
            };
            Ok(menu_item)
        }
    }

    pub fn get_selected_entry_idx(&self) -> usize {
        self.selected_entry_idx
    }

    pub fn set_selected_entry_idx(
        &mut self,
        selected_entry_idx: usize,
    ) -> Result<(), &'static str> {
        if selected_entry_idx >= self.entries.len() {
            Err("Selected entry idx must be between 0 and entries.len()")
        } else {
            self.selected_entry_idx = selected_entry_idx;
            self.focus_selected_entry_idx = selected_entry_idx;
            Ok(())
        }
    }

    pub fn select_next_entry(&mut self) {
        self.selected_entry_idx = if self.selected_entry_idx == self.entries.len() - 1 {
            0
        } else {
            self.selected_entry_idx + 1
        };
        self.focus_selected_entry_idx = self.selected_entry_idx;
    }

    pub fn select_prev_entry(&mut self) {
        self.selected_entry_idx = if self.selected_entry_idx == 0 {
            self.entries.len() - 1
        } else {
            self.selected_entry_idx - 1
        };
        self.focus_selected_entry_idx = self.selected_entry_idx;
    }

    pub fn select_focused_next_entry(&mut self) {
        self.focus_selected_entry_idx = if self.focus_selected_entry_idx == self.entries.len() - 1 {
            0
        } else {
            self.focus_selected_entry_idx + 1
        };
    }

    pub fn select_focused_prev_entry(&mut self) {
        self.focus_selected_entry_idx = if self.focus_selected_entry_idx == 0 {
            self.entries.len() - 1
        } else {
            self.focus_selected_entry_idx - 1
        };
    }

    pub fn get_selected_entry(&self) -> &str {
        &self.entries[self.selected_entry_idx]
    }

    pub fn get_focused_selected_entry(&self) -> &str {
        &self.entries[self.focus_selected_entry_idx]
    }
}

impl<'a> MenuItem<'a> for ListMenuItem<'a> {
    fn get_label(&self, is_focused: bool) -> String<{ LABEL_BYTES }> {
        let selected_entry = if is_focused {
            self.get_focused_selected_entry()
        } else {
            self.get_selected_entry()
        };

        let mut label_str: String<{ LABEL_BYTES }> = String::new();
        write!(label_str, "{}: {}", &self.label, selected_entry).unwrap();
        label_str
    }

    fn press(&mut self, key: &KeyboardKey, is_focused: bool) -> PressResult {
        let mut focus = is_focused;
        let mut handled = false;
        if let Some(function_key) = &key.function_key {
            match function_key {
                FunctionKey::ENTER => {
                    if is_focused {
                        self.selected_entry_idx = self.focus_selected_entry_idx;
                    } else {
                        self.focus_selected_entry_idx = self.selected_entry_idx;
                    }
                    focus = !is_focused;
                    handled = true;
                }
                FunctionKey::BACK => {
                    if is_focused {
                        self.focus_selected_entry_idx = self.selected_entry_idx;
                        handled = true;
                        focus = false;
                    }
                }
                FunctionKey::LEFT => {
                    if is_focused {
                        self.select_focused_prev_entry();
                        handled = true;
                        focus = true;
                    }
                }
                FunctionKey::RIGHT => {
                    if is_focused {
                        self.select_focused_next_entry();
                        handled = true;
                        focus = true;
                    }
                }
                _ => {}
            };
        }

        PressResult { handled, focus }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_next_entry_works() {
        let list_entries: [&str; 3] = ["Elem1", "Elem2", "Elem3"];
        let mut item: ListMenuItem = ListMenuItem::new("label", &list_entries).unwrap();
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem1");
        assert_eq!(item.get_selected_entry_idx(), 0);

        item.select_next_entry();
        assert_eq!(item.get_label(false), "label: Elem2");
        assert_eq!(item.get_label(true), "label: Elem2");
        assert_eq!(item.get_selected_entry_idx(), 1);

        item.select_next_entry();
        assert_eq!(item.get_label(false), "label: Elem3");
        assert_eq!(item.get_label(true), "label: Elem3");
        assert_eq!(item.get_selected_entry_idx(), 2);

        item.select_next_entry();
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem1");
        assert_eq!(item.get_selected_entry_idx(), 0);
    }

    #[test]
    fn select_prev_entry_works() {
        let list_entries: [&str; 3] = ["Elem1", "Elem2", "Elem3"];
        let mut item: ListMenuItem = ListMenuItem::new("label", &list_entries).unwrap();
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem1");
        assert_eq!(item.get_selected_entry_idx(), 0);

        item.select_prev_entry();
        assert_eq!(item.get_label(false), "label: Elem3");
        assert_eq!(item.get_label(true), "label: Elem3");
        assert_eq!(item.get_selected_entry_idx(), 2);

        item.select_prev_entry();
        assert_eq!(item.get_label(false), "label: Elem2");
        assert_eq!(item.get_label(true), "label: Elem2");
        assert_eq!(item.get_selected_entry_idx(), 1);

        item.select_prev_entry();
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem1");
        assert_eq!(item.get_selected_entry_idx(), 0);
    }

    #[test]
    fn set_selected_entry_idx_works() {
        let list_entries: [&str; 3] = ["Elem1", "Elem2", "Elem3"];
        let mut item: ListMenuItem = ListMenuItem::new("label", &list_entries).unwrap();
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem1");
        assert_eq!(item.get_selected_entry_idx(), 0);

        item.set_selected_entry_idx(1).unwrap();
        assert_eq!(item.get_label(false), "label: Elem2");
        assert_eq!(item.get_label(true), "label: Elem2");
        assert_eq!(item.get_selected_entry_idx(), 1);

        item.set_selected_entry_idx(2).unwrap();
        assert_eq!(item.get_label(false), "label: Elem3");
        assert_eq!(item.get_label(true), "label: Elem3");
        assert_eq!(item.get_selected_entry_idx(), 2);

        if let Err(error_msg) = item.set_selected_entry_idx(3) {
            assert_eq!(
                error_msg,
                "Selected entry idx must be between 0 and entries.len()"
            );
        } else {
            panic!("set_selected_entry_idx should return an error");
        }
    }

    #[test]
    fn enter_confirms_selection() {
        let list_entries: [&str; 3] = ["Elem1", "Elem2", "Elem3"];
        let mut item: ListMenuItem = ListMenuItem::new("label", &list_entries).unwrap();
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem1");
        assert_eq!(item.get_selected_entry_idx(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::ENTER), None), false),
            PressResult {
                focus: true,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem1");
        assert_eq!(item.get_selected_entry_idx(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::RIGHT), None), true),
            PressResult {
                focus: true,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem2");
        assert_eq!(item.get_selected_entry_idx(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::BACK), None), true),
            PressResult {
                focus: false,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem1");
        assert_eq!(item.get_selected_entry_idx(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::ENTER), None), false),
            PressResult {
                focus: true,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem1");
        assert_eq!(item.get_selected_entry_idx(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::RIGHT), None), true),
            PressResult {
                focus: true,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem2");
        assert_eq!(item.get_selected_entry_idx(), 0);

        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::ENTER), None), true),
            PressResult {
                focus: false,
                handled: true
            }
        );
        assert_eq!(item.get_label(false), "label: Elem2");
        assert_eq!(item.get_label(true), "label: Elem2");
        assert_eq!(item.get_selected_entry_idx(), 1);
    }
}
