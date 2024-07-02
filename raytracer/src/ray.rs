use crate::color::Color;
use crate::interval::Interval;
use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3, // no need to normalize
    pub color: Color,
    pub time: f64,
    pub interval: Interval,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, color: Color, time: f64, interval: Interval) -> Self {
        Self {
            origin,
            direction,
            color,
            time,
            interval,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
