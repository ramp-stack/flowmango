use quartz::{GameObject, Image, Context, AnimatedSprite};
use crate::{LayerId, Shadow};
use prism::layout::Stack;
use prism::event::OnEvent;
use prism::drawable::Component;

#[derive(Debug, Clone, Component)]
pub struct WorldObject {
    layout: Stack,
    pub(crate) game_object: GameObject,
    #[skip] z_position: f32,
    #[skip] layer_id: LayerId,
    #[skip] base_scale: (f32, f32),
    #[skip] depth_scale: bool,
    #[skip] shadow: Option<Shadow>,
}

impl OnEvent for WorldObject {}

impl From<GameObject> for WorldObject {
    fn from(go: GameObject) -> Self {
        let layer_id = LayerId(go.layer as usize);
        let size     = go.size;
        Self {
            layout:      Stack::default(),
            layer_id,
            base_scale:  size,
            z_position:  0.0,
            depth_scale: false,
            shadow:      None,
            game_object: go,
        }
    }
}

impl WorldObject {
    pub fn new(
        ctx: &mut Context,
        id: String,
        image: Option<Image>,
        size: f32,
        position: (f32, f32, f32),
        layer_id: LayerId,
        tags: Vec<String>,
        momentum: (f32, f32, f32),
        resistance: (f32, f32, f32),
        gravity: f32,
    ) -> Self {
        let game_object = GameObject::new(
            ctx, id, image, size,
            (position.0, position.1),
            tags,
            (momentum.0, momentum.1),
            (resistance.0, resistance.1),
            gravity,
        );
        Self {
            layout: Stack::default(),
            game_object,
            z_position:  position.2.clamp(0.0, 1.0),
            layer_id,
            base_scale:  (size, size),
            depth_scale: false,
            shadow:      None,
        }
    }

    pub fn new_rect(
        ctx: &mut Context,
        id: String,
        image: Option<Image>,
        size: (f32, f32),
        position: (f32, f32, f32),
        layer_id: LayerId,
        tags: Vec<String>,
        momentum: (f32, f32, f32),
        resistance: (f32, f32, f32),
        gravity: f32,
    ) -> Self {
        let game_object = GameObject::new_rect(
            ctx, id, image, size,
            (position.0, position.1),
            tags,
            (momentum.0, momentum.1),
            (resistance.0, resistance.1),
            gravity,
        );
        Self {
            layout: Stack::default(),
            game_object,
            z_position:  position.2.clamp(0.0, 1.0),
            layer_id,
            base_scale:  size,
            depth_scale: false,
            shadow:      None,
        }
    }

    pub fn with_depth_scaling(mut self, enabled: bool) -> Self {
        self.depth_scale = enabled;
        self
    }

    pub fn with_shadow(mut self, shadow: Shadow) -> Self {
        self.shadow = Some(shadow);
        self
    }

    pub fn with_animation(mut self, animation: AnimatedSprite) -> Self {
        self.game_object = self.game_object.with_animation(animation);
        self
    }

    pub fn as_platform(mut self) -> Self {
        self.game_object = self.game_object.as_platform();
        self
    }

    pub fn set_z_position(&mut self, z: f32) {
        self.z_position = z.clamp(0.0, 1.0);
    }

    pub fn get_z_position(&self) -> f32 {
        self.z_position
    }

    pub fn get_world_position(&self) -> (f32, f32, f32) {
        let pos = self.game_object.position;
        (pos.0, pos.1, self.z_position)
    }

    pub fn set_world_position(&mut self, position: (f32, f32, f32)) {
        self.game_object.position = (position.0, position.1);
        self.z_position = position.2.clamp(0.0, 1.0);
    }

    pub fn get_layer_id(&self) -> LayerId { self.layer_id }
    pub fn set_layer_id(&mut self, layer_id: LayerId) { self.layer_id = layer_id; }
    pub fn game_object(&self) -> &GameObject { &self.game_object }
    pub fn game_object_mut(&mut self) -> &mut GameObject { &mut self.game_object }
    pub fn get_shadow(&self) -> Option<&Shadow> { self.shadow.as_ref() }
    pub fn set_shadow(&mut self, shadow: Option<Shadow>) { self.shadow = shadow; }

    pub fn calculate_scale(&self) -> (f32, f32) {
        if self.depth_scale {
            let depth_factor = 0.5 + (self.z_position * 0.5);
            (self.base_scale.0 * depth_factor, self.base_scale.1 * depth_factor)
        } else {
            self.base_scale
        }
    }

    pub fn get_id(&self) -> &str { &self.game_object.id }
    pub fn get_tags(&self) -> &[String] { &self.game_object.tags }
    pub fn has_tag(&self, tag: &str) -> bool { self.game_object.tags.iter().any(|t| t == tag) }
    pub fn is_visible(&self) -> bool { self.game_object.visible }
    pub fn set_visible(&mut self, visible: bool) { self.game_object.visible = visible; }
    pub fn get_base_scale(&self) -> (f32, f32) { self.base_scale }
    pub fn has_depth_scaling(&self) -> bool { self.depth_scale }
}