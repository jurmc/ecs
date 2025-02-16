use std::any::TypeId;

const MAX_ENTITIES: u32 = 100;
type Entity = u32;
type ComponentType = TypeId;

pub use pool::EntitiesPool;

pub use component::ComponentArray;
pub use component::ComponentManager;

pub use system::System;
pub use system::SystemManager;

pub use system::Render;
pub use system::Transform;

mod pool;
mod component;
mod system;

