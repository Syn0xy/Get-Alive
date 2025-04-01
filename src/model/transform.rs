use super::vec2::FVec2;

#[derive(Default, Debug, Clone, Copy)]
pub struct Transform {
    pub position: FVec2,
    pub scale: FVec2,
}
