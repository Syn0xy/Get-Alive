use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use crate::event::Dispatcher;

#[derive(Default)]
pub struct GenericDispatcher {
    handlers: HashMap<TypeId, Vec<Arc<dyn Fn(&dyn Any) + Send + Sync>>>,
}

impl Dispatcher for GenericDispatcher {
    fn subscribe<E: 'static, F>(&mut self, callback: F)
    where
        F: Fn(&E) + Send + Sync + 'static,
    {
        let event_id = TypeId::of::<E>();
        let callback = Arc::new(move |event: &dyn Any| {
            if let Some(event) = event.downcast_ref::<E>() {
                callback(event);
            }
        });
        self.handlers.entry(event_id).or_default().push(callback);
    }

    fn dispatch<E: 'static>(&self, event: &E) {
        let event_id = TypeId::of::<E>();
        self.handlers.get(&event_id).map(|handler| {
            handler.iter().for_each(|h| h(event));
        });
    }
}
