use crate::color::Color;
use crate::vec3d::Vec3d;

pub struct Ray {
    pub origin: Vec3d,
    pub direction: Vec3d, // no need to normalize
    pub color: Color,
}

impl Ray {
    pub fn new(origin: Vec3d, direction: Vec3d, color: Color) -> Self {
        Self {
            origin,
            direction,
            color,
        }
    }

    pub fn at(&self, t: f64) -> Vec3d {
        self.origin + self.direction * t
    }
}
