use std::sync::Arc;

use super::{BasicGraphics, Graphics, RendererManager, draw_command_buffer};

use draw_command_buffer::DrawCommandBuffer;
use pixels::{Error, Pixels, SurfaceTexture, TextureError};
use winit::{dpi::PhysicalSize, window::Window};

pub struct BasicRendererManager {
    surface: Pixels<'static>,
    graphics: Option<Box<dyn Graphics<'static>>>,
}

impl RendererManager for BasicRendererManager {
    fn new(window: Arc<Window>) -> Result<Self, Error> {
        let size = window.inner_size();
        let width = size.width;
        let height = size.height;

        let texture = SurfaceTexture::new(width, height, window.clone());

        Pixels::new(width, height, texture).map(|surface| Self {
            surface,
            graphics: None,
        })
    }

    fn resize(&mut self, inner_size: PhysicalSize<u32>) -> Result<(), TextureError> {
        // let graphics = BasicGraphics::new(self.surface.frame_mut(), 600, 400);
        // self.graphics.replace(Box::new(graphics));
        self.surface
            .resize_surface(inner_size.width, inner_size.height)
    }

    fn draw(&mut self, cmd_buffer: &mut DrawCommandBuffer) -> Result<(), Error> {
        let graphics = self
            .graphics
            .get_or_insert_with(|| {
                let pixels = self.surface.frame_mut();
                Box::new(BasicGraphics::new(pixels, 600, 400))
            })
            .as_mut();
        cmd_buffer.consume(graphics);
        self.surface.render()
    }
}
