use std::{cell::RefCell, rc::Rc};

use crate::app::{
    AppState,
    main::{EntityManager, MainEvent, handle_main_event},
    render::handle_render,
};

use super::{EventLoop, dispatcher::Dispatcher};

pub struct BasicEventLoop<D: Dispatcher, S: EntityManager> {
    dispatcher: Rc<RefCell<D>>,
    state: Rc<RefCell<AppState<S>>>,
}

impl<D: Dispatcher, S: EntityManager> BasicEventLoop<D, S> {
    pub fn new(dispatcher: D, state: AppState<S>) -> Self {
        Self {
            dispatcher: Rc::new(RefCell::new(dispatcher)),
            state: Rc::new(RefCell::new(state)),
        }
    }
}

impl<D: Dispatcher + 'static, S: EntityManager + 'static> EventLoop for BasicEventLoop<D, S> {
    fn run(&mut self) {
        {
            let mut dispatcher = self.dispatcher.borrow_mut();

            dispatcher.subscribe::<MainEvent<D, S>, _>(handle_main_event);
            dispatcher.subscribe(handle_render);
        }

        loop {
            let dispatcher = Rc::downgrade(&self.dispatcher);
            let app_state = Rc::downgrade(&self.state);

            self.dispatcher.borrow_mut().dispatch(&MainEvent::Tick {
                dispatcher,
                app_state,
            });
        }
    }
}
