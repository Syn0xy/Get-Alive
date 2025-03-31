use std::any::Any;

use super::EntityBuilder;

pub type EntityId = usize;

pub trait EntityManager: ComponentSystem + EntityComponentSystem {
    fn new() -> Self;

    fn get_entities(&self) -> &Vec<EntityId>;

    fn create_entity(&mut self) -> EntityId;
    fn destroy_entity(&mut self, entity_id: EntityId) -> bool;

    fn build_entity(&mut self) -> impl EntityBuilder;
}

pub trait EntityComponentSystem {
    fn attach_component<T: Any>(&mut self, entity_id: EntityId, component: T);

    fn entity_components<T: Any>(&self, entity_id: EntityId) -> Vec<&T>;
    fn entity_component<T: Any>(&self, entity_id: EntityId) -> Option<&T>;

    fn entity_components_mut<T: Any>(&mut self, entity_id: EntityId) -> Vec<&mut T>;
    fn entity_component_mut<T: Any>(&mut self, entity_id: EntityId) -> Option<&mut T>;
}

pub trait ComponentSystem {
    fn components<T: Any>(&self) -> Vec<(EntityId, &T)>;
    fn component<T: Any>(&self) -> Option<(EntityId, &T)>;

    fn components_mut<T: Any>(&mut self) -> Vec<(EntityId, &mut T)>;
    fn component_mut<T: Any>(&mut self) -> Option<(EntityId, &mut T)>;
}
