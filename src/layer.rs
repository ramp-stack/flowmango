// layer.rs
use quartz::{Canvas, CanvasMode, Context};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayerId(pub usize);

#[derive(Debug, Clone)]
pub struct Layer {
    pub(crate) canvas: Canvas,
    pub(crate) depth: f32,
    pub(crate) parallax_factor: f32,
    pub(crate) visible: bool,
    pub(crate) id: LayerId,
    pub(crate) camera_offset: (f32, f32),
}

impl Layer {
    pub fn new(
        ctx: &mut Context,
        mode: CanvasMode,
        depth: f32,
        parallax: f32,
        id: LayerId,
    ) -> Self {
        Self {
            canvas: Canvas::new(ctx, mode),
            depth,
            parallax_factor: parallax,
            visible: true,
            id,
            camera_offset: (0.0, 0.0),
        }
    }

    pub fn set_parallax(&mut self, factor: f32) {
        self.parallax_factor = factor;
    }

    pub fn get_parallax(&self) -> f32 {
        self.parallax_factor
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn get_depth(&self) -> f32 {
        self.depth
    }

    pub fn get_id(&self) -> LayerId {
        self.id
    }

    pub fn camera_offset(&self) -> (f32, f32) {
        self.camera_offset
    }

    pub fn set_camera_offset(&mut self, offset: (f32, f32)) {
        self.camera_offset = offset;
    }

    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }
}