use std::{
    ops::{Deref, DerefMut},
    time::Instant,
};

use crate::{
    app::{
        input::{BasicInputSystem, InputError, InputEvent, InputSystem},
        main::MainEvent,
        render::DisplayEvent,
    },
    constants,
};

use super::{Dispatcher, EventLoop, GenericDispatcher};

pub struct BasicEventLoop {
    tickrate: u32,
    dispatcher: GenericDispatcher,
    input_system: Box<dyn InputSystem>,
}

impl Default for BasicEventLoop {
    fn default() -> Self {
        Self {
            tickrate: constants::events::DEFAULT_TICKRATE,
            dispatcher: GenericDispatcher::default(),
            input_system: Box::new(BasicInputSystem::default()),
        }
    }
}

impl Deref for BasicEventLoop {
    type Target = GenericDispatcher;

    fn deref(&self) -> &Self::Target {
        &self.dispatcher
    }
}

impl DerefMut for BasicEventLoop {
    fn deref_mut(&mut self) -> &mut GenericDispatcher {
        &mut self.dispatcher
    }
}

impl EventLoop for BasicEventLoop {
    fn run(&mut self) {
        let mut last_time = Instant::now();

        loop {
            match self.input_system.get_input() {
                Ok(key) => {
                    self.dispatch(&InputEvent::KeyboardInput { key });
                }
                Err(InputError::InvalidInput(invalid_input_str)) => {
                    eprintln!("[ERROR]: invalid input {:?}", invalid_input_str)
                }
                Err(InputError::NoInput) => {}
            }

            self.dispatch(&MainEvent::Tick);

            let crnt_time = Instant::now();
            let elapsed_time = crnt_time - last_time;

            if elapsed_time.as_secs_f32() >= 1.0 / self.tickrate as f32 {
                last_time = crnt_time;
                self.dispatch(&DisplayEvent::RedrawRequested { elapsed_time });
            }
        }
    }
}
