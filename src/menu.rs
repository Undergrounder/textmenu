use crate::keyboard::{FunctionKey, KeyboardKey};
use crate::menu_items::menu_item::MenuItem;
use crate::menu_items::submenu_menu_item::SubmenuMenuItem;

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use core::option::Option::{None, Some};
use core::result::Result;
use core::result::Result::Ok;

use core::fmt::Debug;
use core::prelude::rust_2024::derive;

pub struct Menu {
    submenu_menu_item: SubmenuMenuItem,
}

#[derive(Debug)]
pub enum NewError {
    InvalidItemsLength,
}

impl Menu {
    pub fn new(items: Vec<Box<dyn MenuItem>>) -> Result<Menu, NewError> {
        let submenu = SubmenuMenuItem::new(String::from("Root"), items)
            .map_err(|_| NewError::InvalidItemsLength)?;
        Ok(Menu {
            submenu_menu_item: submenu,
        })
    }

    pub fn get_submenu_menu_item(&self) -> &SubmenuMenuItem {
        &self.submenu_menu_item
    }

    pub fn press(&mut self, key: KeyboardKey) -> bool {
        self.submenu_menu_item.press(&key, true).handled
    }

    pub fn enter(&mut self) -> bool {
        self.press(KeyboardKey::new(Some(FunctionKey::ENTER), None))
    }

    pub fn up(&mut self) -> bool {
        self.press(KeyboardKey::new(Some(FunctionKey::UP), None))
    }

    pub fn down(&mut self) -> bool {
        self.press(KeyboardKey::new(Some(FunctionKey::DOWN), None))
    }

    pub fn left(&mut self) -> bool {
        self.press(KeyboardKey::new(Some(FunctionKey::LEFT), None))
    }

    pub fn right(&mut self) -> bool {
        self.press(KeyboardKey::new(Some(FunctionKey::RIGHT), None))
    }

    pub fn back(&mut self) -> bool {
        self.press(KeyboardKey::new(Some(FunctionKey::BACK), None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menu_items::action_menu_item::ActionMenuItem;
    use crate::menu_items::basic_menu_item::BasicMenuItem;
    use crate::menu_items::list_menu_item::ListMenuItem;
    use crate::menu_items::range_menu_item::RangeMenuItem;
    use crate::menu_items::toggle_menu_item::ToggleMenuItem;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn assert_submenu(
        submenu: &SubmenuMenuItem,
        expected_item_count: usize,
        expected_idx: usize,
        expected_is_focused: bool,
    ) -> () {
        assert_eq!(submenu.item_count(), expected_item_count);
        assert_eq!(submenu.get_selected_item_idx(), expected_idx);
        assert_eq!(submenu.is_focused(), expected_is_focused);
    }

    fn assert_submenu_state(
        menu: &Menu,
        expected_item_count: usize,
        expected_idx: usize,
        expected_is_focused: bool,
    ) -> () {
        let submenu = menu.get_submenu_menu_item();
        assert_submenu(
            submenu,
            expected_item_count,
            expected_idx,
            expected_is_focused,
        );
    }

    #[test]
    fn can_create_simple_menu() {
        let items: Vec<Box<dyn MenuItem>> =
            vec![Box::new(BasicMenuItem::new(String::from("Item1")))];
        let menu = Menu::new(items).unwrap();

        assert_submenu_state(&menu, 1, 0, false);
    }

    #[test]
    fn can_create_big_menu() {
        let items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(BasicMenuItem::new(String::from("Item1"))),
            Box::new(BasicMenuItem::new(String::from("Item2"))),
            Box::new(BasicMenuItem::new(String::from("Item3"))),
            Box::new(BasicMenuItem::new(String::from("Item4"))),
            Box::new(BasicMenuItem::new(String::from("Item5"))),
            Box::new(BasicMenuItem::new(String::from("Item6"))),
            Box::new(BasicMenuItem::new(String::from("Item7"))),
            Box::new(BasicMenuItem::new(String::from("Item8"))),
        ];
        let mut menu = Menu::new(items).unwrap();
        assert_submenu_state(&menu, 8, 0, false);

        assert_eq!(menu.down(), true);
        assert_submenu_state(&menu, 8, 1, false);

        assert_eq!(menu.down(), true);
        assert_submenu_state(&menu, 8, 2, false);

        assert_eq!(menu.down(), true);
        assert_submenu_state(&menu, 8, 3, false);

        assert_eq!(menu.down(), true);
        assert_submenu_state(&menu, 8, 4, false);

        assert_eq!(menu.down(), true);
        assert_submenu_state(&menu, 8, 5, false);
    }

    #[test]
    fn can_create_complex_menu() {
        let items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(BasicMenuItem::new(String::from("Item1"))),
            Box::new(BasicMenuItem::new(String::from("Item2"))),
            Box::new(BasicMenuItem::new(String::from("Item3"))),
            Box::new(BasicMenuItem::new(String::from("Item4"))),
            Box::new(BasicMenuItem::new(String::from("Item5"))),
        ];
        let mut menu = Menu::new(items).unwrap();

        assert_submenu_state(&menu, 5, 0, false);

        assert_eq!(menu.up(), false);
        assert_submenu_state(&menu, 5, 0, false);

        assert_eq!(menu.down(), true);
        assert_submenu_state(&menu, 5, 1, false);

        assert_eq!(menu.down(), true);
        assert_submenu_state(&menu, 5, 2, false);

        assert_eq!(menu.down(), true);
        assert_submenu_state(&menu, 5, 3, false);

        assert_eq!(menu.down(), true);
        assert_submenu_state(&menu, 5, 4, false);

        assert_eq!(menu.down(), false);
        assert_submenu_state(&menu, 5, 4, false);
    }

    #[test]
    fn basic_item_is_usable() {
        let items: Vec<Box<dyn MenuItem>> =
            vec![Box::new(BasicMenuItem::new(String::from("Item1")))];
        let mut menu = Menu::new(items).unwrap();

        assert_submenu_state(&menu, 1, 0, false);

        assert_eq!(menu.enter(), false);
        assert_submenu_state(&menu, 1, 0, false);
    }

    #[test]
    fn action_item_is_usable() {
        let clicked_count = Rc::new(RefCell::new(0));
        let clicked_count_clone = Rc::clone(&clicked_count);
        let on_click = move || {
            *clicked_count_clone.borrow_mut() += 1;
            true
        };

        let items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(ActionMenuItem::new(
                String::from("Item1"),
                Box::new(on_click),
            )),
            Box::new(BasicMenuItem::new(String::from("Item2"))),
        ];
        let mut menu = Menu::new(items).unwrap();

        assert_submenu_state(&menu, 2, 0, false);
        assert_eq!(*clicked_count.borrow(), 0);

        assert_eq!(menu.enter(), true);
        assert_eq!(*clicked_count.borrow(), 1);
    }

