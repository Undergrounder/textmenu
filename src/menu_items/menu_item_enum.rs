use crate::menu_items::action_menu_item::ActionMenuItem;
use crate::menu_items::basic_menu_item::BasicMenuItem;
use crate::menu_items::list_menu_item::ListMenuItem;
use crate::menu_items::toggle_menu_item::ToggleMenuItem;
use enum_dispatch::enum_dispatch;
use crate::menu_items::menu_item::MenuItem;

#[enum_dispatch]
pub enum MenuItemEnum<'a> {
    BasicMenuItem(BasicMenuItem),
    ActionMenuItem(ActionMenuItem<'a>),
    ListMenuItem(ListMenuItem),
    ToggleMenuItem(ToggleMenuItem),
}
