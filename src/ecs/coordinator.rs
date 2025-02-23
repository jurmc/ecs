use crate::EntitiesPool;
use crate::ecs::Entity;
use crate::ComponentManager;
use crate::SystemManager;
use crate::ecs::System;

use std::collections::HashSet;
use std::fmt::Display;
use std::any::Any;
use std::any::TypeId;


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
    pub fn register_system<T: System + Any>(&mut self, s: T) -> TypeId {
        self.sm.register(s)
    }

    pub fn apply(&mut self, id: TypeId) {
        self.sm.apply(id, &mut self.cm);
    }

    pub fn kick_all_systems(&mut self) {
        self.sm.kick_all_systems(&mut self.cm);
    }
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

// TODO: point of focus
// strart UT'ing of coordinator
// ...

    struct SystemU32 {
        entities: HashSet<Entity>,
        component_types: HashSet<TypeId>,
    }

    impl SystemU32 {
        fn new() -> SystemU32 {
            SystemU32 {
                entities: HashSet::new(),
                component_types: vec![TypeId::of::<u32>()].into_iter().collect(),
            }
        }
    }

    impl System for SystemU32{
        fn add(&mut self, e: Entity) {
            self.entities.insert(e);
        }

        fn remove(&mut self, e: Entity) {
            self.entities.remove(&e);
        }

        fn get_component_types(&self) -> &HashSet<TypeId> {
            &self.component_types
        }

        fn apply(&self, cm: &mut ComponentManager) {
            for e in self.entities.iter() {
                let v = cm.get::<u32>(e).unwrap();
                *v += 1;
            }
        }
    }

    #[test]
    fn test_coordinator_for_simple_component() {
        let mut c = Coordinator::new();

        let e1 = c.get_entity();
        let e2 = c.get_entity();

        c.register_component::<i32>();
        c.add_component(e1, 1);
        c.add_component(e2, 1);

        let s = SystemU32::new();
        let sys_id = c.register_system(s);
        c.apply(sys_id);
    }
}


