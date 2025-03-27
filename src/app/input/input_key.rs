use std::str::FromStr;
use strum_macros::EnumString;

use super::InputError;

#[derive(Debug, EnumString)]
pub enum InputKey {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl InputKey {
    pub fn from_char(value: char) -> Result<Self, InputError> {
        let uppercased = value.to_ascii_uppercase().to_string();
        Self::from_str(&uppercased).map_err(|_| InputError::InvalidInput(value.to_string()))
    }
}
