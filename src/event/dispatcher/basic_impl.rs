use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::Dispatcher;

#[derive(Default)]
pub struct BasicDispatcher {
    handlers: HashMap<TypeId, Vec<Arc<Mutex<dyn FnMut(&dyn Any) + Send + Sync>>>>,
}

impl Dispatcher for BasicDispatcher {
    fn subscribe<E: 'static, F>(&mut self, mut callback: F)
    where
        F: FnMut(&E) + Send + Sync + 'static,
    {
        let event_id = TypeId::of::<E>();
        let callback = Arc::new(Mutex::new(move |event: &dyn Any| {
            if let Some(event) = event.downcast_ref::<E>() {
                callback(event);
            }
        }));
        self.handlers.entry(event_id).or_default().push(callback);
    }

    fn dispatch<E: 'static>(&mut self, event: &E) {
        let event_id = TypeId::of::<E>();
        self.handlers.get_mut(&event_id).map(|handlers| {
            for handler in handlers {
                handler.lock().map(|mut h| h(event)).ok();
            }
        });
    }
}
