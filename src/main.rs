use std::collections::HashSet;
use std::fmt::Display;
use std::any::TypeId;
use std::any::Any;

const MAX_ENTITIES: u32 = 10;

struct EntitiesPool {
    entities: HashSet<u32>,
}

impl EntitiesPool {
    fn new() -> EntitiesPool {
        let mut entities = HashSet::with_capacity(MAX_ENTITIES as usize);
        for entity_id in (0..MAX_ENTITIES).rev() {
            entities.insert(entity_id);
        }

        EntitiesPool { entities }
    }

    fn get(&mut self) -> u32 {
        let elem = self.entities.iter().next().unwrap().clone();
        self.entities.remove(&elem);
        elem
    }

    fn give_back(&mut self, entity_id: u32) {
        self.entities.insert(entity_id);
    }
}

struct ComponentArray<T: Display> {
    components: Vec<T>,
}

impl<T: Display> ComponentArray<T> {
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

fn main() {

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

    println!("Test component arrays"); ////////////////////////////////////
    let mut comp_arr1 = ComponentArray::new();
    comp_arr1.add(1);
    comp_arr1.add(2);
    let mut comp_arr2 = ComponentArray::new();
    comp_arr2.add(1.5);
    comp_arr2.add(2.5);

    comp_arr1.dump();
    comp_arr2.dump();

    // Component manager
    let mut cm = ComponentManager::new();
    cm.register(comp_arr1);
    cm.register(comp_arr2);
    cm.dump();
}


