use crate::ecs::Entity;

use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt::Display;
use std::any::TypeId;
use std::any::Any;

trait ComponentArrayIC {
}

pub struct ComponentArray<T: Display> {
    components: HashMap<Entity, T>,
}

impl<T:  Display> ComponentArrayIC for ComponentArray<T> {}

impl<T:  Display> ComponentArray<T> {
    pub fn new() -> ComponentArray<T> {
        ComponentArray { components: HashMap::new() }
    }

    pub fn add(&mut self, entity: Entity, component: T) {
        self.components.insert(entity, component);
    }

    pub fn get(&mut self, entity: &Entity) -> &T {
        self.components.get(entity).unwrap()
    }

    pub fn dump(&mut self) {
        println!("Dump:");
        for (entity, component) in self.components.iter() {
            println!("entity: {}, component: {}", entity, component);
        }
    }
}

pub struct ComponentManager {
    component_types: HashSet<TypeId>,
    component_arrays: HashMap<TypeId, Box<dyn ComponentArrayIC>>,
}

impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            component_types: HashSet::new(),
            component_arrays: HashMap::new(),
        }
    }

    pub fn register<T: Display + Any>(&mut self, component_array: ComponentArray<T>) {
        self.component_types.insert(TypeId::of::<T>());
        self.component_arrays.insert(TypeId::of::<T>(), Box::new(component_array));
    }

    pub fn add_component<T: Display + Any>(&self, entity: Entity, component: T) {
        println!("Component added to ComponentManager");

        let id = TypeId::of::<T>();
        if self.component_types.contains(&id) {
            println!("We have this");
            let array = self.component_arrays.get(&id).unwrap(); // 2. Find relevant component array
            // array.add(entity, component)// 3. add passed component to the relevant array
        }
    }

    // https://users.rust-lang.org/t/convert-generic-trait-t-back-to-struct/11581/2
    // https://doc.rust-lang.org/std/boxed/struct.Box.html#method.downcast
    fn get_component_array<T: Display + Any>(&self, component: T) -> ComponentArray<T> {
        let id = TypeId::of::<T>();
        *self.component_arrays.get(&id).unwrap()
    }

    pub fn dump(&self) {
        for c in self.component_types.iter() {
            println!("c: {:?}", c);
        }
    }
}

