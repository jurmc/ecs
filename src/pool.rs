use crate::MAX_ENTITIES;
use crate::Entity;

use std::collections::HashSet;
use std::collections::hash_set::Iter;

pub struct EntitiesPool {
    available: HashSet<Entity>,
    taken: HashSet<Entity>,
}

impl EntitiesPool {
    pub fn new() -> EntitiesPool {
        let mut available= HashSet::with_capacity(MAX_ENTITIES as usize);
        for e in (0..MAX_ENTITIES).rev() {
            available.insert(e);
        }
        let taken: HashSet<Entity> = HashSet::with_capacity(MAX_ENTITIES as usize);

        EntitiesPool { available, taken }
    }

    // TODO: will become: take
    pub fn get(&mut self) -> Entity { // TODO: more like take, since we take something from pool
                                      // and only can take it back with separate call to giveback
        let e = self.available.iter().next().unwrap().clone();
        self.available.remove(&e);
        self.taken.insert(e);
        e
    }

    // TODO: maybe renamed to: return
    pub fn give_back(&mut self, e: Entity) {
        self.available.insert(e);
        self.taken.remove(&e);
    }

    pub fn taken_iter(&self) -> Iter<'_, Entity> {
        self.taken.iter()
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

    #[test]
    fn test_pool_iter() {
        let mut ep = EntitiesPool::new();
        let mut taken: HashSet<Entity> = HashSet::new();
        for _ in 0..10 {
            taken.insert(ep.get());
        }

        let mut expected: HashSet<Entity> = HashSet::new();
        for e in ep.taken_iter() {
            expected.insert(e.clone());
        }

        assert_eq!(expected, taken);
    }
}
