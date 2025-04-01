use winit::keyboard::KeyCode;

use crate::application::input::{InputAction, InputActionMap};

pub const INPUT_ACTION_MAPS: &[InputActionMap] = &[
    InputActionMap("Menu", &[]),
    InputActionMap(
        "Gameplay",
        &[
            InputAction("Up", &[KeyCode::KeyZ, KeyCode::KeyW]),
            InputAction("Down", &[KeyCode::KeyS]),
            InputAction("Left", &[KeyCode::KeyQ, KeyCode::KeyA]),
            InputAction("Right", &[KeyCode::KeyD]),
        ],
    ),
];
