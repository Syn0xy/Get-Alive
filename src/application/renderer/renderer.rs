use std::sync::Arc;

use super::draw_command_buffer;

use draw_command_buffer::DrawCommandBuffer;
use pixels::{Error, TextureError};
use winit::{dpi::PhysicalSize, window::Window};

pub trait RendererManager {
    fn new(window: Arc<Window>) -> Result<Self, Error>
    where
        Self: Sized;
    fn resize(&mut self, inner_size: PhysicalSize<u32>) -> Result<(), TextureError>;
    fn draw(&mut self, cmd_buffer: &mut DrawCommandBuffer) -> Result<(), Error>;
}
