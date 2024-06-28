use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::hittable::{Hittable, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;
use indicatif::ProgressBar;

pub struct RayTracer {
    camera: Camera,
    canvas: Canvas,
    hittable_list: HittableList,
}

impl RayTracer {
    pub fn new(camera: Camera, canvas: Canvas, hittable_list: HittableList) -> Self {
        Self {
            camera,
            canvas,
            hittable_list,
        }
    }

    fn ray_color(&self, ray: &Ray) -> Color {
        if self
            .hittable_list
            .hit(ray, Interval::new(0.0, f64::INFINITY))
            .is_some()
        {
            return Color::new(1.0, 0.0, 0.0);
        }
        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0 - 0.5 * a, 1.0 - 0.3 * a, 1.0)
    }

    pub fn render(&mut self, show_progress: bool) {
        let width = self.canvas.width();
        let height = self.canvas.height();
        let pixel_width = 1.0 / width as f64;
        let pixel_height = 1.0 / height as f64;
        let progress = if show_progress {
            ProgressBar::new((height * width) as u64)
        } else {
            ProgressBar::hidden()
        };
        for i in 0..width {
            for j in 0..height {
                let u = (i as f64 + 0.5) * pixel_width;
                let v = (j as f64 + 0.5) * pixel_height;
                let ray = self.camera.get_ray(u, v);
                let color = self.ray_color(&ray);
                self.canvas.write(i, j, color);
                progress.inc(1);
            }
        }
        progress.finish();
    }

    pub fn save(&self, path: &str) {
        self.canvas.save(path);
    }
}
