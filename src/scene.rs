use quartz::{CanvasMode, Context, Action, Target, Canvas, Key};
use prism::drawable::{Component, Drawable, SizedTree};
use prism::layout::{Layout, Stack};
use prism::event::{OnEvent, Event};
use crate::{Layer, LayerId, WorldObject, Camera, WorldEvent};
use std::sync::{Arc, Mutex};

pub struct Scene {
    layout: Stack,
    world_objects: std::collections::HashMap<String, WorldObject>,
    layers: Vec<Layer>,
    camera: Camera,
    mode: CanvasMode,
    events: Vec<WorldEvent>,
    key_events: Vec<WorldEvent>,
    tick_callbacks: Vec<Box<dyn SceneCallback>>,
    held_keys: Arc<Mutex<std::collections::HashSet<Key>>>,
    // Stores the actual window size for Fullscreen mode, updated each layout pass
    fullscreen_size: Arc<Mutex<(f32, f32)>>,
}

impl Clone for Scene {
    fn clone(&self) -> Self {
        Self {
            layout:          self.layout.clone(),
            world_objects:   self.world_objects.clone(),
            layers:          self.layers.clone(),
            camera:          self.camera.clone(),
            mode:            self.mode,
            events:          self.events.clone(),
            key_events:      self.key_events.clone(),
            tick_callbacks:  self.tick_callbacks.clone(),
            held_keys:       self.held_keys.clone(),
            fullscreen_size: self.fullscreen_size.clone(),
        }
    }
}

impl std::fmt::Debug for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scene")
            .field("layers",        &self.layers)
            .field("camera",        &self.camera)
            .field("world_objects", &self.world_objects)
            .field("mode",          &self.mode)
            .finish()
    }
}

impl Component for Scene {
    fn children(&self) -> Vec<&dyn Drawable> {
        self.layers
            .iter()
            .filter(|l| l.is_visible())
            .map(|l| l.canvas() as &dyn Drawable)
            .collect()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn Drawable> {
        self.layers
            .iter_mut()
            .filter(|l| l.is_visible())
            .map(|l| l.canvas_mut() as &mut dyn Drawable)
            .collect()
    }

    fn layout(&self) -> &dyn Layout {
        &self.layout
    }
}

impl OnEvent for Scene {
    fn on_event(
        &mut self,
        _ctx: &mut Context,
        _tree: &SizedTree,
        event: Box<dyn Event>,
    ) -> Vec<Box<dyn Event>> {
       
        if self.mode == CanvasMode::Fullscreen {
            if let Some(layer) = self.layers.iter().find(|l| l.is_visible()) {
                let size = layer.canvas().canvas_size();
                if size.0 > 0.0 && size.1 > 0.0 {
                    *self.fullscreen_size.lock().unwrap() = size;
                }
            }
        }

        vec![event]
    }
}

impl Scene {
    pub fn new(ctx: &mut Context, mode: CanvasMode, num_layers: usize) -> Self {
        let mut layers = Vec::with_capacity(num_layers);
        for i in 0..num_layers {
            let depth    = i as f32 / (num_layers - 1).max(1) as f32;
            let parallax = if depth < 0.5 { depth } else { 1.0 + (depth - 0.5) * 0.5 };
            layers.push(Layer::new(ctx, mode, depth, parallax, LayerId(i)));
        }

        let held_keys: Arc<Mutex<std::collections::HashSet<Key>>> =
            Arc::new(Mutex::new(std::collections::HashSet::new()));

        for layer in &mut layers {
            let press_keys = held_keys.clone();
            layer.canvas_mut().on_key_press(move |_canvas, key| {
                press_keys.lock().unwrap().insert(key.clone());
            });

            let release_keys = held_keys.clone();
            layer.canvas_mut().on_key_release(move |_canvas, key| {
                release_keys.lock().unwrap().remove(key);
            });
        }

        Self {
            layout:          Stack::default(),
            layers,
            camera:          Camera::default(),
            world_objects:   std::collections::HashMap::new(),
            mode,
            events:          Vec::new(),
            key_events:      Vec::new(),
            tick_callbacks:  Vec::new(),
            held_keys,
            fullscreen_size: Arc::new(Mutex::new((0.0, 0.0))),
        }
    }

    pub fn add_object(&mut self, object: impl Into<WorldObject>) {
        let object   = object.into();
        let id       = object.get_id().to_string();
        let layer_id = object.get_layer_id();

        if let Some(layer) = self.get_layer_mut(layer_id) {
            let game_obj    = object.game_object().clone();
            let canvas_name = format!("{}_{}", layer_id.0, id);
            layer.canvas_mut().add_game_object(canvas_name, game_obj);
        }

        self.world_objects.insert(id, object);
    }

