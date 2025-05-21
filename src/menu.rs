use crate::keyboard::{FunctionKey, KeyboardKey};
use crate::menu_items::menu_item::MenuItem;
use crate::menu_items::submenu_menu_item::SubmenuMenuItem;

pub struct Menu {
    submenu_menu_item: SubmenuMenuItem,
}

impl Menu {
    pub fn new(items: Vec<Box<dyn MenuItem>>) -> Result<Menu, &'static str> {
        if items.len() == 0 {
            Err("At least 1 menu item required.")
        } else {
            let menu = Menu {
                submenu_menu_item: SubmenuMenuItem::new(String::from("Root"), items),
            };
            Ok(menu)
        }
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

/*
TODO
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

fn assert_submenu_state<'a: 'b, 'b>(menu: &'a Menu<'b>, expected_item_count: usize, expected_idx: usize) -> () {
    let submenu = menu.get_submenu_menu_item();
    assert_eq!(submenu.item_count(), expected_item_count);
    assert_eq!(submenu.get_selected_item_idx(), expected_idx);
    assert_eq!(submenu.is_focused(), false);
}

#[test]
fn can_create_simple_menu() {
    let mut items: [&mut dyn MenuItem; 1] = [&mut BasicMenuItem::new("Item1")];
    let menu = Menu::new(&mut items).unwrap();

    assert_submenu_state(&menu, 1, 0);
}

#[test]
fn can_create_big_menu() {
    let mut item1 = BasicMenuItem::new("Item1");
    let mut items: [&mut dyn MenuItem; 1] = [
        &mut item1,
    ];
    let mut menu: Menu = Menu::new(&mut items).unwrap();
    assert_submenu_state(&menu, 8, 0);

    menu.down();
    /*
    TODO
    {
        assert_submenu_state(&menu, 8, 1);
    }

    menu.down();
    {
        assert_submenu_state(&menu, 8, 2);
    }

    menu.down();
    {
        assert_submenu_state(&menu, 8, 3);
    }

    menu.down();
    {
        assert_submenu_state(&menu, 8, 4);
    }

    menu.down();
    {
        assert_submenu_state(&menu, 8, 5);
    }
     */
}

#[test]
fn can_create_complex_menu() {
    let mut items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 5] = [
        &mut BasicMenuItem::new("Item1"),
        &mut BasicMenuItem::new("Item2"),
        &mut BasicMenuItem::new("Item3"),
        &mut BasicMenuItem::new("Item4"),
        &mut BasicMenuItem::new("Item5"),
    ];
    let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
    assert_eq!(menu.char_width, 16);
    assert_eq!(menu.char_height, 2);

    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Item1          ");
    assert_eq!(lines_to_render[1], " Item2         ↓");

    assert_eq!(menu.up(), false);
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Item1          ");
    assert_eq!(lines_to_render[1], " Item2         ↓");

    assert_eq!(menu.down(), true);
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], " Item1          ");
    assert_eq!(lines_to_render[1], "→Item2         ↓");

    assert_eq!(menu.down(), true);
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Item3         ↑");
    assert_eq!(lines_to_render[1], " Item4         ↓");

    assert_eq!(menu.down(), true);
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], " Item3         ↑");
    assert_eq!(lines_to_render[1], "→Item4         ↓");

    assert_eq!(menu.down(), true);
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 1);
    assert_eq!(lines_to_render[0], "→Item5         ↑");

    assert_eq!(menu.up(), true);
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], " Item3         ↑");
    assert_eq!(lines_to_render[1], "→Item4         ↓");
}

#[test]
fn panics_if_invalid_char_width() {
    let mut items: [&mut dyn MenuItem<2, { 1 * BYTES_PER_CHAR }>; 1] =
        [&mut BasicMenuItem::new("Item1")];
    let menu_result: Result<Menu<1, 2, { 1 * BYTES_PER_CHAR }>, &str> = Menu::new(&mut items);
    if let Err(error) = menu_result {
        assert_eq!(error, "Invalid menu char width. At least 3 chars required.")
    } else {
        panic!("It should return an error");
    }
}

#[test]
fn panics_if_invalid_char_height() {
    let mut items: [&mut dyn MenuItem<1, { 16 * BYTES_PER_CHAR }>; 1] =
        [&mut BasicMenuItem::new("Item1")];
    let menu_result: Result<Menu<16, 1, { 16 * BYTES_PER_CHAR }>, &str> = Menu::new(&mut items);
    if let Err(error) = menu_result {
        assert_eq!(
            error,
            "Invalid menu char height. At least 2 chars required."
        )
    } else {
        panic!("It should return an error");
    }
}

#[test]
fn basic_item_is_usable() {
    let mut items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 1] =
        [&mut BasicMenuItem::new("Item1")];
    let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
    assert_eq!(menu.char_width, 16);
    assert_eq!(menu.char_height, 2);

    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 1);
    assert_eq!(lines_to_render[0], "→Item1          ");

    menu.enter();
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 1);
    assert_eq!(lines_to_render[0], "→Item1          ");
}

#[test]
fn action_item_is_usable() {
    let clicked_count = Rc::new(RefCell::new(0));
    let clicked_count_clone = Rc::clone(&clicked_count);
    let mut on_click = move || {
        *clicked_count_clone.borrow_mut() += 1;
        true
    };
    let mut items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 2] = [
        &mut ActionMenuItem::new("Item1", &mut on_click),
        &mut BasicMenuItem::new("Item2"),
    ];
    let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
    assert_eq!(menu.char_width, 16);
    assert_eq!(menu.char_height, 2);

    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Item1          ");
    assert_eq!(lines_to_render[1], " Item2          ");
    assert_eq!(*clicked_count.borrow(), 0);

    menu.enter();
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Item1          ");
    assert_eq!(lines_to_render[1], " Item2          ");
    assert_eq!(*clicked_count.borrow(), 1);
}

