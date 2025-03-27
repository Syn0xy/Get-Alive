use std::{cell::RefCell, rc::Weak};

use crate::{app::AppState, event::dispatcher::Dispatcher};

use super::EntityManager;

pub enum MainEvent<D: Dispatcher + 'static, S: EntityManager + 'static> {
    Tick {
        dispatcher: Weak<RefCell<D>>,
        app_state: Weak<RefCell<AppState<S>>>,
    },
    LoopExiting,
}
