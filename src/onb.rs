use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Onb {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Onb {
    pub fn normal(n: Vec3) -> Self {
        let w = n.normalize();
        let a = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let u = (a * w).normalize();
        let v = w * u;
        Onb { u, v, w }
    }

    pub fn normal_with_up(n: Vec3, up: Vec3) -> Self {
        let w = n.normalize();
        let u = (up * w).normalize();
        let v = w * u;
        Onb { u, v, w }
    }

    pub fn normal_with_tangent(n: Vec3, t: Vec3) -> Self {
        let w = n.normalize();
        let u = t.normalize();
        let v = w * u;
        Onb { u, v, w }
    }

    pub fn local(&self, a: Vec3) -> Vec3 {
        self.u * a.x + self.v * a.y + self.w * a.z
    }
}
