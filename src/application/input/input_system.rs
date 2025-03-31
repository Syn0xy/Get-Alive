use std::collections::HashMap;

use winit::{event::ElementState, keyboard::KeyCode};

pub type InputActionName = &'static str;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum InputContext {
    Menu,
    Gameplay,
}

#[derive(Default, Debug)]
pub struct InputSystem {
    pub action_maps: &'static [InputActionMap],
}

#[derive(Debug)]
pub struct InputActionMap {
    pub context: InputContext,
    pub actions: &'static [InputAction],
}

#[derive(Default, Debug)]
pub struct InputAction {
    pub action: InputActionName,
    pub bindings: &'static [KeyCode],
}

impl InputSystem {
    pub fn input_event(&self, _key_code: &KeyCode, _state: &ElementState) {}
}
