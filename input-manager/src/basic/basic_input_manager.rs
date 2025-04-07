use std::{collections::HashMap, hash::Hash};

use winit::{event::ElementState, keyboard::KeyCode};

use crate::{Callback, InputActionMap, InputManager};

#[derive(Default)]
pub struct BasicInputManager<M, A> {
    pub _context: HashMap<M, Vec<i32>>,
    pub action_maps: HashMap<KeyCode, Vec<A>>,
    pub callbacks: HashMap<A, Vec<Box<Callback>>>,
}

impl<M: Hash, A: Hash + Eq> BasicInputManager<M, A> {
    fn call_callback(&mut self, action: A) {
        self.callbacks
            .get_mut(&action)
            .map(|callbacks| callbacks.iter_mut().for_each(|f| f()));
    }
}

impl<M: Default + Hash, A: Default + Hash + Eq + Clone> InputManager<M, A>
    for BasicInputManager<M, A>
{
    fn new() -> Self {
        Self::default()
    }

    fn from_bindings(action_maps: &[InputActionMap<M, A>]) -> Self {
        let mut input_system = Self::new();

        for action_map in action_maps {
            for actions in action_map.1 {
                for &key_code in actions.1 {
                    input_system.add_binding(actions.0.clone(), key_code);
                }
            }
        }

        input_system
    }

    fn subscribe<T>(&mut self, action: A, callback: T)
    where
        T: FnMut() + 'static,
    {
        self.callbacks
            .entry(action)
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

    fn add_binding(&mut self, action: A, key_code: KeyCode) {
        self.action_maps.entry(key_code).or_default().push(action)
    }
}
