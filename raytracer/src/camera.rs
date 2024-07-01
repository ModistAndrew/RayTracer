use crate::color::Color;
use rand::Rng;

use crate::ray::Ray;
use crate::vec3d::Vec3d;

pub struct Camera {
    origin: Vec3d,
    viewport_upper_left: Vec3d,
    viewport_u: Vec3d,
    viewport_v: Vec3d,
    pixel_width_ratio: f64,
    pixel_height_ratio: f64,
    sample_per_pixel: u32,
    color: Color,
}

impl Camera {
    pub fn new(
        origin: Vec3d,
        focal_length: f64,
        viewport_width: f64,
        viewport_height: f64,
        resolution_width: u32,
        resolution_height: u32,
        sample_per_pixel: u32,
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
            pixel_width_ratio: 1.0 / resolution_width as f64,
            pixel_height_ratio: 1.0 / resolution_height as f64,
            sample_per_pixel,
            color: Color::WHITE,
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.viewport_upper_left + self.viewport_u * u + self.viewport_v * v - self.origin,
            self.color,
        )
    }

    pub fn get_rays_at(&self, i: u32, j: u32) -> Vec<Ray> {
        let mut rays = Vec::with_capacity(self.sample_per_pixel as usize);
        let mut rng = rand::thread_rng();
        for _ in 0..self.sample_per_pixel {
            let u = (i as f64 + rng.gen_range(-0.5..0.5)) * self.pixel_width_ratio;
            let v = (j as f64 + rng.gen_range(-0.5..0.5)) * self.pixel_height_ratio;
            rays.push(self.get_ray(u, v));
        }
        rays
    }
}
