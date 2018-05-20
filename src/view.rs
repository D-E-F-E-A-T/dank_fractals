use math::Vec2;

pub struct WindowView {
    pub width: u32,
    pub height: u32,
}

impl WindowView {
    pub fn new(width: u32, height: u32) -> WindowView {
        WindowView {
            width,
            height,
        }
    }

    pub fn update(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

pub struct Camera {
    pub position: Vec2,
    pub zoom: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Vec2 { x: 0.0, y: 0.0 },
            zoom: 1.0,
        }
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.position.x += x / self.zoom;
        self.position.y += y / self.zoom;
    }

    pub fn zoom(&mut self, mouse_input: f32) {
        use math::{lower_clamp, zoom_scale_function};

        let scale = zoom_scale_function(self.zoom);
        self.zoom += (mouse_input * scale);
        lower_clamp(&mut self.zoom, 0.5);
    }

    pub fn get_position(&self) -> [f32; 2] {
        [self.position.x, self.position.y]
    }
}