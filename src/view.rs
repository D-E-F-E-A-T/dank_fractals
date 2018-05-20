
pub struct WindowView {
    pub width: u32,
    pub height: u32,
}

impl WindowView {
    pub fn new(width: u32, height: u32) -> WindowView {
        WindowView {
            width: width,
            height: height,
        }
    }

    pub fn update(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

pub struct Camera {
    pub position: (f32, f32),
    pub zoom: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: (0.0, 0.0),
            zoom: 1.0,
        }
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.position.0 += x / self.zoom;
        self.position.1 += y / self.zoom;
    }

    pub fn zoom(&mut self, mouse_input: f32) {
        use math::lower_clamp;

        self.zoom += mouse_input;
        lower_clamp(&mut self.zoom, 0.5);
    }

    pub fn get_position(&self) -> [f32; 2] {
        [self.position.0, self.position.1]
    }
}