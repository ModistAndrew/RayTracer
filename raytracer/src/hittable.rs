use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3d::Vec3d;

pub struct HitRecord {
    pub t: f64,
    pub position: Vec3d,
    pub normal: Vec3d, // always normalized and points opposite to the ray
    pub front_face: bool, // whether outside the object
}

impl HitRecord {
    // outward_normal should be normalized
    pub fn new(ray: &Ray, t: f64, outward_normal: Vec3d) -> Self {
        let position = ray.at(t);
        let front_face = Vec3d::dot(ray.direction, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self { t, position, normal, front_face }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}

struct Sphere {
    center: Vec3d,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3d, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = Vec3d::dot(ray.direction, oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;
        if (!interval.contains(root)) {
            root = (h + sqrt_d) / a;
            if (!interval.contains(root)) {
                return None;
            }
        }
        Some(HitRecord::new(ray, root, (ray.at(root) - self.center) / self.radius))
    }
}
