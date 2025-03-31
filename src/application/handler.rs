use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    keyboard::PhysicalKey, window::WindowId,
};

use super::App;

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Ok(window) = event_loop.create_window(self.window_attributes.clone()) {
            self.window.replace(window);
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
                WindowEvent::Resized(_physical_size) => {}
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::KeyboardInput {
                    device_id: _,
                    event,
                    is_synthetic: _,
                } => {
                    if let PhysicalKey::Code(key_code) = event.physical_key {
                        self.input_system.input_event(&key_code, &event.state);
                        println!("key: {:?} / State: {:?}", key_code, event.state)
                    }

                    {
                        
                    }

                }
                ,
                // WindowEvent::CursorMoved {
                //     device_id,
                //     position,
                // } => {}
                // WindowEvent::MouseInput {
                //     device_id,
                //     state,
                //     button,
                // } => {}
                WindowEvent::RedrawRequested => self.redraw(),
                _ => {}
            }
        }
    }
}
