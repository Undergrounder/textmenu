use crate::menu_items::menu_item::MenuItem;

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
        let mut item = BasicMenuItem::new(String::from("label"));
        assert_eq!(item.get_label(false), "label");
        assert_eq!(item.left(), false);
        assert_eq!(item.right(), false);
        assert_eq!(item.back(), true);
        assert_eq!(item.is_focusable(), false);
    }
}
