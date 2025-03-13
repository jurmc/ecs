use crate::Entity;
use crate::ComponentType;

use std::collections::HashSet;
use std::collections::HashMap;
use std::any::Any;

pub struct GlobalComponent {
    components: HashMap<u8, Box<dyn Any>>, // TODO: instead of u8 I'd like to have some
}

impl GlobalComponent{
    pub fn new() -> GlobalComponent{
        GlobalComponent{
            components: HashMap::new(),
        }
    }

    pub fn add_global<T: Any>(&mut self, name: u8, c: T) {
        self.components.insert(name, Box::new(c));
    }

    pub fn get_global<T: 'static>(&mut self, name: u8) -> Option<&mut T> {
        let val = self.components.get_mut(&name).unwrap().downcast_mut::<T>();
        val
    }
}

pub struct ComponentArray<T> {
    components: HashMap<Entity, T>,
                                                  // string type as a key
}

impl<T> ComponentArray<T> {
    pub fn new(name: &'static str) -> ComponentArray<T> { // TODO: name are is not used...
        ComponentArray {
            components: HashMap::new(),
        }
    }

    pub fn add(&mut self, e: Entity, component: T) {
        self.components.insert(e, component);
    }

    pub fn get(&mut self, e: &Entity) -> Option<&mut T> {
        self.components.get_mut(e)
    }

    pub fn remove(&mut self, e: &Entity) -> Option<T> {
        self.components.remove(e)
    }
}

pub struct ComponentManager {
    global: GlobalComponent,
    component_types: HashSet<ComponentType>,
    component_arrays: HashMap<ComponentType, Box<dyn Any>>,
    entity_to_component_types: HashMap<Entity, HashSet<ComponentType>>,
}

impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            global: GlobalComponent::new(),
            component_types: HashSet::new(),
            component_arrays: HashMap::new(),
            entity_to_component_types: HashMap::new(),
        }
    }

    // Global Components
    pub fn add_global<T: Any>(&mut self, name: u8, c: T) {
        self.global.add_global(name, c);
    }

    pub fn get_global<T: 'static>(&mut self, name: u8) -> Option<&mut T> {
        self.global.get_global(name)
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

    pub fn get<T: Any>(&mut self, e: &Entity) -> Option<&mut T> {
        let array = self.get_component_array();
        array.get(&e)
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
    fn test_global_components() {
        let mut g = GlobalComponent::new();
        g.add_global(1u8, vec![1u8, 2u8, 3u8]);
        assert_eq!(Some(&mut vec![1u8, 2u8, 3u8]), g.get_global::<Vec<u8>>(1u8));

        let v = g.get_global::<Vec<u8>>(1u8).unwrap();
        v.pop();
        assert_eq!(Some(&mut vec![1u8, 2u8]), g.get_global::<Vec<u8>>(1u8));

    }

    #[test]
    fn test_component_array() {
        let mut a = ComponentArray::new("test_array");
        let e1: Entity = 1;
        let e2: Entity = 2;
        a.add(e1, "one");
        a.add(e2, "two");

        assert_eq!(Some(&mut "one"), a.get(&e1));
        assert_eq!(Some(&mut "two"), a.get(&e2));

        assert_eq!(Some("one"), a.remove(&e1));
        assert_eq!(None, a.get(&e1));
        assert_eq!(None, a.remove(&e1));
        assert_eq!(Some(&mut "two"), a.get(&e2));
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

        assert_eq!(Some(&mut 1), cm.get::<i32>(&e1));
        assert_eq!(Some(&mut 2), cm.get::<i32>(&e2));

        assert_eq!(Some(&mut Coords { x: 5, y: 10 }), cm.get::<Coords>(&e1));
        assert_eq!(None, cm.get::<Coords>(&e2));

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
    fn test_cm_panics_if_entity_added_without_prior_registration() {
        let mut cm = ComponentManager::new();
        let e: Entity = 1;
        cm.add(e, 3.14);
    }
}
