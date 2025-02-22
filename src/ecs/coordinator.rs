use crate::EntitiesPool;
use crate::ecs::Entity;
use crate::ComponentManager;
use crate::SystemManager;
use crate::ecs::System;

use std::fmt::Display;
use std::any::Any;


pub struct Coordinator {
    pool: EntitiesPool,
    cm: ComponentManager,
    pub sm: SystemManager,    // TODO: make this field private
}

impl Coordinator {
    pub fn new() -> Coordinator {
        Coordinator {
            pool: EntitiesPool::new(),
            cm: ComponentManager::new(),
            sm: SystemManager::new(),
        }
    }

    // Entities
    pub fn get_entity(&mut self) -> Entity {
        self.pool.get()
    }

    // Components
    pub fn register_component<T: Display + Any>(&mut self) {
        self.cm.register::<T>();
    }

    pub fn add_component<T: Display + Any>(&mut self, e: Entity, c: T) {
        self.cm.add::<T>(e, c);
    }

    // Systems
    pub fn register_system<T: System + Any>(&mut self, s: T) {
        self.sm.register(s);
    }

    pub fn kick_all_systems(&mut self) {
        self.sm.kick_all_systems(&mut self.cm);
    }
    // TODO
}


build fail
// TODO: point of focus
// strart UT'ing of coordinator
// ...

