use enum_dispatch::enum_dispatch;
#[enum_dispatch(MenuItemEnum)]
pub trait MenuItem {
    fn get_label(&self) -> String;
    fn enter(&mut self, is_focused: bool) -> bool;
    fn is_focusable(&self) -> bool;
    fn back(&mut self) -> bool;
    fn left(&mut self) -> bool;
    fn right(&mut self) -> bool;
}
