use super::InputError;

#[derive(Debug)]
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
    pub fn from_char(value: &char) -> Result<InputKey, InputError> {
        match value.to_ascii_uppercase() {
            'A' => Ok(Self::A),
            'B' => Ok(Self::B),
            _ => Err(InputError::InvalidInput(value.to_string())),
        }
    }
}
