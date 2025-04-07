use winit::{dpi::PhysicalSize, event::KeyEvent};

pub trait Application {
    fn start(&mut self);

    fn handle_redraw(&mut self);
    fn handle_key_event(&mut self, event: KeyEvent);
    fn handle_resize(&mut self, physical_size: PhysicalSize<u32>);
}
