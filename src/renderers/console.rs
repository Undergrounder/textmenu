use crate::menu::Menu;

pub struct ConsoleRenderer<'a> {
    last_tick_id: usize,
    last_rendered_lines: Vec<String>,
    menu: &'a Menu<'a>,
}

impl<'a> ConsoleRenderer<'a> {
    pub fn new(menu: &'a Menu<'a>) -> ConsoleRenderer<'a> {
        ConsoleRenderer {
            last_tick_id: usize::MAX,
            menu: &menu,
            last_rendered_lines: vec![],
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
