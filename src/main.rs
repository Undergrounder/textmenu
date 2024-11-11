use textmenu::basic_menu_item::BasicMenuItem;
use textmenu::console::ConsoleRenderer;
use textmenu::menu::Menu;
use textmenu::menu_item_enum::MenuItemEnum;

fn main() {
    let items: Vec<MenuItemEnum> = vec![
        MenuItemEnum::BasicMenuItem(BasicMenuItem::new(String::from("Cuenta atras"))), // TODO
        MenuItemEnum::BasicMenuItem(BasicMenuItem::new(String::from("Dominación"))),   // TODO
        MenuItemEnum::BasicMenuItem(BasicMenuItem::new(String::from("Ajustes"))),      // TODO
    ];
    let menu = Menu::new(16, 2, items).unwrap();
    let mut console_renderer = ConsoleRenderer::new(&menu);

    let mut tick_id: usize = 0;
    loop {
        // TODO update based on input
        console_renderer.tick(tick_id);

        tick_id = tick_id.checked_add(1).unwrap_or_else(|| 0)
    }
}
