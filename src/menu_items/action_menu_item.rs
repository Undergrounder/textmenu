use crate::menu_items::menu_item::{MenuItem, LABEL_BYTES};
use core::str::FromStr;
use heapless::String;

pub struct ActionMenuItem<'a> {
    label: &'a str,
    on_pressed: &'a mut dyn FnMut() -> bool,
}

impl<'a> ActionMenuItem<'a> {
    pub fn new(label: &'a str, on_pressed: &'a mut dyn FnMut() -> bool) -> ActionMenuItem<'a> {
        ActionMenuItem { label, on_pressed }
    }
}

impl<'a> MenuItem for ActionMenuItem<'a> {
    fn get_label(&self, _is_focused: bool) -> String<{ LABEL_BYTES }> {
        String::from_str(self.label).unwrap()
    }

    fn enter(&mut self, _is_focused: bool, _was_focused: bool) -> bool {
        (self.on_pressed)()
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
        let mut item = ActionMenuItem::new("label", &mut on_click);
        assert_eq!(item.get_label(false), "label");
        assert_eq!(*clicked_count.borrow(), 0);

        item.enter(true, false);
        assert_eq!(*clicked_count.borrow(), 1);

        item.enter(true, false);
        assert_eq!(*clicked_count.borrow(), 2);

        assert_eq!(item.left(), false);
        assert_eq!(item.right(), false);
        assert_eq!(item.back(), true);
        assert_eq!(item.is_focusable(), false);
    }
}
