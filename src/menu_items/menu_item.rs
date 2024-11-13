use enum_dispatch::enum_dispatch;
#[enum_dispatch(MenuItemEnum)]
pub trait MenuItem {
    fn get_label(&self) -> String;
    fn press(&mut self, is_focused: bool) -> ();
    fn is_focusable(&self) -> bool;
}
