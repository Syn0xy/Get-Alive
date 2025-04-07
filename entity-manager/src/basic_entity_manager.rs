use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use super::{
    ComponentSystem, EntityBuilder, EntityComponentSystem, EntityId, EntityManager,
    basic_entity_builder::BasicEntityBuilder,
};

#[derive(Debug, Default)]
pub struct BasicEntityManager {
    entity_id_sequence: EntityId,
    entities: Vec<EntityId>,
    component_entity: HashMap<(EntityId, TypeId), Vec<Box<dyn Any>>>,
}

impl EntityManager for BasicEntityManager {
    fn new() -> Self {
        Self::default()
    }

    fn get_entities(&self) -> &Vec<EntityId> {
        &self.entities
    }

    fn create_entity(&mut self) -> EntityId {
        let entity_id = self.entity_id_sequence;
        self.entity_id_sequence += 1;
        self.entities.push(entity_id);

        entity_id
    }

    fn build_entity(&mut self) -> impl EntityBuilder {
        BasicEntityBuilder::new(self.create_entity(), self)
    }

    fn destroy_entity(&mut self, entity_id: EntityId) -> bool {
        let initial_len = self.component_entity.len();
        self.entities.retain(|&x| x != entity_id);
        self.component_entity.retain(|&(id, _), _| id != entity_id);
        self.component_entity.len() < initial_len
    }
}

impl ComponentSystem for BasicEntityManager {
    fn components<T: Any>(&self) -> Vec<(EntityId, &T)> {
        let component_type_id = TypeId::of::<T>();
        self.component_entity
            .iter()
            .filter_map(|(&(ref entity_id, type_id), components)| {
                (type_id == component_type_id).then(|| {
                    components.iter().filter_map(|boxed| {
                        boxed
                            .downcast_ref::<T>()
                            .map(|ref_component| (*entity_id, ref_component))
                    })
                })
            })
            .flatten()
            .collect()
    }

    fn components_mut<T: Any>(&mut self) -> Vec<(EntityId, &mut T)> {
        let component_type_id = TypeId::of::<T>();
        self.component_entity
            .iter_mut()
            .filter_map(|(&(ref entity_id, type_id), components)| {
                (type_id == component_type_id).then(|| {
                    components.iter_mut().filter_map(|boxed| {
                        boxed
                            .downcast_mut::<T>()
                            .map(|ref_component| (*entity_id, ref_component))
                    })
                })
            })
            .flatten()
            .collect()
    }

    fn component<T: Any>(&self) -> Option<(EntityId, &T)> {
        self.components().first().copied()
    }

    fn component_mut<T: Any>(&mut self) -> Option<(EntityId, &mut T)> {
        todo!()
    }
}

impl EntityComponentSystem for BasicEntityManager {
    fn attach_component<T: Any>(&mut self, entity_id: EntityId, component: T) {
        let component_type_id = TypeId::of::<T>();
        let key = (entity_id, component_type_id);
        self.component_entity
            .entry(key)
            .or_default()
            .push(Box::new(component));
    }

    fn entity_components<T: Any>(&self, entity_id: EntityId) -> Vec<&T> {
        let component_type_id = TypeId::of::<T>();
        let key = (entity_id, component_type_id);
        self.component_entity
            .get(&key)
            .map(|components| {
                components
                    .iter()
                    .filter_map(|comp| comp.downcast_ref::<T>())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn entity_components_mut<T: Any>(&mut self, entity_id: EntityId) -> Vec<&mut T> {
        let component_type_id = TypeId::of::<T>();
        let key = (entity_id, component_type_id);
        self.component_entity
            .get_mut(&key)
            .map(|components| {
                components
                    .iter_mut()
                    .filter_map(|comp| comp.downcast_mut::<T>())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn entity_component<T: Any>(&self, entity_id: EntityId) -> Option<&T> {
        self.entity_components(entity_id).first().copied()
    }

    fn entity_component_mut<T: Any>(&mut self, _entity_id: EntityId) -> Option<&mut T> {
        todo!()
    }
}
