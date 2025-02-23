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
        self.cm.add::<T>(e, c); // TODO: do we need ::<T> in this call instance?
        // TODO: after adding component we need to update system ids in registered systems too
        println!("To which systems this entity has to be added? e: {}", e);
        //self.sm.add(&e, c);
    }

    pub fn get_component<T: Display + Any>(&mut self, e: &Entity) -> Option<&mut T> {
        self.cm.get(e)
    }

    // Systems
    pub fn register_system<T: System + Any>(&mut self, s: T) -> TypeId {
        self.sm.register(s)
    }

    pub fn apply(&mut self, id: &TypeId) {
        self.sm.apply(&id, &mut self.cm);
    }

    pub fn kick_all_systems(&mut self) {
        self.sm.kick_all_systems(&mut self.cm);
    }
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

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
            println!("System32: add: e: {}", e);
            self.entities.insert(e);
        }

        fn remove(&mut self, e: Entity) {
            self.entities.remove(&e);
        }

        fn get_component_types(&self) -> &HashSet<TypeId> {
            &self.component_types
        }

        fn apply(&self, cm: &mut ComponentManager) {
            println!("System32: apply");
            for e in self.entities.iter() {
                println!("System32: apply for e: {}", e);
                let v = cm.get::<u32>(e).unwrap();
                *v += 1;
            }
        }
    }

    #[test]
    fn test_coordinator_for_simple_component() {
        let mut c = Coordinator::new();

        let s = SystemU32::new();
        let sys_id = c.register_system(s);

        let e1 = c.get_entity();
        let e2 = c.get_entity();

        c.register_component::<u32>();
        let mut v1: u32 = 1;
        let mut v2: u32 = 1;
        c.add_component(e1, v1);
        c.add_component(e2, v2);

        let s = SystemU32::new();
//        let sys_id = c.register_system(s);
        c.apply(&sys_id);

// TODO: point of focus
//        let v1_updated = c.get_component::<u32>(&e1);
//        assert_eq!(Some(&mut (v1+1)), v1_updated);
//        let v2_updated = c.get_component::<u32>(&e2);
//        assert_eq!(Some(&mut (v2+1)), v2_updated);
    }
}


