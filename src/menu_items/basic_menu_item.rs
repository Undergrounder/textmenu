use crate::menu_items::menu_item::{MenuItem, LABEL_BYTES};
use core::str::FromStr;
use heapless::String;

pub struct BasicMenuItem<'a> {
    label: &'a str,
}

impl<'a> BasicMenuItem<'a> {
    pub fn new(label: &str) -> BasicMenuItem {
        BasicMenuItem { label }
    }
}

impl<'a> MenuItem for BasicMenuItem<'a> {
    fn get_label(&self, _is_focused: bool) -> String<{ LABEL_BYTES }> {
        String::from_str(self.label).unwrap()
    }

    fn enter(&mut self, _is_focused: bool, _was_focused: bool) -> bool {
        false
    }

    fn is_focusable(&self) -> bool {
        false
    }

    fn back(&mut self) -> bool {
        true
    }

    fn left(&mut self) -> bool {
        false
    }

    fn right(&mut self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_a_basic_menu_item() {
        let mut item = BasicMenuItem::new("label");
        assert_eq!(item.get_label(false), "label");
        assert_eq!(item.left(), false);
        assert_eq!(item.right(), false);
        assert_eq!(item.back(), true);
        assert_eq!(item.is_focusable(), false);
    }
}
