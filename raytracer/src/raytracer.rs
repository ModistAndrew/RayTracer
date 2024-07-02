use indicatif::ProgressBar;
use rayon::prelude::*;

use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::color::{BlendMode, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct RayTracer {
    camera: Camera,
    canvas: Canvas,
    world: Box<dyn Hittable>,
    max_depth: u32,
}

impl RayTracer {
    pub fn new(camera: Camera, canvas: Canvas, world: Box<dyn Hittable>, max_depth: u32) -> Self {
        Self {
            camera,
            canvas,
            world,
            max_depth,
        }
    }

    fn raytrace(&self, ray: Ray, depth: u32) -> Color {
        if depth >= self.max_depth || ray.color.is_black() {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut hit_record = HitRecord::new(ray);
        if self.world.hit(&mut hit_record) {
            self.raytrace(hit_record.scatter.unwrap(), depth + 1)
        } else {
            let ray = hit_record.ray; // move the original ray back
            let unit_direction = ray.direction.normalize();
            let a = 0.5 * (unit_direction.y + 1.0);
            let sky_color = Color::new(1.0 - 0.5 * a, 1.0 - 0.3 * a, 1.0);
            sky_color.blend(ray.color, BlendMode::Mul)
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
        progress.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{elapsed_precise} {bar:100.cyan/blue} {pos}/{len} {msg}"),
        );
        for i in 0..width {
            for j in 0..height {
                let color = Color::mix(
                    &self
                        .camera
                        .get_rays_at(i, j)
                        .into_par_iter()
                        .map(|ray| self.raytrace(ray, 0))
                        .collect::<Vec<Color>>(),
                );
                self.canvas.write(i, j, color);
                progress.inc(1);
                progress.set_message(format!("Rendering: {}x{}", i, j));
            }
        }
        progress.finish();
    }

    pub fn save(&self, path: &str) {
        self.canvas.save(path);
    }
}
