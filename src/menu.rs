use crate::menu_item_enum::MenuItemEnum;

pub struct Menu {
    pub char_width: usize,
    pub char_height: usize,
    pub items: Vec<MenuItemEnum>,
    pub selected_item_idx: usize,
    pub top_visible_item_idx: usize,
}

impl Menu {
    pub fn new(char_width: usize, char_height: usize, items: Vec<MenuItemEnum>) -> Menu {
        if items.len() == 0 {
            panic!(
                "At least 1 menu item required."
            );
        }

        if char_width < 3 {
            panic!(
                "Invalid menu char width. At least 3 chars required."
            );
        }

        if char_height < 2 {
            panic!(
                "Invalid menu char height. At least 2 chars required."
            );
        }

        Menu {
            char_width,
            char_height,
            items,
            selected_item_idx: 0,
            top_visible_item_idx: 0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::basic_menu_item::BasicMenuItem;
    use super::*;

    #[test]
    fn can_create_simple_menu() {
        let items: Vec<MenuItemEnum> = vec!(MenuItemEnum::BasicMenuItem(BasicMenuItem::new(String::from("Item1"))));
        let menu = Menu::new(16, 2, items);
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
        assert_eq!(menu.items.len(), 1);
    }

    #[test]
    #[should_panic(expected = "Invalid menu char width. At least 3 chars required.")]
    fn panics_if_invalid_char_width() {
        let items: Vec<MenuItemEnum> = vec!(MenuItemEnum::BasicMenuItem(BasicMenuItem::new(String::from("Item1"))));
        Menu::new(1, 2, items);
    }

    #[test]
    #[should_panic(expected = "Invalid menu char height. At least 2 chars required.")]
    fn panics_if_invalid_char_height() {
        let items: Vec<MenuItemEnum> = vec!(MenuItemEnum::BasicMenuItem(BasicMenuItem::new(String::from("Item1"))));
        Menu::new(16, 1, items);
    }
}
