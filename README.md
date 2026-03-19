# FlowMango 2.5D Game Engine

## Overview

FlowMango is a 2.5D game engine built as a wrapper around Quartz, adding depth layers, perspective, and pseudo-3D effects to create compelling 2.5D gameplay experiences. Think of games like *Paper Mario*, *Donkey Kong Country*, or *Octopath Traveler* - they use 2D rendering with layered depth and perspective tricks.

## Core Concept: 2.5D Architecture

### What is 2.5D?
2.5D (also called pseudo-3D) combines 2D graphics with:
- **Multiple depth layers** (foreground, gameplay, background)
- **Parallax scrolling** (layers move at different speeds)
- **Z-ordering** (objects at different depths)
- **Perspective effects** (scaling, positioning based on depth)
- **Depth-based interactions** (objects on same layer can collide)

### FlowMango's Approach
FlowMango wraps Quartz's 2D Canvas system with:
1. **Multiple Canvas layers** at different depths
2. **A unified coordinate system** with X, Y, and Z axes
3. **Automatic perspective calculations**
4. **Layer management** with parallax support
5. **Depth-aware physics and collision**

## Architecture

```
FlowMango (2.5D World)
  │
  ├── Scene (Container for all layers and objects)
  │     ├── Camera (viewport control, following, zoom)
  │     ├── Layers (multiple Quartz Canvases)
  │     │     ├── Layer 0 (far background) - parallax: 0.1
  │     │     ├── Layer 1 (mid background) - parallax: 0.3
  │     │     ├── Layer 2 (near background) - parallax: 0.6
  │     │     ├── Layer 3 (gameplay layer) - parallax: 1.0
  │     │     ├── Layer 4 (near foreground) - parallax: 1.2
  │     │     └── Layer 5 (far foreground) - parallax: 1.5
  │     └── WorldObjects (enhanced GameObjects with Z-depth)
  │
  ├── WorldObject (GameObject with depth)
  │     ├── Base GameObject (from Quartz)
  │     ├── Z-position (depth value)
  │     ├── Layer assignment
  │     ├── Scale factor (based on depth)
  │     └── Shadow support
  │
  ├── Camera System
  │     ├── Position (X, Y, Z)
  │     ├── Zoom level
  │     ├── Follow target
  │     └── Bounds/limits
  │
  └── Depth Physics
        ├── Layer-specific collision detection
        ├── Depth-based gravity
        └── Inter-layer transitions
```

## Core Components

### 1. Scene

The `Scene` is the main container that replaces direct Canvas usage.

```rust
pub struct Scene {
    layers: Vec<Layer>,
    camera: Camera,
    world_objects: HashMap<String, WorldObject>,
    mode: CanvasMode,
}

impl Scene {
    pub fn new(ctx: &mut Context, mode: CanvasMode, num_layers: usize) -> Self
    pub fn add_layer(&mut self, depth: f32, parallax_factor: f32) -> LayerId
    pub fn add_object(&mut self, object: WorldObject)
    pub fn remove_object(&mut self, id: &str)
    pub fn get_object(&self, id: &str) -> Option<&WorldObject>
    pub fn get_object_mut(&mut self, id: &str) -> Option<&mut WorldObject>
    pub fn update(&mut self, ctx: &mut Context)
    pub fn set_camera_follow(&mut self, target_id: String)
}
```

### 2. Layer

Each `Layer` wraps a Quartz `Canvas` with depth information.

```rust
pub struct Layer {
    canvas: Canvas,
    depth: f32,              // 0.0 = far back, 1.0 = front
    parallax_factor: f32,    // How much this layer moves with camera
    visible: bool,
}

impl Layer {
    pub fn new(ctx: &mut Context, mode: CanvasMode, depth: f32, parallax: f32) -> Self
    pub fn set_parallax(&mut self, factor: f32)
    pub fn set_visible(&mut self, visible: bool)
}
```

### 3. WorldObject

Enhanced GameObject with depth awareness.

