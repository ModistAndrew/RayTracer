use indicatif::ProgressBar;
use std::sync::Arc;
use std::thread;

use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::color::{BlendMode, Color};
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::ray::Ray;

pub struct RayTracer {
    camera: Camera,
    canvas: Canvas,
    world: Arc<HittableList>, // Arc is used to share the world between threads
    max_depth: u32,
}

impl RayTracer {
    pub fn new(camera: Camera, canvas: Canvas, world: HittableList, max_depth: u32) -> Self {
        Self {
            camera,
            canvas,
            world: Arc::new(world),
            max_depth,
        }
    }

    fn raytrace(world: &Arc<HittableList>, ray: Ray, left_depth: u32) -> Color {
        if left_depth == 0 || ray.color.is_black() {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut hit_record = HitRecord::new(ray);
        if world.hit(&mut hit_record) {
            Self::raytrace(world, hit_record.scatter.unwrap(), left_depth - 1)
        } else {
            let ray = hit_record.ray; // move the original ray back
            let unit_direction = ray.direction.normalize();
            let a = 0.5 * (unit_direction.y + 1.0);
            let sky_color = Color::new(1.0 - 0.5 * a, 1.0 - 0.3 * a, 1.0);
            sky_color.blend(ray.color, BlendMode::Mul)
        }
    }

    fn render_task(
        input: Vec<Ray>,
        show_progress: bool,
        world: Arc<HittableList>,
        max_depth: u32,
    ) -> Vec<Color> {
        let progress = if show_progress {
            ProgressBar::new(input.len() as u64)
        } else {
            ProgressBar::hidden()
        };
        progress.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{elapsed_precise} {bar:100.cyan/blue} {pos}/{len} {msg}"),
        );
        input
            .into_iter()
            .map(|ray| {
                progress.inc(1);
                Self::raytrace(&world, ray, max_depth)
            })
            .collect()
    }

    pub fn render(&mut self, show_progress: bool) {
        println!("Rendering with multi-threading. The progress bar only shows the progress of the first sample and can be inaccurate.");
        let width = self.canvas.width();
        let height = self.canvas.height();
        let image_size = (width * height) as usize;
        let max_depth = self.max_depth;
        let sample_per_pixel = self.camera.sample_per_pixel();
        let mut threads = Vec::with_capacity(sample_per_pixel as usize);
        for k in 0..sample_per_pixel {
            let world = self.world.clone();
            let mut task = Vec::with_capacity(image_size);
            for i in 0..width {
                for j in 0..height {
                    task.push(self.camera.get_ray_at(i, j));
                }
            }
            threads.push(thread::spawn(move || {
                Self::render_task(task, show_progress && k == 0, world, max_depth)
            }));
        }
        let mut result = vec![Color::BLACK; image_size];
        threads.into_iter().for_each(|thread| {
            let thread_result = thread.join().unwrap();
            for i in 0..image_size {
                result[i] = result[i].blend(thread_result[i], BlendMode::Add);
            }
        });
        println!("Rendering finished. Writing to the canvas.");
        let lighten_factor = 1.0 / sample_per_pixel as f64;
        for i in 0..width {
            for j in 0..height {
                self.canvas.write(
                    i,
                    j,
                    result[(i * height + j) as usize].lighten(lighten_factor),
                );
            }
        }
    }

    pub fn save(&self, path: &str) {
        self.canvas.save(path);
    }
}
