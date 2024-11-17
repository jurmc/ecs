use crate::ecs::Entity;

use std::collections::HashSet;

pub trait System {
    fn add_entity(&mut self, entity: Entity);
    //fn remove_entity(entity: Entity);

    fn apply(&self);
}

pub struct Render {
    entities: HashSet<Entity>
}

impl Render {
    pub fn new() -> Render {
        Render { entities: HashSet::new() }
    }
}

impl System for Render {
    fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    fn apply(&self) {
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
    fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    fn apply(&self) {
        println!("Apply for Transform");
        for e in self.entities.iter() {
            println!(" e: {}", e);    
        }
    }
}