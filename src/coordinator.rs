// TODO: change name of coordinator to just ecssth or sth ecs
//       this is just public interface to ecs
use crate::EntitiesPool;
use crate::Entity;
use crate::ComponentManager;
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
    pub fn take_entity(&mut self) -> Entity {
        self.pool.take()
    }

    // TODO: returning to pool is indeed missing

    // Components
    pub fn register_component<T: Any>(&mut self) {
        self.cm.register::<T>();
    }

    pub fn add_component<T: Any>(&mut self, e: Entity, c: T) {
        self.cm.add(e, c);
        let component_types_for_entity = self.cm.get_component_types(e);
        self.sm.add_component(e, &component_types_for_entity);
    }

    pub fn get<T: Any>(&mut self, e: &Entity) -> Option<&T> {
        self.cm.get(e)
    }

    pub fn get_mut<T: Any>(&mut self, e: &Entity) -> Option<&mut T> {
        self.cm.get_mut(e)
    }

    // Systems
    pub fn register_system<T: System + Any>(&mut self, s: T) -> SystemType {
        self.sm.register(s)
    }

    pub fn apply(&mut self, sys_id: &SystemType) -> Box<dyn Fn(&mut Coordinator)> {
        self.sm.apply(&sys_id, &mut self.cm)
    }

    pub fn apply_all(&mut self) -> Vec<Box<dyn Fn(&mut Coordinator)>> {
        self.sm.apply_all(&mut self.cm)
    }
}

#[cfg(test)]
mod tests {
    use crate::ComponentType;
    use super::*;
    use std::collections::HashSet;

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

        fn apply(&mut self, cm: &mut ComponentManager) -> Box<dyn Fn(&mut Coordinator)> {
            for e in self.entities.iter() {
                let v = cm.get_mut::<u32>(e).unwrap();
                *v += 1;
            }
            Box::new(| coordinator: &mut Coordinator | {
                coordinator.add_component(50, 100u32); 
            })
        }
    }

    #[test]
    fn test_system_returning_closure() {
        let mut c = Coordinator::new();

        let s = SimpleSystem::new(); // TODO: rename SimpleSystem to just TestSystem
        let sys_id = c.register_system(s);

        c.register_component::<u32>();
        let e1: u32 = c.take_entity();
        let v1: u32 = 1;
        c.add_component(e1, v1);

        // Our system is expected to:
        // 1) instanlty update e1's v1 comonent
        // 2) send us update, which after execution
        //   will cause addition of new entity with u32 component
        let updates = c.apply(&sys_id);

        let expected_e2: u32 = 50;
        let mut expected_v2: u32 = 100;

        // Check 1). e1:c1 updated, no additional entity in the ECS
        assert_eq!(Some(&(v1+1)), c.get::<u32>(&e1));
        assert_eq!(None, c.get::<u32>(&expected_e2));

        // Check 2). aditional component added
        updates(&mut c);
        assert_eq!(Some(&expected_v2), c.get::<u32>(&expected_e2));

    }

    #[test]
    fn test_coordinator_for_simple_component() {
        let mut c = Coordinator::new();

        let s = SimpleSystem::new();
        let sys_id = c.register_system(s); // TODO: this test should work also if system is
                                                   // registered after componets are added (see
                                                   // TODO: below)

        let e1 = c.take_entity();
        let e2 = c.take_entity();

        c.register_component::<u32>();
        let v1: u32 = 1;
        let v2: u32 = 1;

        c.add_component(e1, v1);
        c.add_component(e2, v2);

        //let sys_id = c.register_system(s); // TODO: this test should work also if system is
        //                                           // registered after componets are added (see
        //                                           // TODO: above)

        let _ignored = c.apply(&sys_id); // TODO: _ignored?

        assert_eq!(Some(&(v1+1)), c.get::<u32>(&e1));
        assert_eq!(Some(&(v2+1)), c.get::<u32>(&e2));
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

        fn apply(&mut self, cm: &mut ComponentManager)
            -> Box<dyn Fn(&mut Coordinator)> {

            for e in self.entities.iter() {
                let position = cm.get::<Position>(e).unwrap();  // TODO: get_mut, and modify
                                                                // without copying
                let (x, y) = (position.x, position.y);
                let velocity = cm.get::<Velocity>(e).unwrap();
                let new_pos = Position { x: x + velocity.vx, y: y + velocity.vy };
                cm.add(*e, new_pos);
            }

            Box::new(| _coordinator: &mut Coordinator | {
            })
        }
    }

    #[test]
    fn test_coordinator_for_complex_two_componets() {
        let mut c = Coordinator::new();

        let s = ComplexSystem::new();
        let sys_id = c.register_system(s);

        let e1 = c.take_entity();
        let e2 = c.take_entity();

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

        let _ignored = c.apply(&sys_id);

        let expected_pos1 = &Position { x: 2, y: 4 };
        assert_eq!(expected_pos1, c.get::<Position>(&e1).unwrap());

        let expected_pos2 = &mut Position { x: 0, y: 0 };
        assert_eq!(expected_pos2, c.get::<Position>(&e2).unwrap());
    }

}


