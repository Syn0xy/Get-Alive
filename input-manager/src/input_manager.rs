use winit::{event::ElementState, keyboard::KeyCode};

pub trait InputContext {}

pub struct InputActionMap(pub dyn InputContext, pub &'static [InputAction<A>]);
pub struct InputAction(pub dyn InputContext, pub &'static [KeyCode]);

pub type Callback = dyn FnMut();

pub trait InputManager {
    fn new() -> Self;
    fn from_bindings(action_maps: &[InputActionMap]) -> Self;

    fn subscribe<F>(&mut self, action: InputContext, callback: F)
    where
        F: FnMut() + 'static;
    fn dispatch(&mut self, key_code: KeyCode, state: &ElementState);
    fn add_binding(&mut self, action: InputContext, key_code: KeyCode);
}
