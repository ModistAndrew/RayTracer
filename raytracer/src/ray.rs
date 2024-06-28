use crate::vec3d::Vec3d;

pub struct Ray {
    pub origin: Vec3d,
    pub direction: Vec3d, // no need to normalize
}

impl Ray {
    pub fn new(origin: Vec3d, direction: Vec3d) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3d {
        self.origin + self.direction * t
    }
}
