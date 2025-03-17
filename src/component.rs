use crate::Entity;
use crate::ComponentType;

use std::collections::HashSet;
use std::collections::HashMap;
use std::any::Any;

pub struct ComponentArray<T> {
    components: HashMap<Entity, T>,
                                                  // string type as a key
}

impl<T> ComponentArray<T> {
    pub fn new(_name: &'static str) -> ComponentArray<T> { // TODO: _name are is not used...
        ComponentArray {
            components: HashMap::new(),
        }
    }

    pub fn add(&mut self, e: Entity, component: T) {
        self.components.insert(e, component);
    }

    pub fn get(&mut self, e: &Entity) -> Option<&T> {
        self.components.get(e)
    }

    pub fn get_mut(&mut self, e: &Entity) -> Option<&mut T> {
        self.components.get_mut(e)
    }

    pub fn remove(&mut self, e: &Entity) -> Option<T> {
        self.components.remove(e)
    }
}

pub struct ComponentManager {
    component_types: HashSet<ComponentType>,
    component_arrays: HashMap<ComponentType, Box<dyn Any>>,
    entity_to_component_types: HashMap<Entity, HashSet<ComponentType>>,
}

impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            component_types: HashSet::new(),
            component_arrays: HashMap::new(),
            entity_to_component_types: HashMap::new(),
        }
    }

    // Entities Components
    pub fn register<T: Any>(&mut self) {
        self.component_types.insert(ComponentType::of::<T>());
        let arr: ComponentArray<T>  = ComponentArray::new("coords");
        self.component_arrays.insert(ComponentType::of::<T>(), Box::new(arr));
    }

    pub fn add<T: Any>(&mut self, e: Entity, component: T) {
        let id = ComponentType::of::<T>();
        if self.component_types.contains(&id) {
            let array = self.get_component_array();
            array.add(e, component);
        } else {
            panic!("Component type shoud be registered prior to its use");
        }

        if let Some(hash_set) = self.entity_to_component_types.get_mut(&e) {
            hash_set.insert(id);
        } else {
            self.entity_to_component_types.insert(e, HashSet::from_iter(vec![id]));
        };
    }

    pub fn get<T: Any>(&mut self, e: &Entity) -> Option<&T> {
        let array = self.get_component_array(); // TODO: get_component_array and get_component_array_mut?
        array.get(&e)
    }

    pub fn get_mut<T: Any>(&mut self, e: &Entity) -> Option<&mut T> {
        let array = self.get_component_array();
        array.get_mut(&e)
    }

    pub fn remove<T: Any>(&mut self, e: &Entity) -> Option<T> {
        let id = ComponentType::of::<T>();
        if let Some(hash_set) = self.entity_to_component_types.get_mut(&e) {
            hash_set.remove(&id);
        }

        let array = self.get_component_array();
        array.remove(e)
    }

    pub fn get_component_types(&self, e: Entity) -> HashSet<ComponentType> {
        match self.entity_to_component_types.get(&e) {
            Some(types) => types.clone(),
            None => HashSet::new()
        }
    }

    // Priv

    fn get_component_array<T: Any>(&mut self) -> &mut ComponentArray<T> {
        let id = ComponentType::of::<T>();
        self.component_arrays.get_mut(&id).unwrap().downcast_mut::<ComponentArray<T>>().unwrap()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_array() {
        let mut a = ComponentArray::new("test_array"); // TODO: we are
                                                                                   // not using
                                                                                   // name at the
                                                                                   // moment
        let e1: Entity = 1;
        let e2: Entity = 2;
        a.add(e1, "one");
        a.add(e2, "two");

        assert_eq!(Some(&"one"), a.get(&e1));
        assert_eq!(Some(&"two"), a.get(&e2));

        assert_eq!(Some("one"), a.remove(&e1));
        assert_eq!(None, a.get(&e1));
        assert_eq!(None, a.remove(&e1));
        assert_eq!(Some(&"two"), a.get(&e2));

        a.add(e1, "word1");
        if let Some(mut_ref) = a.get_mut(&e1) {
            *mut_ref = "word2";
        }
        assert_eq!(Some(&"word2"), a.get(&e1));

    }

    #[derive(Debug,PartialEq)]
    struct Coords {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_component_manager() {
        let e1: Entity = 1;
        let e2: Entity = 2;

        let mut cm = ComponentManager::new();
        cm.register::<i32>();
        cm.register::<Coords>();

        cm.add(e1, 1);
        cm.add(e1, Coords { x: 5, y: 10 });

        cm.add(e2, 2);

        assert_eq!(Some(&1), cm.get::<i32>(&e1));
        assert_eq!(Some(&2), cm.get::<i32>(&e2));

        assert_eq!(Some(&Coords { x: 5, y: 10 }), cm.get::<Coords>(&e1));
        assert_eq!(None, cm.get::<Coords>(&e2));

        if let Some(coords) = cm.get_mut::<Coords>(&e1) {
            coords.x = 6;
        }
        assert_eq!(Some(&Coords { x: 6, y: 10 }), cm.get::<Coords>(&e1));

        let e1_types = cm.get_component_types(e1);
        let e1_expected_types = HashSet::from_iter(vec![ComponentType::of::<i32>(), ComponentType::of::<Coords>()]);
        assert_eq!(e1_expected_types, e1_types);
        let e2_types = cm.get_component_types(e2);
        let ew_expected_types = HashSet::from_iter(vec![ComponentType::of::<i32>()]);
        assert_eq!(ew_expected_types, e2_types);

        cm.remove::<i32>(&e1);
        cm.remove::<Coords>(&e1);
        assert_eq!(None, cm.get::<i32>(&e1));
        assert_eq!(None, cm.get::<Coords>(&e1));

        let e1_types = cm.get_component_types(e1);
        let e1_expected_types = HashSet::new(); // Empty HashSet
        assert_eq!(e1_expected_types, e1_types);
    }

    #[test]
    #[should_panic]
    fn test_cm_panics_if_entity_added_without_prior_type_registration() {
        let mut cm = ComponentManager::new();
        let e: Entity = 1;
        cm.add(e, 3.14);
    }
}
