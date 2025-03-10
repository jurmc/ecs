use crate::Entity;
use crate::ComponentManager;

use std::collections::HashSet;
use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;


pub trait System {
    fn add(&mut self, e: Entity);
    fn remove(&mut self, e: Entity);

    fn get_component_types(&self) -> &HashSet<TypeId>;
    fn apply(&mut self, cm: &mut ComponentManager);
}

pub struct SystemManager {
    system_component_types: HashMap<TypeId, HashSet<TypeId>>, // TODO: 2nd TypeId could be aliaed to CompType or sth alike,
                                                              // more generaly, it'd more clear if
                                                              // we have ComponetType alias and
                                                              // SystemType alias
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

    pub fn add_component(&mut self, e: Entity, component_types: &HashSet<TypeId>) { // TODO: this
                                                                                    // method
                                                                                    // should
                                                                                    // rather be
                                                                                    // called
                                                                                    // update_components?
        for (_, sys) in self.systems.iter_mut() {
            let sys_component_types = sys.get_component_types();
            let fit_for_sys = sys_component_types.is_subset(component_types);
            if fit_for_sys {
                sys.add(e);
            }
        }
    }

    pub fn apply(&mut self, id: &TypeId, cm: &mut ComponentManager) {
        self.systems.get_mut(&id).unwrap().apply(cm)
    }

    pub fn apply_all(&mut self, cm: &mut ComponentManager) {
        for (_, system) in self.systems.iter_mut() {
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

        fn apply(&mut self, cm: &mut ComponentManager) {
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
        let s = TestSystem::new();
        // TODO: SystemManager works fine as long as system is registered before entity with
        // relevant componets is added. We have to supplement  sm.register() in a way that all
        // existig entities managed by SM will be checked if they sghould be added to newly added
        // system
        let sys_id = sm.register(s); // TODO: this works fine, but see TODO below
        sm.add_component(e1, &HashSet::from_iter(vec![TypeId::of::<i32>()]));
        //let sys_id = sm.register(s); // TODO: this will no work at the moment, fix it

        sm.apply(&sys_id, &mut cm);
        assert_eq!(Some(&mut (v1+1)), cm.get(&e1), "Should be incremented as this entity IS a part of a TestSystem"); 
        assert_eq!(Some(&mut (v2)), cm.get(&e2), "Should not be incremented as this entity IS NOT part of a TestSystem");

    }
}

