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
    fn get_label(&self) -> String {
        self.label.clone()
    }

    fn press(&mut self, _is_focused: bool) -> () {}

    fn is_focusable(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_a_basic_menu_item() {
        let item = BasicMenuItem::new(String::from("label"));
        assert_eq!(item.get_label(), "label");
    }
}
