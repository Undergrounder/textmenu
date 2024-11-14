use crate::menu_items::menu_item::MenuItem;

pub struct RangeMenuItem {
    label: String,
    value: u32,
    focused_value: u32,
    max_value: u32,
    min_value: u32,
    step_size: u32,
}

impl RangeMenuItem {
    pub fn new(
        label: String,
        min_value: u32,
        max_value: u32,
        step_size: u32,
    ) -> Result<RangeMenuItem, &'static str> {
        if min_value == max_value {
            Err("Min and max value can't be equal")
        } else if min_value > max_value {
            Err("Max value must be bigger than min value")
        } else {
            let value_difference = max_value - min_value;
            let reminder = value_difference % step_size;
            if reminder == 0 {
                Ok(RangeMenuItem {
                    label,
                    value: min_value,
                    focused_value: min_value,
                    min_value,
                    max_value,
                    step_size,
                })
            } else {
                Err("The step size doesn't allow traversing min_value to max_value without any reminder")
            }
        }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn select_next_value(&mut self) {
        let mut new_value = self.value + self.step_size;
        if new_value > self.max_value {
            new_value = self.min_value;
        }
        self.value = new_value;
        self.focused_value = new_value;
    }

    pub fn select_prev_value(&mut self) {
        let new_value = if self.value == self.min_value {
            self.max_value
        } else {
            self.value - self.step_size
        };
        self.value = new_value;
        self.focused_value = new_value;
    }

    pub fn select_focused_next_value(&mut self) {
        let mut new_value = self.focused_value + self.step_size;
        if new_value > self.max_value {
            new_value = self.min_value;
        }
        self.focused_value = new_value;
    }

    pub fn select_focused_prev_value(&mut self) {
        let new_value = if self.focused_value == self.min_value {
            self.max_value
        } else {
            self.focused_value - self.step_size
        };
        self.focused_value = new_value;
    }
}

impl MenuItem for RangeMenuItem {
    fn get_label(&self, is_focused: bool) -> String {
        let value = if is_focused {
            &self.focused_value
        } else {
            &self.value
        };
        format!("{}: {}", self.label, value)
    }

    fn enter(&mut self, is_focused: bool, was_focused: bool) -> bool {
        if is_focused && !was_focused {
            self.focused_value = self.value;
        } else if !is_focused && was_focused {
            self.value = self.focused_value;
        }
        false
    }

    fn is_focusable(&self) -> bool {
        true
    }

    fn back(&mut self) -> bool {
        self.focused_value = self.value;
        true
    }

    fn left(&mut self) -> bool {
        self.select_focused_prev_value();
        true
    }

    fn right(&mut self) -> bool {
        self.select_focused_next_value();
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn assert_new_error(expected_error_msg: &str, min_value: u32, max_value: u32, step_size: u32) {
        let range_menu_item_result =
            RangeMenuItem::new(String::from("label"), min_value, max_value, step_size);
        if let Err(error_msg) = range_menu_item_result {
            assert_eq!(error_msg, expected_error_msg);
        } else {
            panic!("new should return an error");
        }
    }

    #[test]
    fn new_fails_invalid_step_size() {
        assert_new_error(
            "The step size doesn't allow traversing min_value to max_value without any reminder",
            0,
            100,
            9,
        );
    }

    #[test]
    fn new_fails_invalid_min_max() {
        assert_new_error("Min and max value can't be equal", 50, 50, 1);
    }

    #[test]
    fn new_fails_min_bigger_max() {
        assert_new_error("Max value must be bigger than min value", 50, 20, 1);
    }

    #[test]
    fn item_works_as_expected() {
        let mut item: RangeMenuItem =
            RangeMenuItem::new(String::from("label"), 0, 100, 20).unwrap();
        assert_eq!(item.is_focusable(), true);

        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 0");
        assert_eq!(item.get_value(), 0);

        item.enter(true, false);
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 0");
        assert_eq!(item.get_value(), 0);

        assert_eq!(item.right(), true);
        assert_eq!(item.get_label(true), "label: 20");
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_value(), 0);

        assert_eq!(item.back(), true);
        assert_eq!(item.get_label(true), "label: 0");
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_value(), 0);

        item.enter(true, false);
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 0");
        assert_eq!(item.get_value(), 0);

        assert_eq!(item.right(), true);
        assert_eq!(item.get_label(true), "label: 20");
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_value(), 0);

        item.enter(false, true);
        assert_eq!(item.get_label(true), "label: 20");
        assert_eq!(item.get_label(false), "label: 20");
        assert_eq!(item.get_value(), 20);
    }

    #[test]
    fn left_should_overflow_to_max() {
        let mut item: RangeMenuItem =
            RangeMenuItem::new(String::from("label"), 0, 100, 20).unwrap();
        item.enter(true, false);

        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 0");
        assert_eq!(item.get_value(), 0);

        assert_eq!(item.left(), true);
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 100");
        assert_eq!(item.get_value(), 0);

        assert_eq!(item.left(), true);
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 80");
        assert_eq!(item.get_value(), 0);

        item.enter(false, true);
        assert_eq!(item.get_label(false), "label: 80");
        assert_eq!(item.get_label(true), "label: 80");
        assert_eq!(item.get_value(), 80);
    }

    #[test]
    fn left_should_overflow_to_min() {
        let mut item: RangeMenuItem = RangeMenuItem::new(String::from("label"), 0, 40, 20).unwrap();
        item.enter(true, false);

        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 0");
        assert_eq!(item.get_value(), 0);

        assert_eq!(item.right(), true);
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 20");
        assert_eq!(item.get_value(), 0);

        assert_eq!(item.right(), true);
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 40");
        assert_eq!(item.get_value(), 0);

        assert_eq!(item.right(), true);
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 0");
        assert_eq!(item.get_value(), 0);

        assert_eq!(item.right(), true);
        assert_eq!(item.get_label(false), "label: 0");
        assert_eq!(item.get_label(true), "label: 20");
        assert_eq!(item.get_value(), 0);

        item.enter(false, true);
        assert_eq!(item.get_label(false), "label: 20");
        assert_eq!(item.get_label(true), "label: 20");
        assert_eq!(item.get_value(), 20);
    }

    // TODO enhance menu.ts
}
