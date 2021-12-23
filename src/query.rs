use std::any::TypeId;


use crate::{storage::Component, world::Entity};

/// A query is a way to find entities that have certain components.
/// 
/// A query is a collection of component types, and a collection of entities that have those components.
pub struct Query {
    pub components: Vec<TypeId>,
    pub entities: Vec<Entity>,
}

impl Query{
    /// Create a new query.
    pub fn new() -> Self {
        Self{
            components: Vec::new(),
            entities: Vec::new(),
        }
    }

    /// Add a component to the query.
    pub fn add<T: 'static + Component>(&mut self) {
        self.components.push(TypeId::of::<T>());
    }

    /// Get the entities that match the query.
    pub fn get(&self, world: &crate::world::World) -> Vec<Entity> {
        // Use the world to check if an entity has the components
        let mut entities = Vec::new();
        for entity in 0..world.get_entity_count() {
            let mut has_components = true;
            for component in self.components.iter() {
                // Check if the entity has the component
                if !world.has_component(entity, *component) {
                    has_components = false;
                    break;
                }
            }
            if has_components {
                entities.push(entity);
            }
        }        

        entities
    }
}