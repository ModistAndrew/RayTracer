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
    pub defocus_angle: f64,
    pub focus_dist: f64,
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
    defocus_disk_u: Vec3d,
    defocus_disk_v: Vec3d,
}

impl Camera {
    pub fn new(
        perspective_param: PerspectiveParam,
        lens_param: LensParam,
        canvas_param: ImageParam,
    ) -> Self {
        let w = (perspective_param.look_from - perspective_param.look_at).normalize();
        let u = (perspective_param.view_up * w).normalize();
        let v = w * u;

        let viewport_height =
            2.0 * (lens_param.fov.to_radians() / 2.0).tan() * lens_param.focus_dist;
        let viewport_width =
            viewport_height * canvas_param.image_width as f64 / canvas_param.image_height as f64;

        let pixel_width_ratio = 1.0 / canvas_param.image_width as f64;
        let pixel_height_ratio = 1.0 / canvas_param.image_height as f64;
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        let viewport_upper_left = perspective_param.look_from
            - lens_param.focus_dist * w
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let defocus_radius =
            lens_param.focus_dist * (lens_param.defocus_angle.to_radians() / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;
        Self {
            origin: perspective_param.look_from,
            viewport_upper_left,
            viewport_u,
            viewport_v,
            pixel_width_ratio,
            pixel_height_ratio,
            sample_per_pixel: canvas_param.sample_per_pixel,
            color: lens_param.filter,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        let origin = self.defocus_disk_sample();
        let direction =
            self.viewport_upper_left + self.viewport_u * u + self.viewport_v * v - origin;
        Ray::new(origin, direction, self.color)
    }

    fn defocus_disk_sample(&self) -> Vec3d {
        let p = Vec3d::random_in_unit_disk();
        self.origin + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
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
