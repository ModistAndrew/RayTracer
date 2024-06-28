use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::hittable::{Hittable, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3d::Vec3d;
use indicatif::ProgressBar;

pub struct RayTracer {
    camera: Camera,
    canvas: Canvas,
    hittable_list: HittableList,
    max_depth: u32,
}

impl RayTracer {
    pub fn new(
        camera: Camera,
        canvas: Canvas,
        hittable_list: HittableList,
        max_depth: u32,
    ) -> Self {
        Self {
            camera,
            canvas,
            hittable_list,
            max_depth,
        }
    }

    fn raytrace(&self, ray: &Ray, depth: u32) -> Color {
        if depth >= self.max_depth {
            return Color::new(0.0, 0.0, 0.0);
        }
        if let Some(hit_record) = self
            .hittable_list
            .hit(ray, Interval::new(0.001, f64::INFINITY))
        {
            let direction = hit_record.normal + Vec3d::random_unit_vector();
            return self
                .raytrace(&Ray::new(hit_record.position, direction), depth + 1)
                .darken(0.5);
        }
        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0 - 0.5 * a, 1.0 - 0.3 * a, 1.0)
    }

    pub fn render(&mut self, show_progress: bool) {
        let width = self.canvas.width();
        let height = self.canvas.height();
        let progress = if show_progress {
            ProgressBar::new((height * width) as u64)
        } else {
            ProgressBar::hidden()
        };
        for i in 0..width {
            for j in 0..height {
                let color = Color::mix(
                    &self
                        .camera
                        .get_rays_at(i, j)
                        .iter()
                        .map(|ray| self.raytrace(ray, 0))
                        .collect::<Vec<Color>>(),
                );
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
