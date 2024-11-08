use crate::menu::Menu;
use crate::menu_item::MenuItem;
use crate::menu_item_enum::MenuItemEnum;

pub struct ConsoleRenderer<'a> {
    last_tick_id: usize,
    last_rendered_lines: Vec<String>,
    menu: &'a Menu
}

impl ConsoleRenderer<'_> {
    pub fn new(menu: &Menu) -> ConsoleRenderer {
        ConsoleRenderer {
            last_tick_id: usize::MAX,
            menu: &menu,
            last_rendered_lines: vec!()
        }
    }

    fn generate_lines_to_render(&self) -> Vec<String> {
        let menu = self.menu;
        let mut lines_to_render: Vec<String> = Vec::with_capacity(menu.char_height);
        let top_visible_item_idx = menu.top_visible_item_idx;
        let visible_items = &self.menu.items[top_visible_item_idx..top_visible_item_idx + menu.char_height];
        for (item_idx, item) in visible_items.iter().enumerate() {
            let corrected_item_idx = item_idx + top_visible_item_idx;
            let line_to_render = self.generate_line_to_render(corrected_item_idx, item);
            lines_to_render.push(line_to_render);
        }

        lines_to_render
    }

    fn generate_line_to_render(&self, item_idx: usize, item: &MenuItemEnum) -> String {
        let selection_str: &str = if item_idx == self.menu.selected_item_idx {
            "→"
        } else {
            " "
        };
        let label = match item {
            MenuItemEnum::BasicMenuItem(basic_menu_item) => basic_menu_item.get_label()
        };
        let max_length_label = self.menu.char_width - 2;
        let label_trimmed = if label.len() > max_length_label {
            &label[..max_length_label]
        } else {
            label
        };

        let bottom_item_idx = self.menu.top_visible_item_idx + self.menu.char_height - 1;
        let arrow_str: &str = if item_idx == self.menu.top_visible_item_idx{
            if self.menu.top_visible_item_idx != 0 {
                "↑"
            } else {
                " "
            }
        } else if item_idx == bottom_item_idx {
            if bottom_item_idx < self.menu.items.len() - 1 {
                "↓"
            } else {
                " "
            }
        } else {
            " "
        };

        // TODO scrolling if overflow

        format!("{}{:3$}{}", selection_str, label_trimmed, arrow_str, max_length_label)
    }

    pub fn tick(&mut self, tick_id: usize) {
        if tick_id != self.last_tick_id {
            let lines_to_render = self.generate_lines_to_render();
            if !lines_to_render.eq(&self.last_rendered_lines) {
                println!("=============================================================================");
                for line in lines_to_render.iter() {
                    println!("{}", line);
                }
                self.last_rendered_lines = lines_to_render;
            }

            self.last_tick_id = tick_id;
        }
    }
}

// TODO tests