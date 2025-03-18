use crate::input::{BasicInputSystem, InputSystem};

pub struct Game {
    input_system: Box<dyn InputSystem>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            input_system: Box::new(BasicInputSystem::default()),
        }
    }
}

impl Game {
    pub fn get_input_system(&self) -> &dyn InputSystem {
        self.input_system.as_ref()
    }
}
