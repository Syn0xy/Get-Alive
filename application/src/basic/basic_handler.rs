use std::sync::Arc;

use entity::EntityManager;
use input::InputManager;
use winit::{application::ApplicationHandler, event::WindowEvent};

use crate::{Application, basic::BasicApplication};

impl<E: EntityManager, I: InputManager> ApplicationHandler for BasicApplication<E, I> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Ok(window) = event_loop.create_window(self.window_attributes.clone()) {
            let window_arc = Arc::new(window);

            self.window.replace(window_arc);
        }

        self.start();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(window) = self.window.as_ref() {
            if window_id != window.id() {
                return;
            }

            match event {
                WindowEvent::Resized(physical_size) => self.handle_resize(physical_size),
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::KeyboardInput { event, .. } => self.handle_key_event(event),
                WindowEvent::CursorMoved { .. } => {}
                WindowEvent::MouseInput { .. } => {}
                WindowEvent::RedrawRequested => self.handle_redraw(),
                _ => {}
            }
        }
    }
}
