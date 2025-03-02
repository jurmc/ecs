use std::any::TypeId;

const MAX_ENTITIES: u32 = 100;
pub type Entity = u32;
pub type ComponentType = TypeId; // TODO: start using it
pub type SystemType = TypeId;    // TODO: start using it

pub mod coordinator;
pub use coordinator::Coordinator;

pub mod pool;
pub use pool::EntitiesPool;

pub mod component;
pub use component::ComponentArray;
pub use component::ComponentManager;

pub mod system;
pub use system::System;
pub use system::SystemManager;

pub use system::Render;
pub use system::Transform;

