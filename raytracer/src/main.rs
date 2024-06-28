use raytracer::camera::Camera;
use raytracer::hittable::{HittableList, Sphere};
use raytracer::raytracer::RayTracer;
use raytracer::vec3d::Vec3d;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let image_width = 400;
    let focal_length = 1.0;
    let origin = Vec3d::new(0.0, 0.0, 0.0);

    let viewport_width = aspect_ratio * viewport_height;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    let mut hittable_list = HittableList::default();
    hittable_list.add(Box::new(Sphere::new(Vec3d::new(0.0, 0.0, -1.0), 0.5)));
    hittable_list.add(Box::new(Sphere::new(Vec3d::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new(origin, focal_length, viewport_width, viewport_height);
    let picture = raytracer::canvas::Canvas::new(image_width, image_height);
    let mut raytracer = RayTracer::new(camera, picture, hittable_list);
    raytracer.render(true);
    raytracer.save("output/book1/image5.jpg");
}
