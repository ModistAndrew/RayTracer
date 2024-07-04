use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::texture::UV;
use crate::vec3::Vec3;

pub trait Shape {
    // hit_record.ray is the original ray. may contain the former hit record.
    // if hit, update hit_record.hit and ray and return true
    fn hit(&self, hit_record: &mut HitRecord) -> bool;
    // return the bounding box for hit testing. only called once for construction
    fn aabb(&self) -> AABB;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Shape for Sphere {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        let ray = &hit_record.ray;
        let interval = ray.interval;
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
        hit_record.set_hit(root, outward_normal, UV::new(0.0, 0.0));
        true
    }

    fn aabb(&self) -> AABB {
        let r_vec = Vec3::new(self.radius, self.radius, self.radius);
        AABB::from_vec3(self.center - r_vec, self.center + r_vec)
    }
}

pub struct Moving<T: Shape> {
    direction: Vec3,
    shape: T,
}

impl<T: Shape> Moving<T> {
    pub fn new(direction: Vec3, shape: T) -> Self {
        Self { direction, shape }
    }
}

impl<T: Shape> Shape for Moving<T> {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        let shift = self.direction * hit_record.ray.time;
        hit_record.ray.origin -= shift;
        let hit = self.shape.hit(hit_record);
        hit_record.ray.origin += shift;
        if hit {
            hit_record.get_hit_mut().position += shift;
        }
        hit
    }

    fn aabb(&self) -> AABB {
        let stationary_aabb = self.shape.aabb();
        stationary_aabb.union(stationary_aabb + self.direction)
    }
}
