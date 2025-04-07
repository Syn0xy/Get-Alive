use std::sync::Arc;

use pixels::{Error, Pixels, SurfaceTexture, TextureError};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{DrawCommandBuffer, RendererManager};

use super::BasicGraphics;

pub struct BasicRendererManager<'a> {
    surface: Pixels<'a>,
}

impl<'a> BasicRendererManager<'a> {
    pub fn new(window: Arc<Window>) -> Result<Self, Error> {
        let size = window.inner_size();
        let width = size.width;
        let height = size.height;

        let texture = SurfaceTexture::new(width, height, window.clone());
        let surface: Pixels<'a> = Pixels::new(width, height, texture)?;

        Ok(Self { surface })
    }
}

impl<'a> RendererManager for BasicRendererManager<'a> {
    fn handle_resize(&mut self, physical_size: PhysicalSize<u32>) -> Result<(), TextureError> {
        self.surface
            .resize_surface(physical_size.width, physical_size.height)
    }

    fn handle_draw(&mut self, cmd_buffer: &mut DrawCommandBuffer) -> Result<(), Error> {
        let pixels = self.surface.frame_mut();

        let mut graphics = BasicGraphics::new(pixels, 20, 20);

        cmd_buffer.consume(&mut graphics);
        self.surface.render()
    }
}
