use glam::*;
use std::f32::consts::PI;
use crate::camera::{Camera, Viewport};


pub fn camera_transform(point: Vec3, camera: &Camera) -> Vec3 {
    let deg = PI / 180.0f32; //Don't want to deal with radians

    let x = point.x - camera.x_position;
    let y = point.y - camera.y_position;
    let z = point.z - camera.z_position;

    let sin_x = (camera.x_rotation * deg).sin();
    let sin_y = (camera.y_rotation * deg).sin();
    let sin_z = (camera.z_rotation * deg).sin();

    let cos_x = (camera.x_rotation * deg).cos();
    let cos_y = (camera.y_rotation * deg).cos();
    let cos_z = (camera.z_rotation * deg).cos();

    Vec3::new(
        (cos_y * ((sin_z * y) + (cos_x * x))) - sin_y * z,
        (sin_x * ((cos_y * z) + (sin_y * ((sin_z * y) + (cos_z * x))))) + cos_x * ((cos_z * y) - (sin_z * x)),
        (cos_x * ((cos_y * z) + (sin_y * ((sin_z * y) + (cos_z * x))))) - sin_x * ((cos_z * y) - (sin_z * x))
    )
}

pub fn project(point: Vec3, viewport: &Viewport, camera: &Camera) -> Vec2 {

    Vec2::new(
        (viewport.z / point.z) * point.x + viewport.x,
        (viewport.z / point.z) * point.y + viewport.y,
    )
}