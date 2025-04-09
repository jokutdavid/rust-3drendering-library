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
    let mut window = Window::new("window", 800, 600);

    let camera = Camera  {
        x_position: 0.4,
        y_position: 0.1,
        z_position: -0.2,

        x_rotation: 0.0,
        y_rotation: 0.0,
        z_rotation: 0.0
    };


    while !window.should_close() {
        let framebuffer = window.framebuffer();

        framebuffer.clear(from_u8_rgb(217, 217, 217) / 4);

        draw_object_3d(POINTS_3D, TRIG_COLOR, &VIEWPORT, &camera, framebuffer);



        window.display();
    }
}