mod window;
mod projection;
mod camera;
mod draw;

use glam::*;
use crate::camera::Camera;
use crate::window::*;
use crate::draw::*;
static POINTS_3D: &[Vec3] = &[
    Vec3::new(0.3, 0.3, 0.4),
    Vec3::new(0.6, 0.3, 0.4),
    Vec3::new(0.5, 0.4, 0.4),
];

fn main() {
    let mut window = Window::new("window", 800, 600);

    let mut camera = Camera  {
        x_position: 0.3,
        y_position: 0.2,
        z_position: 0.1,

        x_rotation: 0.0,
        y_rotation: 0.0,
        z_rotation: 0.0
    };


    while !window.should_close() {
        let framebuffer = window.framebuffer();

        framebuffer.clear(from_u8_rgb(217, 217, 217) / 4);

        draw_object_3d(POINTS_3D, &camera, framebuffer);

        camera.z_rotation += 0.1;

        window.display();
    }
}