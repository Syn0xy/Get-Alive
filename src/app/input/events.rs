use super::InputKey;

#[derive(Debug)]
pub enum InputEvent {
    KeyboardInput { key: InputKey },
}
