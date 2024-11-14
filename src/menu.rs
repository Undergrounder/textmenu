use crate::menu_items::menu_item::MenuItem;
use crate::menu_items::menu_item_enum::MenuItemEnum;

pub struct Menu<'a> {
    pub char_width: usize,
    pub char_height: usize,
    pub items: Vec<MenuItemEnum<'a>>,
    // View state
    pub selected_item_idx: usize,
    pub is_focused: bool,
}

impl<'a> Menu<'a> {
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
                is_focused: false,
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
        let is_selected_item = item_idx == self.selected_item_idx;
        let is_item_focused = is_selected_item && self.is_focused;
        let selection_str: &str = if is_selected_item {
            if is_item_focused {
                "←"
            } else {
                "→"
            }
        } else {
            " "
        };
        let label = item.get_label(is_item_focused);
        let max_length_label = self.char_width - 2;
        let label_trimmed = if label.len() > max_length_label {
            &label[..max_length_label]
        } else {
            &label
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

        format!(
            "{}{:3$}{}",
            selection_str, label_trimmed, arrow_str, max_length_label
        )
    }

    pub fn up(&mut self) -> bool {
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

    pub fn left(&mut self) -> bool {
        if self.is_focused {
            let selected_item = self.get_mut_selected_item();
            selected_item.left()
        } else {
            false
        }
    }

    pub fn right(&mut self) -> bool {
        if self.is_focused {
            let selected_item = self.get_mut_selected_item();
            selected_item.right()
        } else {
            false
        }
    }

    fn get_mut_selected_item(&mut self) -> &mut MenuItemEnum<'a> {
        &mut self.items[self.selected_item_idx]
    }

    fn back_on_selected_item(&mut self) -> bool {
        let selected_item = self.get_mut_selected_item();
        selected_item.back()
    }

    pub fn back(&mut self) -> bool {
        if self.is_focused {
            let item_back_result = self.back_on_selected_item();
            if item_back_result {
                self.is_focused = false
            }
            item_back_result
        } else {
            false
        }
    }