```rust
pub struct WorldObject {
    game_object: GameObject,
    z_position: f32,         // Depth in world space (0.0 to 1.0)
    layer_id: LayerId,       // Which layer to render on
    base_scale: f32,         // Original scale
    depth_scale: bool,       // Whether to scale with depth
    shadow: Option<Shadow>,  // Optional shadow configuration
}

impl WorldObject {
    pub fn new(
        ctx: &mut Context,
        id: String,
        image: Image,
        size: f32,
        position: (f32, f32, f32),  // X, Y, Z
        layer_id: LayerId,
        tags: Vec<String>,
        momentum: (f32, f32, f32),   // X, Y, Z momentum
        resistance: (f32, f32, f32),
        gravity: f32,
    ) -> Self

    pub fn new_rect(...) -> Self
    pub fn with_depth_scaling(mut self, enabled: bool) -> Self
    pub fn with_shadow(mut self, shadow: Shadow) -> Self
    pub fn set_z_position(&mut self, z: f32)
    pub fn get_z_position(&self) -> f32
    pub fn get_world_position(&self) -> (f32, f32, f32)
}
```

### 4. Camera

Controls the viewport and handles movement/following.

```rust
pub struct Camera {
    position: (f32, f32, f32),
    zoom: f32,
    follow_target: Option<String>,
    follow_smoothing: f32,
    bounds: Option<CameraBounds>,
}

pub struct CameraBounds {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

impl Camera {
    pub fn new(position: (f32, f32, f32)) -> Self
    pub fn set_position(&mut self, pos: (f32, f32, f32))
    pub fn set_zoom(&mut self, zoom: f32)
    pub fn follow(&mut self, target_id: String, smoothing: f32)
    pub fn set_bounds(&mut self, bounds: CameraBounds)
    pub fn get_position(&self) -> (f32, f32, f32)
}
```

### 5. Shadow

Simple shadow configuration for depth perception.

```rust
pub struct Shadow {
    offset: (f32, f32),
    scale: f32,
    opacity: f32,
    color: image::Rgba<u8>,
}

impl Shadow {
    pub fn new(offset: (f32, f32), scale: f32, opacity: f32) -> Self
    pub fn default_drop_shadow() -> Self  // Standard bottom shadow
}
```

## Key Features

### 1. Automatic Depth Scaling

Objects automatically scale based on their Z-position to create perspective:

```rust
// Objects further away (lower Z) appear smaller
let depth_factor = 0.5 + (z_position * 0.5);  // 0.5x to 1.0x scale
let rendered_scale = base_scale * depth_factor;
```

### 2. Parallax Scrolling

Each layer moves at a different rate when the camera moves:

```rust
// Layer offset based on camera position and parallax
let layer_offset_x = camera.position.0 * layer.parallax_factor;
let layer_offset_y = camera.position.1 * layer.parallax_factor;
```

### 3. Z-Ordering

Objects are rendered in depth order within their layer:

```rust
// Sort objects by Z before rendering
objects.sort_by(|a, b| a.z_position.partial_cmp(&b.z_position).unwrap());
```

### 4. Layer-Based Collision

Objects only collide with others on the same layer:

```rust
pub fn collision_on_layer(&self, target1: &Target, target2: &Target, layer_id: LayerId) -> bool
```

### 5. Depth Transitions

Objects can move between layers:

```rust
pub enum Action {
    // ... existing Quartz actions ...
    MoveToLayer {
        target: Target,
        layer_id: LayerId,
        z_position: f32,
    },
    AdjustDepth {
        target: Target,
        z_delta: f32,  // Change in Z position
    },
}
```

## Enhanced Event System

FlowMango extends Quartz's event system with depth awareness:

```rust
pub enum WorldEvent {
    // Standard Quartz events work within layers
    KeyPress { key: Key, action: Action, target: Target },
    
    // New depth-aware events
    DepthCollision {
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
}
```

## Complete Example: 2.5D Platformer

