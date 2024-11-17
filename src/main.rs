use std::vec::Vec;

use ecs::EntitiesPool;
use ecs::ComponentManager;
use ecs::ComponentArray;
use ecs::Render;
use ecs::Transform;
use ecs::System;

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

    println!("Test-component-arrays---------------------------------");
    let mut comp_arr1 = ComponentArray::new();
    comp_arr1.add(1);
    comp_arr1.add(2);
    let mut comp_arr2 = ComponentArray::new();
    comp_arr2.add(1.5);
    comp_arr2.add(2.5);

    comp_arr1.dump();
    comp_arr2.dump();

    let mut cm = ComponentManager::new();
    cm.register(comp_arr1);
    cm.register(comp_arr2);
    cm.dump();

    println!("-Test-system-manipulation-------------------------------------------------");

    let mut t = Transform::new();
    t.add_entity(pool.get());
    t.add_entity(pool.get());

    let mut r = Render::new();
    r.add_entity(pool.get());
    r.add_entity(pool.get());

    let systems: Vec<Box<dyn System>> = vec![Box::new(r), Box::new(t)] ;

    for system in systems {
        system.apply()
    }

}

