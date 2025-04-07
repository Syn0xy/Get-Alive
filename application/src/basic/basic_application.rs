use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use entity::EntityManager;
use input::InputManager;
use winit::{
    dpi::PhysicalSize,
    error::EventLoopError,
    event::KeyEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

use crate::Application;

pub struct BasicApplication<E: EntityManager, I: InputManager> {
    pub(crate) window: Option<Arc<Window>>,
    pub(crate) window_attributes: WindowAttributes,

    pub(crate) entity_manager: E,
    pub(crate) input_manager: I,
    framecount: u32,
    last_time: Instant,
}

impl<E: EntityManager, I: InputManager> BasicApplication<E, I> {
    pub fn new(window_attributes: WindowAttributes, entity_manager: E, input_manager: I) -> Self {
        Self {
            window: None,
            window_attributes,

            entity_manager,
            input_manager,
            framecount: 0,
            last_time: Instant::now(),
        }
    }

    pub fn run(&mut self, control_flow: ControlFlow) {
        EventLoop::new()
            .map(|event_loop| {
                event_loop.set_control_flow(control_flow);
                event_loop
                    .run_app(self)
                    .map_err(handle_event_loop_error)
                    .ok();
            })
            .map_err(handle_event_loop_error)
            .ok();
    }
}

fn handle_event_loop_error(error: EventLoopError) {
    match error {
        EventLoopError::NotSupported(_) => {}
        EventLoopError::Os(_) => {}
        EventLoopError::RecreationAttempt => {}
        EventLoopError::ExitFailure(_) => {}
    }

    eprintln!("ERROR: {:?}", error);
}

impl<E: EntityManager, I: InputManager> Application for BasicApplication<E, I> {
    fn start(&mut self) {}

    fn handle_redraw(&mut self) {
        self.framecount += 1;

        if self.last_time.elapsed() > Duration::from_secs(1) {
            println!("framecount: {:?}", self.framecount);
            self.framecount = 0;
            self.last_time = Instant::now();
        }

        self.window.as_ref().map(|window| window.request_redraw());
    }

    fn handle_key_event(&mut self, _event: KeyEvent) {}

    fn handle_resize(&mut self, _physical_size: PhysicalSize<u32>) {}
}
