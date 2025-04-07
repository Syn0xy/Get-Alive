mod constants;
mod model;

use application::basic::BasicApplication;
use entity::{BasicEntityManager, EntityManager};
use input::{InputManager, basic::BasicInputManager};

use constants::application::{CONTROL_FLOW, window_attributes};
use constants::inputs::INPUT_ACTION_MAPS;

fn main() {
    let window_attributes = window_attributes();

    BasicApplication::new(
        window_attributes,
        BasicEntityManager::new(),
        BasicInputManager::from_bindings(INPUT_ACTION_MAPS),
    )
    .run(CONTROL_FLOW);
}
