use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::hittable::{HittableList, Object};
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::raytracer::RayTracer;
use raytracer::shape::Sphere;
use raytracer::vec3d::Vec3d;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let resolution_width = 400;
    let focal_length = 1.0;
    let origin = Vec3d::new(0.0, 0.0, 0.0);
    let sample_per_pixel = 100;
    let max_depth = 10;

    let viewport_width = aspect_ratio * viewport_height;
    let resolution_height = (resolution_width as f64 / aspect_ratio) as u32;
    let image_height = if resolution_height < 1 {
        1
    } else {
        resolution_height
    };

    let mut hittable_list = HittableList::default();
    hittable_list.add(Box::new(Object::new(
        Box::new(Sphere::new(Vec3d::new(0.0, -100.5, -1.0), 100.0)),
        Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
    )));
    hittable_list.add(Box::new(Object::new(
        Box::new(Sphere::new(Vec3d::new(0.0, 0.0, -1.2), 0.5)),
        Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))),
    )));
    hittable_list.add(Box::new(Object::new(
        Box::new(Sphere::new(Vec3d::new(-1.0, 0.0, -1.0), 0.5)),
        Box::new(Dielectric::new(1.5)),
    )));
    hittable_list.add(Box::new(Object::new(
        Box::new(Sphere::new(Vec3d::new(1.0, 0.0, -1.0), 0.5)),
        Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0)),
    )));

    let camera = Camera::new(
        origin,
        focal_length,
        viewport_width,
        viewport_height,
        resolution_width,
        image_height,
        sample_per_pixel,
    );
    let picture = raytracer::canvas::Canvas::new(resolution_width, image_height);
    let mut raytracer = RayTracer::new(camera, picture, hittable_list, max_depth);
    raytracer.render(true);
    raytracer.save("output/book1/image16.png");
}
