use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
    fmt,
};

use super::{ComponentSystem, EntityComponentSystem, EntityId, EntityManager};

#[derive(Default)]
pub struct BasicEntityManager {
    entity_id_sequence: EntityId,
    entities: Vec<EntityId>,
    component_entity: HashMap<(EntityId, TypeId), Vec<Box<dyn Any>>>,
}

impl EntityManager for BasicEntityManager {
    fn new() -> Self {
        Self::default()
    }

    fn create_entity(&mut self) -> EntityId {
        let entity_id = self.entity_id_sequence;
        self.entity_id_sequence += 1;
        self.entities.push(entity_id);

        entity_id
    }

    fn destroy_entity(&mut self, entity_id: EntityId) -> bool {
        let initial_len = self.component_entity.len();
        self.entities.retain(|&x| x != entity_id);
        self.component_entity.retain(|(id, _), _| *id != entity_id);
        self.component_entity.len() < initial_len
    }

    fn get_entities(&self) -> &Vec<EntityId> {
        &self.entities
    }

    fn attach_component<T: Any>(&mut self, entity_id: EntityId, component: T) {
        let component_type_id = TypeId::of::<T>();
        let key = (entity_id, component_type_id);
        self.component_entity
            .entry(key)
            .or_default()
            .push(Box::new(component));
    }
}

impl ComponentSystem for BasicEntityManager {
    fn borrow_component<T: Any>(&self) -> Vec<(EntityId, &T)> {
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

    fn borrow_component_mut<T: Any>(&mut self) -> Vec<(EntityId, &mut T)> {
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

    fn borrow_single_component<T: Any>(&self) -> Option<&T> {
        todo!()
    }

    fn borrow_single_component_mut<T: Any>(&mut self) -> Option<&mut T> {
        todo!()
    }
}

impl EntityComponentSystem for BasicEntityManager {
    fn borrow_entity_component<T: Any>(&self, entity_id: EntityId) -> Vec<&T> {
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

    fn borrow_entity_component_mut<T: Any>(&mut self, entity_id: EntityId) -> Vec<&mut T> {
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

    fn borrow_single_entity_component<T: Any>(&self, _entity_id: EntityId) -> Option<&T> {
        todo!()
    }

    fn borrow_single_entity_component_mut<T: Any>(
        &mut self,
        _entity_id: EntityId,
    ) -> Option<&mut T> {
        todo!()
    }
}

impl BasicEntityManager {
    pub fn get_components(&self, entity_id: EntityId) -> Vec<&dyn Any> {
        self.component_entity
            .iter()
            .filter_map(|(&(obj_id, _), component_list)| {
                if obj_id == entity_id {
                    Some(
                        component_list
                            .iter()
                            .map(|component| component.as_ref())
                            .collect::<Vec<_>>(),
                    )
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }
}

impl fmt::Display for BasicEntityManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "EntityManager with {} entity(ies):",
            &self.entities.len()
        )?;

        for &entity_id in &self.entities {
            writeln!(f, "\t - Entity {{{}}}:", &entity_id)?;

            let unique_type_ids: HashSet<TypeId> = self
                .component_entity
                .keys()
                .map(|(_, type_id)| *type_id)
                .collect();

            for unique_type_id in unique_type_ids {
                let key = &(entity_id, unique_type_id);

                if let Some(type_components) = self.component_entity.get(key) {
                    writeln!(f, "\t\t - {:?}", &unique_type_id)?;
                    for component in type_components {
                        writeln!(f, "\t\t\t - {:?}", &component)?;
                    }
                }
            }
        }

        Ok(())
    }
}
