use image::Rgba;

#[derive(Debug, Clone)]
pub struct Shadow {
    pub offset: (f32, f32),
    pub scale: f32,
    pub opacity: f32,
    pub color: Rgba<u8>,
}

impl Shadow {
    pub fn new(offset: (f32, f32), scale: f32, opacity: f32) -> Self {
        Self {
            offset,
            scale,
            opacity,
            color: Rgba([0, 0, 0, (opacity * 255.0) as u8]),
        }
    }

    pub fn default_drop_shadow() -> Self {
        Self::new(
            (5.0, 10.0),
            0.8,
            0.3,
        )
    }

    pub fn ground_shadow() -> Self {
        Self::new(
            (0.0, 0.0),
            1.0,
            0.4,
        )
    }

    pub fn long_shadow(direction: (f32, f32)) -> Self {
        Self::new(
            direction,
            0.6,
            0.2,
        )
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
        self.color.0[3] = (self.opacity * 255.0) as u8;
    }

    pub fn set_color(&mut self, color: Rgba<u8>) {
        self.color = color;
    }

    pub fn calculate_position(&self, obj_pos: (f32, f32), _obj_size: (f32, f32)) -> (f32, f32) {
        (
            obj_pos.0 + self.offset.0,
            obj_pos.1 + self.offset.1,
        )
    }

    pub fn calculate_size(&self, obj_size: (f32, f32)) -> (f32, f32) {
        (
            obj_size.0 * self.scale,
            obj_size.1 * self.scale,
        )
    }
}

impl Default for Shadow {
    fn default() -> Self {
        Self::default_drop_shadow()
    }
}