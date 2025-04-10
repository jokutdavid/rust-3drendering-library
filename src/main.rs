mod window;
mod projection;
mod camera;
mod draw;
mod object;

use glam::*;
use crate::camera::*;
use crate::window::*;
use crate::draw::*;
use crate::object::*;
static POINTS_3D: &[Triangle] = &[

    //Back Face
    Triangle {
        Vec3::new(0.4,
        0.3,
        0.8),
        Vec3::new(0.7,
        0.3,
        0.8),
        Vec3::new(0.7,
        0.6,
        0.8),
    },

    Triangle::new(
        Vec3::new(0.4, 0.3, 0.8),
        Vec3::new(0.7, 0.6, 0.8),
        Vec3::new(0.4, 0.6, 0.8),
    ),

    //Bottom Face
    Triangle::new(
        Vec3::new(0.4, 0.6, 0.5),
        Vec3::new(0.4, 0.6, 0.8),
        Vec3::new(0.7, 0.6, 0.8),
    ),

    Triangle::new(
        Vec3::new(0.7, 0.6, 0.8),
        Vec3::new(0.7, 0.6, 0.5),
        Vec3::new(0.4, 0.6, 0.5),
    ),

    //Right Face
    Triangle::new(
        Vec3::new(0.7, 0.3, 0.8),
        Vec3::new(0.7, 0.6, 0.5),
        Vec3::new(0.7, 0.6, 0.8),
    ),

    Triangle::new(
        Vec3::new(0.7, 0.3, 0.8),
        Vec3::new(0.7, 0.3, 0.5),
        Vec3::new(0.7, 0.6, 0.5),
    ),

    //Left Face
    Triangle::new(
        Vec3::new(0.4, 0.6, 0.8),
        Vec3::new(0.4, 0.3, 0.8),
        Vec3::new(0.4, 0.6, 0.5),
    ),

    Triangle::new(
        Vec3::new(0.4, 0.3, 0.8),
        Vec3::new(0.4, 0.3, 0.5),
        Vec3::new(0.4, 0.6, 0.5),
    ),

    //Front Face
    Triangle::new(
        Vec3::new(0.4, 0.3, 0.5),
        Vec3::new(0.7, 0.3, 0.5),
        Vec3::new(0.7, 0.6, 0.5),
    ),

    Triangle::new(
        Vec3::new(0.4, 0.3, 0.5),
        Vec3::new(0.7, 0.6, 0.5),
        Vec3::new(0.4, 0.6, 0.5),
    ),
    //Top Face
    Triangle::new(
        Vec3::new(0.4, 0.3, 0.5),
        Vec3::new(0.4, 0.3, 0.8),
        Vec3::new(0.7, 0.3, 0.8),
    ),

    Triangle::new(
        Vec3::new(0.7, 0.3, 0.5),
        Vec3::new(0.4, 0.3, 0.5),
        Vec3::new(0.7, 0.3, 0.8),
    ),
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

        x_rotation: -12.0,
        y_rotation: 0.0,
        z_rotation: 0.0
    };

    let mut run = true;
    while !window.should_close() && run {
        if frame < 18446744073709551615 { frame += 1; } else {run = false}
        if frame % 10 == 0 {println!("{}", frame);}

        let framebuffer = window.framebuffer();

        framebuffer.clear(from_u8_rgb(217, 217, 217) / 4);

        draw_object_3d(POINTS_3D, TRIG_COLOR, &VIEWPORT, &camera, framebuffer);

        camera.y_rotation = 0f32 + ((frame / 20) as f32).sin() * 3f32;

        window.display();
    }
}