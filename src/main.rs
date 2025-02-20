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

// TODO maybe EntitiesPool::get() should return Option/Result (read about Option and Result in book)
// TODO: add unit tests
// TODO: unit tests in separate files/modules

struct Coords(i32, i32);

// TODO: relax this requirment that Component has to imple Display (maybe)
impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Coords {} {}", self.0, self.1)
    }
}

fn main() {
    some_fun1();
}

fn some_fun1() {
    println!("some_fun1()");

    // WIP: Create and user Coordinator
    let mut c = Coordinator::new();

    let particle1 = c.get_entity();
    let particle2 = c.get_entity();

    c.register_component::<Coords>();
    c.register_component::<i32>();

    c.add_component(particle1, Coords(0, 10));
    c.add_component(particle2, Coords(2, 10));

    c.add_component(particle1, 5);
    c.add_component(particle1, 10);

    let mut sm = c.sm;

    // Systems
    let mut transform_sys = Transform::new();
    let mut render_sys = Render::new();

    // TODO: point of focus1
    // TODO: when entity gets its component, then relevant system should get this entity
    // or I have to understand somehing more?
    transform_sys.add(particle1);
    transform_sys.add(particle2);
    render_sys.add(particle1);
    render_sys.add(particle2);

    sm.register(render_sys);
    sm.register(transform_sys);
//    c.register_system(render_sys);
//    c.register_system(transform_sys);

    // TODO: Point of focus2
    //c.kick_all_systems();

}

