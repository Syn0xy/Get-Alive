use std::any::Any;

use super::EntityId;

pub trait EntityBuilder {
    fn with_component<T: Any>(self, component: T) -> Self;
    fn build(self) -> EntityId;
}
