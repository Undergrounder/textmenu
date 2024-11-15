use textmenu::menu::Menu;
use textmenu::menu_items::basic_menu_item::BasicMenuItem;
use textmenu::menu_items::menu_item_enum::MenuItemEnum;
use textmenu::renderers::console::ConsoleRenderer;

fn main() {
    let mut items = [
        MenuItemEnum::BasicMenuItem(BasicMenuItem::new("Cuenta atras")), // TODO
        MenuItemEnum::BasicMenuItem(BasicMenuItem::new("Dominación")),   // TODO
        MenuItemEnum::BasicMenuItem(BasicMenuItem::new("Ajustes")),      // TODO
    ];
    let menu: Menu<16, 2> = Menu::new(&mut items).unwrap();
    let mut console_renderer = ConsoleRenderer::new(&menu);

    let mut tick_id: usize = 0;
    loop {
        // TODO update based on input
        console_renderer.tick(tick_id);

        tick_id = tick_id.checked_add(1).unwrap_or_else(|| 0)
    }
}
