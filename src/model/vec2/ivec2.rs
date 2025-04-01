use super::Vec2;

type I = i32;
pub type IVec2 = Vec2<I>;

impl IVec2 {
    pub const ZERO: Self = Self::of(0);
    pub const ONE: Self = Self::of(1);

    pub const fn new(x: I, y: I) -> Self {
        Self { x, y }
    }

    pub const fn of(a: I) -> Self {
        Self::new(a, a)
    }
}
