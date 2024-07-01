use crate::hittable::HitRecord;
use crate::interval::Interval;
use crate::vec3d::Vec3d;

pub trait Shape {
    // hit_record.ray is the original ray. may contain the former hit record. if hit, update hit_record.hit and return true
    fn hit(&self, hit_record: &mut HitRecord, interval: Interval) -> bool;
}

pub struct Sphere {
    center: Vec3d,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3d, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Shape for Sphere {
    fn hit(&self, hit_record: &mut HitRecord, interval: Interval) -> bool {
        let ray = &hit_record.ray;
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;
        if !interval.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !interval.surrounds(root) {
                return false;
            }
        }
        let outward_normal = (ray.at(root) - self.center) / self.radius;
        hit_record.set_hit(root, outward_normal);
        true
    }
}