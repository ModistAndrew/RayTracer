use std::collections::HashMap;

use crate::aabb::AABB;
use crate::bvh::{ShapeList, ShapeTree};
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::material::{Lambertian, Material};
use crate::shape::Shape;
use crate::texture::{Texture, UV};
use crate::transform::Transform;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Triangle {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    tq: UV,
    tu: UV,
    tv: UV,
    w: Vec3,
    normal: Vec3,
    d: f64,
    area: f64,
}

impl Triangle {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, tq: UV, tu: UV, tv: UV) -> Self {
        let n = u * v;
        let normal = n.normalize();
        let d = normal.dot(q);
        let w = n / n.length_squared();
        let area = n.length() / 2.0;
        Self {
            q,
            u,
            v,
            tq,
            tu,
            tv,
            w,
            normal,
            d,
            area,
        }
    }

    pub fn vertex(a: Vec3, b: Vec3, c: Vec3, ta: UV, tb: UV, tc: UV, normal: Vec3) -> Self {
        let q = a;
        let u = b - a;
        let v = c - a;
        let tq = ta;
        let tu = tb - ta;
        let tv = tc - ta;
        if normal.dot(u * v) > 0.0 {
            Triangle::new(q, u, v, tq, tu, tv)
        } else {
            Triangle::new(q, v, u, tq, tv, tu)
        }
    }
}

impl Shape for Triangle {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        let ray = hit_record.get_ray();
        let denominator = self.normal.dot(ray.direction);
        let t = (self.d - self.normal.dot(ray.origin)) / denominator;
        if !hit_record.get_interval().contains(t) {
            return false;
        }
        let intersection = ray.at(t);
        let planar_hit_pos = intersection - self.q;
        let alpha = self.w.dot(planar_hit_pos * self.v);
        let beta = self.w.dot(self.u * planar_hit_pos);
        if alpha >= 0.0 && beta >= 0.0 && alpha + beta <= 1.0 {
            hit_record.set_hit(t, self.normal, self.tq + self.tu * alpha + self.tv * beta);
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
        self.area = n.length() / 2.0;
    }

    fn aabb(&self) -> AABB {
        AABB::union(
            AABB::from_vec3(self.q, self.q + self.u + self.v),
            AABB::from_vec3(self.q + self.u, self.q + self.v),
        )
    }
}

#[derive(Default)]
pub struct TextureMap {
    transparency: Option<Box<dyn Texture>>,
    albedo: Option<Box<dyn Texture>>,
}

impl TextureMap {
    pub fn set_transparency<T: Texture + 'static>(&mut self, texture: T) {
        self.transparency = Some(Box::new(texture));
    }

    pub fn set_albedo<T: Texture + 'static>(&mut self, texture: T) {
        self.albedo = Some(Box::new(texture));
    }

    pub fn skip_render(&self, hit_record: &HitRecord) -> bool {
        self.transparency
            .as_ref()
            .map_or(false, |t| t.value(hit_record).r < 0.5)
    }

    pub fn get_color(&self, hit_record: &HitRecord) -> Color {
        self.albedo
            .as_ref()
            .map_or(Color::WHITE, |a| a.value(hit_record))
    }
}

pub struct MeshObject {
    triangles: ShapeTree,
    material: Box<dyn Material>,
    textures: TextureMap,
}

impl MeshObject {
    pub fn new(shape_list: ShapeList) -> Self {
        Self {
            triangles: shape_list.tree(),
            material: Box::new(Lambertian),
            textures: TextureMap::default(),
        }
    }

    pub fn set_material<M: Material + 'static>(&mut self, material: M) {
        self.material = Box::new(material);
    }

    pub fn get_textures_mut(&mut self) -> &mut TextureMap {
        &mut self.textures
    }

    pub fn from_obj(path: &str) -> HashMap<String, MeshObject> {
        let obj = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS);
        let (models, _materials) = obj.unwrap();
        let mut ret = HashMap::default();
        for model in models {
            let mut traingles = ShapeList::default();
            let m = model.mesh;
            m.indices.chunks(3).for_each(|i| {
                let a = Vec3::new(
                    m.positions[i[0] as usize * 3],
                    m.positions[i[0] as usize * 3 + 1],
                    m.positions[i[0] as usize * 3 + 2],
                );
                let b = Vec3::new(
                    m.positions[i[1] as usize * 3],
                    m.positions[i[1] as usize * 3 + 1],
                    m.positions[i[1] as usize * 3 + 2],
                );
                let c = Vec3::new(
                    m.positions[i[2] as usize * 3],
                    m.positions[i[2] as usize * 3 + 1],
                    m.positions[i[2] as usize * 3 + 2],
                );
                let ta = UV::new(
                    m.texcoords[i[0] as usize * 2],
                    m.texcoords[i[0] as usize * 2 + 1],
                );
                let tb = UV::new(
                    m.texcoords[i[1] as usize * 2],
                    m.texcoords[i[1] as usize * 2 + 1],
                );
                let tc = UV::new(
                    m.texcoords[i[2] as usize * 2],
                    m.texcoords[i[2] as usize * 2 + 1],
                );
                let normal = Vec3::new(
                    m.normals[i[0] as usize * 3],
                    m.normals[i[0] as usize * 3 + 1],
                    m.normals[i[0] as usize * 3 + 2],
                ); // simply use the first normal. three normals are expected to be the same
                let triangle = Triangle::vertex(a, b, c, ta, tb, tc, normal);
                traingles.push(triangle);
            });
            ret.insert(model.name, MeshObject::new(traingles));
        }
        ret
    }
}

impl Hittable for MeshObject {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        if !self.triangles.hit(hit_record) {
            return false;
        }
        if self.textures.skip_render(hit_record) {
            hit_record.set_scatter_pass();
            return true;
        }
        self.material.scatter(hit_record);
        hit_record.get_hit_mut().attenuation = self.textures.get_color(hit_record);
        true
    }

    fn aabb(&self) -> AABB {
        self.triangles.aabb()
    }
}
