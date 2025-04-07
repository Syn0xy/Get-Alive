use std::any::Any;

use crate::EntityComponentSystem;

use super::{EntityBuilder, EntityId};

#[derive(Debug)]
pub(super) struct BasicEntityBuilder<'a, S: EntityComponentSystem> {
    entity_id: EntityId,
    manager: &'a mut S,
}

impl<'a, S: EntityComponentSystem> BasicEntityBuilder<'a, S> {
    pub(super) fn new(entity_id: EntityId, manager: &'a mut S) -> Self {
        Self { entity_id, manager }
    }
}

impl<'a, S: EntityComponentSystem> EntityBuilder for BasicEntityBuilder<'a, S> {
    fn with_component<T: Any>(self, component: T) -> Self {
        self.manager.attach_component(self.entity_id, component);
        self
    }

    fn build(self) -> EntityId {
        self.entity_id
    }
}
