use winit::{event::ElementState, keyboard::KeyCode};

pub type InputContext = &'static str;
pub type InputActionName = &'static str;
pub type Callback = dyn FnMut();

pub trait InputSystem {
    fn new() -> Self;
    fn from_bindings(action_maps: &[InputActionMap]) -> Self;

    fn subscribe<T>(&mut self, action_name: InputActionName, callback: T)
    where
        T: FnMut() + 'static;
    fn dispatch(&mut self, key_code: KeyCode, state: &ElementState);
    fn add_binding(&mut self, action_name: InputActionName, key_code: KeyCode);
}

pub struct InputActionMap {
    pub context: InputContext,
    pub actions: &'static [InputAction],
}

#[derive(Default, Debug)]
pub struct InputAction {
    pub action: InputActionName,
    pub bindings: &'static [KeyCode],
}
