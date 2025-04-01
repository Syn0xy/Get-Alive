mod application;
mod constants;
mod model;

use application::{
    App,
    entity::BasicEntityManager,
    input::{BasicInputSystem, InputSystem},
    renderer::BasicRendererManager,
    runtime::BasicRuntimeManager,
};
use winit::{error::EventLoopError, event_loop::EventLoop};

fn main() {
    EventLoop::new()
        .map(run_application)
        .map_err(handle_event_loop_error)
        .ok();
}

fn run_application(event_loop: EventLoop<()>) {
    let window_attributes = constants::application::window_attributes();
    let runtime_manager = BasicRuntimeManager::default();
    let input_system = BasicInputSystem::from_bindings(constants::inputs::INPUT_ACTION_MAPS);
    let entity_manager = BasicEntityManager::default();

    let mut app: App<
        BasicRuntimeManager,
        BasicInputSystem,
        BasicEntityManager,
        BasicRendererManager,
    > = App::new(
        window_attributes,
        runtime_manager,
        input_system,
        entity_manager,
    );

    event_loop.set_control_flow(constants::application::CONTROL_FLOW);
    event_loop
        .run_app(&mut app)
        .map_err(handle_event_loop_error)
        .ok();
}

fn handle_event_loop_error(error: EventLoopError) {
    match error {
        EventLoopError::NotSupported(_) => {}
        EventLoopError::Os(_) => {}
        EventLoopError::RecreationAttempt => {}
        EventLoopError::ExitFailure(_) => {}
    }
}