    pub fn remove_object(&mut self, id: &str) -> Option<WorldObject> {
        if let Some(object) = self.world_objects.remove(id) {
            let layer_id = object.get_layer_id();
            if let Some(layer) = self.get_layer_mut(layer_id) {
                let canvas_name = format!("{}_{}", layer_id.0, id);
                layer.canvas_mut().remove_game_object(&canvas_name);
            }
            Some(object)
        } else {
            None
        }
    }

    pub fn get_object(&self, id: &str) -> Option<&WorldObject> {
        self.world_objects.get(id)
    }

    pub fn get_object_mut(&mut self, id: &str) -> Option<&mut WorldObject> {
        self.world_objects.get_mut(id)
    }

    pub fn get_objects_by_tag(&self, tag: &str) -> Vec<&WorldObject> {
        self.world_objects.values().filter(|o| o.has_tag(tag)).collect()
    }

    pub fn get_objects_by_tag_mut(&mut self, tag: &str) -> Vec<&mut WorldObject> {
        self.world_objects.values_mut().filter(|o| o.has_tag(tag)).collect()
    }

    pub fn get_layer(&self, id: LayerId) -> Option<&Layer> {
        self.layers.get(id.0)
    }

    pub fn get_layer_mut(&mut self, id: LayerId) -> Option<&mut Layer> {
        self.layers.get_mut(id.0)
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    pub fn add_layer(&mut self, ctx: &mut Context, depth: f32, parallax_factor: f32) -> LayerId {
        let id    = LayerId(self.layers.len());
        let layer = Layer::new(ctx, self.mode, depth, parallax_factor, id);
        self.layers.push(layer);
        id
    }

    pub fn camera(&self)         -> &Camera     { &self.camera }
    pub fn camera_mut(&mut self) -> &mut Camera { &mut self.camera }

    pub fn set_camera_follow(&mut self, target_id: String, smoothing: f32) {
        self.camera.follow(target_id, smoothing);
    }

    pub fn is_key_held(&self, key: &Key) -> bool {
        self.held_keys.lock().unwrap().contains(key)
    }

    pub fn add_event(&mut self, event: WorldEvent) {
        if event.is_key_press() || event.is_key_release() {
            self.key_events.push(event);
        } else {
            self.events.push(event);
        }
    }

    pub fn run(&mut self, action: Action) {
        for layer in &mut self.layers {
            layer.canvas_mut().run(action.clone());
        }
    }

    pub fn on_update<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Scene) + Clone + 'static,
    {
        self.tick_callbacks.push(Box::new(callback));
    }

    pub fn update(&mut self, _ctx: &mut Context) {
        self.sync_world_to_canvas();

        let mut callbacks = std::mem::take(&mut self.tick_callbacks);
        for cb in &mut callbacks { cb(self); }
        self.tick_callbacks = callbacks;

        self.process_key_events();
        self.process_events();
        self.sync_canvas_to_world();

        let follow_pos = self
            .camera
            .get_follow_target()
            .and_then(|id| self.world_objects.get(id))
            .map(|obj| obj.get_world_position());
        if let Some(pos) = follow_pos {
            self.camera.update_follow(pos);
        }

        self.camera.update_shake(1.0 / 60.0);

        let offsets: Vec<(f32, f32)> = self
            .layers
            .iter()
            .map(|l| self.camera.calculate_layer_offset(l.parallax_factor))
            .collect();

        for (layer, offset) in self.layers.iter_mut().zip(offsets) {
            layer.set_camera_offset(offset);
        }
    }

    fn sync_world_to_canvas(&mut self) {
        for (id, world_obj) in &self.world_objects {
            let layer_id    = world_obj.get_layer_id();
            let canvas_name = format!("{}_{}", layer_id.0, id);
            let src         = world_obj.game_object();

            if let Some(layer) = self.layers.get_mut(layer_id.0) {
                if let Some(go) = layer.canvas_mut().get_game_object_mut(&canvas_name) {
                    go.position = src.position;
                    go.visible  = src.visible;
                    go.momentum = src.momentum;
                    go.gravity  = src.gravity;

                    if let Some(sprite) = &src.animated_sprite {
                        go.set_animation(sprite.clone());
                    }
                }
            }
        }
    }