```rust
use flowmango::{Scene, WorldObject, Layer, Camera, CanvasMode, Shadow};
use quartz::{Key, Context, Image, ShapeType, Action, Target, GameEvent};
use ramp::prism;
use prism::drawable::Drawable;

pub struct My2D5Game;

impl My2D5Game {
    fn new(ctx: &mut Context) -> impl Drawable {
        let mode = CanvasMode::Landscape;
        
        // Create scene with 5 layers
        let mut scene = Scene::new(ctx, mode, 5);
        
        // Configure layers with different parallax
        let far_bg_layer = scene.add_layer(0.0, 0.2);      // Slow moving background
        let mid_bg_layer = scene.add_layer(0.3, 0.5);      // Medium speed
        let gameplay_layer = scene.add_layer(0.6, 1.0);    // Main gameplay (no parallax)
        let near_fg_layer = scene.add_layer(0.8, 1.3);     // Foreground elements
        
        // Setup camera
        scene.camera.set_zoom(1.0);
        scene.camera.follow("player".to_string(), 0.1);
        
        // Create far background (mountains)
        let mountain_image = create_image(ctx, [100, 100, 150, 255]);
        let mountains = WorldObject::new_rect(
            ctx,
            "mountains".to_string(),
            mountain_image,
            (3840.0, 800.0),
            (0.0, 1360.0, 0.0),  // X, Y, Z
            far_bg_layer,
            vec!["background".to_string()],
            (0.0, 0.0, 0.0),
            (1.0, 1.0, 1.0),
            0.0,
        );
        scene.add_object(mountains);
        
        // Create ground platform on gameplay layer
        let ground_image = create_image(ctx, [100, 100, 100, 255]);
        let ground = WorldObject::new_rect(
            ctx,
            "ground".to_string(),
            ground_image,
            (3840.0, 50.0),
            (0.0, 2000.0, 0.6),  // At gameplay depth
            gameplay_layer,
            vec!["platform".to_string()],
            (0.0, 0.0, 0.0),
            (1.0, 1.0, 1.0),
            0.0,
        ).as_platform();
        scene.add_object(ground);
        
        // Create player with shadow
        let player_image = create_image(ctx, [0, 255, 0, 255]);
        let player = WorldObject::new(
            ctx,
            "player".to_string(),
            player_image,
            100.0,
            (400.0, 1900.0, 0.6),  // Start on gameplay layer
            gameplay_layer,
            vec!["player".to_string()],
            (0.0, 0.0, 0.0),
            (0.98, 0.98, 1.0),
            1.2,  // Gravity
        )
        .with_depth_scaling(true)
        .with_shadow(Shadow::default_drop_shadow());
        scene.add_object(player);
        
        // Create foreground trees
        let tree_image = create_image(ctx, [50, 150, 50, 255]);
        let tree = WorldObject::new_rect(
            ctx,
            "tree_fg".to_string(),
            tree_image,
            (200.0, 400.0),
            (1000.0, 1700.0, 0.85),  // In front of player
            near_fg_layer,
            vec!["foreground".to_string()],
            (0.0, 0.0, 0.0),
            (1.0, 1.0, 1.0),
            0.0,
        ).with_depth_scaling(true);
        scene.add_object(tree);
        
        // Add movement controls
        scene.add_event(
            WorldEvent::KeyPress {
                key: Key::Character("a".to_string().into()),
                action: Action::SetMomentum {
                    target: Target::ById("player".to_string()),
                    value: (-10.0, 0.0, 0.0),
                },
                target: Target::ById("player".to_string()),
            },
            gameplay_layer,
        );
        
        scene.add_event(
            WorldEvent::KeyPress {
                key: Key::Character("d".to_string().into()),
                action: Action::SetMomentum {
                    target: Target::ById("player".to_string()),
                    value: (10.0, 0.0, 0.0),
                },
                target: Target::ById("player".to_string()),
            },
            gameplay_layer,
        );
        
        // Jump
        scene.add_event(
            WorldEvent::KeyPress {
                key: Key::Character("w".to_string().into()),
                action: Action::ApplyMomentum {
                    target: Target::ById("player".to_string()),
                    value: (0.0, -25.0, 0.0),
                },
                target: Target::ById("player".to_string()),
            },
            gameplay_layer,
        );
        
        // Move between layers with Q/E keys
        scene.add_event(
            WorldEvent::KeyPress {
                key: Key::Character("q".to_string().into()),
                action: Action::AdjustDepth {
                    target: Target::ById("player".to_string()),
                    z_delta: -0.1,  // Move toward background
                },
                target: Target::ById("player".to_string()),
            },
            gameplay_layer,
        );
        
        scene.add_event(
            WorldEvent::KeyPress {
                key: Key::Character("e".to_string().into()),
                action: Action::AdjustDepth {
                    target: Target::ById("player".to_string()),
                    z_delta: 0.1,  // Move toward foreground
                },
                target: Target::ById("player".to_string()),
            },
            gameplay_layer,
        );
        
        scene
    }
}

fn create_image(ctx: &mut Context, color: [u8; 4]) -> Image {
    Image {
        shape: ShapeType::Rectangle(0.0, (1.0, 1.0), 0.0),
        image: image::RgbaImage::from_pixel(1, 1, image::Rgba(color)).into(),
        color: None,
    }
}

ramp::run! {|ctx: &mut Context| {
    My2D5Game::new(ctx)
}}
```

