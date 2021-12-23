use std::any::TypeId;


use crate::world::Entity;


// Define Component
//
// Component is a trait that defines a component. 
// 
// A component is a blob of data that is attached to an entity.
//
// It does not contain any logic, and is instead used to define data used by systems.
pub trait Component: Send + Sync {
    /// Get the component as a reference.
    fn get(&self) -> &dyn std::any::Any;
    /// Get the component as a mutable reference.
    fn get_mut(&mut self) -> &mut dyn std::any::Any;
    /// Get the type id of the component.
    fn get_type_id(&self) -> TypeId;
}
pub struct ComponentStorage {
    // We define a component as an optional type, so that if an entity does not have a component,
    // it will be None.
    components: Vec<Option<Box<dyn Component>>>,

    pub type_id: TypeId,
}

impl ComponentStorage {
    pub fn new() -> Self {
        Self{
            components: Vec::new(),
            type_id: TypeId::of::<()>(),
        }
    }

    /// Initialize the component storage with default values.
    /// 
    /// This is used when creating a new entity.
    pub fn init(&mut self) {
        self.components.push(None);
    }

    /// Set the type id of the component storage.
    pub fn set_type_id<T: 'static + Component>(&mut self) {
        self.type_id = TypeId::of::<T>();
    }

    /// Check the type id of the component storage against a given type
    pub fn check_type_id<T: 'static + Component>(&self) -> bool {
        self.type_id == TypeId::of::<T>()
    }

    /// Get the component from the given entity.
    pub fn get<T: 'static + Component>(&self, index: Entity) -> Option<&T> {
        if let Some(v) = self.components[index].as_ref().map(|c| c.get()){
            if let Some(component) = v.downcast_ref::<T>(){
                return Some(component);
            }else{
                panic!("Component type mismatch");
            }
        }

        None
    }

    /// Get the component from the given entity, but mutable.
    pub fn get_mut<T: 'static + Component>(&mut self, index: Entity) -> Option<&mut T> {
        if let Some(v) = self.components[index].as_mut().map(|c| c.get_mut()){
            if let Some(component) = v.downcast_mut::<T>(){
                return Some(component);
            }else{
                panic!("Component type mismatch");
            }
        }
        None
    }

    /// Sets the component for the given entity.
    pub fn set<T: 'static + Component>(&mut self, index: Entity, component: T) {
        self.components[index] = Some(Box::new(component));
    }

    /// Check if the entity has a component.
    pub fn has(&self, index: Entity, component: TypeId) -> bool {
        if let Some(v) = self.components[index].as_ref().map(|c| c.get()){
            if v.type_id() == component {
                return true;
            }
        }
        false
    }
}

