use quartz::{Action, Key, Target};
use crate::LayerId;

/// High-level world events that Scene processes each tick.
/// KeyPress / KeyRelease are fired by Scene when a canvas reports a key
/// change via its on_key_press / on_key_release callbacks.
#[derive(Clone, Debug)]
pub enum WorldEvent {
    /// Fired once when a key transitions from up → down.
    KeyPress {
        key: Key,
        action: Action,
        target: Target,
    },

    /// Fired once when a key transitions from down → up.
    KeyRelease {
        key: Key,
        action: Action,
        target: Target,
    },

    LayerCollision {
        action: Action,
        target: Target,
        layer: LayerId,
    },

    LayerEnter {
        action: Action,
        target: Target,
        layer: LayerId,
    },

    LayerExit {
        action: Action,
        target: Target,
        layer: LayerId,
    },

    CameraEnter {
        action: Action,
        target: Target,
    },

    CameraExit {
        action: Action,
        target: Target,
    },

    BoundaryCollision {
        action: Action,
        target: Target,
        layer: LayerId,
    },

    GlobalCollision {
        action: Action,
        target: Target,
        with_target: Target,
    },
}

impl WorldEvent {
    pub fn get_target(&self) -> &Target {
        match self {
            WorldEvent::KeyPress    { target, .. } => target,
            WorldEvent::KeyRelease  { target, .. } => target,
            WorldEvent::LayerCollision { target, .. } => target,
            WorldEvent::LayerEnter  { target, .. } => target,
            WorldEvent::LayerExit   { target, .. } => target,
            WorldEvent::CameraEnter { target, .. } => target,
            WorldEvent::CameraExit  { target, .. } => target,
            WorldEvent::BoundaryCollision { target, .. } => target,
            WorldEvent::GlobalCollision   { target, .. } => target,
        }
    }

    pub fn get_action(&self) -> &Action {
        match self {
            WorldEvent::KeyPress    { action, .. } => action,
            WorldEvent::KeyRelease  { action, .. } => action,
            WorldEvent::LayerCollision { action, .. } => action,
            WorldEvent::LayerEnter  { action, .. } => action,
            WorldEvent::LayerExit   { action, .. } => action,
            WorldEvent::CameraEnter { action, .. } => action,
            WorldEvent::CameraExit  { action, .. } => action,
            WorldEvent::BoundaryCollision { action, .. } => action,
            WorldEvent::GlobalCollision   { action, .. } => action,
        }
    }

    pub fn get_layer(&self) -> Option<LayerId> {
        match self {
            WorldEvent::LayerCollision    { layer, .. } => Some(*layer),
            WorldEvent::LayerEnter        { layer, .. } => Some(*layer),
            WorldEvent::LayerExit         { layer, .. } => Some(*layer),
            WorldEvent::BoundaryCollision { layer, .. } => Some(*layer),
            _ => None,
        }
    }

    /// Returns the key associated with this event, if any.
    pub fn get_key(&self) -> Option<&Key> {
        match self {
            WorldEvent::KeyPress   { key, .. } => Some(key),
            WorldEvent::KeyRelease { key, .. } => Some(key),
            _ => None,
        }
    }

    pub fn is_key_press(&self) -> bool {
        matches!(self, WorldEvent::KeyPress { .. })
    }

    pub fn is_key_release(&self) -> bool {
        matches!(self, WorldEvent::KeyRelease { .. })
    }
}