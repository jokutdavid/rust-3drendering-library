use glam::*;
use std::f32::consts::PI;
use crate::camera::Camera;


fn camera_transform(point: Vec3, camera: &Camera) -> Vec3 {
    let deg = PI / 180.0f32; //Don't want to deal with radians

    let y = point.y - camera.y_position;
    let x = point.x - camera.x_position;
    let z = point.z - camera.z_position;

    let sin_x = (camera.x_rotation * deg).sin();
    let sin_y = (camera.y_rotation * deg).sin();
    let sin_z = (camera.z_rotation * deg).sin();

    Vec3::new(
        (camera.y_position * ((sin_z * y) + (camera.z_position * x))) - sin_y * z,
        (sin_x             * ((camera.y_position * z) + (sin_y * ((sin_z * y) + (camera.z_position * x))))) + camera.x_position * ((camera.z_position * y) - (sin_z * x)),
        (camera.x_position * ((camera.y_position * z) + (sin_y * ((sin_z * y) + (camera.z_position * x))))) - sin_x             * ((camera.z_position * y) - (sin_z * x))
    )
}

pub fn project(point: Vec3, camera: &Camera) -> Vec2 {
    let transformed_point = camera_transform(point, camera);

    Vec2::new(
        transformed_point.x / transformed_point.z,
        transformed_point.y / transformed_point.z
    )
}