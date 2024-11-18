use crate::ecs::Entity;

use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt::Display;
use std::any::TypeId;
use std::any::Any;

pub struct ComponentArray<T: Display> {
    components: HashMap<Entity, T>,
}

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

