use std::any::Any;

pub type EntityId = usize;

pub trait EntityManager: ComponentSystem + EntityComponentSystem {
    fn new() -> Self;

    fn create_entity(&mut self) -> EntityId;
    fn destroy_entity(&mut self, entity_id: EntityId) -> bool;

    fn get_entities(&self) -> &Vec<EntityId>;
    fn attach_component<T: Any>(&mut self, entity_id: EntityId, component: T);
}

pub trait EntityComponentSystem {
    fn borrow_entity_component<T: Any>(&self, entity_id: EntityId) -> Vec<&T>;
    fn borrow_entity_component_mut<T: Any>(&mut self, entity_id: EntityId) -> Vec<&mut T>;

    fn borrow_single_entity_component<T: Any>(&self, entity_id: EntityId) -> Option<&T>;
    fn borrow_single_entity_component_mut<T: Any>(&mut self, entity_id: EntityId)
    -> Option<&mut T>;
}

pub trait ComponentSystem {
    fn borrow_component<T: Any>(&self) -> Vec<(EntityId, &T)>;
    fn borrow_component_mut<T: Any>(&mut self) -> Vec<(EntityId, &mut T)>;

    fn borrow_single_component<T: Any>(&self) -> Option<&T>;
    fn borrow_single_component_mut<T: Any>(&mut self) -> Option<&mut T>;
}
