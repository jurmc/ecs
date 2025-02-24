use crate::ecs::MAX_ENTITIES;
use crate::ecs::Entity;
use crate::ecs::ComponentType;

use std::collections::HashMap;
use std::collections::HashSet;
use std::any::TypeId;
use std::any::Any;

pub struct EntitiesPool {
    available_entities: HashSet<Entity>,
    used_entities: HashSet<Entity>,
    components: HashMap<Entity, HashSet<ComponentType>>, // TODO: this rathre should move to
}

impl EntitiesPool {
    pub fn new() -> EntitiesPool {
        let mut available_entities = HashSet::with_capacity(MAX_ENTITIES as usize);
        for e in (0..MAX_ENTITIES).rev() {
            available_entities.insert(e);
        }
        let used_entities: HashSet<Entity> = HashSet::with_capacity(MAX_ENTITIES as usize);

        let components: HashMap<Entity, HashSet<ComponentType>> = HashMap::new();
        EntitiesPool { available_entities, used_entities, components }
    }

    pub fn get(&mut self) -> Entity {
        let e = self.available_entities.iter().next().unwrap().clone();
        self.available_entities.remove(&e);
        self.used_entities.insert(e);
        self.components.insert(e, HashSet::new());
        e
    }

    pub fn give_back(&mut self, e: Entity) {
        self.available_entities.insert(e);
        self.used_entities.remove(&e);
        self.components.remove(&e);
    }

    // TODO: move this to ComponentManager
    pub fn add_component_type<T: Any>(&mut self, e: Entity) {
        let type_id = TypeId::of::<T>();
        let current_types = self.components.get_mut(&e);
        match current_types {
            Some(current_types) => current_types.insert(type_id),
            None => panic!("ERROR: component to illegal entity"),
        };
    }

    // TODO: move this to ComponentManager
    pub fn get_component_types(&self, e: Entity) -> Option<&HashSet<TypeId>> {
        self.components.get(&e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool() {
        let mut ep = EntitiesPool::new();
        for _i in 0..MAX_ENTITIES-1 {
            ep.get();
        }

        let last_e = ep.get();
        ep.give_back(last_e);
        assert_eq!(last_e, ep.get());
    }

    #[test]
    fn test_entities_to_components_associations() {
        let mut pool = EntitiesPool::new();

        let e1 = pool.get();
        let c_type1 = TypeId::of::<u32>();
        let c_type2 = TypeId::of::<i32>();
        pool.add_component_type::<u32>(e1);
        pool.add_component_type::<i32>(e1);

        let e2 = pool.get();
        pool.add_component_type::<u32>(e2);

        let types1 = pool.get_component_types(e1);
        let types2 = pool.get_component_types(e2);

        let expected_types1: HashSet<TypeId> = HashSet::from_iter(vec![c_type1, c_type2]);
        let expected_types2: HashSet<TypeId> = HashSet::from_iter(vec![c_type1]);

        assert_eq!(Some(&expected_types1), types1);
        assert_eq!(Some(&expected_types2), types2);

        pool.give_back(e1);
        let types1 = pool.get_component_types(e1);
        let types2 = pool.get_component_types(e2);
        assert_eq!(None, types1);
        assert_eq!(Some(&expected_types2), types2);
    }

}
