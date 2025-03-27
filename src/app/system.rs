use std::time::Instant;

use crate::constants;

use super::{input::InputSystem, main::EntityManager};

pub struct AppState<S: EntityManager> {
    tickrate: u32,
    entity_manager: S,
    input_system: Box<dyn InputSystem>,
    last_tick: Option<Instant>,
}

impl<S: EntityManager> AppState<S> {
    pub fn new(entity_manager: S, input_system: Box<dyn InputSystem>) -> Self {
        Self {
            tickrate: constants::app::DEFAULT_TICKRATE,
            entity_manager,
            input_system,
            last_tick: None,
        }
    }

    pub fn get_tickrate(&self) -> u32 {
        self.tickrate
    }

    pub fn get_entity_manager_mut(&mut self) -> &mut S {
        &mut self.entity_manager
    }

    pub fn get_input_system(&self) -> &dyn InputSystem {
        self.input_system.as_ref()
    }

    pub fn get_last_tick(&self) -> &Option<Instant> {
        &self.last_tick
    }

    pub fn set_last_tick(&mut self, instant: Instant) {
        self.last_tick.replace(instant);
    }
}
