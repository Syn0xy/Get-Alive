use super::{InputError, InputKey};

pub trait InputSystem {
    fn get_input(&self) -> Result<InputKey, InputError>;
}