## Implementation Phases

### Phase 1: Core Layer System
- [ ] Create `Scene` struct with multiple `Canvas` instances
- [ ] Implement `Layer` wrapper around `Canvas`
- [ ] Add basic rendering with Z-ordering
- [ ] Test multi-layer rendering

### Phase 2: Camera System
- [ ] Implement `Camera` struct
- [ ] Add camera position tracking
- [ ] Implement parallax calculation
- [ ] Add camera following with smoothing

### Phase 3: WorldObject
- [ ] Create `WorldObject` wrapper around `GameObject`
- [ ] Add Z-position support
- [ ] Implement depth-based scaling
- [ ] Add layer assignment

### Phase 4: Depth Physics
- [ ] Extend collision detection for layers
- [ ] Add 3D momentum (X, Y, Z)
- [ ] Implement layer transitions
- [ ] Add depth-aware events

### Phase 5: Visual Enhancements
- [ ] Add shadow rendering
- [ ] Implement depth fog (optional)
- [ ] Add lighting system (optional)
- [ ] Performance optimization

### Phase 6: Advanced Features
- [ ] Camera zoom
- [ ] Camera bounds/limits
- [ ] Animated layer transitions
- [ ] Depth-of-field effects

## Design Principles

1. **Wrap, Don't Replace**: FlowMango uses Quartz underneath, maintaining compatibility
2. **Automatic Depth**: The engine handles perspective math automatically
3. **Layer Independence**: Each layer is a full Quartz Canvas
4. **Simple API**: Hide complexity, expose intuitive 2.5D concepts
5. **Performance**: Minimize overhead, leverage Quartz's efficiency

## Use Cases

FlowMango excels at:
- **2.5D Platformers**: Side-scrolling with depth (like Donkey Kong Country)
- **Beat 'em ups**: Games with vertical lanes (like Streets of Rage)
- **Adventure Games**: Exploration with parallax (like Octopath Traveler)
- **Puzzle Games**: Depth-based mechanics
- **Visual Novels**: Layered character sprites and backgrounds

## Technical Considerations

### Rendering Order
```
1. Sort layers by depth (back to front)
2. For each layer:
   a. Calculate parallax offset
   b. Sort objects by Z-position
   c. Render objects
   d. Render shadows
```

### Coordinate Conversion
```rust
// World coordinates (X, Y, Z) -> Screen coordinates (X, Y)
let screen_x = world_x - camera_x * parallax;
let screen_y = world_y - camera_y * parallax;

// Apply depth scaling
let scale = base_scale * (0.5 + z * 0.5);
```

### Memory Usage
Each layer has its own Canvas, so a 5-layer scene uses 5× the memory of a single Quartz canvas. Design with memory constraints in mind.

## API Compatibility

FlowMango maintains Quartz API compatibility:
- All Quartz `Action` types work
- All Quartz `Target` types work
- All Quartz `GameEvent` types work
- All Quartz physics behavior is preserved per-layer

## Future Enhancements

- **Lighting system**: Dynamic lights with depth falloff
- **Particle system**: Depth-aware particle effects
- **Tiled map support**: Import from Tiled editor with layers
- **Layer blending**: Alpha blending between layers
- **Dynamic layer creation**: Add/remove layers at runtime
- **Camera shake**: Depth-affected screen shake
- **Depth fog**: Atmospheric depth effect

---

**FlowMango** = Flow (smooth depth transitions) + Mango (because it's built on top of another fruit... Quartz isn't a fruit but let's roll with it! 🥭)# flowmango
# island_cafe
