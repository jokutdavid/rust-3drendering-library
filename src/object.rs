use glam::Vec3;

pub  struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Triangle {
        Triangle { a, b, c }
    }
}

pub struct Object {
    pub triangles: Box<[Triangle]>,
    pub colors: Box<[[u8; 3]]>,

    pub x_position: f32,
    pub y_position: f32,
    pub z_position: f32,

    pub x_rotation: f32,
    pub y_rotation: f32,
    pub z_rotation: f32,
}