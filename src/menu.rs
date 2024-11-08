use crate::menu_item_enum::MenuItemEnum;

pub struct Menu {
    pub char_width: usize,
    pub char_height: usize,
    pub items: Vec<MenuItemEnum>,
}

impl Menu {
    pub fn new(char_width: usize, char_height: usize, items: Vec<MenuItemEnum>) -> Menu {
        if char_width < 2 || char_height < 2 {
            panic!(
                "Invalid menu char width and/or height. At least 2 chars required per dimension."
            );
        }

        Menu {
            char_width,
            char_height,
            items
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::basic_menu_item::BasicMenuItem;
    use super::*;

    #[test]
    fn can_create_empty_menu() {
        let items: Vec<MenuItemEnum> = Vec::new();
        let menu = Menu::new(16, 2, items);
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
        assert_eq!(menu.items.len(), 0);
    }

    #[test]
    fn can_create_simple_menu() {
        let items: Vec<MenuItemEnum> = vec!(MenuItemEnum::BasicMenuItem(BasicMenuItem::new(String::from("Item1"))));
        let menu = Menu::new(16, 2, items);
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
        assert_eq!(menu.items.len(), 1);
    }

    #[test]
    #[should_panic(expected = "Invalid menu char width and/or height. At least 2 chars required per dimension.")]
    fn panics_if_invalid_char_width() {
        let items: Vec<MenuItemEnum> = Vec::new();
        Menu::new(1, 2, items);
    }

    #[test]
    #[should_panic(expected = "Invalid menu char width and/or height. At least 2 chars required per dimension.")]
    fn panics_if_invalid_char_height() {
        let items: Vec<MenuItemEnum> = Vec::new();
        Menu::new(16, 1, items);
    }
}
