use super::Vec2;

type I = f32;
pub type FVec2 = Vec2<I>;

impl FVec2 {
    pub const ZERO: Self = Self::of(0.0);
    pub const ONE: Self = Self::of(1.0);

    pub const fn new(x: I, y: I) -> Self {
        Self { x, y }
    }

    pub const fn of(a: I) -> Self {
        Self::new(a, a)
    }
}
