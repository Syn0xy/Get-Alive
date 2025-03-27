pub mod app;
pub mod constants;
pub mod event;

use app::{AppState, input::BasicInputSystem, main::BasicEntityManager};
use event::{BasicEventLoop, EventLoop, dispatcher::BasicDispatcher};

fn main() {
    let dispatcher = BasicDispatcher::default();
    let entity_manager = BasicEntityManager::default();
    let input_system = BasicInputSystem::default();
    let app = AppState::new(entity_manager, Box::new(input_system));

    BasicEventLoop::new(dispatcher, app).run();
}
