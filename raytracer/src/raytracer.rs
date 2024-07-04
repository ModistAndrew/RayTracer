use std::sync::{Arc, Mutex};
use std::thread;

use crate::bvh::BVHNode;
use indicatif::ProgressBar;

use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::color::{BlendMode, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct RayTracer {
    camera: Camera,
    canvas: Canvas,
    world: BVHNode, // Arc is used to share the world between threads
    max_depth: u32,
}

impl RayTracer {
    pub fn new(camera: Camera, canvas: Canvas, world: BVHNode, max_depth: u32) -> Self {
        Self {
            camera,
            canvas,
            world,
            max_depth,
        }
    }

    fn raytrace(world: &BVHNode, ray: Ray, left_depth: u32) -> Color {
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
        progress_bar: Arc<ProgressBar>,
        raytracer: Arc<Self>,
        output: Arc<Mutex<Vec<Color>>>,
    ) {
        let width = raytracer.canvas.width();
        let height = raytracer.canvas.height();
        let image_size = (width * height) as usize;
        let mut result = Vec::with_capacity(image_size);
        for i in 0..width {
            for j in 0..height {
                let ray = raytracer.camera.get_ray_at(i, j);
                let color = Self::raytrace(&raytracer.world, ray, raytracer.max_depth);
                result.push(color);
            }
        }
        let mut output = output.lock().unwrap();
        for i in 0..image_size {
            output[i].blend_assign(result[i], BlendMode::Add);
        }
        progress_bar.inc(1);
    }

    pub fn render(self) -> Self {
        let raytracer = self;
        let width = raytracer.canvas.width();
        let height = raytracer.canvas.height();
        let image_size = (width * height) as usize;
        let sample_per_pixel = raytracer.camera.sample_per_pixel();
        let mut threads = Vec::with_capacity(sample_per_pixel as usize);
        let progress = ProgressBar::new(sample_per_pixel as u64);
        progress.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{elapsed_precise} {bar:100.cyan/blue} {pos}/{len}"),
        );
        let progress = Arc::new(progress);
        let raytracer = Arc::new(raytracer);
        let output = Arc::new(Mutex::new(vec![Color::BLACK; image_size]));
        for _ in 0..sample_per_pixel {
            let progress_copy = progress.clone();
            let raytracer_copy = raytracer.clone();
            let output_copy = output.clone();
            threads.push(thread::spawn(move || {
                Self::render_task(progress_copy, raytracer_copy, output_copy)
            }));
        }
        // wait for all threads to finish
        threads
            .into_iter()
            .for_each(|thread| thread.join().unwrap());
        // unwrap the Arcs
        let mut raytracer = Arc::into_inner(raytracer).unwrap();
        let output = Arc::into_inner(output).unwrap().into_inner().unwrap();
        let progress = Arc::into_inner(progress).unwrap();
        progress.finish();
        // notice that the color should be darkened as it is accumulated from multiple samples
        let lighten_factor = 1.0 / sample_per_pixel as f64;
        for i in 0..width {
            for j in 0..height {
                raytracer.canvas.write(
                    i,
                    j,
                    output[(i * height + j) as usize].lighten(lighten_factor),
                );
            }
        }
        raytracer
    }

    pub fn save(self, path: &str) -> Self {
        self.canvas.save(path);
        self
    }
}
