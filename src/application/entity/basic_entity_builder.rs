use std::any::Any;

use super::{EntityBuilder, EntityId, EntityManager};

#[derive(Debug)]
pub(super) struct BasicEntityBuilder<'a, M: EntityManager> {
    entity_id: EntityId,
    manager: &'a mut M,
}

impl<'a, M: EntityManager> BasicEntityBuilder<'a, M> {
    pub(super) fn new(entity_id: EntityId, manager: &'a mut M) -> Self {
        Self { entity_id, manager }
    }
}

impl<'a, M: EntityManager> EntityBuilder for BasicEntityBuilder<'a, M> {
    fn with_component<T: Any>(self, component: T) -> Self {
        self.manager.attach_component(self.entity_id, component);
        self
    }

    fn build(self) -> EntityId {
        self.entity_id
    }
}
