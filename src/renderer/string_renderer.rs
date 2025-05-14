use heapless::{String, Vec};
use crate::menu::Menu;
use crate::menu_items::menu_item::BYTES_PER_CHAR;

pub struct StringRenderer {}

impl <const LINE_COUNT: usize, const LINE_MAX_CHAR_COUNT: usize> StringRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(menu: &Menu) -> Vec<String<{LINE_MAX_CHAR_COUNT*BYTES_PER_CHAR}>, LINE_COUNT> {


        /*

                TODO
                if LINE_BYTES_SIZE_CONST != CHAR_WIDTH_CONST * BYTES_PER_CHAR {
                    Err("LINE_BYTES_SIZE_CONST must be equal to CHAR_WIDTH_CONST*BYTES_PER_CHAR.")
                } else else if CHAR_WIDTH_CONST < 3 {
                    Err("Invalid menu char width. At least 3 chars required.")
                } else if CHAR_HEIGHT_CONST < 2 {
                    Err("Invalid menu char height. At least 2 chars required.")
                }

        Menu
            pub fn generate_lines_to_render(
                &self,
            ) -> Vec<String<LINE_BYTES_SIZE_CONST>, CHAR_HEIGHT_CONST> {
                self.submenu_menu_item.generate_lines_to_render().unwrap()
            }

            ActioMenuItem, BasicMenuItem, RangeMenuItem, ToggleMenuItem, ListMenuItem

            fn generate_lines_to_render(
                &self,
            ) -> Option<Vec<String<LINE_BYTES_SIZE_CONST>, CHAR_HEIGHT_CONST>> {
                None
            }

        SubMenuMenu

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

                 */
    }
}

/*
TODO tests
 */
