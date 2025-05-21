use crate::menu_items::action_menu_item::ActionMenuItem;
use crate::menu_items::basic_menu_item::BasicMenuItem;
use crate::menu_items::list_menu_item::ListMenuItem;
use crate::menu_items::range_menu_item::RangeMenuItem;
use crate::menu_items::submenu_menu_item::SubmenuMenuItem;
use crate::menu_items::toggle_menu_item::ToggleMenuItem;

pub enum MenuItemKind<'a> {
    ActionMenuItem(&'a ActionMenuItem),
    BasicMenuItem(&'a BasicMenuItem),
    ListMenuItem(&'a ListMenuItem),
    RangeMenuItem(&'a RangeMenuItem),
    ToggleMenuItem(&'a ToggleMenuItem),
    SubmenuMenuItem(&'a SubmenuMenuItem),
}
