use crate::menu::Menu;
use crate::menu_item::MenuItem;
use crate::menu_item_enum::MenuItemEnum;

// TODO move char width and height
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
        let mut lines_to_render: Vec<String> = Vec::with_capacity(self.menu.char_height);
        // TODO overflow
        // TODO render arrows
        // TODO scrolling if overflow
        for (item_idx, item) in self.menu.items.iter().enumerate() {
            let selection_str: &str = if item_idx == self.menu.focused_item_idx {
                "→"
            } else {
                " "
            };
            let label = match item {
                MenuItemEnum::BasicMenuItem(basic_menu_item) => basic_menu_item.get_label()
            };

            let mut line_to_render = selection_str.to_owned();
            line_to_render.push_str(label);
            lines_to_render.push(line_to_render);
        }

        lines_to_render
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