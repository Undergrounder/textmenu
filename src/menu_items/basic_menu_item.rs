use crate::keyboard::KeyboardKey;
use crate::menu_items::menu_item::{MenuItem, PressResult, LABEL_BYTES};
use core::str::FromStr;
use heapless::{String, Vec};

pub struct BasicMenuItem<'a, const CHAR_HEIGHT_CONST: usize, const LINE_BYTES_SIZE_CONST: usize> {
    label: &'a str,
}

impl<'a, const CHAR_HEIGHT_CONST: usize, const LINE_BYTES_SIZE_CONST: usize>
    BasicMenuItem<'a, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>
{
    pub fn new(label: &str) -> BasicMenuItem<CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST> {
        BasicMenuItem { label }
    }
}

impl<'a, const CHAR_HEIGHT_CONST: usize, const LINE_BYTES_SIZE_CONST: usize>
    MenuItem<'a, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>
    for BasicMenuItem<'a, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>
{
    fn get_label(&self, _is_focused: bool) -> String<{ LABEL_BYTES }> {
        String::from_str(self.label).unwrap()
    }

    fn press(&mut self, _key: &KeyboardKey, _is_focused: bool) -> PressResult {
        PressResult {
            handled: false,
            focus: false,
        }
    }

    fn generate_lines_to_render(
        &self,
    ) -> Option<Vec<String<LINE_BYTES_SIZE_CONST>, CHAR_HEIGHT_CONST>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::BYTES_PER_CHAR;
    use crate::keyboard::FunctionKey;

    #[test]
    fn can_create_a_basic_menu_item() {
        let mut item: BasicMenuItem<2, { 16 * BYTES_PER_CHAR }> = BasicMenuItem::new("label");
        assert_eq!(item.get_label(false), "label");
        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::LEFT), None), false),
            PressResult {
                focus: false,
                handled: false
            }
        );
        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::RIGHT), None), false),
            PressResult {
                focus: false,
                handled: false
            }
        );
        assert_eq!(
            item.press(&KeyboardKey::new(Some(FunctionKey::BACK), None), false),
            PressResult {
                focus: false,
                handled: false
            }
        );
    }
}
