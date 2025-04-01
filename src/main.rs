use application::{
    App,
    input::{BasicInputSystem, InputSystem},
};
use winit::{error::EventLoopError, event_loop::EventLoop};

mod application;
mod constants;

fn main() {
    EventLoop::new()
        .map(run_application)
        .map_err(handle_event_loop_error)
        .ok();
}

fn run_application(event_loop: EventLoop<()>) {
    let window_attributes = constants::application::window_attributes();
    let input_system = BasicInputSystem::from_bindings(constants::inputs::INPUT_ACTION_MAPS);

    let mut app = App::new(window_attributes, input_system);

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
