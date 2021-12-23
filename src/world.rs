use std::any::TypeId;

use crate::storage::Component;
use crate::storage::ComponentStorage;
use crate::system::System;
use crate::system::SystemController;

pub type Entity = usize;


/// A world is a collection of entities and components, and a collection of systems.
/// 
/// A world is used to create entities, and to add components to entities.
/// 
/// A world also provides a way to manage systems.
pub struct World{
    // Storage contains all the components for each entity.
    storage: Vec<ComponentStorage>,

    // Systems Controller contains all the systems that are added to the world, as well as handles the systems
    systems_controller: Option<SystemController>,
    
    // Entity counter stores the next available entity.
    entity_counter: usize,
}

impl World{
    /// Create a new world.
    pub fn new() -> Self{
        Self{
            storage: Vec::new(),
            systems_controller: Some(SystemController::new()),
            entity_counter: 0,
        }
    }

    /* 
     *    Entity - defines various Entity functions
     *    - create_entity() - creates a new entity
     *    - create_entity_with_components() - creates a new entity with the given components
     *    - has() - checks if an entity has a component
     */

    /// Create a new entity.
    /// 
    /// Init all components with default values (None).
    /// 
    /// Returns the entity id.
    pub fn create_entity(&mut self) -> Entity{
        for storage in &mut self.storage.iter_mut(){
            storage.init();
        }

        let entity = self.entity_counter;
        self.entity_counter += 1;
        entity
    }

    /// Create a new entity with the given components, which are stored as vectors
    /// 
    /// Returns the entity id.
    pub fn create_entity_with_components<T: 'static + Component>(&mut self, components: Vec<T>) -> Entity{
        let entity = self.create_entity();
        for component in components{
            self.add_component(entity, component);
        }
        entity
    }

    /// Check if an entity has a component.
    pub fn has_component(&self, entity: Entity, component_id: TypeId) -> bool{
        for storage in &self.storage{
            if storage.has_component(entity, component_id){
                return true;
            }
        }
        false
    }

    /// Get the current entity counter.
    pub fn get_entity_count(&self) -> usize{
        self.entity_counter
    }



    /* 
     *   Components - defines various component functions
     *  
     *    add_component - add a component to an entity
     *    get_component - get a component from an entity
     *    get_component_mut - get a component from an entity, mutable
     *    set_component - set a component for an entity
     *    has_component - check if an entity has a component
     */

    /// Add a component to the entity.
    /// 
    /// If the component doesn't exist, it will be created.
    pub fn add_component<T: 'static + Component>(&mut self, entity: Entity, component: T) {
        // Before we add the component, we need to check if the storage already exists
        if self.check_storage_exists::<T>(){
            // Storage exists, so we can just set the component
            self.set_component(entity, component);
            return;
        }

        // Init storage for the entity, using the component type
        self.init_storage::<T>();

        self.set_component(entity, component);
    }

    /// Get the component from the given entity.
    pub fn get_component<T: 'static + Component>(&self, entity: Entity) -> Option<&T> {
        if !self.check_storage_exists::<T>(){
            panic!("Error: Storage does not exist");
        }
        for storage in &self.storage{
            if storage.check_type_id::<T>(){
                return storage.get::<T>(entity); 
            }
        }   
        
        None
    }

    /// Get the component from the given entity, but mutable.
    pub fn get_component_mut<T: 'static + Component>(&mut self, entity: Entity) -> Option<&mut T> {
        if !self.check_storage_exists::<T>(){
            panic!("Error: Storage does not exist");
        }

        for storage in &mut self.storage{
            if storage.check_type_id::<T>(){
                return storage.get_mut::<T>(entity); 
            }
        }   
        
        None
    }

    /// Set the component from the given entity.
    pub fn set_component<T: 'static + Component>(&mut self, entity: Entity, component: T) {
        if !self.check_storage_exists::<T>(){
            panic!("Error: Storage does not exist");
        }

        for storage in &mut self.storage{
            if storage.check_type_id::<T>(){
                storage.set(entity, component);
                return;
            }
        }
    }

    /* Storage */

    /// Initialize a new storage of component type T, setting all components to None.
    pub fn init_storage<T: 'static + Component>(&mut self) {
        if self.check_storage_exists::<T>(){
            panic!("Error: Storage already exists");
        }

        let mut storage = ComponentStorage::new();

        storage.set_type_id::<T>();

        for _ in 0..self.entity_counter{
            storage.init();
        }

        self.storage.push(storage);
    }

    /// Check if the storage for the given entity exists.
    /// 
    /// Returns true if it does, false if it doesn't.
    fn check_storage_exists<T: 'static + Component>(&self) -> bool{
        for storage in &self.storage{
            if storage.check_type_id::<T>(){
                return true;
            }
        }
        false
    }

    /* Systems */

    /// Adds a system to the world.
    pub fn add_system<T: System + 'static>(&mut self, system: T) {
        let controller = self.systems_controller.as_mut().unwrap();
        
        controller.add_system(system);
    }

    /// Update all systems.
    pub fn update(&mut self){
        let mut controller = self.systems_controller.take().unwrap();
        controller.update(self);

        self.systems_controller = Some(controller);
    }

    /// Remove a system from the world.
    pub fn remove_system<T: System + 'static>(&mut self) {
        let controller = self.systems_controller.as_mut().unwrap();
        
        controller.remove_system::<T>();
    }

}