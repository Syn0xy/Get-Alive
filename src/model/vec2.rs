mod fvec2;
mod ivec2;

pub use fvec2::*;
pub use ivec2::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}
