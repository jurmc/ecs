use crate::ecs::Entity;
use crate::ecs::ComponentManager;

use std::collections::HashSet;
use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;


pub trait System {
    fn add(&mut self, e: Entity);
    fn remove(&mut self, e: Entity);

    fn get_component_types(&self) -> &HashSet<TypeId>;
    fn apply(&self, cm: &mut ComponentManager);
}

pub struct SystemManager {
    system_component_types: HashMap<TypeId, HashSet<TypeId>>, // TODO: 2nd TypeId could be aliaed to CompType or sth alike
    systems               : HashMap<TypeId, Box<dyn System>>,
}

impl SystemManager {
    pub fn new() -> SystemManager{
        SystemManager {
            system_component_types: HashMap::new(),
            systems: HashMap::new(),
        }
    }

    pub fn register<T: System + Any>(&mut self, system: T) -> TypeId {
        let sys_id = TypeId::of::<T>();
        self.system_component_types.insert(sys_id, system.get_component_types().clone());
        self.systems.insert(sys_id, Box::new(system));
        sys_id
    }

    pub fn apply(&self, id: TypeId, cm: &mut ComponentManager) {
        self.systems.get(&id).unwrap().apply(cm)
    }

    // TODO: we rather iterate over containted systems, this function will be removed
    pub fn kick_all_systems(&self, cm: &mut ComponentManager) {
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
        entities: HashSet<Entity>,
        component_types: HashSet<TypeId>,
    }

    impl TestSystem {
        fn new() -> TestSystem {
            TestSystem {
                entities: HashSet::new(),
                component_types: HashSet::new(),
            }
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

        fn get_component_types(&self) -> &HashSet<TypeId> {
            &self.component_types
        }

        fn apply(&self, cm: &mut ComponentManager) {
            for e in self.entities.iter() {
                let v = cm.get::<i32>(e).unwrap();
                *v += 1;
            }
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
        let e1: Entity = 1; // Will become a part of TestSystem
        let e2: Entity = 2; // Will not be a part of TestSystem
        let v1: i32 = 1;
        let mut v2: i32 = 2;

        let mut cm = ComponentManager::new();
        cm.register::<i32>();
        cm.add(e1, v1);
        cm.add(e2, v2);

        let mut sm = SystemManager::new();
        let mut s = TestSystem::new();
        s.add(e1);

        let sys_id = sm.register(s);

        sm.apply(sys_id, &mut cm);

        assert_eq!(Some(&mut (v1+1)), cm.get(&e1), "Should be incremented as this entity IS a part of a TestSystem"); 
        assert_eq!(Some(&mut (v2)), cm.get(&e2), "Should not be incremented as this entity IS NOT part of a TestSystem");
    }
}

//
// System implementations
// TODO: move system implementations out of this module

pub struct Render {
    entities: HashSet<Entity>,
    component_types: HashSet<TypeId>,
}

impl Render {
    pub fn new() -> Render {
        Render {
            entities: HashSet::new(),
            component_types: HashSet::new(),
        }
    }
}

impl System for Render {
    fn add(&mut self, e: Entity) {
        self.entities.insert(e);
    }
    fn remove(&mut self, e: Entity) {
        // TODO: not implemented
    }

    fn get_component_types(&self) -> &HashSet<TypeId> {
        &self.component_types
    }

    fn apply(&self, cm: &mut ComponentManager) {
        println!("Apply for Render");
        for e in self.entities.iter() {
            println!(" e: {}", e);
        }
    }
}

pub struct Transform {
    entities: HashSet<Entity>,
    component_types: HashSet<TypeId>,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            entities: HashSet::new(),
            component_types: HashSet::new(),
        }
    }
}

impl System for Transform {
    fn add(&mut self, e: Entity) {
        self.entities.insert(e);
    }

    fn remove(&mut self, e: Entity) {
        // TODO: not implemented
    }

    fn get_component_types(&self) -> &HashSet<TypeId> {
        &self.component_types
    }

    fn apply(&self, cm: &mut ComponentManager) {
        println!("Apply for Transform");
        for e in self.entities.iter() {
            println!(" e: {}", e);
            println!("   component1: {}", e);
        }
    }
}
