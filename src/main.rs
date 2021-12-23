mod query;
mod storage;
mod system;
mod world;

use ecs_proc_macro::{Component, system};


#[derive(Debug, Component)]
pub struct Position{
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Component)]
pub struct SomeRandomComponent{
    pub some_value: u32,
}

/// A system is a way to update the world.
/// 
/// When using the `#[system_]` macro, a new struct is generated with the name of the function, consuming
/// the function. The name of the struct is the name of the function, converted to camel case, with
/// the prefix "System".
/// 
/// For example, the function `my_system` will generate a struct called `SystemMySystem`.
#[system]
pub fn some_random_system(world: std::sync::Arc<std::sync::Mutex<&mut world::World>>) {
    let mut query = query::Query::new();
    query.add::<Position>();
    query.add::<SomeRandomComponent>();

    let mut world = world.lock().unwrap();
    
    for entity in query.get(&world).iter(){
        // Handle immutable references first
        let position = world.get_component::<Position>(*entity).unwrap();
        let added_value = (position.x + position.y) as u32;

        // Then handle mutable references
        let some_random_component = world.get_component_mut::<SomeRandomComponent>(*entity).unwrap();
        some_random_component.some_value += added_value;
    }
}

fn benchmark(world: &mut world::World) {
    for _ in 0..10{
        let entity = world.create_entity_with_components(vec![Position {x: 1.0, y: 1.0}]);
        world.add_component(entity, SomeRandomComponent {some_value: 1});
    }

    world.update();
}

fn main() {

    println!("Starting benchmark");

    // Get the time 
    let start = std::time::Instant::now();

    // Create a new world
    let mut world = world::World::new();

    // Initialize some components

    world.init_storage::<Position>();
    world.init_storage::<SomeRandomComponent>();

    // Add some systems

    world.add_system(SystemSomeRandomSystem);

    // Run the benchmark

    for i in 0..1000{
        benchmark(&mut world);

        if i % 100 == 0 {
            println!("Reached {}", i);
            println!("Num entities: {}", world.get_entity_count());
        }
    }

    // Test removing systems

    world.remove_system::<SystemSomeRandomSystem>();


    let end = std::time::Instant::now();


    println!("{:?}s", end.duration_since(start).as_secs_f32());
    println!("Num entities: {}", world.get_entity_count());

    // Hang the program so the user can see the output
    std::thread::park();
}
