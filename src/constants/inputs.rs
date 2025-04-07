use input::{InputAction, InputActionMap};
use winit::keyboard::KeyCode;

pub enum InputContext {
    Menu,
    Gameplay,
}

use InputContext::*;

pub const INPUT_ACTION_MAPS: &[InputActionMap] = &[
    InputActionMap(Menu, &[InputAction("Exit", &[KeyCode::Escape])]),
    InputActionMap(
        Gameplay,
        &[
            InputAction("Up", &[KeyCode::KeyZ, KeyCode::KeyW]),
            InputAction("Down", &[KeyCode::KeyS]),
            InputAction("Left", &[KeyCode::KeyQ, KeyCode::KeyA]),
            InputAction("Right", &[KeyCode::KeyD]),
        ],
    ),
];
