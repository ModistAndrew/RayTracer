use crate::ray::Ray;
use crate::vec3d::Vec3d;

pub struct Camera {
    origin: Vec3d,
    viewport_upper_left: Vec3d,
    viewport_u: Vec3d,
    viewport_v: Vec3d,
}

impl Camera {
    pub fn new(
        origin: Vec3d,
        focal_length: f64,
        viewport_width: f64,
        viewport_height: f64,
    ) -> Self {
        let viewport_u = Vec3d::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3d::new(0.0, -viewport_height, 0.0);
        let viewport_upper_left =
            origin - viewport_u / 2.0 - viewport_v / 2.0 - Vec3d::new(0.0, 0.0, focal_length);
        Self {
            origin,
            viewport_upper_left,
            viewport_u,
            viewport_v,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.viewport_upper_left + self.viewport_u * u + self.viewport_v * v - self.origin,
        )
    }
}
