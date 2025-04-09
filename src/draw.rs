use glam::*;
use crate::camera::{Camera, Viewport};
use crate::window::Framebuffer;
use crate::projection::*;

pub fn from_u8_rgb(r: u8, g:u8, b:u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn edge_function(a: &Vec2, c: &Vec2, b: &Vec2) -> f32 {
    ((c.x - a.x) * (b.y - a.y)) - ((c.y - a.y) * (b.x - a.x))
}

pub fn inside_triangle(a: &Vec2, b: &Vec2, c: &Vec2, p: &Vec2) -> bool {
    let a0 = edge_function(b, c, p);
    let a1 = edge_function(c, a, p);
    let a2 = edge_function(a, b, p);

    let mut overlap = true;
    overlap &= a0 > 0.0;
    overlap &= a1 > 0.0;
    overlap &= a2 > 0.0;

    overlap
}
pub fn draw_triangle(framebuffer: &mut Framebuffer, a: &Vec2, b: &Vec2, c: &Vec2, color: u32) {
    let width = framebuffer.width();
    let height = framebuffer.height();

    let min = a.min(b.min(*c)).max(Vec2::ZERO) * Vec2::new(width as f32, height as f32);
    let max = a.max(b.max(*c)).min(Vec2::ONE) * Vec2::new(width as f32, height as f32);

    for x in (min.x as usize)..(max.x as usize) {
        for y in (min.y as usize)..(max.y as usize) {
            let p = Vec2::new(
                x as f32 / width as f32,
                y as f32 / height as f32);


            if inside_triangle(a, b, c, &p) {
                framebuffer.set_pixel(x, y, color);
            }


        }
    }
}


pub fn draw_object_3d(points_3d: &[Vec3], colors: &[[u8; 3]], viewport: &Viewport, camera: &Camera, framebuffer: &mut Framebuffer) {
    let mut points: Vec<Vec2> = vec![];

    for point in points_3d {
        points.push(project(*point, viewport, camera));
    }

    if points.len() % 3 == 0 { //Make sure that there are groups of three for triangles
        let mut color: usize = 0;

        for i in 0..(points.len() / 3) {
            draw_triangle(framebuffer, &points[i * 3], &points[i * 3 + 1], &points[i * 3+ 2], from_u8_rgb(colors[color][0], colors[color][1], colors[color][2]));
            color += 1;
        }
    }
}