// TODO: change name of coordinator to just ecssth or sth ecs
//       this is just public interface to ecs
use crate::EntitiesPool;
use crate::Entity;
use crate::ComponentManager;
use crate::ComponentType;
use crate::SystemManager;
use crate::System;
use crate::SystemType;

use std::any::Any;

pub struct Coordinator {
    pool: EntitiesPool,
    cm: ComponentManager,
    sm: SystemManager,
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

    // TODO: returning to pool is missing?

    // Entities Components
    pub fn register_component<T: Any>(&mut self) {
        self.cm.register::<T>();
    }

    pub fn add_component<T: Any>(&mut self, e: Entity, c: T) {
        self.cm.add(e, c);
        let component_types_for_entity = self.cm.get_component_types(e);
        self.sm.add_component(e, &component_types_for_entity);
    }

    pub fn get_component<T: Any>(&mut self, e: &Entity) -> Option<&mut T> {
        self.cm.get(e)
    }

    // Systems
    pub fn register_system<T: System + Any>(&mut self, s: T) -> SystemType {
        self.sm.register(s)
    }

    pub fn apply(&mut self, sys_id: &SystemType) {
        self.sm.apply(&sys_id, &mut self.cm);
    }

    pub fn apply_all(&mut self) {
        self.sm.apply_all(&mut self.cm);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    struct SimpleSystem{
        entities: HashSet<Entity>,
        component_types: HashSet<ComponentType>,
    }

    impl SimpleSystem {
        fn new() -> SimpleSystem {
            SimpleSystem {
                entities: HashSet::new(),
                component_types: vec![ComponentType::of::<u32>()].into_iter().collect(),
            }
        }
    }

    impl System for SimpleSystem {
        fn add(&mut self, e: Entity) {
            self.entities.insert(e);
        }

        fn remove(&mut self, e: Entity) {
            self.entities.remove(&e);
        }

        fn get_component_types(&self) -> &HashSet<ComponentType> {
            &self.component_types
        }

        fn apply(&mut self, cm: &mut ComponentManager) {
            for e in self.entities.iter() {
                let v = cm.get::<u32>(e).unwrap();
                *v += 1;
            }
        }
    }

    #[test]
    fn test_coordinator_for_simple_component() {
        let mut c = Coordinator::new();

        let s = SimpleSystem::new();
        let sys_id = c.register_system(s); // TODO: this test should work also if system is
                                                   // registered after componets are added (see
                                                   // TODO: below)

        let e1 = c.get_entity();
        let e2 = c.get_entity();

        c.register_component::<u32>();
        let v1: u32 = 1;
        let v2: u32 = 1;

        c.add_component(e1, v1);
        c.add_component(e2, v2);

        //let sys_id = c.register_system(s); // TODO: this test should work also if system is
        //                                           // registered after componets are added (see
        //                                           // TODO: above)

        c.apply(&sys_id);

        let v1_updated = c.get_component::<u32>(&e1);
        assert_eq!(Some(&mut (v1+1)), v1_updated);
        let v2_updated = c.get_component::<u32>(&e2);
        assert_eq!(Some(&mut (v2+1)), v2_updated);
    }

    #[derive(Debug, PartialEq)]
    struct Position { x: i32, y: i32, }
    struct Velocity { vx: i32, vy: i32, }

    struct ComplexSystem{
        entities: HashSet<Entity>,
        component_types: HashSet<ComponentType>,
    }

    impl ComplexSystem {
        fn new() -> ComplexSystem {
            ComplexSystem {
                entities: HashSet::new(),
                component_types: vec![
                    ComponentType::of::<Position>(),
                    ComponentType::of::<Velocity>(),
                ].into_iter().collect(),
            }
        }
    }

    impl System for ComplexSystem {
        fn add(&mut self, e: Entity) {
            self.entities.insert(e);
        }

        fn remove(&mut self, e: Entity) {
            self.entities.remove(&e);
        }

        fn get_component_types(&self) -> &HashSet<ComponentType> {
            &self.component_types
        }

        fn apply(&mut self, cm: &mut ComponentManager) {
            for e in self.entities.iter() {
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