    fn assert_list_menu_item(
        item: &ListMenuItem,
        is_focused: bool,
        expected_label: &str,
        expected_selected_entry_idx: usize,
        expected_selected_entry: &str,
    ) {
        assert_eq!(item.get_label(is_focused), expected_label);
        assert_eq!(item.get_selected_entry_idx(), expected_selected_entry_idx);
        assert_eq!(item.get_selected_entry(), expected_selected_entry);
    }

    fn assert_focused_list_menu_item_state(
        menu: &Menu,
        expected_label: &str,
        expected_selected_entry_idx: usize,
        expected_selected_entry: &str,
    ) {
        let submenu_menu_item = menu.get_submenu_menu_item();
        let selected_item = submenu_menu_item.get_selected_item();
        if let Some(list_menu_item) = selected_item.as_any().downcast_ref::<ListMenuItem>() {
            assert_list_menu_item(
                list_menu_item,
                submenu_menu_item.is_focused(),
                expected_label,
                expected_selected_entry_idx,
                expected_selected_entry,
            );
        } else {
            panic!("Selected item must be of type ListMenuItem");
        }
    }

    #[test]
    fn list_item_is_usable() {
        let list_entries = vec![
            String::from("Elem1"),
            String::from("Elem2"),
            String::from("Elem3"),
        ];

        let items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(ListMenuItem::new(String::from("Item1"), list_entries).unwrap()),
            Box::new(BasicMenuItem::new(String::from("Item2"))),
        ];
        let mut menu = Menu::new(items).unwrap();

        assert_submenu_state(&menu, 2, 0, false);
        assert_focused_list_menu_item_state(&menu, "Item1: Elem1", 0, "Elem1");

        assert_eq!(menu.left(), false);
        assert_eq!(menu.right(), false);

        assert_eq!(menu.enter(), true);
        assert_submenu_state(&menu, 2, 0, true);
        assert_focused_list_menu_item_state(&menu, "Item1: Elem1", 0, "Elem1");

        // Can't move while focused
        assert_eq!(menu.up(), false);
        assert_eq!(menu.down(), false);

