pub use quartz::{
    Action, AnimatedSprite, Anchor, Canvas, CanvasMode, Condition,
    Context, GameEvent, GameObject, GameObjectBuilder, Image, Key,
    Location, ShapeType, Target, SoundOptions, SoundHandle,
};
mod scene;
mod layer;
mod world_object;
mod camera;
mod shadow;
mod world_event;

pub use scene::Scene;
pub use layer::{Layer, LayerId};
pub use world_object::WorldObject;
pub use camera::{Camera, CameraBounds};
pub use shadow::Shadow;
pub use world_event::WorldEvent;

pub mod prelude {
    pub use crate::scene::Scene;
    pub use crate::layer::{Layer, LayerId};
    pub use crate::world_object::WorldObject;
    pub use crate::camera::{Camera, CameraBounds};
    pub use crate::shadow::Shadow;
    pub use crate::world_event::WorldEvent;

    pub use quartz::{
        Action, AnimatedSprite, Anchor, Canvas, CanvasMode, Condition,
        Context, GameEvent, GameObject, GameObjectBuilder, Image, Key,
        Location, ShapeType, Target, SoundOptions, SoundHandle,
    };
}