use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::vec3d::Vec3d;


pub struct Hit {
    pub t: f64,
    pub position: Vec3d, // the hit position
    pub normal: Vec3d,    // always normalized and points opposite to the ray
    pub front_face: bool, // whether outside the object
}

pub struct HitRecord {
    pub ray: Ray, // the original ray
    pub hit: Option<Hit>,
    pub scatter: Option<Ray>,
}

impl HitRecord {
    // outward_normal should be normalized
    pub fn new(ray: Ray) -> Self {
        Self {
            ray,
            hit: None,
            scatter: None,
        }
    }

    pub fn set_hit(&mut self, t: f64, outward_normal: Vec3d) {
        let front_face = self.ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        let position = self.ray.at(t);
        self.hit = Some(Hit {
            t,
            position,
            normal,
            front_face,
        });
    }

    pub fn set_scatter(&mut self, scatter: Ray) {
        self.scatter = Some(scatter);
    }

    pub fn does_hit(&self) -> bool {
        self.hit.is_some()
    }

    pub fn get_hit(&self) -> &Hit {
        self.hit.as_ref().unwrap()
    }

    pub fn get_scatter(&self) -> &Ray {
        self.scatter.as_ref().unwrap()
    }
}

pub trait Hittable {
    fn hit(&self, hit_record: &mut HitRecord, interval: Interval) -> bool;
}

pub struct Object {
    pub shape: Box<dyn Shape>,
    pub material: Box<dyn Material>,
}

impl Object {
    pub fn new(shape: Box<dyn Shape>, material: Box<dyn Material>) -> Self {
        Self { shape, material }
    }
}

impl Hittable for Object {
    fn hit(&self, hit_record: &mut HitRecord, interval: Interval) -> bool {
        self.shape.hit(hit_record, interval) && {
            self.material.scatter(hit_record);
            true
        }
    }
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, hit_record: &mut HitRecord, interval: Interval) -> bool {
        let mut hit_anything = false;
        let mut closest = interval.max;
        for object in &self.objects {
            if object.hit(hit_record, Interval::new(interval.min, closest)) {
                closest = hit_record.get_hit().t;
                hit_anything = true;
            }
        }
        hit_anything
    }
}
