use crate::color::Color;
use rand::Rng;

use crate::ray::Ray;
use crate::vec3d::Vec3d;

pub struct PerspectiveParam {
    pub look_from: Vec3d,
    pub look_at: Vec3d,
    pub view_up: Vec3d,
}

pub struct LensParam {
    pub fov: f64,
    pub filter: Color,
}

pub struct ImageParam {
    pub image_width: u32,
    pub image_height: u32,
    pub sample_per_pixel: u32,
}

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
        perspective_param: PerspectiveParam,
        lens_param: LensParam,
        canvas_param: ImageParam,
    ) -> Self {
        let direction = perspective_param.look_from - perspective_param.look_at;
        let focal_length = direction.length();
        let w = direction.normalize();
        let u = (perspective_param.view_up * w).normalize();
        let v = w * u;

        let viewport_height = 2.0 * (lens_param.fov.to_radians() / 2.0).tan() * focal_length;
        let viewport_width =
            viewport_height * canvas_param.image_width as f64 / canvas_param.image_height as f64;

        let pixel_width_ratio = viewport_width / canvas_param.image_width as f64;
        let pixel_height_ratio = viewport_height / canvas_param.image_height as f64;
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        let viewport_upper_left =
            perspective_param.look_from - focal_length * w - viewport_u / 2.0 - viewport_v / 2.0;
        Self {
            origin: perspective_param.look_from,
            viewport_upper_left,
            viewport_u,
            viewport_v,
            pixel_width_ratio,
            pixel_height_ratio,
            sample_per_pixel: canvas_param.sample_per_pixel,
            color: lens_param.filter,
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
