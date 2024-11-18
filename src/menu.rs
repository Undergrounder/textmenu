use crate::consts::BYTES_PER_CHAR;
use crate::keyboard::{FunctionKey, KeyboardKey};
use crate::menu_items::menu_item::MenuItem;
use crate::menu_items::submenu_menu_item::SubmenuMenuItem;
use heapless::{String, Vec};

pub struct Menu<
    'a,
    const CHAR_WIDTH_CONST: usize,
    const CHAR_HEIGHT_CONST: usize,
    const LINE_BYTES_SIZE_CONST: usize,
> {
    pub char_width: usize,
    pub char_height: usize,
    submenu_menu_item:
        SubmenuMenuItem<'a, CHAR_WIDTH_CONST, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>,
}

impl<
        'a,
        const CHAR_WIDTH_CONST: usize,
        const CHAR_HEIGHT_CONST: usize,
        const LINE_BYTES_SIZE_CONST: usize,
    > Menu<'a, CHAR_WIDTH_CONST, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>
{
    pub fn new(
        items: &'a mut [&'a mut dyn MenuItem<'a, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>],
    ) -> Result<Menu<'a, CHAR_WIDTH_CONST, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>, &'static str>
    {
        if LINE_BYTES_SIZE_CONST != CHAR_WIDTH_CONST * BYTES_PER_CHAR {
            Err("LINE_BYTES_SIZE_CONST must be equal to CHAR_WIDTH_CONST*BYTES_PER_CHAR.")
        } else if items.len() == 0 {
            Err("At least 1 menu item required.")
        } else if CHAR_WIDTH_CONST < 3 {
            Err("Invalid menu char width. At least 3 chars required.")
        } else if CHAR_HEIGHT_CONST < 2 {
            Err("Invalid menu char height. At least 2 chars required.")
        } else {
            let menu = Menu {
                char_width: CHAR_WIDTH_CONST,
                char_height: CHAR_HEIGHT_CONST,
                submenu_menu_item: SubmenuMenuItem::new("Root", items),
            };
            Ok(menu)
        }
    }

    pub fn generate_lines_to_render(
        &self,
    ) -> Vec<String<LINE_BYTES_SIZE_CONST>, CHAR_HEIGHT_CONST> {
        self.submenu_menu_item.generate_lines_to_render().unwrap()
    }

    pub fn press(&mut self, key: KeyboardKey) -> bool {
        self.submenu_menu_item.press(&key, false).handled
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

    #[test]
    fn can_create_simple_menu() {
        let mut items: [&mut dyn MenuItem<2, { 16 * BYTES_PER_CHAR }>; 1] =
            [&mut BasicMenuItem::new("Item1")];
        let menu: Menu<16, 2, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 1);
        assert_eq!(lines_to_render[0], "→Item1          ");
    }

    #[test]
    fn can_create_big_menu() {
        let mut items: [&mut dyn MenuItem<5, { 16 * BYTES_PER_CHAR }>; 8] = [
            &mut BasicMenuItem::new("Item1"),
            &mut BasicMenuItem::new("Item2"),
            &mut BasicMenuItem::new("Item3"),
            &mut BasicMenuItem::new("Item4"),
            &mut BasicMenuItem::new("Item5"),
            &mut BasicMenuItem::new("Item6"),
            &mut BasicMenuItem::new("Item7"),
            &mut BasicMenuItem::new("Item8"),
        ];
        let mut menu: Menu<16, 5, { 16 * BYTES_PER_CHAR }> = Menu::new(&mut items).unwrap();
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 5);

        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], "→Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(lines_to_render[2], " Item3          ");
        assert_eq!(lines_to_render[3], " Item4          ");
        assert_eq!(lines_to_render[4], " Item5         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], " Item1          ");
        assert_eq!(lines_to_render[1], "→Item2          ");
        assert_eq!(lines_to_render[2], " Item3          ");
        assert_eq!(lines_to_render[3], " Item4          ");
        assert_eq!(lines_to_render[4], " Item5         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], " Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(lines_to_render[2], "→Item3          ");
        assert_eq!(lines_to_render[3], " Item4          ");
        assert_eq!(lines_to_render[4], " Item5         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], " Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(lines_to_render[2], " Item3          ");
        assert_eq!(lines_to_render[3], "→Item4          ");
        assert_eq!(lines_to_render[4], " Item5         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 5);
        assert_eq!(lines_to_render[0], " Item1          ");
        assert_eq!(lines_to_render[1], " Item2          ");
        assert_eq!(lines_to_render[2], " Item3          ");
        assert_eq!(lines_to_render[3], " Item4          ");
        assert_eq!(lines_to_render[4], "→Item5         ↓");

        assert_eq!(menu.down(), true);
        let lines_to_render = menu.generate_lines_to_render();
        assert_eq!(lines_to_render.len(), 3);
        assert_eq!(lines_to_render[0], "→Item6         ↑");
        assert_eq!(lines_to_render[1], " Item7          ");
        assert_eq!(lines_to_render[2], " Item8          ");
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
}

// TODO improvements:
// TODO separate renderer and menu
// TODO input item
// TODO charset input item
// TODO screens
// TODO horizontal scrolling if overflow
// TODO disable functionality via features
// TODO more range items (u8, signed, float, ....)

// TODO
// TODO don't loose focus if invalid input (char for example)
// TODO test with multiple submenus
