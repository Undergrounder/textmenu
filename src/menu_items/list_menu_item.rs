use crate::menu_items::menu_item::MenuItem;

pub struct ListMenuItem {
    label: String,
    entries: Vec<String>,
    selected_entry_idx: usize,
    focus_selected_entry_idx: usize,
}

impl ListMenuItem {
    pub fn new(label: String, entries: Vec<String>) -> Result<ListMenuItem, String> {
        if entries.is_empty() {
            Err("At least one entry required".to_string())
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

    pub fn set_selected_entry_idx(&mut self, selected_entry_idx: usize) -> Result<(), String> {
        if selected_entry_idx >= self.entries.len() {
            Err("Selected entry idx must be between 0 and entries.len()".to_string())
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

    pub fn get_selected_entry(&self) -> &String {
        &self.entries[self.selected_entry_idx]
    }

    pub fn get_focused_selected_entry(&self) -> &String {
        &self.entries[self.focus_selected_entry_idx]
    }
}

impl MenuItem for ListMenuItem {
    fn get_label(&self, is_focused: bool) -> String {
        let selected_entry = if is_focused {
            self.get_focused_selected_entry()
        } else {
            self.get_selected_entry()
        };
        format!("{}: {}", &self.label, selected_entry)
    }

    fn enter(&mut self, is_focused: bool, was_focused: bool) -> bool {
        if is_focused && !was_focused {
            self.focus_selected_entry_idx = self.selected_entry_idx;
        } else if !is_focused && was_focused {
            self.selected_entry_idx = self.focus_selected_entry_idx;
        }
        false
    }

    fn is_focusable(&self) -> bool {
        true
    }

    fn back(&mut self) -> bool {
        self.focus_selected_entry_idx = self.selected_entry_idx;
        true
    }

    fn left(&mut self) -> bool {
        self.select_focused_prev_entry();
        true
    }

    fn right(&mut self) -> bool {
        self.select_focused_next_entry();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_next_entry_works() {
        let list_entries: Vec<String> = vec![
            "Elem1".to_string(),
            "Elem2".to_string(),
            "Elem3".to_string(),
        ];
        let mut item = ListMenuItem::new(String::from("label"), list_entries).unwrap();
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
        let list_entries: Vec<String> = vec![
            "Elem1".to_string(),
            "Elem2".to_string(),
            "Elem3".to_string(),
        ];
        let mut item = ListMenuItem::new(String::from("label"), list_entries).unwrap();
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
        let list_entries: Vec<String> = vec![
            "Elem1".to_string(),
            "Elem2".to_string(),
            "Elem3".to_string(),
        ];
        let mut item = ListMenuItem::new(String::from("label"), list_entries).unwrap();
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
        let list_entries: Vec<String> = vec![
            "Elem1".to_string(),
            "Elem2".to_string(),
            "Elem3".to_string(),
        ];
        let mut item = ListMenuItem::new(String::from("label"), list_entries).unwrap();
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem1");
        assert_eq!(item.get_selected_entry_idx(), 0);

        item.enter(true, false);
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem1");
        assert_eq!(item.get_selected_entry_idx(), 0);

        assert_eq!(item.right(), true);
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem2");
        assert_eq!(item.get_selected_entry_idx(), 0);

        assert_eq!(item.back(), true);
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem1");
        assert_eq!(item.get_selected_entry_idx(), 0);

        item.enter(true, false);
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem1");
        assert_eq!(item.get_selected_entry_idx(), 0);

        assert_eq!(item.right(), true);
        assert_eq!(item.get_label(false), "label: Elem1");
        assert_eq!(item.get_label(true), "label: Elem2");
        assert_eq!(item.get_selected_entry_idx(), 0);

        item.enter(false, true);
        assert_eq!(item.get_label(false), "label: Elem2");
        assert_eq!(item.get_label(true), "label: Elem2");
        assert_eq!(item.get_selected_entry_idx(), 1);
    }
}
