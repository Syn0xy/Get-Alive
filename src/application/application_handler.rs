use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    keyboard::PhysicalKey, window::WindowId,
};

use super::{App, input::InputSystem};

impl<I: InputSystem> ApplicationHandler for App<I> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Ok(window) = event_loop.create_window(self.window_attributes.clone()) {
            self.window.replace(window);

            self.input_system.subscribe("Up", || {
                println!("oui");
            });
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(window) = self.window.as_ref() {
            if window_id != window.id() {
                return;
            }

            match event {
                WindowEvent::Resized(..) => {}
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::KeyboardInput { event, .. } => {
                    if let PhysicalKey::Code(key_code) = event.physical_key {
                        self.input_system.dispatch(key_code, &event.state);
                    }
                }
                WindowEvent::CursorMoved { .. } => {}
                WindowEvent::MouseInput { .. } => {}
                WindowEvent::RedrawRequested => self.redraw(),
                _ => {}
            }
        }
    }
}
