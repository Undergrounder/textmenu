use crate::menu_items::menu_item::MenuItem;
use core::fmt::Write;
use heapless::{String, Vec};

pub const BYTES_PER_CHAR: usize = 4;

pub struct Menu<
    'a,
    const CHAR_WIDTH_CONST: usize,
    const CHAR_HEIGHT_CONST: usize,
    const LINE_BYTES_SIZE_CONST: usize,
> {
    pub char_width: usize,
    pub char_height: usize,
    pub items: &'a mut [&'a mut dyn MenuItem],
    // View state
    pub selected_item_idx: usize,
    pub is_focused: bool,
}

impl<
        'a,
        const CHAR_WIDTH_CONST: usize,
        const CHAR_HEIGHT_CONST: usize,
        const LINE_BYTES_SIZE_CONST: usize,
    > Menu<'a, CHAR_WIDTH_CONST, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>
{
    pub fn new(
        items: &'a mut [&'a mut dyn MenuItem],
    ) -> Result<Menu<CHAR_WIDTH_CONST, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>, &'static str>
    {
        if LINE_BYTES_SIZE_CONST != CHAR_WIDTH_CONST * BYTES_PER_CHAR {
            Err("LINE_BYTES_SIZE_CONST must be equal to CHAR_WIDTH_CONST*BYTES_PER_CHAR.")
        } else if items.len() == 0 {
            Err("At least 1 menu item required.")
        } else if CHAR_WIDTH_CONST < 3 {
            Err("Invalid menu char width. At least 3 chars required.")
        } else if CHAR_HEIGHT_CONST < 2 {
            Err("Invalid menu char height. At least 2 chars required.")
        } else {
            let menu = Menu {
                char_width: CHAR_WIDTH_CONST,
                char_height: CHAR_HEIGHT_CONST,
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

    pub fn generate_lines_to_render(
        &self,
    ) -> Vec<String<LINE_BYTES_SIZE_CONST>, CHAR_HEIGHT_CONST> {
        let items_length = self.items.len();
        let mut lines_to_render: Vec<String<LINE_BYTES_SIZE_CONST>, CHAR_HEIGHT_CONST> = Vec::new();
        let top_visible_item_idx = self.get_top_visible_item_idx();
        let bottom_idx = core::cmp::min(self.char_height + top_visible_item_idx, items_length);
        let visible_items = &self.items[top_visible_item_idx..bottom_idx];
        for (item_idx, item) in visible_items.iter().enumerate() {
            let corrected_item_idx = item_idx + top_visible_item_idx;
            let line_to_render = self.generate_line_to_render(corrected_item_idx, *item);
            lines_to_render.push(line_to_render).unwrap();
        }

        lines_to_render
    }

    fn generate_line_to_render(
        &self,
        item_idx: usize,
        item: &dyn MenuItem,
    ) -> String<LINE_BYTES_SIZE_CONST> {
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

        let mut line_str: String<LINE_BYTES_SIZE_CONST> = String::new();
        write!(
            line_str,
            "{}{:3$}{}",
            selection_str, label_trimmed, arrow_str, max_length_label
        )
        .unwrap();
        line_str
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

    fn get_mut_selected_item(&mut self) -> &mut dyn MenuItem {
        self.items[self.selected_item_idx]
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
    use crate::menu_items::range_menu_item::RangeMenuItem;
    use crate::menu_items::toggle_menu_item::ToggleMenuItem;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn can_create_simple_menu() {
        let mut items: [&mut dyn MenuItem; 1] = [&mut BasicMenuItem::new("Item1")];
        let menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
        assert_eq!(menu.items.len(), 1);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 1);
        assert_eq!(lines_to_render[0], "→Item1          ");
    }

    #[test]
    fn can_create_big_menu() {
        let mut items: [&mut dyn MenuItem; 8] = [
            &mut BasicMenuItem::new("Item1"),
            &mut BasicMenuItem::new("Item2"),
            &mut BasicMenuItem::new("Item3"),
            &mut BasicMenuItem::new("Item4"),
            &mut BasicMenuItem::new("Item5"),
            &mut BasicMenuItem::new("Item6"),
            &mut BasicMenuItem::new("Item7"),
            &mut BasicMenuItem::new("Item8"),
        ];
        let mut menu: Menu<16, 5, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 5);
        assert_eq!(menu.items.len(), 8);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(lines_to_render[2], " Item3          ");
        assert_eq!(lines_to_render[3], " Item4          ");
        assert_eq!(lines_to_render[4], " Item5         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], " Item1          ");
        assert_eq!(lines_to_render[1], "→Item2          ");
        assert_eq!(lines_to_render[2], " Item3          ");
        assert_eq!(lines_to_render[3], " Item4          ");
        assert_eq!(lines_to_render[4], " Item5         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], " Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(lines_to_render[2], "→Item3          ");
        assert_eq!(lines_to_render[3], " Item4          ");
        assert_eq!(lines_to_render[4], " Item5         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], " Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(lines_to_render[2], " Item3          ");
        assert_eq!(lines_to_render[3], "→Item4          ");
        assert_eq!(lines_to_render[4], " Item5         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], " Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(lines_to_render[2], " Item3          ");
        assert_eq!(lines_to_render[3], " Item4          ");
        assert_eq!(lines_to_render[4], "→Item5         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 3);
        assert_eq!(lines_to_render[0], "→Item6         ↑");
        assert_eq!(lines_to_render[1], " Item7          ");
        assert_eq!(lines_to_render[2], " Item8          ");
    }

    #[test]
    fn can_create_complex_menu() {
        let mut items: [&mut dyn MenuItem; 5] = [
            &mut BasicMenuItem::new("Item1"),
            &mut BasicMenuItem::new("Item2"),
            &mut BasicMenuItem::new("Item3"),
            &mut BasicMenuItem::new("Item4"),
            &mut BasicMenuItem::new("Item5"),
        ];
        let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
        assert_eq!(menu.items.len(), 5);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], " Item2         ↓");

        assert_eq!(menu.up(), false);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], " Item2         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], " Item1          ");
        assert_eq!(lines_to_render[1], "→Item2         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item3         ↑");
        assert_eq!(lines_to_render[1], " Item4         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], " Item3         ↑");
        assert_eq!(lines_to_render[1], "→Item4         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 1);
        assert_eq!(lines_to_render[0], "→Item5         ↑");

        assert_eq!(menu.up(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], " Item3         ↑");
        assert_eq!(lines_to_render[1], "→Item4         ↓");
    }

    #[test]
    fn panics_if_invalid_char_width() {
        let mut items: [&mut dyn MenuItem; 1] = [&mut BasicMenuItem::new("Item1")];
        let menu_result: Result<Menu<1, 2, { 1 * BYTES_PER_CHAR }>, &str> = Menu::new(&mut items);
        if let Err(error) = menu_result {
            assert_eq!(error, "Invalid menu char width. At least 3 chars required.")
        } else {
            panic!("It should return an error");
        }
    }

    #[test]
    fn panics_if_invalid_char_height() {
        let mut items: [&mut dyn MenuItem; 1] = [&mut BasicMenuItem::new("Item1")];
        let menu_result: Result<Menu<16, 1, { 16 * BYTES_PER_CHAR }>, &str> = Menu::new(&mut items);
        if let Err(error) = menu_result {
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
        let mut items: [&mut dyn MenuItem; 1] = [&mut BasicMenuItem::new("Item1")];
        let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
        assert_eq!(menu.items.len(), 1);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 1);
        assert_eq!(lines_to_render[0], "→Item1          ");

        menu.enter();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 1);
        assert_eq!(lines_to_render[0], "→Item1          ");
    }

    #[test]
    fn action_item_is_usable() {
        let clicked_count = Rc::new(RefCell::new(0));
        let clicked_count_clone = Rc::clone(&clicked_count);
        let mut on_click = move || {
            *clicked_count_clone.borrow_mut() += 1;
            true
        };
        let mut items: [&mut dyn MenuItem; 2] = [
            &mut ActionMenuItem::new("Item1", &mut on_click),
            &mut BasicMenuItem::new("Item2"),
        ];
        let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
        assert_eq!(menu.items.len(), 2);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(*clicked_count.borrow(), 0);

        menu.enter();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(*clicked_count.borrow(), 1);
    }

    #[test]
    fn list_item_is_usable() {
        let list_entries = ["Elem1", "Elem2", "Elem3"];

        let mut items: [&mut dyn MenuItem; 2] = [
            &mut ListMenuItem::new("Item1", &list_entries).unwrap(),
            &mut BasicMenuItem::new("Item2"),
        ];

        let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
        assert_eq!(menu.items.len(), 2);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1: Elem1   ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(menu.is_focused, false);
        assert_eq!(menu.left(), false);
        assert_eq!(menu.right(), false);

        menu.enter();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "←Item1: Elem1   ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(menu.is_focused, true);

        // Can't move while focused
        assert_eq!(menu.up(), false);
        assert_eq!(menu.selected_item_idx, 0);
        assert_eq!(menu.down(), false);
        assert_eq!(menu.selected_item_idx, 0);

        assert_eq!(menu.right(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "←Item1: Elem2   ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(menu.is_focused, true);

        assert_eq!(menu.back(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1: Elem1   ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(menu.is_focused, false);

        menu.enter();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "←Item1: Elem1   ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(menu.is_focused, true);

        assert_eq!(menu.left(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "←Item1: Elem3   ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(menu.is_focused, true);

        menu.enter();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1: Elem3   ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(menu.is_focused, false);
    }

    #[test]
    fn toggle_item_is_usable() {
        let mut items: [&mut dyn MenuItem; 2] = [
            &mut ToggleMenuItem::new("Item1"),
            &mut BasicMenuItem::new("Item2"),
        ];
        let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
        assert_eq!(menu.items.len(), 2);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1: OFF     ");
        assert_eq!(lines_to_render[1], " Item2          ");

        menu.enter();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1: ON      ");
        assert_eq!(lines_to_render[1], " Item2          ");

        menu.enter();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1: OFF     ");
        assert_eq!(lines_to_render[1], " Item2          ");
    }

    #[test]
    fn range_item_is_usable() {
        let mut items: [&mut dyn MenuItem; 2] = [
            &mut RangeMenuItem::new("Item1", 3, 10, 1).unwrap(),
            &mut BasicMenuItem::new("Item2"),
        ];
        let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
        assert_eq!(menu.items.len(), 2);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1: 3       ");
        assert_eq!(lines_to_render[1], " Item2          ");

        menu.enter();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "←Item1: 3       ");
        assert_eq!(lines_to_render[1], " Item2          ");

        menu.left();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "←Item1: 10      ");
        assert_eq!(lines_to_render[1], " Item2          ");

        menu.left();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "←Item1: 9       ");
        assert_eq!(lines_to_render[1], " Item2          ");

        menu.right();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "←Item1: 10      ");
        assert_eq!(lines_to_render[1], " Item2          ");

        menu.enter();
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1: 10      ");
        assert_eq!(lines_to_render[1], " Item2          ");
    }
}

// TODO improvements:
// TODO input item
// TODO charset input item
// TODO submenus
// TODO screens
// TODO horizontal scrolling if overflow
// TODO disable functionality via features
// TODO more range items (u8, signed, float, ....)
