use crate::MAX_ENTITIES;
use crate::Entity;

use std::collections::HashSet;

pub struct EntitiesPool {
    available_entities: HashSet<Entity>,
    used_entities: HashSet<Entity>,
}

impl EntitiesPool {
    pub fn new() -> EntitiesPool {
        let mut available_entities = HashSet::with_capacity(MAX_ENTITIES as usize);
        for e in (0..MAX_ENTITIES).rev() {
            available_entities.insert(e);
        }
        let used_entities: HashSet<Entity> = HashSet::with_capacity(MAX_ENTITIES as usize);

        EntitiesPool { available_entities, used_entities }
    }

    pub fn get(&mut self) -> Entity { // TODO: this should rather be called 'take'
        let e = self.available_entities.iter().next().unwrap().clone();
        self.available_entities.remove(&e);
        self.used_entities.insert(e);
        e
    }

    pub fn give_back(&mut self, e: Entity) {
        self.available_entities.insert(e);
        self.used_entities.remove(&e);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool() {
        let mut pool = EntitiesPool::new();
        for _i in 0..MAX_ENTITIES-1 {
            pool.get();
        }

        let last_e = pool.get();
        pool.give_back(last_e);
        assert_eq!(last_e, pool.get());
    }
}
