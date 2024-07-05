use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::interval::Interval;
use crate::texture::UV;
use crate::vec3::Vec3;
use std::f64::consts::PI;

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

    fn uv(p: Vec3) -> UV {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;
        UV::new(phi / (2.0 * PI), theta / PI)
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
        hit_record.set_hit(root, outward_normal, Self::uv(outward_normal));
        true
    }

    fn aabb(&self) -> AABB {
        let r_vec = Vec3::new(self.radius, self.radius, self.radius);
        AABB::from_vec3(self.center - r_vec, self.center + r_vec)
    }
}

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3) -> Self {
        let n = u * v;
        let normal = n.normalize();
        let d = normal.dot(q);
        let w = n / n.length_squared();
        Self {
            q,
            u,
            v,
            w,
            normal,
            d,
        }
    }
}

impl Shape for Quad {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        let ray = &hit_record.ray;
        let denominator = self.normal.dot(ray.direction);
        let t = (self.d - self.normal.dot(ray.origin)) / denominator;
        if !ray.interval.contains(t) {
            return false;
        }
        let intersection = ray.at(t);
        let hit_point = intersection - self.q;
        let alpha = self.w.dot(hit_point * self.v);
        let beta = self.w.dot(self.u * hit_point);
        if Interval::UNIT.contains(alpha) && Interval::UNIT.contains(beta) {
            hit_record.set_hit(t, self.normal, UV::new(alpha, beta));
            return true;
        }
        false
    }

    fn aabb(&self) -> AABB {
        AABB::union(
            AABB::from_vec3(self.q, self.q + self.u + self.v),
            AABB::from_vec3(self.q + self.u, self.q + self.v),
        )
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
