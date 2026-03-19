#[derive(Debug, Clone)]
pub struct CameraBounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl CameraBounds {
    pub fn new(min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    pub fn clamp_position(&self, pos: (f32, f32, f32)) -> (f32, f32, f32) {
        (
            pos.0.clamp(self.min_x, self.max_x),
            pos.1.clamp(self.min_y, self.max_y),
            pos.2,
        )
    }
}

#[derive(Debug, Clone)]
struct ShakeState {
    intensity: f32,
    elapsed: f32,
    duration: f32,
}

 #[derive(Debug, Clone)] 
pub struct Camera {
    position: (f32, f32, f32),
    zoom: f32,
    follow_target: Option<String>,
    follow_smoothing: f32,
    bounds: Option<CameraBounds>,
    target_position: (f32, f32, f32),
    shake_state: Option<ShakeState>,
}

impl Camera {
    pub fn new(position: (f32, f32, f32)) -> Self {
        Self {
            position,
            zoom: 1.0,
            follow_target: None,
            follow_smoothing: 0.1,
            bounds: None,
            target_position: position,
            shake_state: None,
        }
    }

    pub fn default() -> Self {
        Self::new((0.0, 0.0, 0.0))
    }

    pub fn set_position(&mut self, pos: (f32, f32, f32)) {
        let pos = if let Some(ref bounds) = self.bounds {
            bounds.clamp_position(pos)
        } else {
            pos
        };
        self.position = pos;
        self.target_position = pos;
    }

    pub fn get_position(&self) -> (f32, f32, f32) {
        self.position
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.max(0.1); 
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn follow(&mut self, target_id: String, smoothing: f32) {
        self.follow_target = Some(target_id);
        self.follow_smoothing = smoothing.clamp(0.0, 1.0);
    }

    pub fn stop_following(&mut self) {
        self.follow_target = None;
    }

    pub fn get_follow_target(&self) -> Option<&String> {
        self.follow_target.as_ref()
    }

    pub fn set_bounds(&mut self, bounds: CameraBounds) {
        self.bounds = Some(bounds.clone());
        self.position = bounds.clamp_position(self.position.clone());
    }

    pub fn clear_bounds(&mut self) {
        self.bounds = None;
    }

    pub fn get_bounds(&self) -> Option<&CameraBounds> {
        self.bounds.as_ref()
    }

    pub(crate) fn update_follow(&mut self, target_pos: (f32, f32, f32)) {
        if self.follow_target.is_some() {
            self.target_position = target_pos;
            
            let t = 1.0 - self.follow_smoothing;
            let new_pos = (
                self.position.0 + (self.target_position.0 - self.position.0) * t,
                self.position.1 + (self.target_position.1 - self.position.1) * t,
                self.position.2 + (self.target_position.2 - self.position.2) * t,
            );

            self.position = if let Some(ref bounds) = self.bounds {
                bounds.clamp_position(new_pos)
            } else {
                new_pos
            };
        }
    }

    pub(crate) fn update_shake(&mut self, delta_time: f32) {
        if let Some(ref mut shake) = self.shake_state {
            shake.elapsed += delta_time;
            if shake.elapsed >= shake.duration {
                self.shake_state = None;
            }
        }
    }

    pub(crate) fn calculate_layer_offset(&self, parallax_factor: f32) -> (f32, f32) {
        let mut offset = (
            -self.position.0 * parallax_factor * self.zoom,
            -self.position.1 * parallax_factor * self.zoom,
        );

        if let Some(ref shake) = self.shake_state {
            let progress = shake.elapsed / shake.duration;
            let decay = 1.0 - progress;
            let shake_x = (shake.elapsed * 50.0).sin() * shake.intensity * decay;
            let shake_y = (shake.elapsed * 43.0).cos() * shake.intensity * decay;
            offset.0 += shake_x;
            offset.1 += shake_y;
        }

        offset
    }

    pub fn shake(&mut self, intensity: f32, duration: f32) {
        self.shake_state = Some(ShakeState {
            intensity,
            elapsed: 0.0,
            duration,
        });
    }

    pub fn is_visible(&self, position: (f32, f32), size: (f32, f32), viewport_size: (f32, f32)) -> bool {
        let (cam_x, cam_y, _) = self.position;
        let half_width = viewport_size.0 / (2.0 * self.zoom);
        let half_height = viewport_size.1 / (2.0 * self.zoom);
        
        position.0 + size.0 > cam_x - half_width &&
        position.0 < cam_x + half_width &&
        position.1 + size.1 > cam_y - half_height &&
        position.1 < cam_y + half_height
    }
}