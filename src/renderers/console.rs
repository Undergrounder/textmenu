use heapless::String;
use crate::menu::Menu;

pub struct ConsoleRenderer<'a, const char_width_const: usize, const char_height_const: usize> {
    last_tick_id: usize,
    last_rendered_lines: Option<[String<char_width_const>;char_height_const]>,
    menu: &'a Menu<'a, char_width_const, char_height_const>,
}

impl<'a, const char_width_const: usize, const char_height_const: usize> ConsoleRenderer<'a, char_width_const, char_height_const> {
    pub fn new(menu: &'a Menu<'a, char_width_const, char_height_const>) -> ConsoleRenderer<'a, char_width_const, char_height_const> {
        ConsoleRenderer {
            last_tick_id: usize::MAX,
            menu: &menu,
            last_rendered_lines: None,
        }
    }

    pub fn tick(&mut self, tick_id: usize) {
        if tick_id != self.last_tick_id {
            let lines_to_render = self.menu.generate_lines_to_render();
            if !lines_to_render.eq(&self.last_rendered_lines) {
                println!(
                    "============================================================================="
                );
                for line in lines_to_render.iter() {
                    println!("{}", line);
                }
                self.last_rendered_lines = lines_to_render;
            }

            self.last_tick_id = tick_id;
        }
    }
}
