#[derive(Debug, Default)]
pub enum GraphicsColor {
    #[default]
    WHITE,
    BLACK,
    RED,
    GREEN,
    BLUE,
}

pub trait Graphics<'a> {
    // fn new(frames: &'a mut [u8], width: u32, height: u32) -> Self
    // where
    //     Self: Sized;
    fn set_color(&mut self, color: &'a GraphicsColor);
    fn set_color_buffer(&mut self, color_buffer: &'a [u8; 4]);
    fn pixel(&mut self, x: u32, y: u32);
    fn full_fill(&mut self);
    fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32);
}

impl GraphicsColor {
    pub const fn to_buffer(&self) -> &[u8; 4] {
        use GraphicsColor::*;

        match self {
            WHITE => &[0xFF, 0xFF, 0xFF, 0xFF],
            BLACK => &[0x00, 0x00, 0x00, 0xFF],
            RED => &[0xFF, 0x00, 0x00, 0xFF],
            GREEN => &[0x00, 0xFF, 0x00, 0xFF],
            BLUE => &[0x00, 0x00, 0xFF, 0xFF],
        }
    }
}