    fn sync_canvas_to_world(&mut self) {
        let ids_and_layers: Vec<(String, LayerId)> = self
            .world_objects
            .iter()
            .map(|(id, obj)| (id.clone(), obj.get_layer_id()))
            .collect();

        for (id, layer_id) in ids_and_layers {
            let canvas_name = format!("{}_{}", layer_id.0, id);

            let canvas_state = self
                .layers
                .get(layer_id.0)
                .and_then(|l| l.canvas().get_game_object(&canvas_name))
                .map(|go| (go.position, go.momentum));

            if let Some((pos, momentum)) = canvas_state {
                if let Some(world_obj) = self.world_objects.get_mut(&id) {
                    let z = world_obj.get_z_position();
                    world_obj.set_world_position((pos.0, pos.1, z));
                    world_obj.game_object_mut().momentum = momentum;
                }
            }
        }
    }

    fn process_key_events(&mut self) {
        let held = self.held_keys.lock().unwrap().clone();

        let actions: Vec<Action> = self
            .key_events
            .iter()
            .filter_map(|event| match event {
                WorldEvent::KeyPress { key, action, .. } if held.contains(key) => {
                    Some(action.clone())
                }
                _ => None,
            })
            .collect();

        for action in actions { self.run(action); }
    }

    fn process_events(&mut self) {
        let events = std::mem::take(&mut self.events);
        let mut remaining: Vec<WorldEvent> = Vec::new();

        for event in &events {
            match event {
                WorldEvent::LayerCollision { action, layer, target } => {
                    if self.check_layer_collision_for_target(target, *layer) {
                        self.run(action.clone());
                    } else {
                        remaining.push(event.clone());
                    }
                }
                WorldEvent::GlobalCollision { action, target, with_target } => {
                    let collides = (0..self.layers.len())
                        .any(|i| self.collision_on_layer(target, with_target, LayerId(i)));
                    if collides {
                        self.run(action.clone());
                    } else {
                        remaining.push(event.clone());
                    }
                }
                WorldEvent::BoundaryCollision { action, layer, target } => {
                    let virtual_size = self.get_virtual_size();
                    let layer_id     = *layer;
                    let obj_ids      = self.resolve_target_ids(target, Some(layer_id));
                    let hit = obj_ids.iter().any(|id| {
                        self.world_objects.get(id).map_or(false, |obj| {
                            let pos  = obj.game_object().position;
                            let size = obj.calculate_scale();
                            pos.0 <= 0.0
                                || pos.0 + size.0 >= virtual_size.0
                                || pos.1 <= 0.0
                                || pos.1 + size.1 >= virtual_size.1
                        })
                    });
                    if hit { self.run(action.clone()); } else { remaining.push(event.clone()); }
                }
                WorldEvent::CameraEnter { action, target } => {
                    let viewport = self.get_virtual_size();
                    let obj_ids  = self.resolve_target_ids(target, None);
                    let entered  = obj_ids.iter().any(|id| {
                        self.world_objects.get(id).map_or(false, |obj| {
                            self.camera.is_visible(obj.game_object().position, obj.calculate_scale(), viewport)
                        })
                    });
                    if entered { self.run(action.clone()); } else { remaining.push(event.clone()); }
                }
                WorldEvent::CameraExit { action, target } => {
                    let viewport = self.get_virtual_size();
                    let obj_ids  = self.resolve_target_ids(target, None);
                    let exited   = obj_ids.iter().any(|id| {
                        self.world_objects.get(id).map_or(false, |obj| {
                            !self.camera.is_visible(obj.game_object().position, obj.calculate_scale(), viewport)
                        })
                    });
                    if exited { self.run(action.clone()); } else { remaining.push(event.clone()); }
                }
                WorldEvent::LayerEnter { action, target, layer } => {
                    let in_layer = self.resolve_target_ids(target, None).iter().any(|id| {
                        self.world_objects.get(id).map_or(false, |o| o.get_layer_id() == *layer)
                    });
                    if in_layer { self.run(action.clone()); } else { remaining.push(event.clone()); }
                }
                WorldEvent::LayerExit { action, target, layer } => {
                    let left = self.resolve_target_ids(target, None).iter().any(|id| {
                        self.world_objects.get(id).map_or(false, |o| o.get_layer_id() != *layer)
                    });
                    if left { self.run(action.clone()); } else { remaining.push(event.clone()); }
                }
                WorldEvent::KeyPress { .. } | WorldEvent::KeyRelease { .. } => {
                    remaining.push(event.clone());
                }
            }
        }

        self.events = remaining;
    }

    fn check_layer_collision_for_target(&self, target: &Target, layer_id: LayerId) -> bool {
        let target_objs    = self.objects_for_target(target, layer_id);
        let all_layer_objs: Vec<&WorldObject> = self
            .world_objects.values()
            .filter(|o| o.get_layer_id() == layer_id)
            .collect();

        for t in &target_objs {
            for other in &all_layer_objs {
                if !std::ptr::eq(*t, *other) && self.check_collision(t, other) {
                    return true;
                }
            }
        }
        false
    }

