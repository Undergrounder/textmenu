use crate::menu_item::MenuItem;
use crate::menu_item_enum::MenuItemEnum;

pub struct Menu {
    pub char_width: usize,
    pub char_height: usize,
    pub items: Vec<MenuItemEnum>,
    // View state
    pub selected_item_idx: usize,
}

impl Menu {
    pub fn new(
        char_width: usize,
        char_height: usize,
        items: Vec<MenuItemEnum>,
    ) -> Result<Menu, String> {
        if items.len() == 0 {
            Err("At least 1 menu item required.".to_string())
        } else if char_width < 3 {
            Err("Invalid menu char width. At least 3 chars required.".to_string())
        } else if char_height < 2 {
            Err("Invalid menu char height. At least 2 chars required.".to_string())
        } else {
            let menu = Menu {
                char_width,
                char_height,
                items,
                selected_item_idx: 0,
            };
            Ok(menu)
        }
    }

    fn get_top_visible_item_idx(&self) -> usize {
        let div = self.selected_item_idx.div_euclid(self.char_height);
        div * self.char_height
    }

    pub fn generate_lines_to_render(&self) -> Vec<String> {
        let items_length = self.items.len();
        let mut lines_to_render: Vec<String> = Vec::with_capacity(self.char_height);
        let top_visible_item_idx = self.get_top_visible_item_idx();
        let bottom_idx = std::cmp::min(self.char_height + top_visible_item_idx, items_length);
        let visible_items = &self.items[top_visible_item_idx..bottom_idx];
        for (item_idx, item) in visible_items.iter().enumerate() {
            let corrected_item_idx = item_idx + top_visible_item_idx;
            let line_to_render = self.generate_line_to_render(corrected_item_idx, item);
            lines_to_render.push(line_to_render);
        }

        lines_to_render
    }

    fn generate_line_to_render(&self, item_idx: usize, item: &MenuItemEnum) -> String {
        let selection_str: &str = if item_idx == self.selected_item_idx {
            "→"
        } else {
            " "
        };
        let label = match item {
            MenuItemEnum::BasicMenuItem(basic_menu_item) => basic_menu_item.get_label(),
        };
        let max_length_label = self.char_width - 2;
        let label_trimmed = if label.len() > max_length_label {
            &label[..max_length_label]
        } else {
            label
        };

        let top_visible_item_idx = self.get_top_visible_item_idx();
        let bottom_item_idx = top_visible_item_idx + self.char_height - 1;
        let arrow_str: &str = if item_idx == top_visible_item_idx {
            if top_visible_item_idx != 0 {
                "↑"
            } else {
                " "
            }
        } else if item_idx == bottom_item_idx {
            if bottom_item_idx < self.items.len() - 1 {
                "↓"
            } else {
                " "
            }
        } else {
            " "
        };

        // TODO horizontal scrolling if overflow

        format!(
            "{}{:3$}{}",
            selection_str, label_trimmed, arrow_str, max_length_label
        )
    }

    pub fn go_up(&mut self) -> bool {
        if let Some(new_selected_item_idx) = self.selected_item_idx.checked_sub(1) {
            self.selected_item_idx = new_selected_item_idx;
            true
        } else {
            false
        }
    }

    pub fn go_down(&mut self) -> bool {
        if let Some(new_selected_item_idx) = self.selected_item_idx.checked_add(1) {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::basic_menu_item::BasicMenuItem;
    use log::error;

    #[test]
    fn can_create_simple_menu() {
        let items: Vec<MenuItemEnum> = vec![MenuItemEnum::BasicMenuItem(BasicMenuItem::new(
            String::from("Item1"),
        ))];
        let menu = Menu::new(16, 2, items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
        assert_eq!(menu.items.len(), 1);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 1);
        assert_eq!(lines_to_render[0], String::from("→Item1          "));
    }

    #[test]
    fn can_create_complex_menu() {
        let items: Vec<MenuItemEnum> = vec![
            MenuItemEnum::BasicMenuItem(BasicMenuItem::new(String::from("Item1"))),
            MenuItemEnum::BasicMenuItem(BasicMenuItem::new(String::from("Item2"))),
            MenuItemEnum::BasicMenuItem(BasicMenuItem::new(String::from("Item3"))),
            MenuItemEnum::BasicMenuItem(BasicMenuItem::new(String::from("Item4"))),
            MenuItemEnum::BasicMenuItem(BasicMenuItem::new(String::from("Item5"))),
        ];
        let mut menu = Menu::new(16, 2, items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
        assert_eq!(menu.items.len(), 5);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("→Item1          "));
        assert_eq!(lines_to_render[1], String::from(" Item2         ↓"));

        assert_eq!(menu.go_up(), false);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("→Item1          "));
        assert_eq!(lines_to_render[1], String::from(" Item2         ↓"));

        assert_eq!(menu.go_down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from(" Item1          "));
        assert_eq!(lines_to_render[1], String::from("→Item2         ↓"));

        assert_eq!(menu.go_down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("→Item3         ↑"));
        assert_eq!(lines_to_render[1], String::from(" Item4         ↓"));

        assert_eq!(menu.go_down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from(" Item3         ↑"));
        assert_eq!(lines_to_render[1], String::from("→Item4         ↓"));

        assert_eq!(menu.go_down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 1);
        assert_eq!(lines_to_render[0], String::from("→Item5         ↑"));

        assert_eq!(menu.go_up(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from(" Item3         ↑"));
        assert_eq!(lines_to_render[1], String::from("→Item4         ↓"));
    }

    #[test]
    fn panics_if_invalid_char_width() {
        let items: Vec<MenuItemEnum> = vec![MenuItemEnum::BasicMenuItem(BasicMenuItem::new(
            String::from("Item1"),
        ))];
        if let (Err(error)) = Menu::new(1, 2, items) {
            assert_eq!(error, "Invalid menu char width. At least 3 chars required.")
        } else {
            panic!("It should return an error");
        }
    }

    #[test]
    fn panics_if_invalid_char_height() {
        let items: Vec<MenuItemEnum> = vec![MenuItemEnum::BasicMenuItem(BasicMenuItem::new(
            String::from("Item1"),
        ))];
        if let (Err(error)) = Menu::new(16, 1, items) {
            assert_eq!(
                error,
                "Invalid menu char height. At least 2 chars required."
            )
        } else {
            panic!("It should return an error");
        }
    }
}

// TODO improvements:
// TODO Action item
// TODO list item
// TODO range item (int, float)
// TODO Toggle item + with customizable labels
// TODO input item
// TODO charset input item
// TODO submenus
// TODO screens
