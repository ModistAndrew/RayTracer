use std::collections::HashMap;

use crate::aabb::Aabb;
use crate::bvh::ShapeTree;
use crate::hit_record::HitRecord;
use crate::shape::Shape;
use crate::texture::{Atlas, UV};
use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Triangle {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    tq: UV,
    tu: UV,
    tv: UV,
    w: Vec3,
    normal: Vec3,
    nq: Vec3,
    nu: Vec3,
    nv: Vec3,
    d: f64,
}

impl Triangle {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, tq: UV, tu: UV, tv: UV, nq: Vec3, nu: Vec3, nv: Vec3) -> Self {
        let n = u * v;
        let normal = n.normalize();
        let d = normal.dot(q);
        let w = n / n.length_squared();
        Self {
            q,
            u,
            v,
            tq,
            tu,
            tv,
            w,
            normal,
            nq,
            nu,
            nv,
            d,
        }
    }

    pub fn vertex(a: Vec3, b: Vec3, c: Vec3, ta: UV, tb: UV, tc: UV, na: Vec3, nb: Vec3, nc: Vec3) -> Self {
        let q = a;
        let u = b - a;
        let v = c - a;
        let tq = ta;
        let tu = tb - ta;
        let tv = tc - ta;
        let nq = na;
        let nu = nb - na;
        let nv = nc - na;
        if nq.dot(u * v) > 0.0 {
            Triangle::new(q, u, v, tq, tu, tv, nq, nu, nv)
        } else {
            Triangle::new(q, v, u, tq, tv, tu, nq, nv, nu)
        }
    }

    pub fn set_single_uv(&mut self, uv: UV) {
        self.tq = uv;
        self.tu = UV::default();
        self.tv = UV::default();
    }
}

impl Shape for Triangle {
    fn hit(&self, hit_record: &mut HitRecord, atlas: &Atlas) -> bool {
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
            return hit_record.set_hit(
                t,
                self.nq + self.nu * alpha + self.nv * beta,
                self.tq + self.tu * alpha + self.tv * beta,
                atlas,
            );
        }
        false
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::union(
            Aabb::from_vec3(self.q, self.q + self.u),
            Aabb::from_vec3(self.q, self.q + self.v),
        )
    }
}

pub struct Mesh {
    pub shapes: HashMap<String, Vec<Triangle>>,
}

impl Mesh {
    pub fn load_obj(path: &str) -> Self {
        let obj = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS);
        let (models, _materials) = obj.unwrap();
        let mut map = HashMap::default();
        for model in models {
            let mut triangles = Vec::default();
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
                let na = Vec3::new(
                    m.normals[i[0] as usize * 3],
                    m.normals[i[0] as usize * 3 + 1],
                    m.normals[i[0] as usize * 3 + 2],
                );
                let nb = Vec3::new(
                    m.normals[i[1] as usize * 3],
                    m.normals[i[1] as usize * 3 + 1],
                    m.normals[i[1] as usize * 3 + 2],
                );
                let nc = Vec3::new(
                    m.normals[i[2] as usize * 3],
                    m.normals[i[2] as usize * 3 + 1],
                    m.normals[i[2] as usize * 3 + 2],
                );
                let triangle = Triangle::vertex(a, b, c, ta, tb, tc, na, nb, nc);
                triangles.push(triangle);
            });
            map.insert(model.name, triangles);
        }
        Self { shapes: map }
    }

    pub fn remove_shape(&mut self, key: &str) -> ShapeTree {
        let vec = self.shapes.remove(key).unwrap();
        ShapeTree::new(vec.into_iter().map(|t| Box::new(t) as Box<dyn Shape>).collect())
    }

    pub fn get_names(&self) -> Vec<&String> {
        self.shapes.keys().collect()
    }
}
