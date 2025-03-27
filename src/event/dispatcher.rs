mod basic_impl;

pub use basic_impl::*;

pub trait Dispatcher {
    fn subscribe<E: 'static, F>(&mut self, callback: F)
    where
        F: Fn(&E) + Send + Sync + 'static;
    fn dispatch<E: 'static>(&mut self, event: &E);
}
