use indicatif::ProgressBar;

use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::color::{BlendMode, Color};
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;

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

    fn raytrace(&self, ray: Ray, depth: u32) -> Color {
        if depth >= self.max_depth {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut hit_record = HitRecord::new(ray);
        if self.hittable_list.hit(&mut hit_record, Interval::new(0.001, f64::INFINITY)) {
            self.raytrace(hit_record.scatter.unwrap(), depth + 1)
        } else {
            let unit_direction = hit_record.ray.direction.normalize();
            let a = 0.5 * (unit_direction.y + 1.0);
            Color::new(1.0 - 0.5 * a, 1.0 - 0.3 * a, 1.0).blend(hit_record.ray.color, BlendMode::Mul)
        }
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
                        .into_iter()
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
