use std::sync::{Arc, Mutex};
use std::thread;

use crate::bvh::BVHNode;
use indicatif::ProgressBar;

use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::color::{BlendMode, Color};
use crate::hittable::{HitRecord, HitResult, Hittable};
use crate::ray::Ray;

pub struct RayTracer {
    camera: Camera,
    canvas: Canvas,
    world: BVHNode, // Arc is used to share the world between threads
    max_depth: u32,
    background: Color,
}

impl RayTracer {
    pub fn new(
        camera: Camera,
        canvas: Canvas,
        world: BVHNode,
        max_depth: u32,
        background: Color,
    ) -> Self {
        Self {
            camera,
            canvas,
            world,
            max_depth,
            background,
        }
    }

    fn raytrace(&self, ray: Ray, left_depth: u32) -> Color {
        if left_depth == 0 {
            return Color::BLACK;
        }
        let mut hit_record = HitRecord::new(ray);
        match self.world.hit(&mut hit_record) {
            HitResult::Miss => self.background,
            HitResult::Absorb => hit_record.get_scatter().emission,
            HitResult::Scatter => {
                let emission = hit_record.get_scatter().emission;
                let attenuation = hit_record.get_scatter().attenuation;
                self.raytrace(hit_record.get_output(), left_depth - 1)
                    .blend(attenuation, BlendMode::Mul)
                    .blend(emission, BlendMode::Add)
            }
        }
    }

    fn render_task(
        progress_bar: Arc<ProgressBar>,
        raytracer: Arc<Self>,
        output: Arc<Mutex<Vec<Color>>>,
        si: u32,
        sj_length: u32,
    ) {
        let width = raytracer.canvas.width();
        let height = raytracer.canvas.height();
        let image_size = (width * height) as usize;
        let mut result = vec![Color::BLACK; image_size];
        for sj in 0..sj_length {
            for i in 0..width {
                for j in 0..height {
                    let ray = raytracer.camera.get_ray_at(i, j, si, sj);
                    let color = Self::raytrace(&raytracer, ray, raytracer.max_depth);
                    result[(i * height + j) as usize].blend_assign(color, BlendMode::Add);
                }
            }
            progress_bar.inc(1);
        }
        let mut output = output.lock().unwrap();
        for i in 0..image_size {
            output[i].blend_assign(result[i], BlendMode::Add);
        }
    }

    pub fn render(self) -> Self {
        let raytracer = self;
        let width = raytracer.canvas.width();
        let height = raytracer.canvas.height();
        let image_size = (width * height) as usize;
        let sqrt_spp = raytracer.camera.sqrt_spp();
        let mut threads = Vec::with_capacity(sqrt_spp as usize);
        let progress = ProgressBar::new((sqrt_spp * sqrt_spp) as u64);
        progress.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{elapsed_precise} {bar:100.cyan/blue} {pos}/{len}"),
        );
        let progress = Arc::new(progress);
        let raytracer = Arc::new(raytracer);
        let output = Arc::new(Mutex::new(vec![Color::BLACK; image_size]));
        for si in 0..sqrt_spp {
            let progress_copy = progress.clone();
            let raytracer_copy = raytracer.clone();
            let output_copy = output.clone();
            threads.push(thread::spawn(move || {
                Self::render_task(progress_copy, raytracer_copy, output_copy, si, sqrt_spp);
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
        let lighten_factor = 1.0 / (sqrt_spp * sqrt_spp) as f64;
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
