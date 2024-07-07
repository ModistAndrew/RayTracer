use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::interval::Interval;
use crate::ray::Ray;
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
        let position = ray.at(root);
        let outward_normal = (position - self.center) / self.radius;
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
        let planar_hit_point = intersection - self.q;
        let alpha = self.w.dot(planar_hit_point * self.v);
        let beta = self.w.dot(self.u * planar_hit_point);
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

pub struct Cube {
    aabb: AABB,
    quads: [Quad; 6],
}

impl Cube {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        let aabb = AABB::from_vec3(a, b);
        let min_point = aabb.min_point();
        let max_point = aabb.max_point();
        let dx = Vec3::new(aabb.x.length(), 0.0, 0.0);
        let dy = Vec3::new(0.0, aabb.y.length(), 0.0);
        let dz = Vec3::new(0.0, 0.0, aabb.z.length());
        let quads = [
            Quad::new(Vec3::new(min_point.x, min_point.y, max_point.z), dx, dy),
            Quad::new(Vec3::new(max_point.x, min_point.y, max_point.z), -dz, dy),
            Quad::new(Vec3::new(max_point.x, min_point.y, min_point.z), -dx, dy),
            Quad::new(Vec3::new(min_point.x, min_point.y, min_point.z), dz, dy),
            Quad::new(Vec3::new(min_point.x, max_point.y, max_point.z), dx, -dz),
            Quad::new(Vec3::new(min_point.x, min_point.y, min_point.z), dx, dz),
        ];
        Self { aabb, quads }
    }
}

impl Shape for Cube {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        let mut hit = false;
        for quad in &self.quads {
            hit |= quad.hit(hit_record);
        }
        hit
    }

    fn aabb(&self) -> AABB {
        self.aabb
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

pub struct Translate<T: Shape> {
    offset: Vec3,
    shape: T,
}

impl<T: Shape> Translate<T> {
    pub fn new(offset: Vec3, shape: T) -> Self {
        Self { offset, shape }
    }
}

impl<T: Shape> Shape for Translate<T> {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        hit_record.ray.origin -= self.offset;
        let hit = self.shape.hit(hit_record);
        hit_record.ray.origin += self.offset;
        if hit {
            hit_record.get_hit_mut().position += self.offset;
        }
        hit
    }
    fn aabb(&self) -> AABB {
        self.shape.aabb() + self.offset
    }
}

pub struct RotationY<T: Shape> {
    radians: f64,
    shape: T,
}

impl<T: Shape> RotationY<T> {
    pub fn new(angle: f64, shape: T) -> Self {
        let radians = angle.to_radians();
        Self { radians, shape }
    }
}

impl<T: Shape> Shape for RotationY<T> {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        hit_record.ray.origin = hit_record.ray.origin.rotate_y(-self.radians);
        hit_record.ray.direction = hit_record.ray.direction.rotate_y(-self.radians);
        let hit = self.shape.hit(hit_record);
        hit_record.ray.origin = hit_record.ray.origin.rotate_y(self.radians);
        hit_record.ray.direction = hit_record.ray.direction.rotate_y(self.radians);
        if hit {
            let mut hit_mut = hit_record.get_hit_mut();
            hit_mut.position = hit_mut.position.rotate_y(self.radians);
            hit_mut.normal = hit_mut.normal.rotate_y(self.radians);
        }
        hit
    }

    fn aabb(&self) -> AABB {
        self.shape.aabb().rotate_y(self.radians)
    }
}

pub struct ConstantMedium<T: Shape> {
    neg_inv_density: f64,
    boundary: T,
}

impl<T: Shape> ConstantMedium<T> {
    pub fn new(density: f64, boundary: T) -> Self {
        Self {
            neg_inv_density: -1.0 / density,
            boundary,
        }
    }
}

impl<T: Shape> Shape for ConstantMedium<T> {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        let ray = &hit_record.ray;
        let mut rec1 = HitRecord::new(Ray::new(
            ray.origin,
            ray.direction,
            ray.time,
            Interval::UNIVERSE,
        ));
        if !self.boundary.hit(&mut rec1) {
            return false;
        }
        let t1 = rec1.get_hit().t;
        let mut rec2 = HitRecord::new(Ray::new(
            ray.origin,
            ray.direction,
            ray.time,
            Interval::new(t1 + 0.0001, f64::INFINITY),
        ));
        if !self.boundary.hit(&mut rec2) {
            return false;
        }
        let t2 = rec2.get_hit().t;
        let interval = Interval::new(t1, t2).intersect(ray.interval);
        if interval.empty() {
            return false;
        }
        let ray_length = hit_record.ray.direction.length();
        let hit_distance = self.neg_inv_density * rand::random::<f64>().ln();
        let t = interval.min + hit_distance / ray_length;
        if !interval.surrounds(t) {
            return false;
        }
        hit_record.set_hit(t, Vec3::default(), UV::default());
        true
    }

    fn aabb(&self) -> AABB {
        self.boundary.aabb()
    }
}
