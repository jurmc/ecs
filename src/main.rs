//use std::vec::Vec;

use ecs::EntitiesPool;
use ecs::ComponentManager;
use ecs::ComponentArray;
//use ecs::Render;
//use ecs::Transform;
//use ecs::System;

pub mod ecs;

fn main() {
    println!("Test entity pool");
    let mut pool = EntitiesPool::new();
    let entity1 = pool.get();
    println!("Got entity: {}", entity1);

    let entity2 = pool.get();
    println!("Got entity: {}", entity2);

    pool.give_back(entity1);
    pool.give_back(entity1);

    let entity3 = pool.get();
    println!("Got entity: {}", entity3);
    println!("-------------------------------------------------------");

    println!("-Test-simple-component-arrays---------------------------------");
    let mut comp_arr1 = ComponentArray::new();
    let (e1, e2) = (pool.get(), pool.get());
    comp_arr1.add(e1, 1);
    comp_arr1.add(e2, 2);

    let mut comp_arr2 = ComponentArray::new();
    let (e3, e4) = (pool.get(), pool.get());
    comp_arr2.add(e3, 1.5);
    comp_arr2.add(e4, 2.5);

    comp_arr1.dump();
    comp_arr2.dump();

    println!("comp_arr1.get(e1): {}", comp_arr1.get(&e1));
    println!("comp_arr1.get(e2): {}", comp_arr1.get(&e2));

    println!("comp_arr2.get(e3): {}", comp_arr2.get(&e3));
    println!("comp_arr2.get(e4): {}", comp_arr2.get(&e4));

    println!("-[TODO]-Test-component-manger----------------------------------");
    println!("-[TODO]-add-component-(after-registering)-of-particular-type-for-an-entity-");
    println!("-[TODO]-get-component-of-particular-type-for-an-entity-");

    let mut cm = ComponentManager::new();
    cm.register(comp_arr1);
    cm.register(comp_arr2);

    cm.dump();

    //cm.add_component(pool.get(), 1);
    //cm.add_component(pool.get(), 1.5);

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

}