    fn resolve_target_ids(&self, target: &Target, layer_filter: Option<LayerId>) -> Vec<String> {
        match target {
            Target::ById(id)     => vec![id.clone()],
            Target::ByName(name) => vec![name.clone()],
            Target::ByTag(tag)   => self
                .world_objects.values()
                .filter(|o| o.has_tag(tag) && layer_filter.map_or(true, |l| o.get_layer_id() == l))
                .map(|o| o.get_id().to_string())
                .collect(),
        }
    }

    pub fn collision_on_layer(&self, t1: &Target, t2: &Target, layer_id: LayerId) -> bool {
        let objs1 = self.objects_for_target(t1, layer_id);
        let objs2 = self.objects_for_target(t2, layer_id);
        for o1 in &objs1 {
            for o2 in &objs2 {
                if !std::ptr::eq(*o1, *o2) && self.check_collision(o1, o2) {
                    return true;
                }
            }
        }
        false
    }

    fn objects_for_target<'a>(&'a self, target: &Target, layer_id: LayerId) -> Vec<&'a WorldObject> {
        match target {
            Target::ById(id)     => self.world_objects.get(id.as_str())
                .filter(|o| o.get_layer_id() == layer_id).into_iter().collect(),
            Target::ByName(name) => self.world_objects.get(name.as_str())
                .filter(|o| o.get_layer_id() == layer_id).into_iter().collect(),
            Target::ByTag(tag)   => self.world_objects.values()
                .filter(|o| o.has_tag(tag) && o.get_layer_id() == layer_id).collect(),
        }
    }

    fn check_collision(&self, a: &WorldObject, b: &WorldObject) -> bool {
        let (p1, s1) = (a.game_object().position, a.calculate_scale());
        let (p2, s2) = (b.game_object().position, b.calculate_scale());
        p1.0 < p2.0 + s2.0
            && p1.0 + s1.0 > p2.0
            && p1.1 < p2.1 + s2.1
            && p1.1 + s1.1 > p2.1
    }

    pub fn get_mode(&self) -> CanvasMode { self.mode }

    /// Returns the virtual size of the scene.
    ///
    /// - `Landscape`  → fixed 3840×2160
    /// - `Portrait`   → fixed 2160×3840
    /// - `Fullscreen` → the actual window size, captured from the layer's
    ///                  `CanvasLayout::canvas_size` cell each frame via
    ///                  `on_event`. Falls back to the layer directly if the
    ///                  cached value has not been populated yet.
    pub fn get_virtual_size(&self) -> (f32, f32) {
        match self.mode {
            CanvasMode::Landscape  => (3840.0, 2160.0),
            CanvasMode::Portrait   => (2160.0, 3840.0),
            CanvasMode::Fullscreen => {
                // Prefer the value kept in sync by on_event
                let cached = *self.fullscreen_size.lock().unwrap();
                if cached.0 > 0.0 && cached.1 > 0.0 {
                    return cached;
                }
                // Fallback: read directly from the first visible layer's canvas layout
                self.layers
                    .iter()
                    .find(|l| l.is_visible())
                    .map(|l| {
                        let size = l.canvas().canvas_size();
                        if size.0 > 0.0 { size } else { (0.0, 0.0) }
                    })
                    .unwrap_or((0.0, 0.0))
            }
        }
    }

    pub fn get_canvases(&self) -> Vec<&Canvas> {
        self.layers.iter().filter(|l| l.is_visible()).map(|l| l.canvas()).collect()
    }

    pub fn get_canvases_mut(&mut self) -> Vec<&mut Canvas> {
        self.layers.iter_mut().filter(|l| l.is_visible()).map(|l| l.canvas_mut()).collect()
    }
}

pub trait SceneCallback: FnMut(&mut Scene) + 'static {
    fn clone_box(&self) -> Box<dyn SceneCallback>;
}

impl PartialEq for dyn SceneCallback {
    fn eq(&self, _: &Self) -> bool { true }
}

impl<F: FnMut(&mut Scene) + Clone + 'static> SceneCallback for F {
    fn clone_box(&self) -> Box<dyn SceneCallback> { Box::new(self.clone()) }
}

impl Clone for Box<dyn SceneCallback> {
    fn clone(&self) -> Self { self.as_ref().clone_box() }
}

impl std::fmt::Debug for dyn SceneCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<SceneCallback>")
    }
}