use std::f64::consts::PI;
use std::fmt::Debug;

use crate::aabb::AABB;
use crate::bvh::ShapeList;
use crate::hittable::HitRecord;
use crate::interval::Interval;
use crate::onb::ONB;
use crate::ray::Ray;
use crate::texture::UV;
use crate::transform::Transform;
use crate::vec3::Vec3;

pub trait Shape: Sync + Send {
    // hit_record.ray is the original ray. (may contain the former hit info)
    // if hit, set hit info before returning true
    fn hit(&self, hit_record: &mut HitRecord) -> bool;

    // should return a new shape transformed by the matrix. you may call aabb() on the new shape to get the new bounding box
    // may not be able to handle all transformations
    // remember to update member variables
    fn transform(&mut self, matrix: Transform);

    // return the bounding box for hit testing. only called once for construction
    fn aabb(&self) -> AABB;
}

pub trait ShapePDFProvider: Sync + Send + Debug {
    // similar to PDF but we specify the origin
    fn prob(&self, origin: Vec3, direction: Vec3) -> f64;
    fn generate(&self, origin: Vec3) -> Vec3;
}

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }

    fn uv_from_normal(p: Vec3) -> UV {
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
        hit_record.set_hit(root, outward_normal, Self::uv_from_normal(outward_normal));
        true
    }

    fn transform(&mut self, matrix: Transform) {
        self.center = matrix.pos(self.center);
        // make sure the transformed radius is the same in all directions as we can't handle ellipsoids
        self.radius = matrix.direction(Vec3::new(self.radius, 0.0, 0.0)).length();
    }

    fn aabb(&self) -> AABB {
        let r_vec = Vec3::new(self.radius, self.radius, self.radius);
        AABB::from_vec3(self.center - r_vec, self.center + r_vec)
    }
}

impl ShapePDFProvider for Sphere {
    fn prob(&self, origin: Vec3, direction: Vec3) -> f64 {
        let ray = Ray::new(origin, direction, 0.0, Interval::UNIVERSE);
        let mut hit_record = HitRecord::new(ray);
        if !self.hit(&mut hit_record) {
            return 0.0;
        }
        let cos_theta_max =
            (1.0 - self.radius * self.radius / (self.center - origin).length_squared()).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
        1.0 / solid_angle
    }

    fn generate(&self, origin: Vec3) -> Vec3 {
        let direction = self.center - origin;
        let distance_squared = direction.length_squared();
        let uvw = ONB::normal(direction);
        uvw.local(Vec3::random_to_sphere(self.radius, distance_squared))
    }
}

#[derive(Debug)]
pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: Vec3,
    d: f64,
    area: f64,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3) -> Self {
        let n = u * v;
        let normal = n.normalize();
        let d = normal.dot(q);
        let w = n / n.length_squared();
        let area = n.length();
        Self {
            q,
            u,
            v,
            w,
            normal,
            d,
            area,
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
        let planar_hit_pos = intersection - self.q;
        let alpha = self.w.dot(planar_hit_pos * self.v);
        let beta = self.w.dot(self.u * planar_hit_pos);
        if Interval::UNIT.contains(alpha) && Interval::UNIT.contains(beta) {
            hit_record.set_hit(t, self.normal, UV::new(alpha, beta));
            return true;
        }
        false
    }

    fn transform(&mut self, matrix: Transform) {
        self.q = matrix.pos(self.q);
        self.u = matrix.direction(self.u);
        self.v = matrix.direction(self.v);
        let n = self.u * self.v;
        self.normal = n.normalize();
        self.d = self.normal.dot(self.q);
        self.w = n / n.length_squared();
        self.area = n.length();
    }

    fn aabb(&self) -> AABB {
        AABB::union(
            AABB::from_vec3(self.q, self.q + self.u + self.v),
            AABB::from_vec3(self.q + self.u, self.q + self.v),
        )
    }
}

impl ShapePDFProvider for Quad {
    fn prob(&self, origin: Vec3, direction: Vec3) -> f64 {
        let ray = Ray::new(origin, direction, 0.0, Interval::UNIVERSE);
        let mut hit_record = HitRecord::new(ray);
        if !self.hit(&mut hit_record) {
            return 0.0;
        }
        let hit = hit_record.get_hit();
        let distance_squared = hit.t * hit.t * direction.length_squared();
        let cosine = -direction.dot(hit.normal) / direction.length();
        distance_squared / (cosine * self.area)
    }

    fn generate(&self, origin: Vec3) -> Vec3 {
        self.q + self.u * rand::random::<f64>() + self.v * rand::random::<f64>() - origin
    }
}

impl ShapeList {
    pub fn cube(a: Vec3, b: Vec3) -> ShapeList {
        let aabb = AABB::from_vec3(a, b);
        let min_pos = aabb.min_pos();
        let max_pos = aabb.max_pos();
        let dx = Vec3::new(aabb.x.length(), 0.0, 0.0);
        let dy = Vec3::new(0.0, aabb.y.length(), 0.0);
        let dz = Vec3::new(0.0, 0.0, aabb.z.length());
        let mut quads = ShapeList::default();
        quads.push(Quad::new(
            Vec3::new(min_pos.x, min_pos.y, max_pos.z),
            dx,
            dy,
        ));
        quads.push(Quad::new(
            Vec3::new(max_pos.x, min_pos.y, max_pos.z),
            -dz,
            dy,
        ));
        quads.push(Quad::new(
            Vec3::new(max_pos.x, min_pos.y, min_pos.z),
            -dx,
            dy,
        ));
        quads.push(Quad::new(
            Vec3::new(min_pos.x, min_pos.y, min_pos.z),
            dz,
            dy,
        ));
        quads.push(Quad::new(
            Vec3::new(min_pos.x, max_pos.y, max_pos.z),
            dx,
            -dz,
        ));
        quads.push(Quad::new(
            Vec3::new(min_pos.x, min_pos.y, min_pos.z),
            dx,
            dz,
        ));
        quads
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

    fn transform(&mut self, matrix: Transform) {
        self.direction = matrix.direction(self.direction);
        self.shape.transform(matrix);
    }

    fn aabb(&self) -> AABB {
        let stationary_aabb = self.shape.aabb();
        stationary_aabb.union(stationary_aabb + self.direction)
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
            Interval::POSITIVE + t1,
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

    fn transform(&mut self, matrix: Transform) {
        self.boundary.transform(matrix);
    }

    fn aabb(&self) -> AABB {
        self.boundary.aabb()
    }
}
