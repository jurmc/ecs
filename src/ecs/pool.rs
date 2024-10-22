use std::collections::HashSet;
use crate::ecs::MAX_ENTITIES;
use std::collections::HashMap;
use crate::ecs::Entity;
use crate::ecs::ComponentType;

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

    pub fn set_components(&mut self, entity: Entity, components: HashSet<ComponentType>) {
        // TODO: add check if entity is alredy taken (it doesn't exists in available_entities)
        if self.used_entities.contains(&entity) {
            self.components.insert(entity, components).unwrap();
        }
    }
    // TODO: add functions for adding and removing single components for entity,
    //       so this will be modification ofr self.components, instead of setting it from
    //       scratch as this is already done for set_components
}

