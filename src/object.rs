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