        assert_eq!(menu.right(), true);
        assert_submenu_state(&menu, 2, 0, true);
        assert_focused_list_menu_item_state(&menu, "Item1: Elem2", 0, "Elem1");

        assert_eq!(menu.back(), true);
        assert_submenu_state(&menu, 2, 0, false);
        assert_focused_list_menu_item_state(&menu, "Item1: Elem1", 0, "Elem1");

        assert_eq!(menu.enter(), true);
        assert_submenu_state(&menu, 2, 0, true);
        assert_focused_list_menu_item_state(&menu, "Item1: Elem1", 0, "Elem1");

        assert_eq!(menu.left(), true);
        assert_submenu_state(&menu, 2, 0, true);
        assert_focused_list_menu_item_state(&menu, "Item1: Elem3", 0, "Elem1");

        menu.enter();
        assert_submenu_state(&menu, 2, 0, false);
        assert_focused_list_menu_item_state(&menu, "Item1: Elem3", 2, "Elem3");
    }

    fn assert_toggle_menu_item(
        item: &ToggleMenuItem,
        is_focused: bool,
        expected_label: &str,
        expected_value: bool,
    ) {
        assert_eq!(item.get_label(is_focused), expected_label);
        assert_eq!(item.get_value(), expected_value);
    }

    fn get_required_toggle(item: &dyn MenuItem) -> &ToggleMenuItem {
        if let Some(toggle_menu_item) = item.as_any().downcast_ref::<ToggleMenuItem>() {
            toggle_menu_item
        } else {
            panic!("Selected item must be of type ToggleMenuItem");
        }
    }

    fn assert_focused_toggle_menu_item_state(
        menu: &Menu,
        expected_label: &str,
        expected_value: bool,
    ) {
        let submenu_menu_item = menu.get_submenu_menu_item();
        let selected_item = submenu_menu_item.get_selected_item();
        let toggle_menu_item = get_required_toggle(selected_item);
        assert_toggle_menu_item(
            toggle_menu_item,
            submenu_menu_item.is_focused(),
            expected_label,
            expected_value,
        );
    }

    #[test]
    fn toggle_item_is_usable() {
        let items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(ToggleMenuItem::new(String::from("Item1"))),
            Box::new(BasicMenuItem::new(String::from("Item2"))),
        ];
        let mut menu = Menu::new(items).unwrap();

        assert_focused_toggle_menu_item_state(&menu, "Item1: OFF", false);
        assert_submenu_state(&menu, 2, 0, false);

        assert_eq!(menu.enter(), true);
        assert_submenu_state(&menu, 2, 0, false);
        assert_focused_toggle_menu_item_state(&menu, "Item1: ON", true);

        assert_eq!(menu.enter(), true);
        assert_submenu_state(&menu, 2, 0, false);
        assert_focused_toggle_menu_item_state(&menu, "Item1: OFF", false);
    }

    fn get_required_range(item: &dyn MenuItem) -> &RangeMenuItem {
        if let Some(range_menu_item) = item.as_any().downcast_ref::<RangeMenuItem>() {
            range_menu_item
        } else {
            panic!("Selected item must be of type RangeMenuItem");
        }
    }

    fn assert_focused_range_menu_item_state(
        menu: &Menu,
        expected_label: &str,
        expected_value: u32,
    ) {
        let submenu_menu_item = menu.get_submenu_menu_item();
        let selected_item = submenu_menu_item.get_selected_item();
        let range_menu_item = get_required_range(selected_item);
        assert_eq!(
            range_menu_item.get_label(submenu_menu_item.is_focused()),
            expected_label
        );
        assert_eq!(range_menu_item.get_value(), expected_value);
    }

    #[test]
    fn range_item_is_usable() {
        let items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(RangeMenuItem::new(String::from("Item1"), 3, 10, 1).unwrap()),
            Box::new(BasicMenuItem::new(String::from("Item2"))),
        ];
        let mut menu = Menu::new(items).unwrap();

        assert_submenu_state(&menu, 2, 0, false);
        assert_focused_range_menu_item_state(&menu, "Item1: 3", 3);

        assert_eq!(menu.enter(), true);
        assert_submenu_state(&menu, 2, 0, true);
        assert_focused_range_menu_item_state(&menu, "Item1: 3", 3);

        assert_eq!(menu.left(), true);
        assert_submenu_state(&menu, 2, 0, true);
        assert_focused_range_menu_item_state(&menu, "Item1: 10", 3);

        assert_eq!(menu.left(), true);
        assert_submenu_state(&menu, 2, 0, true);
        assert_focused_range_menu_item_state(&menu, "Item1: 9", 3);

        assert_eq!(menu.right(), true);
        assert_submenu_state(&menu, 2, 0, true);
        assert_focused_range_menu_item_state(&menu, "Item1: 10", 3);

        assert_eq!(menu.enter(), true);
        assert_submenu_state(&menu, 2, 0, false);
        assert_focused_range_menu_item_state(&menu, "Item1: 10", 10);
    }

    fn get_required_submenu(item: &dyn MenuItem) -> &SubmenuMenuItem {
        if let Some(submenu_menu_item) = item.as_any().downcast_ref::<SubmenuMenuItem>() {
            submenu_menu_item
        } else {
            panic!("Selected item must be of type SubmenuMenuItem");
        }
    }

    fn assert_focused_submenu_menu_item_state(
        menu: &Menu,
        expected_item_count: usize,
        expected_idx: usize,
        expected_is_focused: bool,
    ) {
        let submenu_menu_item = menu.get_submenu_menu_item();
        let selected_item = submenu_menu_item.get_selected_item();
        let submenu_item = get_required_submenu(selected_item);
        assert_submenu(
            submenu_item,
            expected_item_count,
            expected_idx,
            expected_is_focused,
        );
    }

    fn assert_focused_lvl2_toggle_menu_item_state(
        menu: &Menu,
        expected_label: &str,
        expected_value: bool,
    ) {
        let submenu_menu_item = menu.get_submenu_menu_item();
        let selected_item = submenu_menu_item.get_selected_item();
        let submenu = get_required_submenu(selected_item);
        let selected_lvl2_item = submenu.get_selected_item();
        let toggle_menu_item = get_required_toggle(selected_lvl2_item);
        assert_toggle_menu_item(
            toggle_menu_item,
            submenu_menu_item.is_focused(),
            expected_label,
            expected_value,
        );
    }

    fn assert_focused_lvl2_submenu_menu_item_state(
        menu: &Menu,
        expected_item_count: usize,
        expected_idx: usize,
        expected_is_focused: bool,
    ) {
        let submenu_menu_item = menu.get_submenu_menu_item();
        let submenu_item = get_required_submenu(submenu_menu_item.get_selected_item());
        let selected_lvl2_item = submenu_item.get_selected_item();
        let submenu_lvl2_item = get_required_submenu(selected_lvl2_item);
        assert_submenu(
            submenu_lvl2_item,
            expected_item_count,
            expected_idx,
            expected_is_focused,
        );
    }

    #[test]
    fn submenu_is_usable() {
        let submenu2_items: Vec<Box<dyn MenuItem>> =
            vec![Box::new(BasicMenuItem::new(String::from("Sub2 Item1")))];
        let subitem2 = SubmenuMenuItem::new(String::from("Sub Item2"), submenu2_items).unwrap();
        let submenu1_items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(ToggleMenuItem::new(String::from("Sub Item1"))),
            Box::new(subitem2),
        ];
        let item1 = SubmenuMenuItem::new(String::from("Item1"), submenu1_items).unwrap();
        let items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(item1),
            Box::new(BasicMenuItem::new(String::from("Item2"))),
        ];
        let mut menu = Menu::new(items).unwrap();

        assert_submenu_state(&menu, 2, 0, false);

        assert_eq!(menu.enter(), true);
        assert_submenu_state(&menu, 2, 0, true);
        assert_focused_submenu_menu_item_state(&menu, 2, 0, false);

        assert_eq!(menu.enter(), true);
        assert_submenu_state(&menu, 2, 0, true);
        assert_focused_submenu_menu_item_state(&menu, 2, 0, false);
        assert_focused_lvl2_toggle_menu_item_state(&menu, "Sub Item1: ON", true);

        assert_eq!(menu.down(), true);
        assert_submenu_state(&menu, 2, 0, true);
        assert_focused_submenu_menu_item_state(&menu, 2, 1, false);

        assert_eq!(menu.enter(), true);
        assert_submenu_state(&menu, 2, 0, true);
        assert_focused_submenu_menu_item_state(&menu, 2, 1, true);
        assert_focused_lvl2_submenu_menu_item_state(&menu, 1, 0, false);

        assert_eq!(menu.back(), true);
        assert_submenu_state(&menu, 2, 0, true);
        assert_focused_submenu_menu_item_state(&menu, 2, 1, false);

        assert_eq!(menu.back(), true);
        assert_submenu_state(&menu, 2, 0, false);
    }
}
