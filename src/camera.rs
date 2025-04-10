use glam::*;

pub struct Camera {
    pub x_position: f32,
    pub y_position: f32,
    pub z_position: f32,

    pub x_rotation: f32,
    pub y_rotation: f32,
    pub z_rotation: f32,
}

pub struct Viewport {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

