
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
        self.position.0 += x;
        self.position.1 += y;
    }

    pub fn zoom(&mut self, scale: f32) {
        self.zoom *= scale;
    }

    pub fn get_position(&self) -> [f32; 2] {
        [self.position.0, self.position.1]
    }
}