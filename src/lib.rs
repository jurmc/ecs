use std::any::TypeId;

const MAX_ENTITIES: u32 = 100;
pub type Entity = u32;
pub type ComponentType = TypeId;
pub type SystemType = TypeId;

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

pub mod globals;
pub use globals::Globals;
