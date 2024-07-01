use raytracer::camera::{Camera, ImageParam, LensParam, PerspectiveParam};
use raytracer::color::Color;
use raytracer::hittable::{HittableList, Object};
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::raytracer::RayTracer;
use raytracer::shape::Sphere;
use raytracer::vec3d::Vec3d;

fn main() {
    let look_from = Vec3d::new(-2.0, 2.0, 1.0);
    let look_at = Vec3d::new(0.0, 0.0, -1.0);
    let view_up = Vec3d::new(0.0, 1.0, 0.0);

    let fov = 20.0;
    let filter = Color::WHITE;

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as u32;
    if image_height < 1 {
        image_height = 1;
    }
    let sample_per_pixel = 100;
    let max_depth = 50;

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
        Box::new(Dielectric::new(1.50)),
    )));
    hittable_list.add(Box::new(Object::new(
        Box::new(Sphere::new(Vec3d::new(-1.0, 0.0, -1.0), 0.4)),
        Box::new(Dielectric::new(1.00 / 1.50)),
    )));
    hittable_list.add(Box::new(Object::new(
        Box::new(Sphere::new(Vec3d::new(1.0, 0.0, -1.0), 0.5)),
        Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0)),
    )));

    let camera = Camera::new(
        PerspectiveParam {
            look_from,
            look_at,
            view_up,
        },
        LensParam { fov, filter },
        ImageParam {
            image_width,
            image_height,
            sample_per_pixel,
        },
    );
    let picture = raytracer::canvas::Canvas::new(image_width, image_height);
    let mut raytracer = RayTracer::new(camera, picture, hittable_list, max_depth);
    raytracer.render(true);
    raytracer.save("output/book1/image20.png");
}
