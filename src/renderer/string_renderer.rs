use heapless::{String, Vec};
use crate::menu::Menu;
use crate::menu_items::menu_item::{MenuItem, BYTES_PER_CHAR};
use crate::menu_items::menu_item_kind::MenuItemKind;
use crate::menu_items::submenu_menu_item::SubmenuMenuItem;
use core::fmt::Write;

pub struct StringRenderer<const LINE_MAX_CHAR_COUNT: usize, const LINE_COUNT: usize, const MAX_BYTES_PER_LINE: usize> {}

impl <const LINE_MAX_CHAR_COUNT: usize, const LINE_COUNT: usize, const MAX_BYTES_PER_LINE: usize> StringRenderer<LINE_MAX_CHAR_COUNT, LINE_COUNT, MAX_BYTES_PER_LINE> {
    pub fn new() -> Self {
        assert!(LINE_MAX_CHAR_COUNT >= 3, "Invalid line max char count. At least 3 required.");
        assert!(LINE_COUNT >= 2, "Invalid menu line count. At least 2 lines required.");
        assert_eq!(MAX_BYTES_PER_LINE, LINE_MAX_CHAR_COUNT * BYTES_PER_CHAR, "Invalid MAX_BYTES_PER_LINE. It must be equal to LINE_MAX_CHAR_COUNT * BYTES_PER_CHAR.");

        Self {}
    }

    pub fn render<'a>(&self, menu: &'a Menu<'a>) -> Vec<String<MAX_BYTES_PER_LINE>, LINE_COUNT> {
        let submenu_item = menu.get_submenu_menu_item();
        let mut lines = self.generate_lines_to_render(submenu_item).unwrap_or_else(move || Vec::new());

        while lines.len() != LINE_COUNT {
            let mut line: String<MAX_BYTES_PER_LINE> = String::new();
            for _char_idx in 0..LINE_MAX_CHAR_COUNT {
                line.push(' ').unwrap();
            }

            lines.push(line).unwrap();
        }

