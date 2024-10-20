use std::collections::HashSet;
use std::collections::HashMap;
use std::vec::Vec;
use std::fmt::Display;
use std::any::TypeId;
use std::any::Any;

const MAX_ENTITIES: u32 = 10;
type Entity = u32;

struct EntitiesPool {
    available_entities: HashSet<Entity>,
    used_entities: HashSet<Entity>,
    signatures: HashMap<Entity, HashSet<u32>>,
}

impl EntitiesPool {
    fn new() -> EntitiesPool {
        let mut available_entities = HashSet::with_capacity(MAX_ENTITIES as usize);
        for entity in (0..MAX_ENTITIES).rev() {
            available_entities.insert(entity as Entity);
        }
        let used_entities: HashSet<Entity> = HashSet::with_capacity(MAX_ENTITIES as usize);

        let signatures: HashMap<Entity, HashSet<u32>> = HashMap::new();
        EntitiesPool { available_entities, used_entities, signatures }
    }

    fn get(&mut self) -> Entity {
        let entity = self.available_entities.iter().next().unwrap().clone();
        self.available_entities.remove(&entity);
        self.used_entities.insert(entity);
        entity
    }

    fn give_back(&mut self, entity: Entity) {
        self.available_entities.insert(entity);
        self.used_entities.remove(&entity);
    }

    // TODO: check adding (and modifying) singatures of Entities
    //                                                             this u32 is id for components,
    //                                                             TODO: use some type aliasing
    fn set_signature(&mut self, entity: Entity, signature: HashSet<u32>) {
        // TODO: add check if entity is alredy taken (it doesn't exists in available_entities)
        self.signatures.insert(entity, signature).unwrap();
    }
}

struct ComponentArray<T: Display> {
    components: Vec<T>,
}

impl<T:  Display> ComponentArray<T> {
    fn new() -> ComponentArray<T> {
        ComponentArray { components: Vec::new(), }
    }
    fn add(&mut self, component: T) {
        self.components.push(component);
    }

    fn dump(&mut self) {
        for c in self.components.iter() {
            println!("c: {}", c);
        }
    }
}

struct ComponentManager {
    components: HashSet<TypeId>,
}

impl ComponentManager {
    fn new() -> ComponentManager {
        ComponentManager { components: HashSet::new(), }
    }

    fn register<T: Display + Any>(&mut self, component_array: ComponentArray<T>) {
        self.components.insert(TypeId::of::<T>());
    }

    fn dump(&self) {
        for c in self.components.iter() {
            println!("c: {:?}", c);
        }
    }
}

trait System {
    fn apply(&self);
}

struct Render {
}

impl System for Render {
    fn apply(&self) {
        println!("Apply for Render");
    }
}

struct Transform {
}

impl System for Transform {
    fn apply(&self) {
        println!("Apply for Transform");
    }
}

fn main() {
    println!("Test component arrays ---------------------------------");
    let mut comp_arr1 = ComponentArray::new();
    comp_arr1.add(1);
    comp_arr1.add(2);
    let mut comp_arr2 = ComponentArray::new();
    comp_arr2.add(1.5);
    comp_arr2.add(2.5);

    comp_arr1.dump();
    comp_arr2.dump();

    let mut cm = ComponentManager::new();
    cm.register(comp_arr1);
    cm.register(comp_arr2);
    cm.dump();
    println!("-------------------------------------------------------");


    println!("Test entity pool"); ////////////////////////////////////
    let mut pool = EntitiesPool::new();
    let entity1 = pool.get();
    println!("Got entity: {}", entity1);

    let entity2 = pool.get();
    println!("Got entity: {}", entity2);

    pool.give_back(entity1);
    pool.give_back(entity1);

    let entity3 = pool.get();
    println!("Got entity: {}", entity3);

    // System
    let r = Render{};
    let t = Transform{};
    let systems: Vec<Box<dyn System>> = vec![Box::new(Render{}), Box::new(Transform{})] ;

    for system in systems {
        system.apply()
    }

}


