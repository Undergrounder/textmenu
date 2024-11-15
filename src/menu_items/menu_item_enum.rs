use crate::menu_items::action_menu_item::ActionMenuItem;
use crate::menu_items::basic_menu_item::BasicMenuItem;
use crate::menu_items::list_menu_item::ListMenuItem;
use crate::menu_items::menu_item::{MenuItem, LABEL_BYTES};
use crate::menu_items::range_menu_item::RangeMenuItem;
use crate::menu_items::toggle_menu_item::ToggleMenuItem;
use enum_dispatch::enum_dispatch;
use heapless::String;

#[enum_dispatch]
pub enum MenuItemEnum<'a> {
    ActionMenuItem(ActionMenuItem<'a>),
    BasicMenuItem(BasicMenuItem<'a>),
    ListMenuItem(ListMenuItem<'a>),
    RangeMenuItem(RangeMenuItem<'a>),
    ToggleMenuItem(ToggleMenuItem<'a>),
}
