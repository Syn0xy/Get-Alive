use winit::{dpi::PhysicalSize, event_loop::ControlFlow, window::WindowAttributes};

pub const WINDOW_TITLE: &str = "Get Alive";
pub const DEFAULT_WINDOW_WIDTH: u32 = 600;
pub const DEFAULT_WINDOW_HEIGHT: u32 = 400;
pub const CONTROL_FLOW: ControlFlow = ControlFlow::Poll;

pub fn window_attributes() -> WindowAttributes {
    WindowAttributes::default()
        .with_title(WINDOW_TITLE)
        .with_inner_size(PhysicalSize {
            width: DEFAULT_WINDOW_WIDTH,
            height: DEFAULT_WINDOW_HEIGHT,
        })
}
