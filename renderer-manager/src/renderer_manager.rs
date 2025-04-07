use pixels::{Error, TextureError};
use winit::dpi::PhysicalSize;

use crate::DrawCommandBuffer;

pub trait RendererManager {
    fn handle_resize(&mut self, physical_size: PhysicalSize<u32>) -> Result<(), TextureError>;
    fn handle_draw(&mut self, cmd_buffer: &mut DrawCommandBuffer) -> Result<(), Error>;
}
