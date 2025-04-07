use crate::model::vec2::IVec2;

#[derive(Debug)]
pub struct Player;

#[derive(Default, Debug)]
pub struct PlayerController {
    pub direction: IVec2,
}
