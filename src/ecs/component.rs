use std::collections::HashSet;
use std::vec::Vec;
use std::fmt::Display;
use std::any::TypeId;
use std::any::Any;

pub struct ComponentArray<T: Display> {
    components: Vec<T>,
}

impl<T:  Display> ComponentArray<T> {
    pub fn new() -> ComponentArray<T> {
        ComponentArray { components: Vec::new(), }
    }
    pub fn add(&mut self, component: T) {
        self.components.push(component);
    }

    pub fn dump(&mut self) {
        for c in self.components.iter() {
            println!("c: {}", c);
        }
    }
}

pub struct ComponentManager {
    components: HashSet<TypeId>,
}

impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager { components: HashSet::new(), }
    }

    pub fn register<T: Display + Any>(&mut self, component_array: ComponentArray<T>) {
        self.components.insert(TypeId::of::<T>());
    }

    pub fn dump(&self) {
        for c in self.components.iter() {
            println!("c: {:?}", c);
        }
    }
}

