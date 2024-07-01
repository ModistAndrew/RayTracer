use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::hittable::{HittableList, Object};
use raytracer::material::Lambertian;
use raytracer::raytracer::RayTracer;
use raytracer::shape::Sphere;
use raytracer::vec3d::Vec3d;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let resolution_width = 400;

    let focal_length = 1.0;
    let fov = 90.0;
    let origin = Vec3d::new(0.0, 0.0, 0.0);

    let sample_per_pixel = 100;
    let max_depth = 10;

    let mut resolution_height = (resolution_width as f64 / aspect_ratio) as u32;
    resolution_height = if resolution_height < 1 {
        1
    } else {
        resolution_height
    };

    let mut hittable_list = HittableList::default();
    let r = (std::f64::consts::PI / 4.0).cos();
    hittable_list.add(Box::new(Object::new(
        Box::new(Sphere::new(Vec3d::new(-r, 0.0, -1.0), r)),
        Box::new(Lambertian::new(Color::new(0.0, 0.0, 1.0))),
    )));
    hittable_list.add(Box::new(Object::new(
        Box::new(Sphere::new(Vec3d::new(r, 0.0, -1.0), r)),
        Box::new(Lambertian::new(Color::new(1.0, 0.0, 0.0))),
    )));

    let camera = Camera::new(
        origin,
        focal_length,
        fov,
        resolution_width,
        resolution_height,
        sample_per_pixel,
    );
    let picture = raytracer::canvas::Canvas::new(resolution_width, resolution_height);
    let mut raytracer = RayTracer::new(camera, picture, hittable_list, max_depth);
    raytracer.render(true);
    raytracer.save("output/book1/image19.png");
}
