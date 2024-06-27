use crate::vec3d::Vec3d;

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    #[allow(unused)]
    origin: Vec3d,
    pub direction: Vec3d, // no need to normalize
}

impl Ray {
    pub fn new(origin: Vec3d, direction: Vec3d) -> Self {
        Self { origin, direction }
    }

    #[allow(unused)]
    fn at(&self, t: f64) -> Vec3d {
        self.origin + self.direction * t
    }
}
