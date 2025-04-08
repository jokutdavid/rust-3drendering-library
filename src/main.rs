mod window;
use window::Window;

use glam::*;
use crate::window::Framebuffer;

static POINTS: &[Vec2] = &[
    Vec2::new(0.3, 0.3),
    Vec2::new(0.7, 0.3),
    Vec2::new(0.5, 0.7),

    Vec2::new(0.1, 0.3),
    Vec2::new(0.5, 0.1),
    Vec2::new(0.2, 0.6),

    Vec2::new(0.5, 0.7),
    Vec2::new(0.9, 0.7),
    Vec2::new(0.5, 0.9),
];

fn from_u8_rgb(r: u8, g:u8, b:u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn edge_function(a: &Vec2, c: &Vec2, b: &Vec2) -> f32 {
    ((c.x - a.x) * (b.y - a.y)) - ((c.y - a.y) * (b.x - a.x))
}

fn inside_triangle(a: &Vec2, b: &Vec2, c: &Vec2, p: &Vec2) -> bool {
    let a0 = edge_function(b, c, p);
    let a1 = edge_function(c, a, p);
    let a2 = edge_function(a, b, p);

    let mut overlap = true;
    overlap &= a0 > 0.0;
    overlap &= a1 > 0.0;
    overlap &= a2 > 0.0;

    overlap
}

fn draw_triangle(framebuffer: &mut Framebuffer, a: &Vec2, b: &Vec2, c: &Vec2, color: u32) {
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

fn main() {
    let mut window = Window::new("window", 800, 600);


    while !window.should_close() {
        let framebuffer = window.framebuffer();

        framebuffer.clear(from_u8_rgb(217, 217, 217) / 4);

        draw_triangle(framebuffer, &POINTS[0], &POINTS[1], &POINTS[2], from_u8_rgb(255, 0, 0));
        draw_triangle(framebuffer, &POINTS[3], &POINTS[4], &POINTS[5], from_u8_rgb(0, 255, 0));
        draw_triangle(framebuffer, &POINTS[6], &POINTS[7], &POINTS[8], from_u8_rgb(255, 0, 255));


        window.display();
    }
}