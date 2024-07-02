use std::ops;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Default)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_vec3(v0: Vec3, v1: Vec3) -> Self {
        Self {
            x: Interval::min_max(v0.x, v1.x),
            y: Interval::min_max(v0.y, v1.y),
            z: Interval::min_max(v0.z, v1.z),
        }
    }

    pub fn hit(&self, ray: &Ray, interval: Interval) -> bool {
        let mut t_interval = interval;
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let t0 = (self[i].min - ray.origin[i]) * inv_d;
            let t1 = (self[i].max - ray.origin[i]) * inv_d;
            let t_interval_i = Interval::min_max(t0, t1);
            t_interval.intersect(t_interval_i);
            if t_interval.empty() {
                return false;
            }
        }
        true
    }
}

impl ops::Index<usize> for AABB {
    type Output = Interval;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}