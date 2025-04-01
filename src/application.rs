pub mod application_handler;
pub mod entity;
pub mod input;

use std::time::Instant;

use input::InputSystem;
use winit::window::{Window, WindowAttributes};

pub struct App<I: InputSystem> {
    pub window_attributes: WindowAttributes,
    pub window: Option<Window>,
    pub input_system: I,
    pub tickrate: u32,
    pub last_tick_time: Option<Instant>,
}

impl<I: InputSystem> App<I> {
    pub fn new(window_attributes: WindowAttributes, input_system: I) -> Self {
        Self {
            window_attributes,
            window: Default::default(),
            input_system,
            tickrate: Default::default(),
            last_tick_time: Default::default(),
        }
    }

    pub fn redraw(&mut self) {
        self.tickrate += 1;

        if let Some(last_tick_time) = self.last_tick_time {
            let elapsed_time = last_tick_time.elapsed();

            if elapsed_time.as_secs_f32() > 1.0 {
                println!("tickrate: {}", self.tickrate);
                self.tickrate = 0;
                self.last_tick_time.replace(Instant::now());
            }
        } else {
            self.tickrate = 0;
            self.last_tick_time.replace(Instant::now());
        }

        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}
