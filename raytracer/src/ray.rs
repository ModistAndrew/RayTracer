use crate::interval::Interval;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3, // no need to normalize
    pub time: f64,
    pub interval: Interval,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f64, interval: Interval) -> Self {
        Self {
            origin,
            direction,
            time,
            interval,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn new_ray(&self, origin: Vec3, direction: Vec3) -> Self {
        Self::new(origin, direction, self.time, Interval::POSITIVE)
    }
}
