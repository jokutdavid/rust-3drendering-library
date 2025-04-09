mod window;
mod projection;
mod camera;
mod draw;

use glam::*;
use crate::camera::*;
use crate::window::*;
use crate::draw::*;
static POINTS_3D: &[Vec3] = &[

    //Back Face
    Vec3::new(0.4, 0.3, 0.8),
    Vec3::new(0.7, 0.3, 0.8),
    Vec3::new(0.7, 0.6, 0.8),

    Vec3::new(0.4, 0.3, 0.8),
    Vec3::new(0.7, 0.6, 0.8),
    Vec3::new(0.4, 0.6, 0.8),

    //Bottom Face
    Vec3::new(0.4, 0.6, 0.5),
    Vec3::new(0.4, 0.6, 0.8),
    Vec3::new(0.7, 0.6, 0.8),

    Vec3::new(0.7, 0.6, 0.5),
    Vec3::new(0.4, 0.6, 0.5),
    Vec3::new(0.7, 0.6, 0.8),

    //Right Face

    Vec3::new(0.7, 0.3, 0.8),
    Vec3::new(0.7, 0.6, 0.5),
    Vec3::new(0.7, 0.6, 0.8),

    Vec3::new(0.7, 0.3, 0.8),
    Vec3::new(0.7, 0.3, 0.5),
    Vec3::new(0.7, 0.6, 0.5),

    //Left Face
    Vec3::new(0.4, 0.3, 0.8),
    Vec3::new(0.4, 0.6, 0.5),
    Vec3::new(0.4, 0.6, 0.8),

    Vec3::new(0.4, 0.3, 0.8),
    Vec3::new(0.4, 0.3, 0.5),
    Vec3::new(0.4, 0.6, 0.5),

    //Front Face
    Vec3::new(0.4, 0.3, 0.5),
    Vec3::new(0.7, 0.3, 0.5),
    Vec3::new(0.7, 0.6, 0.5),

    Vec3::new(0.4, 0.3, 0.5),
    Vec3::new(0.7, 0.6, 0.5),
    Vec3::new(0.4, 0.6, 0.5),

    //Top Face
    Vec3::new(0.4, 0.3, 0.5),
    Vec3::new(0.4, 0.3, 0.8),
    Vec3::new(0.7, 0.3, 0.8),

    Vec3::new(0.7, 0.3, 0.5),
    Vec3::new(0.4, 0.3, 0.5),
    Vec3::new(0.7, 0.3, 0.8),
];

static TRIG_COLOR: &[[u8; 3]] = &[
    [255, 0, 0],
    [255, 0, 0],

    [0, 255, 0],
    [0, 255, 0],

    [0, 0, 255],
    [0, 0, 255],

    [0, 255, 255],
    [0, 255, 255],

    [255, 255, 0],
    [255, 255, 0],

    [255, 0, 255],
    [255, 0, 255],
];

static VIEWPORT: Viewport = Viewport {
    x: 0.5,
    y: 0.0,
    z: 1.0
};

fn main() {
    let mut frame: u64 = 0;
    let mut window = Window::new("window", 800, 600);

    let mut camera = Camera  {
        x_position: 0.8f32,
        y_position: -0.3,
        z_position: -0.2,

        x_rotation: -8.0,
        y_rotation: 0.0,
        z_rotation: 0.0
    };

    let mut run = true;
    while !window.should_close() && run {
        if frame < 18446744073709551615 { frame += 1; } else {run = false}

        let framebuffer = window.framebuffer();

        framebuffer.clear(from_u8_rgb(217, 217, 217) / 4);

        draw_object_3d(POINTS_3D, TRIG_COLOR, &VIEWPORT, &camera, framebuffer);

        camera.y_rotation = 0f32 + ((frame / 50) as f32).sin() * 5f32;

        window.display();
    }
}