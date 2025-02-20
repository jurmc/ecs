use crate::ecs::Entity;

use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt::Display;
use std::any::TypeId;
use std::any::Any;

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

    pub fn get(&mut self, entity: &Entity) -> &T {
        self.components.get(entity).unwrap()
    }

    pub fn remove(&mut self, entity: &Entity) {
        self.components.remove(entity).unwrap();
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
        let mut arr: ComponentArray<T>  = ComponentArray::new("coords");
        self.component_arrays.insert(TypeId::of::<T>(), Box::new(arr));
    }

    pub fn add<T: Display + Any>(&mut self, entity: Entity, component: T) {
        println!("Component added to ComponentManager");

        let id = TypeId::of::<T>();
        if self.component_types.contains(&id) {
            println!("We have this");
            let array = self.get_component_array();
            array.add(entity, component);
            array.dump();
        }
    }

    // Priv //////////////

    fn get_component_array<T: Display + Any>(&mut self) -> &mut ComponentArray<T> {
        let id = TypeId::of::<T>();
        self.component_arrays.get_mut(&id).unwrap().downcast_mut::<ComponentArray<T>>().unwrap()
    }

    fn dump(&self) {
        for c in self.component_types.iter() {
            println!("c: {:?}", c);
        }
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
        assert_eq!(&"one", a.get(&e1));
        assert_eq!(&"two", a.get(&e2));

        let e3: Entity = 3;
        //a.get(&e3);
    }

}
