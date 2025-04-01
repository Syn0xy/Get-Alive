use winit::keyboard::KeyCode;

use crate::application::input::{InputAction, InputActionMap};

pub const INPUT_ACTION_MAPS: &[InputActionMap] = &[
    InputActionMap {
        context: "Menu",
        actions: &[],
    },
    InputActionMap {
        context: "Gameplay",
        actions: &[
            InputAction {
                action: "Up",
                bindings: &[KeyCode::KeyZ, KeyCode::KeyW],
            },
            InputAction {
                action: "Down",
                bindings: &[KeyCode::KeyS],
            },
            InputAction {
                action: "Left",
                bindings: &[KeyCode::KeyQ, KeyCode::KeyA],
            },
            InputAction {
                action: "Right",
                bindings: &[KeyCode::KeyD],
            },
        ],
    },
];
