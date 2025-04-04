use crate::Entity;
use crate::ComponentManager;
use crate::ComponentType;
use crate::SystemType;
use crate::Coordinator;

use std::collections::HashSet;
use std::collections::HashMap;
use std::any::Any;
use std::rc::Rc;
use std::cell::{Ref, RefCell};

pub trait System {
    fn add(&mut self, e: Entity);
    fn remove(&mut self, e: Entity);

    fn get_component_types(&self) -> &HashSet<ComponentType>;
    fn apply(&mut self, cm: &mut ComponentManager) -> Box<dyn Fn(&mut Coordinator)>;
}

pub struct SystemManager {
    system_component_types: HashMap<SystemType, HashSet<ComponentType>>,
    systems               : HashMap<SystemType, Rc<RefCell<dyn System>>>,
}

impl SystemManager {
    pub fn new() -> SystemManager{
        SystemManager {
            system_component_types: HashMap::new(),
            systems: HashMap::new(),
        }
    }

    pub fn register<T: System + Any>(&mut self, system: Rc<RefCell<T>>) {
        let sys_id = SystemType::of::<T>();
        self.system_component_types.insert(sys_id, system.borrow().get_component_types().clone());
        self.systems.insert(sys_id, system);
    }

    pub fn add_component(&mut self, e: Entity, component_types: &HashSet<ComponentType>) { // TODO: this
                                                                                    // method
                                                                                    // should
                                                                                    // rather be
                                                                                    // called
                                                                                    // update_components?
        for (_, sys) in self.systems.iter_mut() {
            let fit_for_sys = sys.borrow().get_component_types().is_subset(component_types);
            if fit_for_sys {
                sys.borrow_mut().add(e);
            }
        }
    }

    pub fn apply_all(&mut self, cm: &mut ComponentManager) -> Vec<Box<dyn Fn(&mut Coordinator)>> {
        let mut result = Vec::new();
        for (_, system) in self.systems.iter_mut() {
            let outcome = system.borrow_mut().apply(cm);
            result.push(outcome);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestSystem {
        entities: HashSet<Entity>,
        component_types: HashSet<ComponentType>,
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

        fn get_component_types(&self) -> &HashSet<ComponentType> {
            &self.component_types
        }

        fn apply(&mut self, cm: &mut ComponentManager) -> Box<dyn Fn(&mut Coordinator)> {

            for e in self.entities.iter() {
                let v = cm.get_mut::<i32>(e).unwrap();
                *v += 1;
            }

            Box::new(| coordinator: &mut Coordinator | {
                let e = coordinator.entity_take();
                let c:  i32 = 100;
                coordinator.add_component(e, c);
            })
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
        let test_sys = Rc::new(RefCell::new(TestSystem::new()));
        // TODO: SystemManager works fine as long as system is registered before entity with
        // relevant componets is added. We have to supplement  sm.register() in a way that all
        // existig entities managed by SM will be checked if they sghould be added to newly added
        // system
        sm.register(test_sys.clone()); // TODO: sys_id is not needed
        sm.add_component(e1, &HashSet::from_iter(vec![ComponentType::of::<i32>()]));
        assert_eq!(
            HashSet::from_iter(vec![e1]),
            test_sys.borrow().entities);
        test_sys.borrow_mut().apply(&mut cm);

        assert_eq!(Some(&(v1+1)), cm.get(&e1), "Should be incremented as this entity IS a part of a TestSystem");
        assert_eq!(Some(&(v2)), cm.get(&e2), "Should not be incremented as this entity IS NOT part of a TestSystem");
    }

}

