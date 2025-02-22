use crate::ecs::Entity;

use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt::Display;
use std::any::TypeId;
use std::any::Any;

use std::fmt;

pub struct ComponentArray<T: Display> {
    name: &'static str,
    components: HashMap<Entity, T>,
}

impl<T:  Display> ComponentArray<T> {
    pub fn new(name: &'static str) -> ComponentArray<T> {
        ComponentArray {
            name,
            components: HashMap::new(),
        }
    }

    pub fn add(&mut self, entity: Entity, component: T) {
        self.components.insert(entity, component);
    }

    pub fn get(&mut self, entity: &Entity) -> Option<&mut T> {
        self.components.get_mut(entity)
    }

    pub fn remove(&mut self, entity: &Entity) -> Option<T> {
        self.components.remove(entity)
    }

    pub fn dump(&self) {
        println!("Dump (type {:?}):", self.name);
        for (entity, component) in self.components.iter() {
            println!("entity: {}, component: {}", entity, component);
        }
    }
}

pub struct ComponentManager {
    component_types: HashSet<TypeId>,
    component_arrays: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            component_types: HashSet::new(),
            component_arrays: HashMap::new(),
        }
    }

    pub fn register<T: Display + Any>(&mut self) {
        self.component_types.insert(TypeId::of::<T>());
        let arr: ComponentArray<T>  = ComponentArray::new("coords");
        self.component_arrays.insert(TypeId::of::<T>(), Box::new(arr));
    }

    pub fn add<T: Display + Any>(&mut self, entity: Entity, component: T) {
        println!("Component added to ComponentManager");

        let id = TypeId::of::<T>();
        if self.component_types.contains(&id) {
            let array = self.get_component_array();
            array.add(entity, component);
            array.dump();
        }
    }

    pub fn get<T: Display + Any>(&mut self, entity: &Entity) -> Option<&mut T> {
        let array = self.get_component_array();
        array.get(&entity)
    }

    pub fn remove<T: Display + Any>(&mut self, entity: &Entity) -> Option<T> {
        let array = self.get_component_array();
        array.remove(entity)
    }

    fn get_component_array<T: Display + Any>(&mut self) -> &mut ComponentArray<T> {
        let id = TypeId::of::<T>();
        self.component_arrays.get_mut(&id).unwrap().downcast_mut::<ComponentArray<T>>().unwrap()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

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

    // TODO: relax this requirment that Component has to imple Display (maybe)
    impl fmt::Display for Coords {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "coords display (TODO: to be removed)")
        }
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

        cm.remove::<i32>(&e1);
        cm.remove::<Coords>(&e1);
        assert_eq!(None, cm.get::<i32>(&e1));
        assert_eq!(None, cm.get::<Coords>(&e1));
    }

}
