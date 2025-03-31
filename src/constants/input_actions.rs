use winit::keyboard::KeyCode;

use crate::application::input::{InputAction, InputActionMap, InputContext, InputSystem};

pub const INPUT_SYSTEM: InputSystem = InputSystem {
    action_maps: &[INPUT_GAMEPLAY_MAPS],
};

const INPUT_GAMEPLAY_MAPS: InputActionMap = InputActionMap {
    context: InputContext::Gameplay,
    actions: &[
        InputAction {
            action: "Up",
            bindings: &[KeyCode::KeyZ],
        },
        InputAction {
            action: "Down",
            bindings: &[KeyCode::KeyS],
        },
        InputAction {
            action: "Left",
            bindings: &[KeyCode::KeyQ],
        },
        InputAction {
            action: "Right",
            bindings: &[KeyCode::KeyD],
        },
    ],
};
