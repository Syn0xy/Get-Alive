use winit::{event::ElementState, keyboard::KeyCode};

pub type InputContext = &'static str;
pub type InputActionName = &'static str;

pub struct InputActionMap(pub InputContext, pub &'static [InputAction]);
pub struct InputAction(pub InputActionName, pub &'static [KeyCode]);

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
