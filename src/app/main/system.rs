use std::any::Any;

pub type EntityId = usize;

pub trait EntityManager {
    fn new() -> Self;
    fn create_entity(&mut self) -> EntityId;
    fn destroy_entity(&mut self, entity_id: EntityId) -> bool;
    fn get_entities(&self) -> &Vec<EntityId>;
    fn attach_component<T: Any>(&mut self, entity_id: EntityId, component: T);
    fn borrow_component<T: Any>(&self, entity_id: EntityId) -> Vec<&T>;
    fn borrow_component_mut<T: Any>(&mut self, entity_id: EntityId) -> Vec<&mut T>;
}
