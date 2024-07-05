use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::ops::{Add, Index, Sub};

#[derive(Clone, Copy, Default)]
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
        AABB::new(
            Interval::min_max(v0.x, v1.x),
            Interval::min_max(v0.y, v1.y),
            Interval::min_max(v0.z, v1.z),
        )
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        let mut t_interval = ray.interval;
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let t0 = (self[i].min - ray.origin[i]) * inv_d;
            let t1 = (self[i].max - ray.origin[i]) * inv_d;
            t_interval = t_interval.intersect(Interval::min_max(t0, t1));
            if t_interval.empty() {
                return false;
            }
        }
        true
    }

    pub fn union(self, other: Self) -> Self {
        AABB::new(
            self.x.union(other.x),
            self.y.union(other.y),
            self.z.union(other.z),
        )
    }

    pub fn contains(&self, v: Vec3) -> bool {
        self.x.contains(v.x) && self.y.contains(v.y) && self.z.contains(v.z)
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }
}

impl Add<Vec3> for AABB {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        AABB::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vec3> for AABB {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        AABB::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Index<usize> for AABB {
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
