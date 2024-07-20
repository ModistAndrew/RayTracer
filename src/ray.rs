use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3, // no need to normalize
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            time: rand::random::<f64>(),
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    // return a new Ray with the same time
    pub fn new_ray(&self, origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            time: self.time,
        }
    }

    pub fn offset(&self, v: Vec3) -> Self {
        self.new_ray(self.origin + v, self.direction)
    }

    pub fn ray3(&self) -> Ray3 {
        Ray3::new(self.origin.into(), self.direction.into())
    }
}

pub type Ray3 = bvh::ray::Ray<f64, 3>;
