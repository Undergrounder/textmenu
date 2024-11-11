enum KeyboardAction {
    UP,
    DOWN,
    ENTER,
    BACK,
    BACKSPACE,
    CLEAR,
    NUMBER(u8),
    TEXT(String)
}

trait KeyboardAdapter {
    fn get_action() -> Option<KeyboardAction>;
}