#[test]
fn list_item_is_usable() {
    let list_entries = ["Elem1", "Elem2", "Elem3"];

    let mut items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 2] = [
        &mut ListMenuItem::new("Item1", &list_entries).unwrap(),
        &mut BasicMenuItem::new("Item2"),
    ];

    let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
    assert_eq!(menu.char_width, 16);
    assert_eq!(menu.char_height, 2);

    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Item1: Elem1   ");
    assert_eq!(lines_to_render[1], " Item2          ");
    assert_eq!(menu.left(), false);
    assert_eq!(menu.right(), false);

    menu.enter();
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "←Item1: Elem1   ");
    assert_eq!(lines_to_render[1], " Item2          ");

    // Can't move while focused
    assert_eq!(menu.up(), false);
    assert_eq!(menu.down(), false);

    assert_eq!(menu.right(), true);
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "←Item1: Elem2   ");
    assert_eq!(lines_to_render[1], " Item2          ");

    assert_eq!(menu.back(), true);
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Item1: Elem1   ");
    assert_eq!(lines_to_render[1], " Item2          ");

    menu.enter();
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "←Item1: Elem1   ");
    assert_eq!(lines_to_render[1], " Item2          ");

    assert_eq!(menu.left(), true);
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "←Item1: Elem3   ");
    assert_eq!(lines_to_render[1], " Item2          ");

    menu.enter();
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Item1: Elem3   ");
    assert_eq!(lines_to_render[1], " Item2          ");
}

#[test]
fn toggle_item_is_usable() {
    let mut items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 2] = [
        &mut ToggleMenuItem::new("Item1"),
        &mut BasicMenuItem::new("Item2"),
    ];
    let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
    assert_eq!(menu.char_width, 16);
    assert_eq!(menu.char_height, 2);

    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Item1: OFF     ");
    assert_eq!(lines_to_render[1], " Item2          ");

    menu.enter();
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Item1: ON      ");
    assert_eq!(lines_to_render[1], " Item2          ");

    menu.enter();
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Item1: OFF     ");
    assert_eq!(lines_to_render[1], " Item2          ");
}

#[test]
fn range_item_is_usable() {
    let mut items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 2] = [
        &mut RangeMenuItem::new("Item1", 3, 10, 1).unwrap(),
        &mut BasicMenuItem::new("Item2"),
    ];
    let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
    assert_eq!(menu.char_width, 16);
    assert_eq!(menu.char_height, 2);

    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Item1: 3       ");
    assert_eq!(lines_to_render[1], " Item2          ");

    menu.enter();
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "←Item1: 3       ");
    assert_eq!(lines_to_render[1], " Item2          ");

    menu.left();
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "←Item1: 10      ");
    assert_eq!(lines_to_render[1], " Item2          ");

    menu.left();
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "←Item1: 9       ");
    assert_eq!(lines_to_render[1], " Item2          ");

    menu.right();
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "←Item1: 10      ");
    assert_eq!(lines_to_render[1], " Item2          ");

    menu.enter();
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Item1: 10      ");
    assert_eq!(lines_to_render[1], " Item2          ");
}

#[test]
fn submenu_is_usable() {
    let mut submenu2_items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 1] =
        [&mut BasicMenuItem::new("Sub2 Item1")];

    let mut subitem2: SubmenuMenuItem<16, 2, { 16 * BYTES_PER_CHAR }> =
        SubmenuMenuItem::new("Sub Item2", &mut submenu2_items);

    let mut submenu1_items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 2] =
        [&mut ToggleMenuItem::new("Sub Item1"), &mut subitem2];

    let mut item1: SubmenuMenuItem<16, 2, { 16 * BYTES_PER_CHAR }> =
        SubmenuMenuItem::new("Item1", &mut submenu1_items);
    let mut items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 2] =
        [&mut item1, &mut BasicMenuItem::new("Item2")];
    let mut menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
    assert_eq!(menu.char_width, 16);
    assert_eq!(menu.char_height, 2);

    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Item1          ");
    assert_eq!(lines_to_render[1], " Item2          ");

    assert_eq!(menu.enter(), true);
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Sub Item1: OFF ");
    assert_eq!(lines_to_render[1], " Sub Item2      ");

    assert_eq!(menu.enter(), true);
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Sub Item1: ON  ");
    assert_eq!(lines_to_render[1], " Sub Item2      ");

    assert_eq!(menu.down(), true);
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], " Sub Item1: ON  ");
    assert_eq!(lines_to_render[1], "→Sub Item2      ");

    assert_eq!(menu.enter(), true);
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 1);
    assert_eq!(lines_to_render[0], "→Sub2 Item1     ");

    assert_eq!(menu.back(), true);
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], " Sub Item1: ON  ");
    assert_eq!(lines_to_render[1], "→Sub Item2      ");

    assert_eq!(menu.back(), true);
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Item1          ");
    assert_eq!(lines_to_render[1], " Item2          ");

    assert_eq!(menu.back(), true);
    let lines_to_render = menu.generate_lines_to_render();
    assert_eq!(lines_to_render.len(), 2);
    assert_eq!(lines_to_render[0], "→Item1          ");
    assert_eq!(lines_to_render[1], " Item2          ");
}
}
 */

// TODO improvements:
// TODO input item
// TODO charset input item
// TODO horizontal scrolling if overflow
// TODO disable functionality via features
// TODO more range items (u8, signed, float, ....)
