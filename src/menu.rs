pub struct Menu {
    pub char_width: usize,
    pub char_height: usize,
}

impl Menu {
    pub fn new(char_width: usize, char_height: usize) -> Menu {
        if char_width < 2 || char_height < 2 {
            panic!(
                "Invalid menu char width and/or height. At least 2 chars required per dimension."
            );
        }

        Menu {
            char_width,
            char_height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_empty_menu() {
        let menu = Menu::new(16, 2);
        assert_eq!(menu.char_width, 16);
        assert_eq!(menu.char_height, 2);
    }

    #[test]
    #[should_panic(expected = "Invalid menu char width and/or height. At least 2 chars required per dimension.")]
    fn panics_if_invalid_char_width() {
        Menu::new(1, 2);
    }

    #[test]
    #[should_panic(expected = "Invalid menu char width and/or height. At least 2 chars required per dimension.")]
    fn panics_if_invalid_char_height() {
        Menu::new(16, 1);
    }
}