        lines
    }

    fn generate_lines_to_render<'a>(
        &self, item: &'a dyn MenuItem<'a>
    ) -> Option<Vec<String<MAX_BYTES_PER_LINE>, LINE_COUNT>> {
        let selected_item_kind = item.kind();
        if let MenuItemKind::SubmenuMenuItem(&ref sub_submenu) = &selected_item_kind {
            let lines_from_item_option = if sub_submenu.is_focused() {
                let selected_item = sub_submenu.get_selected_item();
                let item_lines_to_render_option = self.generate_lines_to_render(selected_item);
                if let Some(item_lines_to_render) = item_lines_to_render_option {
                    Some(item_lines_to_render)
                } else {
                    None
                }
            } else {
                None
            };

            if lines_from_item_option.is_some() {
                lines_from_item_option
            } else {
                let mut lines_to_render: Vec<String<MAX_BYTES_PER_LINE>, LINE_COUNT> =
                    Vec::new();
                let selected_item_idx = sub_submenu.get_selected_item_idx();
                let top_visible_item_idx = self.get_top_visible_item_idx(selected_item_idx);
                let items_length = sub_submenu.item_count();
                let bottom_idx = core::cmp::min(LINE_COUNT + top_visible_item_idx, items_length);
                for visible_item_idx in top_visible_item_idx..bottom_idx {
                    let visible_item = sub_submenu.get_item(visible_item_idx).unwrap();
                    let line_to_render = self.generate_submenu_line_to_render(&sub_submenu, visible_item_idx, visible_item);
                    lines_to_render.push(line_to_render).unwrap();
                }

                Some(lines_to_render)
            }
        } else {
            None
        }
    }

    fn generate_submenu_line_to_render(
        &self,
        submenu: &SubmenuMenuItem,
        item_idx: usize,
        item: &dyn MenuItem
    ) -> String<MAX_BYTES_PER_LINE> {
        let selected_item_idx = submenu.get_selected_item_idx();
        let is_selected_item = item_idx == selected_item_idx;
        let is_item_focused = is_selected_item && submenu.is_focused();
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
        let max_length_label = LINE_MAX_CHAR_COUNT - 2;
        let label_trimmed = if label.len() > max_length_label {
            &label[..max_length_label]
        } else {
            &label
        };

        let top_visible_item_idx = self.get_top_visible_item_idx(selected_item_idx);
        let bottom_item_idx = top_visible_item_idx + LINE_COUNT - 1;
        let arrow_str: &str = if item_idx == top_visible_item_idx {
            if top_visible_item_idx != 0 {
                "↑"
            } else {
                " "
            }
        } else if item_idx == bottom_item_idx {
            if bottom_item_idx < submenu.item_count() - 1 {
                "↓"
            } else {
                " "
            }
        } else {
            " "
        };

        let mut line_str: String<MAX_BYTES_PER_LINE> = String::new();
        write!(
            line_str,
            "{}{:3$}{}",
            selection_str, label_trimmed, arrow_str, max_length_label
        )
            .unwrap();
        line_str
    }

    fn get_top_visible_item_idx(&self, selected_item_idx: usize) -> usize {
        let div = selected_item_idx.div_euclid(LINE_COUNT);
        div * LINE_COUNT
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
        let mut items: [&mut dyn MenuItem; 1] =
            [&mut BasicMenuItem::new("Item1")];
        let menu: Menu = Menu::new(&mut items).unwrap();

        let renderer: StringRenderer<16, 2, { 16 * BYTES_PER_CHAR }> = StringRenderer::new();

        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], "                ");
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
        let mut menu: Menu = Menu::new(&mut items).unwrap();

        let renderer: StringRenderer<16, 5, { 16 * BYTES_PER_CHAR }> = StringRenderer::new();


        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(lines_to_render[2], " Item3          ");
        assert_eq!(lines_to_render[3], " Item4          ");
        assert_eq!(lines_to_render[4], " Item5         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], " Item1          ");
        assert_eq!(lines_to_render[1], "→Item2          ");
        assert_eq!(lines_to_render[2], " Item3          ");
        assert_eq!(lines_to_render[3], " Item4          ");
        assert_eq!(lines_to_render[4], " Item5         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], " Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(lines_to_render[2], "→Item3          ");
        assert_eq!(lines_to_render[3], " Item4          ");
        assert_eq!(lines_to_render[4], " Item5         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], " Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(lines_to_render[2], " Item3          ");
        assert_eq!(lines_to_render[3], "→Item4          ");
        assert_eq!(lines_to_render[4], " Item5         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], " Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(lines_to_render[2], " Item3          ");
        assert_eq!(lines_to_render[3], " Item4          ");
        assert_eq!(lines_to_render[4], "→Item5         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], "→Item6         ↑");
        assert_eq!(lines_to_render[1], " Item7          ");
        assert_eq!(lines_to_render[2], " Item8          ");
        assert_eq!(lines_to_render[3], "                ");
        assert_eq!(lines_to_render[4], "                ");
    }

    /*
 TODO

 #[test]
 fn can_create_complex_menu() {
     let mut items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 5] = [
         &mut BasicMenuItem::new("Item1"),
         &mut BasicMenuItem::new("Item2"),
         &mut BasicMenuItem::new("Item3"),
         &mut BasicMenuItem::new("Item4"),
         &mut BasicMenuItem::new("Item5"),
     ];
     let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
     assert_eq!(menu.char_width, 16);
     assert_eq!(menu.char_height, 2);

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
     let mut items: [&mut dyn MenuItem<2, { 1 * BYTES_PER_CHAR }>; 1] =
         [&mut BasicMenuItem::new("Item1")];
     let menu_result: Result<Menu<1, 2, { 1 * BYTES_PER_CHAR }>, &str> = Menu::new(&mut items);
     if let Err(error) = menu_result {
         assert_eq!(error, "Invalid menu char width. At least 3 chars required.")
     } else {
         panic!("It should return an error");
     }
 }

 #[test]
 fn panics_if_invalid_char_height() {
     let mut items: [&mut dyn MenuItem<1, { 16 * BYTES_PER_CHAR }>; 1] =
         [&mut BasicMenuItem::new("Item1")];
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
     let mut items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 1] =
         [&mut BasicMenuItem::new("Item1")];
     let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
     assert_eq!(menu.char_width, 16);
     assert_eq!(menu.char_height, 2);

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
     let mut items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 2] = [
         &mut ActionMenuItem::new("Item1", &mut on_click),
         &mut BasicMenuItem::new("Item2"),
     ];
     let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
     assert_eq!(menu.char_width, 16);
     assert_eq!(menu.char_height, 2);

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

     let mut items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 2] = [
         &mut ListMenuItem::new("Item1", &list_entries).unwrap(),
         &mut BasicMenuItem::new("Item2"),
     ];

     let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
     assert_eq!(menu.char_width, 16);
     assert_eq!(menu.char_height, 2);

     let lines_to_render = menu.generate_lines_to_render();
     assert_eq!(lines_to_render.len(), 2);
     assert_eq!(lines_to_render[0], "→Item1: Elem1   ");
     assert_eq!(lines_to_render[1], " Item2          ");
     assert_eq!(menu.left(), false);
     assert_eq!(menu.right(), false);

     menu.enter();
     let lines_to_render = menu.generate_lines_to_render();
     assert_eq!(lines_to_render.len(), 2);
     assert_eq!(lines_to_render[0], "←Item1: Elem1   ");
     assert_eq!(lines_to_render[1], " Item2          ");

     // Can't move while focused
     assert_eq!(menu.up(), false);
     assert_eq!(menu.down(), false);

     assert_eq!(menu.right(), true);
     let lines_to_render = menu.generate_lines_to_render();
     assert_eq!(lines_to_render.len(), 2);
     assert_eq!(lines_to_render[0], "←Item1: Elem2   ");
     assert_eq!(lines_to_render[1], " Item2          ");

     assert_eq!(menu.back(), true);
     let lines_to_render = menu.generate_lines_to_render();
     assert_eq!(lines_to_render.len(), 2);
     assert_eq!(lines_to_render[0], "→Item1: Elem1   ");
     assert_eq!(lines_to_render[1], " Item2          ");

     menu.enter();
     let lines_to_render = menu.generate_lines_to_render();
     assert_eq!(lines_to_render.len(), 2);
     assert_eq!(lines_to_render[0], "←Item1: Elem1   ");
     assert_eq!(lines_to_render[1], " Item2          ");

     assert_eq!(menu.left(), true);
     let lines_to_render = menu.generate_lines_to_render();
     assert_eq!(lines_to_render.len(), 2);
     assert_eq!(lines_to_render[0], "←Item1: Elem3   ");
     assert_eq!(lines_to_render[1], " Item2          ");

     menu.enter();
     let lines_to_render = menu.generate_lines_to_render();
     assert_eq!(lines_to_render.len(), 2);
     assert_eq!(lines_to_render[0], "→Item1: Elem3   ");
     assert_eq!(lines_to_render[1], " Item2          ");
 }

 #[test]
 fn toggle_item_is_usable() {
     let mut items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 2] = [
         &mut ToggleMenuItem::new("Item1"),
         &mut BasicMenuItem::new("Item2"),
     ];
     let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
     assert_eq!(menu.char_width, 16);
     assert_eq!(menu.char_height, 2);

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
     let mut items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 2] = [
         &mut RangeMenuItem::new("Item1", 3, 10, 1).unwrap(),
         &mut BasicMenuItem::new("Item2"),
     ];
     let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
     assert_eq!(menu.char_width, 16);
     assert_eq!(menu.char_height, 2);

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

 #[test]
 fn submenu_is_usable() {
     let mut submenu2_items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 1] =
         [&mut BasicMenuItem::new("Sub2 Item1")];

     let mut subitem2: SubmenuMenuItem<16, 2, { 16 * BYTES_PER_CHAR }> =
         SubmenuMenuItem::new("Sub Item2", &mut submenu2_items);

     let mut submenu1_items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 2] =
         [&mut ToggleMenuItem::new("Sub Item1"), &mut subitem2];

     let mut item1: SubmenuMenuItem<16, 2, { 16 * BYTES_PER_CHAR }> =
         SubmenuMenuItem::new("Item1", &mut submenu1_items);
     let mut items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 2] =
         [&mut item1, &mut BasicMenuItem::new("Item2")];
     let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
     assert_eq!(menu.char_width, 16);
     assert_eq!(menu.char_height, 2);

     let lines_to_render = menu.generate_lines_to_render();
     assert_eq!(lines_to_render.len(), 2);
     assert_eq!(lines_to_render[0], "→Item1          ");
     assert_eq!(lines_to_render[1], " Item2          ");

     assert_eq!(menu.enter(), true);
     let lines_to_render = menu.generate_lines_to_render();
     assert_eq!(lines_to_render.len(), 2);
     assert_eq!(lines_to_render[0], "→Sub Item1: OFF ");
     assert_eq!(lines_to_render[1], " Sub Item2      ");

     assert_eq!(menu.enter(), true);
     let lines_to_render = menu.generate_lines_to_render();
     assert_eq!(lines_to_render.len(), 2);
     assert_eq!(lines_to_render[0], "→Sub Item1: ON  ");
     assert_eq!(lines_to_render[1], " Sub Item2      ");

     assert_eq!(menu.down(), true);
     let lines_to_render = menu.generate_lines_to_render();
     assert_eq!(lines_to_render.len(), 2);
     assert_eq!(lines_to_render[0], " Sub Item1: ON  ");
     assert_eq!(lines_to_render[1], "→Sub Item2      ");

     assert_eq!(menu.enter(), true);
     let lines_to_render = menu.generate_lines_to_render();
     assert_eq!(lines_to_render.len(), 1);
     assert_eq!(lines_to_render[0], "→Sub2 Item1     ");

     assert_eq!(menu.back(), true);
     let lines_to_render = menu.generate_lines_to_render();
     assert_eq!(lines_to_render.len(), 2);
     assert_eq!(lines_to_render[0], " Sub Item1: ON  ");
     assert_eq!(lines_to_render[1], "→Sub Item2      ");

     assert_eq!(menu.back(), true);
     let lines_to_render = menu.generate_lines_to_render();
     assert_eq!(lines_to_render.len(), 2);
     assert_eq!(lines_to_render[0], "→Item1          ");
     assert_eq!(lines_to_render[1], " Item2          ");

     assert_eq!(menu.back(), true);
     let lines_to_render = menu.generate_lines_to_render();
     assert_eq!(lines_to_render.len(), 2);
     assert_eq!(lines_to_render[0], "→Item1          ");
     assert_eq!(lines_to_render[1], " Item2          ");
 }

  */
}
