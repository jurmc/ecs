use crate::ecs::Entity;
use crate::ecs::ComponentManager;

use std::collections::HashSet;
use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;


pub trait System {
    fn add(&mut self, e: Entity);
    fn remove(&mut self, e: Entity);

    fn apply(&self, cm: &ComponentManager);
}

pub struct SystemManager {
    system_types: HashSet<TypeId>,
    systems: HashMap<TypeId, Box<dyn System>>,
}

impl SystemManager {
    pub fn new() -> SystemManager{
        SystemManager {
            system_types: HashSet::new(),
            systems: HashMap::new(),
        }
    }

    pub fn register<T: System + Any>(&mut self, system: T) {
        self.system_types.insert(TypeId::of::<T>());
        self.systems.insert(TypeId::of::<T>(), Box::new(system));
    }

    // TODO: we rather iterate over containted systems, this function will be removed
    pub fn kick_all_systems(&self, cm: &ComponentManager) {
        for (_, system) in self.systems.iter() {
            println!("some system will be kicked");
            system.apply(cm)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestSystem {
        entities: HashSet<Entity>
    }

    impl TestSystem {
        fn new() -> TestSystem {
            TestSystem { entities: HashSet::new() }
        }

        fn get_entities(&mut self) -> &HashSet<Entity> {
            &self.entities
        }
    }

    impl System for TestSystem {
        fn add(&mut self, e: Entity) {
            self.entities.insert(e);
        }

        fn remove(&mut self, e: Entity) {
            self.entities.remove(&e);
        }

        fn apply(&self, cm: &ComponentManager) {
        }
    }

    #[test]
    fn test_system() {
        let e1: Entity = 1;
        let e2: Entity = 2;

        let mut s = TestSystem::new();
        s.add(e1);
        s.add(e2);
        s.remove(e1);

        let expected: HashSet<Entity> = vec![e2].into_iter().collect();
        assert_eq!(expected, *s.get_entities());
    }

    #[test]
    fn test_system_manager() {
        let mut sm = SystemManager::new();
        let s = TestSystem::new();

        sm.register(s);
        // TODO: point of focus
        //sm.set_components_for_system(HashSet<TypeId-orSignatures>);
    }
}

//
// System implementations
// TODO: move system implementations out of this module

pub struct Render {
    entities: HashSet<Entity>
}

impl Render {
    pub fn new() -> Render {
        Render { entities: HashSet::new() }
    }
}

impl System for Render {
    fn add(&mut self, e: Entity) {
        self.entities.insert(e);
    }
    fn remove(&mut self, e: Entity) {
        // TODO: not implemented
    }

    fn apply(&self, cm: &ComponentManager) {
        println!("Apply for Render");
        for e in self.entities.iter() {
            println!(" e: {}", e);
        }
    }
}

pub struct Transform {
    entities: HashSet<Entity>
}

impl Transform {
    pub fn new() -> Transform {
        Transform { entities: HashSet::new() }
    }
}

impl System for Transform {
    fn add(&mut self, e: Entity) {
        self.entities.insert(e);
    }

    fn remove(&mut self, e: Entity) {
        // TODO: not implemented
    }

    fn apply(&self, cm: &ComponentManager) {
        println!("Apply for Transform");
        for e in self.entities.iter() {
            println!(" e: {}", e);
            println!("   component1: {}", e);
        }
    }
}
