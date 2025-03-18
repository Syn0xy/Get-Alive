use super::InputError;
use super::InputKey;

pub trait InputSystem {
    fn get_input(&self) -> Result<InputKey, InputError>;
}
