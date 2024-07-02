use crate::hittable::HitRecord;
use crate::interval::Interval;
use crate::vec3::Vec3;

pub trait Shape {
    // hit_record.ray is the original ray. may contain the former hit record. if hit, update hit_record.hit and return true
    fn hit(&self, hit_record: &mut HitRecord, interval: Interval) -> bool;
}

pub struct Core {
    center: Vec3,
    direction: Vec3,

}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    direction: Vec3,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, direction: Vec3) -> Self {
        Self {
            center,
            radius,
            direction,
        }
    }
}

impl Shape for Sphere {
    fn hit(&self, hit_record: &mut HitRecord, interval: Interval) -> bool {
        let ray = &hit_record.ray;
        let oc = self.center + self.direction * ray.time - ray.origin;
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
