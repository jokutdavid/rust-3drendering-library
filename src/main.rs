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

static VIEWPORT: Viewport = Viewport {
    x: 0.5,
    y: 0.5,
    z: 1.0
};

fn main() {
    let mut cube: Object = Object {
        triangles: Box::new([
            Triangle::new(
                Vec3::new(0.4, 0.3, 0.8,),
                Vec3::new(0.7, 0.3, 0.8),
                Vec3::new(0.7, 0.6, 0.8),
                [255, 0, 0]
            ),

            Triangle::new(
                Vec3::new(0.4, 0.3, 0.8),
                Vec3::new(0.7, 0.6, 0.8),
                Vec3::new(0.4, 0.6, 0.8),
                [255, 0, 0]
            ),

            //Bottom Face
            Triangle::new(
                Vec3::new(0.4, 0.6, 0.5),
                Vec3::new(0.4, 0.6, 0.8),
                Vec3::new(0.7, 0.6, 0.8),
                [0, 255, 0]
            ),

            Triangle::new(
                Vec3::new(0.7, 0.6, 0.8),
                Vec3::new(0.7, 0.6, 0.5),
                Vec3::new(0.4, 0.6, 0.5),
                [0, 255, 0]
            ),

            //Right Face
            Triangle::new(
                Vec3::new(0.7, 0.3, 0.8),
                Vec3::new(0.7, 0.6, 0.5),
                Vec3::new(0.7, 0.6, 0.8),
                [0, 0, 255]
            ),

            Triangle::new(
                Vec3::new(0.7, 0.3, 0.8),
                Vec3::new(0.7, 0.3, 0.5),
                Vec3::new(0.7, 0.6, 0.5),
                [0, 0, 255]
            ),

            //Left Face
            Triangle::new(
                Vec3::new(0.4, 0.6, 0.8),
                Vec3::new(0.4, 0.3, 0.8),
                Vec3::new(0.4, 0.6, 0.5),
                [255, 0, 255]
            ),

            Triangle::new(
                Vec3::new(0.4, 0.3, 0.8),
                Vec3::new(0.4, 0.3, 0.5),
                Vec3::new(0.4, 0.6, 0.5),
                [255, 0, 255]
            ),

            //Front Face
            Triangle::new(
                Vec3::new(0.4, 0.3, 0.5),
                Vec3::new(0.7, 0.3, 0.5),
                Vec3::new(0.7, 0.6, 0.5),
                [255, 255, 0]
            ),

            Triangle::new(
                Vec3::new(0.4, 0.3, 0.5),
                Vec3::new(0.7, 0.6, 0.5),
                Vec3::new(0.4, 0.6, 0.5),
                [255, 255, 0]
            ),

            //Top Face
            Triangle::new(
                Vec3::new(0.4, 0.3, 0.5),
                Vec3::new(0.4, 0.3, 0.8),
                Vec3::new(0.7, 0.3, 0.8),
                [0, 255, 255]
            ),

            Triangle::new(
                Vec3::new(0.7, 0.3, 0.5),
                Vec3::new(0.4, 0.3, 0.5),
                Vec3::new(0.7, 0.3, 0.8),
                [0, 255, 255]
            ),
        ]),

        x_position: 0.0,
        y_position: 0.0,
        z_position: 0.0,
        x_rotation: 0.0,
        y_rotation: 0.0,
        z_rotation: 0.0,
    };


    let mut frame: u64 = 0;
    let mut window = Window::new("window", 800, 600);

    let mut camera = Camera  {
        x_position: 0.4f32,
        y_position: 0.3,
        z_position: -0.2,

        x_rotation: 0.0,
        y_rotation: 0.0,
        z_rotation: 0.0
    };

    let mut run = true;
    while !window.should_close() && run {
        cube.triangles.sort_by(|tri1, tri2| tri2.a.z.partial_cmp(&tri1.a.z).unwrap());

        if frame < 18446744073709551615 { frame += 1; } else {run = false}
        if frame % 10 == 0 {println!("{}", frame);}

        let framebuffer = window.framebuffer();

        framebuffer.clear(from_u8_rgb(217, 217, 217) / 4);

        draw_object_3d(&cube, &VIEWPORT, &camera, framebuffer);

        camera.x_position -= 0.003;
        camera.y_position += 0.003;


        window.display();
    }
}