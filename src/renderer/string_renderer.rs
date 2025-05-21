use crate::menu::Menu;
use crate::menu_items::menu_item::MenuItem;
use crate::menu_items::menu_item_kind::MenuItemKind;
use crate::menu_items::submenu_menu_item::SubmenuMenuItem;
use core::fmt::Write;

pub struct StringRenderer {
    char_count: usize,
    line_count: usize,
}

impl StringRenderer {
    pub fn new(char_count: usize, line_count: usize) -> Result<Self, &'static str> {
        if char_count < 3 {
            Err("Invalid char count. At least 3 chars required.")
        } else if line_count < 2 {
            Err("Invalid line count. At least 2 lines required.")
        } else {
            Ok(Self {
                line_count,
                char_count,
            })
        }
    }

    pub fn render(&self, menu: &Menu) -> Vec<String> {
        let submenu_item = menu.get_submenu_menu_item();
        let mut lines = self
            .generate_lines_to_render(submenu_item)
            .unwrap_or_else(move || Vec::new());

        while lines.len() != self.line_count {
            let mut line: String = String::new();
            for _char_idx in 0..self.char_count {
                line.push(' ');
            }

            lines.push(line);
        }

        lines
    }

    fn generate_lines_to_render<'a>(&self, item: &dyn MenuItem) -> Option<Vec<String>> {
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
                let mut lines_to_render: Vec<String> = Vec::new();
                let selected_item_idx = sub_submenu.get_selected_item_idx();
                let top_visible_item_idx = self.get_top_visible_item_idx(selected_item_idx);
                let items_length = sub_submenu.item_count();
                let bottom_idx =
                    core::cmp::min(self.line_count + top_visible_item_idx, items_length);
                for visible_item_idx in top_visible_item_idx..bottom_idx {
                    let visible_item = sub_submenu.get_item(visible_item_idx).unwrap();
                    let line_to_render = self.generate_submenu_line_to_render(
                        &sub_submenu,
                        visible_item_idx,
                        visible_item,
                    );
                    lines_to_render.push(line_to_render);
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
        item: &dyn MenuItem,
    ) -> String {
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
        let max_length_label = self.char_count - 2;
        let label_trimmed = if label.len() > max_length_label {
            &label[..max_length_label]
        } else {
            &label
        };

        let top_visible_item_idx = self.get_top_visible_item_idx(selected_item_idx);
        let bottom_item_idx = top_visible_item_idx + self.line_count - 1;
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

        let mut line_str: String = String::new();
        write!(
            line_str,
            "{}{:3$}{}",
            selection_str, label_trimmed, arrow_str, max_length_label
        )
        .unwrap();
        line_str
    }

    fn get_top_visible_item_idx(&self, selected_item_idx: usize) -> usize {
        let div = selected_item_idx.div_euclid(self.line_count);
        div * self.line_count
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
        let items: Vec<Box<dyn MenuItem>> =
            vec![Box::new(BasicMenuItem::new(String::from("Item1")))];
        let menu: Menu = Menu::new(items).unwrap();

        let renderer: StringRenderer = StringRenderer::new(16, 2).unwrap();

        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], "                ");
    }

    #[test]
    fn can_create_big_menu() {
        let items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(BasicMenuItem::new(String::from("Item1"))),
            Box::new(BasicMenuItem::new(String::from("Item2"))),
            Box::new(BasicMenuItem::new(String::from("Item3"))),
            Box::new(BasicMenuItem::new(String::from("Item4"))),
            Box::new(BasicMenuItem::new(String::from("Item5"))),
            Box::new(BasicMenuItem::new(String::from("Item6"))),
            Box::new(BasicMenuItem::new(String::from("Item7"))),
            Box::new(BasicMenuItem::new(String::from("Item8"))),
        ];
        let mut menu: Menu = Menu::new(items).unwrap();

        let renderer: StringRenderer = StringRenderer::new(16, 5).unwrap();

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

    #[test]
    fn can_create_complex_menu() {
        let items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(BasicMenuItem::new(String::from("Item1"))),
            Box::new(BasicMenuItem::new(String::from("Item2"))),
            Box::new(BasicMenuItem::new(String::from("Item3"))),
            Box::new(BasicMenuItem::new(String::from("Item4"))),
            Box::new(BasicMenuItem::new(String::from("Item5"))),
        ];
        let mut menu: Menu = Menu::new(items).unwrap();

        let renderer = StringRenderer::new(16, 2).unwrap();

        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], " Item2         ↓");

        assert_eq!(menu.up(), false);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], " Item2         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], " Item1          ");
        assert_eq!(lines_to_render[1], "→Item2         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item3         ↑");
        assert_eq!(lines_to_render[1], " Item4         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], " Item3         ↑");
        assert_eq!(lines_to_render[1], "→Item4         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item5         ↑");
        assert_eq!(lines_to_render[1], "                ");

        assert_eq!(menu.up(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], " Item3         ↑");
        assert_eq!(lines_to_render[1], "→Item4         ↓");
    }

    #[test]
    fn panics_if_invalid_char_count() {
        let renderer_result = StringRenderer::new(2, 2);

        if let Err(error) = renderer_result {
            assert_eq!(error, "Invalid char count. At least 3 chars required.")
        } else {
            panic!("It should return an error");
        }
    }

    #[test]
    fn panics_if_invalid_line_count() {
        let renderer_result = StringRenderer::new(16, 1);

        if let Err(error) = renderer_result {
            assert_eq!(error, "Invalid line count. At least 2 lines required.")
        } else {
            panic!("It should return an error");
        }
    }

    #[test]
    fn basic_item_ignores_enter() {
        let items: Vec<Box<dyn MenuItem>> =
            vec![Box::new(BasicMenuItem::new(String::from("Item1")))];
        let mut menu: Menu = Menu::new(items).unwrap();

        let renderer = StringRenderer::new(16, 2).unwrap();

        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], "                ");

        assert_eq!(false, menu.enter());
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], "                ");
    }

    #[test]
    fn action_item_is_usable() {
        let clicked_count = Rc::new(RefCell::new(0));
        let clicked_count_clone = Rc::clone(&clicked_count);
        let on_click = move || {
            *clicked_count_clone.borrow_mut() += 1;
            true
        };
        let items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(ActionMenuItem::new(
                String::from("Item1"),
                Box::new(on_click),
            )),
            Box::new(BasicMenuItem::new(String::from("Item2"))),
        ];
        let mut menu: Menu = Menu::new(items).unwrap();

        let renderer = StringRenderer::new(16, 2).unwrap();

        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(*clicked_count.borrow(), 0);

        assert_eq!(menu.enter(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(*clicked_count.borrow(), 1);
    }

    #[test]
    fn list_item_is_usable() {
        let list_entries = vec![
            String::from("Elem1"),
            String::from("Elem2"),
            String::from("Elem3"),
        ];

        let items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(ListMenuItem::new(String::from("Item1"), list_entries).unwrap()),
            Box::new(BasicMenuItem::new(String::from("Item2"))),
        ];

        let mut menu: Menu = Menu::new(items).unwrap();

        let renderer = StringRenderer::new(16, 2).unwrap();

        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1: Elem1   ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(menu.left(), false);
        assert_eq!(menu.right(), false);

        assert_eq!(menu.enter(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "←Item1: Elem1   ");
        assert_eq!(lines_to_render[1], " Item2          ");

        // Can't move while focused
        assert_eq!(menu.up(), false);
        assert_eq!(menu.down(), false);

        assert_eq!(menu.right(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "←Item1: Elem2   ");
        assert_eq!(lines_to_render[1], " Item2          ");

        assert_eq!(menu.back(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1: Elem1   ");
        assert_eq!(lines_to_render[1], " Item2          ");

        assert_eq!(menu.enter(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "←Item1: Elem1   ");
        assert_eq!(lines_to_render[1], " Item2          ");

        assert_eq!(menu.left(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "←Item1: Elem3   ");
        assert_eq!(lines_to_render[1], " Item2          ");

        assert_eq!(menu.enter(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1: Elem3   ");
        assert_eq!(lines_to_render[1], " Item2          ");
    }

    #[test]
    fn toggle_item_is_usable() {
        let items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(ToggleMenuItem::new(String::from("Item1"))),
            Box::new(BasicMenuItem::new(String::from("Item2"))),
        ];

        let mut menu: Menu = Menu::new(items).unwrap();

        let renderer = StringRenderer::new(16, 2).unwrap();

        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1: OFF     ");
        assert_eq!(lines_to_render[1], " Item2          ");

        assert_eq!(menu.enter(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1: ON      ");
        assert_eq!(lines_to_render[1], " Item2          ");

        assert_eq!(menu.enter(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1: OFF     ");
        assert_eq!(lines_to_render[1], " Item2          ");
    }

    #[test]
    fn range_item_is_usable() {
        let items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(RangeMenuItem::new(String::from("Item1"), 3, 10, 1).unwrap()),
            Box::new(BasicMenuItem::new(String::from("Item2"))),
        ];

        let mut menu: Menu = Menu::new(items).unwrap();

        let renderer = StringRenderer::new(16, 2).unwrap();

        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1: 3       ");
        assert_eq!(lines_to_render[1], " Item2          ");

        assert_eq!(menu.enter(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "←Item1: 3       ");
        assert_eq!(lines_to_render[1], " Item2          ");

        assert_eq!(menu.left(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "←Item1: 10      ");
        assert_eq!(lines_to_render[1], " Item2          ");

        assert_eq!(menu.left(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "←Item1: 9       ");
        assert_eq!(lines_to_render[1], " Item2          ");

        assert_eq!(menu.right(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "←Item1: 10      ");
        assert_eq!(lines_to_render[1], " Item2          ");

        assert_eq!(menu.enter(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1: 10      ");
        assert_eq!(lines_to_render[1], " Item2          ");
    }

    #[test]
    fn submenu_is_usable() {
        let submenu2_items: Vec<Box<dyn MenuItem>> =
            vec![Box::new(BasicMenuItem::new(String::from("Sub2 Item1")))];
        let subitem2 = SubmenuMenuItem::new(String::from("Sub Item2"), submenu2_items);
        let submenu1_items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(ToggleMenuItem::new(String::from("Sub Item1"))),
            Box::new(subitem2),
        ];
        let item1 = SubmenuMenuItem::new(String::from("Item1"), submenu1_items);

        let items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(item1),
            Box::new(BasicMenuItem::new(String::from("Item2"))),
        ];

        let mut menu: Menu = Menu::new(items).unwrap();

        let renderer = StringRenderer::new(16, 2).unwrap();

        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");

        assert_eq!(menu.enter(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Sub Item1: OFF ");
        assert_eq!(lines_to_render[1], " Sub Item2      ");

        assert_eq!(menu.enter(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Sub Item1: ON  ");
        assert_eq!(lines_to_render[1], " Sub Item2      ");

        assert_eq!(menu.down(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], " Sub Item1: ON  ");
        assert_eq!(lines_to_render[1], "→Sub Item2      ");

        assert_eq!(menu.enter(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Sub2 Item1     ");
        assert_eq!(lines_to_render[1], "                ");

        assert_eq!(menu.back(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], " Sub Item1: ON  ");
        assert_eq!(lines_to_render[1], "→Sub Item2      ");

        assert_eq!(menu.back(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");

        assert_eq!(menu.back(), true);
        let lines_to_render = renderer.render(&menu);
        assert_eq!(lines_to_render.len(), 2);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
    }
}
