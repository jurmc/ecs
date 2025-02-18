use crate::ecs::Entity;
use crate::ecs::ComponentManager;

use std::collections::HashSet;
use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;


pub trait System {
    fn add(&mut self, entity: Entity);
    //fn remove_entity(entity: Entity);

    fn apply(&self, cm: &ComponentManager);
}

pub struct SystemManager {
    system_types: HashSet<TypeId>,
    systems: HashMap<TypeId, Box<dyn System>>,
}

impl SystemManager {
    pub fn new() -> SystemManager{
        SystemManager {
            system_types: HashSet::new(),
            systems: HashMap::new(),
        }
    }

    pub fn register<T: System + Any>(&mut self, system: T) {
        self.system_types.insert(TypeId::of::<T>());
        self.systems.insert(TypeId::of::<T>(), Box::new(system));
    }

    // TODO: we rather iterate over containted systems, this function will be removed
    pub fn kick_all_systems(&self, cm: &ComponentManager) {
        for (_, system) in self.systems.iter() {
            println!("some system will be kicked");
            system.apply(cm)
        }
    }
}

//
// System implementations

pub struct Render {
    entities: HashSet<Entity>
}

impl Render {
    pub fn new() -> Render {
        Render { entities: HashSet::new() }
    }
}

impl System for Render {
    fn add(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    fn apply(&self, cm: &ComponentManager) {
        println!("Apply for Render");
        for e in self.entities.iter() {
            println!(" e: {}", e);
        }
    }
}

pub struct Transform {
    entities: HashSet<Entity>
}

impl Transform {
    pub fn new() -> Transform {
        Transform { entities: HashSet::new() }
    }
}

impl System for Transform {
    fn add(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    fn apply(&self, cm: &ComponentManager) {
        println!("Apply for Transform");
        for e in self.entities.iter() {
            println!(" e: {}", e);
            println!("   component1: {}", e);
        }
    }
}
