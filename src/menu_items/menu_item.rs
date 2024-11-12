pub trait MenuItem {
    fn get_label(&self) -> &String;
    fn press(&mut self) -> ();
}
