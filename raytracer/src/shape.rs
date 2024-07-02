use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;

pub trait Shape: Sync + Send {
    // hit_record.ray is the original ray. may contain the former hit record. if hit, update hit_record.hit and ray and return true
    fn hit(&self, hit_record: &mut HitRecord) -> bool;
    fn aabb(&self) -> AABB;
}

pub struct BoundingBox {
    center: Vec3,
    aabb: AABB,
    direction: Option<Vec3>, // present if the shape is moving
}

impl BoundingBox {
    pub fn new(center: Vec3, stationary_aabb: AABB, direction: Option<Vec3>) -> Self {
        Self {
            center,
            aabb: if let Some(direction) = direction {
                stationary_aabb.union(stationary_aabb.moved(direction))
            } else {
                stationary_aabb
            },
            direction,
        }
    }

    pub fn center_at(&self, time: f64) -> Vec3 {
        match self.direction {
            Some(direction) => self.center + direction * time,
            None => self.center,
        }
    }

    pub fn aabb(&self) -> AABB {
        self.aabb
    }
}

pub struct Sphere {
    position: BoundingBox,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, direction: Option<Vec3>) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        Self {
            position: BoundingBox::new(
                center,
                AABB::from_vec3(center - r_vec, center + r_vec),
                direction,
            ),
            radius,
        }
    }
}

impl Shape for Sphere {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        let ray = &hit_record.ray;
        let interval = ray.interval;
        let center = self.position.center_at(ray.time);
        let oc = center - ray.origin;
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
        let outward_normal = (ray.at(root) - center) / self.radius;
        hit_record.set_hit(root, outward_normal);
        true
    }

    fn aabb(&self) -> AABB {
        self.position.aabb()
    }
}
