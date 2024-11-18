use crate::keyboard::{FunctionKey, KeyboardKey};
use crate::menu_items::menu_item::{MenuItem, PressResult, LABEL_BYTES};
use core::fmt::Write;
use core::str::FromStr;
use heapless::{String, Vec};

pub struct SubmenuMenuItem<
    'a,
    const CHAR_WIDTH_CONST: usize,
    const CHAR_HEIGHT_CONST: usize,
    const LINE_BYTES_SIZE_CONST: usize,
> {
    label: &'a str,
    pub items: &'a mut [&'a mut dyn MenuItem<'a, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>],
    // View state
    pub selected_item_idx: usize,
    pub is_focused: bool,
}

impl<
        'a,
        const CHAR_WIDTH_CONST: usize,
        const CHAR_HEIGHT_CONST: usize,
        const LINE_BYTES_SIZE_CONST: usize,
    > SubmenuMenuItem<'a, CHAR_WIDTH_CONST, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>
{
    pub fn new(
        label: &'a str,
        items: &'a mut [&'a mut dyn MenuItem<'a, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>],
    ) -> SubmenuMenuItem<'a, CHAR_WIDTH_CONST, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST> {
        SubmenuMenuItem {
            label,
            items,
            selected_item_idx: 0,
            is_focused: false,
        }
    }

    fn get_top_visible_item_idx(&self) -> usize {
        let div = self.selected_item_idx.div_euclid(CHAR_HEIGHT_CONST);
        div * CHAR_HEIGHT_CONST
    }

    fn generate_line_to_render(
        &self,
        item_idx: usize,
        item: &dyn MenuItem<CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>,
    ) -> String<LINE_BYTES_SIZE_CONST> {
        let is_selected_item = item_idx == self.selected_item_idx;
        let is_item_focused = is_selected_item && self.is_focused;
        let selection_str: &str = if is_selected_item {
            if is_item_focused {
                "←"
            } else {
                "→"
            }
        } else {
            " "
        };
        let label = item.get_label(is_item_focused);
        let max_length_label = CHAR_WIDTH_CONST - 2;
        let label_trimmed = if label.len() > max_length_label {
            &label[..max_length_label]
        } else {
            &label
        };

        let top_visible_item_idx = self.get_top_visible_item_idx();
        let bottom_item_idx = top_visible_item_idx + CHAR_HEIGHT_CONST - 1;
        let arrow_str: &str = if item_idx == top_visible_item_idx {
            if top_visible_item_idx != 0 {
                "↑"
            } else {
                " "
            }
        } else if item_idx == bottom_item_idx {
            if bottom_item_idx < self.items.len() - 1 {
                "↓"
            } else {
                " "
            }
        } else {
            " "
        };

        let mut line_str: String<LINE_BYTES_SIZE_CONST> = String::new();
        write!(
            line_str,
            "{}{:3$}{}",
            selection_str, label_trimmed, arrow_str, max_length_label
        )
        .unwrap();
        line_str
    }

    fn get_selected_item(&self) -> &dyn MenuItem<'a, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST> {
        self.items[self.selected_item_idx]
    }

    fn get_mut_selected_item(
        &mut self,
    ) -> &mut dyn MenuItem<'a, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST> {
        self.items[self.selected_item_idx]
    }

    fn up(&mut self) -> bool {
        if self.is_focused {
            false
        } else if let Some(new_selected_item_idx) = self.selected_item_idx.checked_sub(1) {
            self.selected_item_idx = new_selected_item_idx;
            true
        } else {
            false
        }
    }

    pub fn down(&mut self) -> bool {
        if self.is_focused {
            false
        } else if let Some(new_selected_item_idx) = self.selected_item_idx.checked_add(1) {
            if new_selected_item_idx < self.items.len() {
                self.selected_item_idx = new_selected_item_idx;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<
        'a,
        const CHAR_WIDTH_CONST: usize,
        const CHAR_HEIGHT_CONST: usize,
        const LINE_BYTES_SIZE_CONST: usize,
    > MenuItem<'a, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>
    for SubmenuMenuItem<'a, CHAR_WIDTH_CONST, CHAR_HEIGHT_CONST, LINE_BYTES_SIZE_CONST>
{
    fn get_label(&self, _is_focused: bool) -> String<{ LABEL_BYTES }> {
        String::from_str(self.label).unwrap()
    }

    fn press(&mut self, key: &KeyboardKey, is_focused: bool) -> PressResult {
        if is_focused {
            let item_press_result = {
                let is_focused = self.is_focused;
                let selected_item = self.get_mut_selected_item();
                selected_item.press(key, is_focused)
            };
            self.is_focused = item_press_result.focus;
            if item_press_result.handled {
                PressResult {
                    handled: true,
                    focus: true,
                }
            } else {
                if let Some(function_key) = &key.function_key {
                    match function_key {
                        FunctionKey::BACK => PressResult {
                            focus: false,
                            handled: true,
                        },
                        FunctionKey::UP => {
                            let handled = self.up();
                            PressResult {
                                focus: true,
                                handled,
                            }
                        }
                        FunctionKey::DOWN => {
                            let handled = self.down();
                            PressResult {
                                focus: true,
                                handled,
                            }
                        }
                        _ => PressResult {
                            handled: false,
                            focus: true,
                        },
                    }
                } else {
                    PressResult {
                        handled: false,
                        focus: true,
                    }
                }
            }
        } else {
            match key.function_key {
                Some(FunctionKey::ENTER) => PressResult {
                    handled: true,
                    focus: true,
                },
                _ => PressResult {
                    handled: false,
                    focus: false,
                },
            }
        }
    }

    fn generate_lines_to_render(
        &self,
    ) -> Option<Vec<String<LINE_BYTES_SIZE_CONST>, CHAR_HEIGHT_CONST>> {
        let lines_from_item_option = if self.is_focused {
            let selected_item = self.get_selected_item();
            let item_lines_to_render_option = selected_item.generate_lines_to_render();
            if let Some(item_lines_to_render) = item_lines_to_render_option {
                Some(item_lines_to_render)
            } else {
                None
            }
        } else {
            None
        };

        if lines_from_item_option.is_some() {
            lines_from_item_option
        } else {
            let items_length = self.items.len();
            let mut lines_to_render: Vec<String<LINE_BYTES_SIZE_CONST>, CHAR_HEIGHT_CONST> =
                Vec::new();
            let top_visible_item_idx = self.get_top_visible_item_idx();
            let bottom_idx = core::cmp::min(CHAR_HEIGHT_CONST + top_visible_item_idx, items_length);
            let visible_items = &self.items[top_visible_item_idx..bottom_idx];
            for (item_idx, item) in visible_items.iter().enumerate() {
                let corrected_item_idx = item_idx + top_visible_item_idx;
                let line_to_render = self.generate_line_to_render(corrected_item_idx, *item);
                lines_to_render.push(line_to_render).unwrap();
            }

            Some(lines_to_render)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::BYTES_PER_CHAR;
    use crate::menu_items::basic_menu_item::BasicMenuItem;

    #[test]
    fn can_create_a_menu_item() {
        let mut items: [&mut dyn MenuItem<2, { BYTES_PER_CHAR * 16 }>; 1] =
            [&mut BasicMenuItem::new("Item1")];
        let item: SubmenuMenuItem<16, 2, { 16 * BYTES_PER_CHAR }> =
            SubmenuMenuItem::new("label", &mut items);

        assert_eq!(item.get_label(false), "label");
    }
}
