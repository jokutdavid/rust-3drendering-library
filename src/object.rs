use glam::Vec3;

pub  struct Triangle3D {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,

    pub color: [u8; 3]
}

impl Triangle3D {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, color: [u8; 3]) -> Triangle3D {
        Triangle3D { a, b, c, color }
    }
}

pub struct Object {
    pub triangles: Box<[Triangle3D]>,

    pub x_position: f32,
    pub y_position: f32,
    pub z_position: f32,

    pub x_rotation: f32,
    pub y_rotation: f32,
    pub z_rotation: f32,
}