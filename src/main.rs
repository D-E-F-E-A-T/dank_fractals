#[macro_use]
extern crate glium;

use glium::{Surface, Display};
use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder, WindowEvent, Event, VirtualKeyCode, MouseScrollDelta};
use std::time::{Instant, Duration};
use math::HasLength;

mod view;

mod math {

    pub fn clamp(value: &mut f32, min: f32, max: f32) {
        lower_clamp(value, min);
        upper_clamp(value, max);
    }

    pub fn lower_clamp(value: &mut f32, min: f32) {
        if *value < min {
            *value = min;
        }
    }

    pub fn upper_clamp(value: &mut f32, max: f32) {
        if *value > max {
            *value = max;
        }
    }

    pub trait HasLength {

        fn length(&self) -> f32;
        fn normalize(&self) -> Self;
    }

    impl HasLength for (f32, f32) {

        fn length(&self) -> f32 {
            (self.0 * self.0 + self.1 * self.1).sqrt()
        }

        fn normalize(&self) -> (f32, f32) {
            let len = self.length();
            if len == 0.0 {
                *self
            } else {
                (self.0 / len, self.1 / len)
            }
        }
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

const CAMERA_SPEED: f32 = 5.0; //units / sec
const CAMERA_ZOOM_UNIT: f32 = 0.1;

fn main() {
    let mut view = view::WindowView::new(1024, 767);
    let mut camera = view::Camera::new();

    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_title("Dank Fractals".to_string())
        .with_dimensions(view.width, view.height);
    let context = ContextBuilder::new()
        .with_vsync(true);
    let display = Display::new(window, context, &events_loop)
        .unwrap();

    let vert_buf = glium::VertexBuffer::new(&display, &[
        Vertex { position: [-1.0,  1.0] },
        Vertex { position: [ 1.0,  1.0] },
        Vertex { position: [-1.0, -1.0] },

        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [ 1.0,  1.0] },
        Vertex { position: [ 1.0, -1.0] },
    ]).unwrap();

    let program = glium::Program::from_source(
        &display,
        include_str!("basic_vertex.glsl"),
        include_str!("mandelbrot_fragment.glsl"),
        None).unwrap();

    let mut running = true;

    while running {
        let then = Instant::now();

        let mut move_direction: (f32, f32) = (0.0, 0.0);
        let mut mouse_wheel_input: f32 = 0.0;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vert_buf,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program,
                    &uniform! {
                        cam_position: camera.get_position(),
                        cam_zoom: camera.zoom,
                        screen_height: view.height as f32,
                    },
                    &Default::default()).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|event| match event {
            Event::WindowEvent { event: wevent, .. } => match wevent {
                WindowEvent::Closed => running = false,
                WindowEvent::KeyboardInput {
                    input: glium::glutin::KeyboardInput {
                        virtual_keycode: Some(kc),
                        ..
                    },
                    ..
                } => match kc {
                    VirtualKeyCode::W => move_direction.1 +=  1.0,
                    VirtualKeyCode::A => move_direction.0 += -1.0,
                    VirtualKeyCode::S => move_direction.1 += -1.0,
                    VirtualKeyCode::D => move_direction.0 +=  1.0,
                    _ => (),
                },
                WindowEvent::MouseWheel {
                    delta: MouseScrollDelta::LineDelta(_, y),
                    ..
                } => mouse_wheel_input += y,
                WindowEvent::Resized(width, height) => view.update(width, height),
                _ => ()
            },
            _ => (),
        });

        move_direction = move_direction.normalize();

        let duration = then.elapsed();
        let elapsed = (duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9) as f32;

        //do updates that require delta time here
        camera.translate(move_direction.0 * CAMERA_SPEED * elapsed, move_direction.1 * CAMERA_SPEED * elapsed);
        camera.zoom(mouse_wheel_input * CAMERA_ZOOM_UNIT);

    }

}
