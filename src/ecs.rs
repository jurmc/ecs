use std::any::TypeId;

const MAX_ENTITIES: u32 = 100;
type Entity = u32;
type ComponentType = TypeId; // TODO: start using it
type SystemType = TypeId;    // TODO: start using it

pub use coordinator::Coordinator;

pub use pool::EntitiesPool;

pub use component::ComponentArray;
pub use component::ComponentManager;

pub use system::System;
pub use system::SystemManager;

pub use system::Render;
pub use system::Transform;

mod coordinator;
mod pool;
mod component;
mod system;