    pub fn enter(&mut self) -> bool {
        let was_focused = self.is_focused;
        let selected_item = self.get_mut_selected_item();
        let is_focusable: bool = selected_item.is_focusable();
        let is_focused = if is_focusable { !was_focused } else { false };
        self.is_focused = is_focused;

        let selected_item = self.get_mut_selected_item();
        selected_item.enter(is_focused, was_focused)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menu_items::action_menu_item::ActionMenuItem;
    use crate::menu_items::basic_menu_item::BasicMenuItem;
    use crate::menu_items::list_menu_item::ListMenuItem;
    use crate::menu_items::toggle_menu_item::ToggleMenuItem;
    use std::cell::RefCell;
    use std::rc::Rc;

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
    fn can_create_big_menu() {
        let items: Vec<MenuItemEnum> = vec![
            MenuItemEnum::BasicMenuItem(BasicMenuItem::new(
                String::from("Item1"),
            )),
            MenuItemEnum::BasicMenuItem(BasicMenuItem::new(
                String::from("Item2"),
            )),
            MenuItemEnum::BasicMenuItem(BasicMenuItem::new(
                String::from("Item3"),
            )),
            MenuItemEnum::BasicMenuItem(BasicMenuItem::new(
                String::from("Item4"),
            )),
            MenuItemEnum::BasicMenuItem(BasicMenuItem::new(
                String::from("Item5"),
            )),
            MenuItemEnum::BasicMenuItem(BasicMenuItem::new(
                String::from("Item6"),
            )),
            MenuItemEnum::BasicMenuItem(BasicMenuItem::new(
                String::from("Item7"),
            )),
            MenuItemEnum::BasicMenuItem(BasicMenuItem::new(
                String::from("Item8"),
            ))
        ];
        let mut menu = Menu::new(16, 5, items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 5);
        assert_eq!(menu.items.len(), 8);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], String::from("→Item1          "));
        assert_eq!(lines_to_render[1], String::from(" Item2          "));
        assert_eq!(lines_to_render[2], String::from(" Item3          "));
        assert_eq!(lines_to_render[3], String::from(" Item4          "));
        assert_eq!(lines_to_render[4], String::from(" Item5         ↓"));

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], String::from(" Item1          "));
        assert_eq!(lines_to_render[1], String::from("→Item2          "));
        assert_eq!(lines_to_render[2], String::from(" Item3          "));
        assert_eq!(lines_to_render[3], String::from(" Item4          "));
        assert_eq!(lines_to_render[4], String::from(" Item5         ↓"));

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], String::from(" Item1          "));
        assert_eq!(lines_to_render[1], String::from(" Item2          "));
        assert_eq!(lines_to_render[2], String::from("→Item3          "));
        assert_eq!(lines_to_render[3], String::from(" Item4          "));
        assert_eq!(lines_to_render[4], String::from(" Item5         ↓"));

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], String::from(" Item1          "));
        assert_eq!(lines_to_render[1], String::from(" Item2          "));
        assert_eq!(lines_to_render[2], String::from(" Item3          "));
        assert_eq!(lines_to_render[3], String::from("→Item4          "));
        assert_eq!(lines_to_render[4], String::from(" Item5         ↓"));

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], String::from(" Item1          "));
        assert_eq!(lines_to_render[1], String::from(" Item2          "));
        assert_eq!(lines_to_render[2], String::from(" Item3          "));
        assert_eq!(lines_to_render[3], String::from(" Item4          "));
        assert_eq!(lines_to_render[4], String::from("→Item5         ↓"));

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 3);
        assert_eq!(lines_to_render[0], String::from("→Item6         ↑"));
        assert_eq!(lines_to_render[1], String::from(" Item7          "));
        assert_eq!(lines_to_render[2], String::from(" Item8          "));
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

        assert_eq!(menu.up(), false);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("→Item1          "));
        assert_eq!(lines_to_render[1], String::from(" Item2         ↓"));

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from(" Item1          "));
        assert_eq!(lines_to_render[1], String::from("→Item2         ↓"));

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("→Item3         ↑"));
        assert_eq!(lines_to_render[1], String::from(" Item4         ↓"));

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from(" Item3         ↑"));
        assert_eq!(lines_to_render[1], String::from("→Item4         ↓"));

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 1);
        assert_eq!(lines_to_render[0], String::from("→Item5         ↑"));

        assert_eq!(menu.up(), true);
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
        if let Err(error) = Menu::new(1, 2, items) {
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
        if let Err(error) = Menu::new(16, 1, items) {
            assert_eq!(
                error,
                "Invalid menu char height. At least 2 chars required."
            )
        } else {
            panic!("It should return an error");
        }
    }

    #[test]
    fn basic_item_is_usable() {
        let items: Vec<MenuItemEnum> = vec![MenuItemEnum::BasicMenuItem(BasicMenuItem::new(
            String::from("Item1"),
        ))];
        let mut menu = Menu::new(16, 2, items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
        assert_eq!(menu.items.len(), 1);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 1);
        assert_eq!(lines_to_render[0], String::from("→Item1          "));

        menu.enter();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 1);
        assert_eq!(lines_to_render[0], String::from("→Item1          "));
    }

    #[test]
    fn action_item_is_usable() {
        let clicked_count = Rc::new(RefCell::new(0));
        let clicked_count_clone = Rc::clone(&clicked_count);
        let mut on_click = move || {
            *clicked_count_clone.borrow_mut() += 1;
            true
        };
        let items: Vec<MenuItemEnum> = vec![
            MenuItemEnum::ActionMenuItem(ActionMenuItem::new(String::from("Item1"), &mut on_click)),
            MenuItemEnum::BasicMenuItem(BasicMenuItem::new(String::from("Item2"))),
        ];
        let mut menu = Menu::new(16, 2, items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
        assert_eq!(menu.items.len(), 2);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("→Item1          "));
        assert_eq!(lines_to_render[1], String::from(" Item2          "));
        assert_eq!(*clicked_count.borrow(), 0);

        menu.enter();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("→Item1          "));
        assert_eq!(lines_to_render[1], String::from(" Item2          "));
        assert_eq!(*clicked_count.borrow(), 1);
    }

    #[test]
    fn list_item_is_usable() {
        let list_entries: Vec<String> = vec![
            "Elem1".to_string(),
            "Elem2".to_string(),
            "Elem3".to_string(),
        ];

        let items: Vec<MenuItemEnum> = vec![
            MenuItemEnum::ListMenuItem(
                ListMenuItem::new(String::from("Item1"), list_entries).unwrap(),
            ),
            MenuItemEnum::BasicMenuItem(BasicMenuItem::new(String::from("Item2"))),
        ];
        let mut menu = Menu::new(16, 2, items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
        assert_eq!(menu.items.len(), 2);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("→Item1: Elem1   "));
        assert_eq!(lines_to_render[1], String::from(" Item2          "));
        assert_eq!(menu.is_focused, false);
        assert_eq!(menu.left(), false);
        assert_eq!(menu.right(), false);

        menu.enter();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("←Item1: Elem1   "));
        assert_eq!(lines_to_render[1], String::from(" Item2          "));
        assert_eq!(menu.is_focused, true);

        // Can't move while focused
        assert_eq!(menu.up(), false);
        assert_eq!(menu.selected_item_idx, 0);
        assert_eq!(menu.down(), false);
        assert_eq!(menu.selected_item_idx, 0);

        assert_eq!(menu.right(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("←Item1: Elem2   "));
        assert_eq!(lines_to_render[1], String::from(" Item2          "));
        assert_eq!(menu.is_focused, true);

        assert_eq!(menu.back(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("→Item1: Elem1   "));
        assert_eq!(lines_to_render[1], String::from(" Item2          "));
        assert_eq!(menu.is_focused, false);

        menu.enter();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("←Item1: Elem1   "));
        assert_eq!(lines_to_render[1], String::from(" Item2          "));
        assert_eq!(menu.is_focused, true);

        assert_eq!(menu.left(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("←Item1: Elem3   "));
        assert_eq!(lines_to_render[1], String::from(" Item2          "));
        assert_eq!(menu.is_focused, true);

        menu.enter();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("→Item1: Elem3   "));
        assert_eq!(lines_to_render[1], String::from(" Item2          "));
        assert_eq!(menu.is_focused, false);
    }

    #[test]
    fn toggle_item_is_usable() {
        let items: Vec<MenuItemEnum> = vec![
            MenuItemEnum::ToggleMenuItem(ToggleMenuItem::new(String::from("Item1"))),
            MenuItemEnum::BasicMenuItem(BasicMenuItem::new(String::from("Item2"))),
        ];
        let mut menu = Menu::new(16, 2, items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
        assert_eq!(menu.items.len(), 2);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("→Item1: OFF     "));
        assert_eq!(lines_to_render[1], String::from(" Item2          "));

        menu.enter();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("→Item1: ON      "));
        assert_eq!(lines_to_render[1], String::from(" Item2          "));

        menu.enter();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], String::from("→Item1: OFF     "));
        assert_eq!(lines_to_render[1], String::from(" Item2          "));
    }
}

// TODO improvements:
// TODO range item (int, float)
// TODO input item
// TODO charset input item
// TODO submenus
// TODO screens
// TODO horizontal scrolling if overflow
