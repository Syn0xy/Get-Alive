use std::sync::Arc;

use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

use super::{
    App, entity::EntityManager, input::InputSystem, renderer::RendererManager,
    runtime::RuntimeManager,
};

impl<R: RuntimeManager, I: InputSystem, E: EntityManager, D: RendererManager> ApplicationHandler
    for App<R, I, E, D>
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Ok(window) = event_loop.create_window(self.window_attributes.clone()) {
            let window_arc = Arc::new(window);
            self.window.replace(window_arc.clone());

            if let Ok(renderer_manager) = D::new(window_arc.clone()) {
                self.renderer_manager.replace(renderer_manager);
            }

            self.start();
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
                WindowEvent::Resized(inner_size) => self.handle_resize(inner_size),
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
