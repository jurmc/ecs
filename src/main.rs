use std::collections::HashSet;

type SignatureT = u32;

struct Vec2 {
    x: f64,
    y: f64,
}

struct ComponentManager {
}

impl ComponentManager {
    fn get_position(&self, _s: SignatureT) ->Vec2 {
        Vec2 {x: 555.0, y: 666.0}
    }
}

struct System {
    entities: HashSet<SignatureT>,
    //component_manager:

}

impl System {
    fn new() -> System {
        System {entities: HashSet::new()}
    }

    fn add(&mut self, entity: SignatureT) {
        self.entities.insert(entity);
    }

    fn apply(&self, cm: &ComponentManager) {
        for i in self.entities.iter() {
            let pos = cm.get_position(*i); // TODO: will be get_component, once we have parametrized
                                           // component type
            println!("i: {}, pos.x: {}, pos.y: {}", i, pos.x, pos.y);
        }
    }
}

fn main() {
    println!("Hello, world! System creation.");

    let cm = ComponentManager {};
    let mut system = System::new();
    system.add(1u32);
    system.add(15u32);

    system.apply(&cm);
}
