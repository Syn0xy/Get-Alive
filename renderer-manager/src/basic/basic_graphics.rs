use crate::{Graphics, GraphicsColor};

pub struct BasicGraphics<'a> {
    frames: &'a mut [u8],
    width: u32,
    height: u32,
    color: &'a GraphicsColor,
    color_buffer: &'a [u8; 4],
}

impl<'a> BasicGraphics<'a> {
    pub const fn new(frames: &'a mut [u8], width: u32, height: u32) -> Self {
        Self {
            frames,
            width,
            height,
            color: &GraphicsColor::WHITE,
            color_buffer: GraphicsColor::WHITE.to_buffer(),
        }
    }

    pub const fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize * 4
    }
}

impl<'a> Graphics<'a> for BasicGraphics<'a> {
    fn set_color(&mut self, color: &'a GraphicsColor) {
        self.color_buffer = color.to_buffer();
        self.color = color;
    }

    fn set_color_buffer(&mut self, color_buffer: &'a [u8; 4]) {
        self.color_buffer = color_buffer;
    }

    fn pixel(&mut self, x: u32, y: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index = self.get_index(x, y);
        if index + 3 < self.frames.len() {
            self.frames[index..index + 4].copy_from_slice(self.color_buffer);
        }
    }

    fn full_fill(&mut self) {
        let total_pixels: usize = (self.width * self.height) as usize * 4;
        self.frames[..total_pixels]
            .copy_from_slice(&self.color_buffer.repeat((total_pixels + 3) / 4)[..total_pixels]);
    }

    fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        let start_x = x.clamp(0, self.width - 1);
        let start_y = y.clamp(0, self.height - 1);
        let end_x = (x + width).clamp(0, self.width);
        let end_y = (y + height).clamp(0, self.height);

        let mut start_index: usize;
        let mut end_index: usize;
        let mut delta: usize;

        for py in start_y..end_y {
            start_index = self.get_index(start_x, py);
            end_index = self.get_index(end_x, py);
            delta = end_index - start_index;

            self.frames[start_index..end_index]
                .copy_from_slice(&self.color_buffer.repeat((delta + 3) / 4)[..]);
        }
    }
}
