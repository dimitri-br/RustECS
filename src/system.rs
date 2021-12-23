use crossbeam::thread::ScopedJoinHandle;
use crossbeam::thread::scope;
use std::sync::Arc;
use std::sync::Mutex;


/// A system is a function that is called every frame, and is used to update the world.
/// 
/// It takes in a query of components, then applies the logic to the entities that match the query.
/// 
/// It must by Send + Sync

pub trait System: Send + Sync {
    /// The function that is called every frame.
    fn update(&mut self, world: Arc<Mutex<&mut crate::world::World>>);
    /// Get the type of the system.
    fn get_type_id(&self) -> std::any::TypeId;
}


/// System controller is used to add and remove systems from the world.
/// 
/// It manages the systems and their order. 
/// 
/// It is able to run independently of the world, and thus can be run in parallel.
pub struct SystemController{
    /// The systems that are currently running.
    systems: Vec<Box<dyn System>>,
}

impl SystemController{
    /// Create a new system controller.
    pub fn new() -> Self{
        Self{
            systems: Vec::new(),
        }
    }

    /// Add a system to the controller.
    pub fn add_system<T: 'static + System>(&mut self, system: T) {
        if self.has_system::<T>(){
            panic!("System already exists");
        }

        self.systems.push(Box::new(system));
    }

    /// Remove a system from the controller.
    pub fn remove_system<T: 'static + System>(&mut self) {
        if !self.has_system::<T>(){
            panic!("System does not exist");
        }
        
        self.systems.retain(|system| system.get_type_id() != std::any::TypeId::of::<T>());
    }

    /// Update the systems.
    pub fn update(&mut self, world: &mut crate::world::World) {
        let world = Arc::new(Mutex::new(world));
        // Use crossbeam to run the systems in parallel

        // Create a vec of handles to the systems

        scope(|scope| {
            let mut handles = Vec::<ScopedJoinHandle<()>>::new();

            for system in &mut self.systems{
                let world_clone = world.clone();
                let handle = scope.spawn(move |_| {
                    system.update(world_clone);
                });
                handles.push(handle);
            }

            // Wait for all the handles to finish
            for handle in handles{
                handle.join().unwrap();
            }
        }).unwrap();

        /*for system in &mut self.systems {
            system.update(world);
        }*/
    }

    /// Check if a system is in the controller.
    fn has_system<T: 'static + System>(&self) -> bool {
        self.systems.iter().any(|system| system.get_type_id() == std::any::TypeId::of::<T>())
    }
}