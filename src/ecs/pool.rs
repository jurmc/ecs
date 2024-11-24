use crate::ecs::MAX_ENTITIES;
use crate::ecs::Entity;
use crate::ecs::ComponentType;

use std::collections::HashMap;
use std::collections::HashSet;

pub struct EntitiesPool {
    available_entities: HashSet<Entity>,
    used_entities: HashSet<Entity>,
    components: HashMap<Entity, HashSet<ComponentType>>,
}

impl EntitiesPool {
    pub fn new() -> EntitiesPool {
        let mut available_entities = HashSet::with_capacity(MAX_ENTITIES as usize);
        for entity in (0..MAX_ENTITIES).rev() {
            available_entities.insert(entity as Entity);
        }
        let used_entities: HashSet<Entity> = HashSet::with_capacity(MAX_ENTITIES as usize);

        let components: HashMap<Entity, HashSet<ComponentType>> = HashMap::new();
        EntitiesPool { available_entities, used_entities, components }
    }

    pub fn get(&mut self) -> Entity {
        let entity = self.available_entities.iter().next().unwrap().clone();
        self.available_entities.remove(&entity);
        self.used_entities.insert(entity);
        entity
    }

    pub fn give_back(&mut self, entity: Entity) {
        self.available_entities.insert(entity);
        self.used_entities.remove(&entity);
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
}
