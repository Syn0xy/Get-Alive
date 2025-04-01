use std::collections::HashMap;

use winit::{event::ElementState, keyboard::KeyCode};

use super::{Callback, InputActionMap, InputActionName, InputSystem};

#[derive(Default)]
pub struct BasicInputSystem {
    pub action_maps: HashMap<KeyCode, Vec<InputActionName>>,
    pub callbacks: HashMap<InputActionName, Vec<Box<Callback>>>,
}

impl BasicInputSystem {
    fn call_callback(&mut self, action_name: InputActionName) {
        println!("{}", action_name);
        self.callbacks
            .get_mut(&action_name)
            .map(|callbacks| callbacks.iter_mut().for_each(|f| f()));
    }
}

impl InputSystem for BasicInputSystem {
    fn new() -> Self {
        Self::default()
    }

    fn from_bindings(action_maps: &[InputActionMap]) -> Self {
        let mut input_system = Self::new();

        for action_map in action_maps {
            for actions in action_map.1 {
                let action_name = actions.0;

                for &key_code in actions.1 {
                    input_system.add_binding(action_name, key_code);
                }
            }
        }

        input_system
    }

    fn subscribe<T>(&mut self, action_name: InputActionName, callback: T)
    where
        T: FnMut() + 'static,
    {
        self.callbacks
            .entry(action_name)
            .or_default()
            .push(Box::new(callback));
    }

    fn dispatch(&mut self, key_code: KeyCode, _state: &ElementState) {
        if let Some(action_names) = self.action_maps.get(&key_code).cloned() {
            for action_name in action_names {
                self.call_callback(action_name);
            }
        }
    }

    fn add_binding(&mut self, action_name: InputActionName, key_code: KeyCode) {
        self.action_maps
            .entry(key_code)
            .or_default()
            .push(action_name)
    }
}
