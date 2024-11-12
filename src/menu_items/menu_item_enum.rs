use crate::menu_items::action_menu_item::ActionMenuItem;
use crate::menu_items::basic_menu_item::BasicMenuItem;

pub enum MenuItemEnum<'a> {
    BasicMenuItem(BasicMenuItem),
    ActionMenuItem(ActionMenuItem<'a>),
}
