use crate::menu_items::menu_item::MenuItem;

pub struct ToggleMenuItem<'a> {
    label: &'a str,
    text_true: &'a str,
    text_false: &'a str,
    value: bool,
}

impl <'a> ToggleMenuItem<'a> {
    pub fn new(label: &str) -> ToggleMenuItem {
        ToggleMenuItem {
            label,
            text_true: "ON",
            text_false: "OFF",
            value: false,
        }
    }

    pub fn get_value(&self) -> bool {
        self.value
    }
}

impl <'a> MenuItem for ToggleMenuItem<'a> {
    fn get_label(&self, _is_focused: bool) -> &str {
        let value_text = if self.value {
            self.text_true
        } else {
            self.text_false
        };
        // TODO nostd format!("{}: {}", self.label, &value_text)
        value_text
    }

    fn enter(&mut self, _is_focused: bool, _was_focused: bool) -> bool {
        self.value = !self.value;
        true
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
    fn item_is_usable() {
        let mut item = ToggleMenuItem::new("label");
        assert_eq!(item.left(), false);
        assert_eq!(item.right(), false);
        assert_eq!(item.back(), true);
        assert_eq!(item.is_focusable(), false);

        assert_eq!(item.get_label(false), "label: OFF");
        assert_eq!(item.get_value(), false);

        assert_eq!(item.enter(false, false), true);
        assert_eq!(item.get_label(false), "label: ON");
        assert_eq!(item.get_value(), true);

        assert_eq!(item.enter(false, false), true);
        assert_eq!(item.get_label(false), "label: OFF");
        assert_eq!(item.get_value(), false);
    }
}
