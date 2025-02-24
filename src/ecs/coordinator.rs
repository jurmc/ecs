use crate::EntitiesPool;
use crate::ecs::Entity;
use crate::ComponentManager;
use crate::SystemManager;
use crate::ecs::System;

use std::collections::HashSet;
use std::fmt::Display;
use std::any::Any;
use std::any::TypeId;
use std::fmt;

pub struct Coordinator {
    pool: EntitiesPool,
    cm: ComponentManager,
    sm: SystemManager,    // TODO: make this field private
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
        self.pool.add_component_type::<T>(e);
        self.cm.add::<T>(e, c); // TODO: do we need ::<T> in this call instance?

        let component_types_for_entity = self.pool.get_component_types(e);
        match component_types_for_entity {
            Some(component_types_for_entity) =>
                self.sm.add_component(e, component_types_for_entity),
                None => {
                    // TODO: If let might be bettre, we're not interestend in handling None
                }
        }
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

    struct SimpleSystem{
        entities: HashSet<Entity>,
        component_types: HashSet<TypeId>,
    }

    impl SimpleSystem {
        fn new() -> SimpleSystem {
            SimpleSystem {
                entities: HashSet::new(),
                component_types: vec![
                    TypeId::of::<u32>()
                ].into_iter().collect(),
            }
        }
    }

    impl System for SimpleSystem {
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

        let s = SimpleSystem::new();
        let sys_id = c.register_system(s);

        let e1 = c.get_entity();
        let e2 = c.get_entity();

        c.register_component::<u32>();
        let v1: u32 = 1;
        let v2: u32 = 1;

        c.add_component(e1, v1);
        c.add_component(e2, v2);
        c.apply(&sys_id);

        let v1_updated = c.get_component::<u32>(&e1);
        assert_eq!(Some(&mut (v1+1)), v1_updated);
        let v2_updated = c.get_component::<u32>(&e2);
        assert_eq!(Some(&mut (v2+1)), v2_updated);
    }

    #[derive(Debug, PartialEq)]
    struct Position { x: i32, y: i32, }
    struct Velocity { vx: i32, vy: i32, }
    impl fmt::Display for Position {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Position display (TODO: to be removed)")
        }
    }
    impl fmt::Display for Velocity {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Velocity display (TODO: to be removed)")
        }
    }

    struct ComplexSystem{
        entities: HashSet<Entity>,
        component_types: HashSet<TypeId>,
    }

    impl ComplexSystem {
        fn new() -> ComplexSystem {
            ComplexSystem {
                entities: HashSet::new(),
                component_types: vec![
                    TypeId::of::<Position>(),
                    TypeId::of::<Velocity>(),
                ].into_iter().collect(),
            }
        }
    }

    impl System for ComplexSystem {
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
            println!("ComplexSystem: apply");
            for e in self.entities.iter() {
                println!("ComplexSystem: apply for e: {}", e);
                let position = cm.get::<Position>(e).unwrap();
                let (x, y) = (position.x, position.y);
                let velocity = cm.get::<Velocity>(e).unwrap();
                let new_pos = Position { x: x + velocity.vx, y: y + velocity.vy };
                cm.add(*e, new_pos);
            }
        }
    }

    #[test]
    fn test_coordinator_for_complex_two_componets() {
        let mut c = Coordinator::new();

        let s = ComplexSystem::new();
        let sys_id = c.register_system(s);

        let e1 = c.get_entity();
        let e2 = c.get_entity();

        c.register_component::<Position>();
        c.register_component::<Velocity>();

        let pos1 = Position { x: 1, y: 2 };
        let vel1 = Velocity { vx: 1, vy: 2 };
        c.add_component(e1, pos1);
        c.add_component(e1, vel1);
        let pos2 = Position { x: 1, y: 2 };
        let vel2 = Velocity { vx: -1, vy: -2 };
        c.add_component(e2, pos2);
        c.add_component(e2, vel2);

        c.apply(&sys_id);

        let updated_pos1 = c.get_component::<Position>(&e1).unwrap();
        let expected_pos1 = &mut Position { x: 2, y: 4 };
        assert_eq!(expected_pos1, updated_pos1);

        let updated_pos2 = c.get_component::<Position>(&e2).unwrap();
        let expected_pos2 = &mut Position { x: 0, y: 0 };
        assert_eq!(expected_pos2, updated_pos2);
    }
}


