use rand::Rng;

use crate::onb::ONB;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct PerspectiveParam {
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub view_up: Vec3,
}

pub struct LensParam {
    pub fov: f64,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

pub struct ImageParam {
    pub image_width: u32,
    pub image_height: u32,
    pub sample_per_pixel: u32,
}

pub struct Camera {
    origin: Vec3,
    viewport_upper_left: Vec3,
    viewport_u: Vec3,
    viewport_v: Vec3,
    pixel_width_ratio: f64,
    pixel_height_ratio: f64,
    sqrt_spp: u32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        perspective_param: PerspectiveParam,
        lens_param: LensParam,
        canvas_param: ImageParam,
    ) -> Self {
        let uvw = ONB::normal_with_up(
            perspective_param.look_from - perspective_param.look_at,
            perspective_param.view_up,
        );

        let viewport_height =
            2.0 * (lens_param.fov.to_radians() / 2.0).tan() * lens_param.focus_dist;
        let viewport_width =
            viewport_height * canvas_param.image_width as f64 / canvas_param.image_height as f64;

        let pixel_width_ratio = 1.0 / canvas_param.image_width as f64;
        let pixel_height_ratio = 1.0 / canvas_param.image_height as f64;
        let viewport_u = uvw.local(Vec3::new(viewport_width, 0.0, 0.0));
        let viewport_v = uvw.local(Vec3::new(0.0, -viewport_height, 0.0));
        let viewport_upper_left = perspective_param.look_from
            - uvw.local(Vec3::new(
                viewport_width / 2.0,
                -viewport_height / 2.0,
                lens_param.focus_dist,
            ));

        let defocus_radius =
            lens_param.focus_dist * (lens_param.defocus_angle.to_radians() / 2.0).tan();
        let defocus_disk_u = uvw.local(Vec3::new(defocus_radius, 0.0, 0.0));
        let defocus_disk_v = uvw.local(Vec3::new(0.0, defocus_radius, 0.0));
        let sample_per_pixel = canvas_param.sample_per_pixel;
        let sqrt_spp = (sample_per_pixel as f64).sqrt() as u32;
        Self {
            origin: perspective_param.look_from,
            viewport_upper_left,
            viewport_u,
            viewport_v,
            pixel_width_ratio,
            pixel_height_ratio,
            sqrt_spp,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        let origin = self.defocus_disk_sample();
        let direction =
            self.viewport_upper_left + self.viewport_u * u + self.viewport_v * v - origin;
        Ray::new(origin, direction)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.origin + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }

    pub fn get_ray_at(&self, i: u32, j: u32, si: u32, sj: u32) -> Ray {
        let mut rng = rand::thread_rng();
        let u = (i as f64 + (si as f64 + rng.gen::<f64>()) / self.sqrt_spp as f64 - 0.5)
            * self.pixel_width_ratio;
        let v = (j as f64 + (sj as f64 + rng.gen::<f64>()) / self.sqrt_spp as f64 - 0.5)
            * self.pixel_height_ratio;
        self.get_ray(u, v)
    }

    pub fn sqrt_spp(&self) -> u32 {
        self.sqrt_spp
    }
}
