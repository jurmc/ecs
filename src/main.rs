use ecs::Coordinator;
use ecs::EntitiesPool;
use ecs::ComponentManager;
use ecs::ComponentArray;
use ecs::Render;
use ecs::Transform;
use ecs::SystemManager;
use ecs::System;

use std::fmt;

use std::vec::Vec;

pub mod ecs;

// TODO EntitiesPool::get() should return Option/Result (read about Option and Result in book)
// TODO: add unit tests
// TODO: unit tests in separate files/modules

struct Coords(i32, i32);

impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Coords {} {}", self.0, self.1)
    }
}

fn main() {
    some_fun1();
    //some_fun2();
}

fn some_fun1() {
    println!("some_fun1()");
    
    // WIP: Create and user Coordinator
    let mut c = Coordinator::new();
    println!("Cooridnator created");
    //c.register_component<>() 
    //////////////////////

    let mut pool = c.pool;
    let mut cm = c.cm;
    let mut sm = c.sm;

    cm.register_new::<Coords>();
    cm.register_new::<i32>();

    let particle1 = pool.get();
    let particle2 = pool.get();

    // Coords are tupels (x, y)
    cm.add_component(particle1, Coords(0, 10));
    cm.add_component(particle2, Coords(2, 10));

    // Weight is just u32
    cm.add_component(particle1, 5);
    cm.add_component(particle1, 10);

    // Systems
    let mut transform_sys = Transform::new();
    transform_sys.add_entity(particle1);
    transform_sys.add_entity(particle2);

    let mut render_sys = Render::new();
    render_sys.add_entity(particle1);
    render_sys.add_entity(particle2);

    sm.register(render_sys);
    sm.register(transform_sys);

    sm.kick_all_systems(&cm);

}

//fn some_fun2() {
//    println!("Test entity pool");
//    let mut pool = EntitiesPool::new();
//    let entity1 = pool.get();
//    println!("Got entity: {}", entity1);
//
//    let entity2 = pool.get();
//    println!("Got entity: {}", entity2);
//
//    pool.give_back(entity1);
//    pool.give_back(entity1);
//
//    let entity3 = pool.get();
//    println!("Got entity: {}", entity3);
//    println!("-------------------------------------------------------");
//
//    println!("-Test-simple-component-arrays---------------------------------");
//    let mut comp_arr1 = ComponentArray::new("comp_arr1");
//    let (e1, e2) = (pool.get(), pool.get());
//
//    let mut comp_arr2 = ComponentArray::new("comp_arr2");
//    let (e3, e4) = (pool.get(), pool.get());
//    comp_arr2.add(e3, 1.5);
//    comp_arr2.add(e4, 2.5);
//
//    comp_arr1.dump();
//    comp_arr2.dump();
//
//    println!("comp_arr1.get(e1): {}", comp_arr1.get(&e1));
//    println!("comp_arr1.get(e2): {}", comp_arr1.get(&e2));
//
//    println!("comp_arr2.get(e3): {}", comp_arr2.get(&e3));
//    println!("comp_arr2.get(e4): {}", comp_arr2.get(&e4));
//
//    println!("-Test-component-manger----------------------------------");
//
//    let mut cm = ComponentManager::new();
//    cm.register(comp_arr1);
//    cm.register(comp_arr2);
//
//    cm.dump();
//
//    cm.add_component(pool.get(), 9);
//    cm.add_component(pool.get(), 8.5);
//
//    println!("-Test-system-manipulation-------------------------------------------------");
//
//    let mut t = Transform::new();
//    t.add_entity(pool.get());
//    t.add_entity(pool.get());
//
//    let mut r = Render::new();
//    r.add_entity(pool.get());
//    r.add_entity(pool.get());
//
//    let systems: Vec<Box<dyn System>> = vec![Box::new(r), Box::new(t)] ;
//
//    for system in systems {
//        system.apply(&cm)
//    }
//
//}

