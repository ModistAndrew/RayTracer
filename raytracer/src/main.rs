use rand::Rng;

use raytracer::camera::{Camera, ImageParam, LensParam, PerspectiveParam};
use raytracer::color::{BlendMode, Color};
use raytracer::hittable::{Hittable, HittableList, Object};
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::raytracer::RayTracer;
use raytracer::shape::Sphere;
use raytracer::texture::{CheckerTexture, SolidColor};
use raytracer::vec3::Vec3;

fn create_lambertian(center: Vec3, radius: f64, albedo: Color) -> Box<Object> {
    Box::new(Object::new(
        Box::new(Sphere::new(center, radius, None)),
        Box::new(Lambertian::new(Box::new(SolidColor::new(albedo)))),
    ))
}

fn create_metal(center: Vec3, radius: f64, albedo: Color, fuzz: f64) -> Box<Object> {
    Box::new(Object::new(
        Box::new(Sphere::new(center, radius, None)),
        Box::new(Metal::new(albedo, fuzz)),
    ))
}

fn create_dielectric(center: Vec3, radius: f64, refraction_index: f64) -> Box<Object> {
    Box::new(Object::new(
        Box::new(Sphere::new(center, radius, None)),
        Box::new(Dielectric::new(refraction_index)),
    ))
}

fn create_lambertian_moving(
    center: Vec3,
    radius: f64,
    albedo: Color,
    direction: Vec3,
) -> Box<Object> {
    Box::new(Object::new(
        Box::new(Sphere::new(center, radius, Some(direction))),
        Box::new(Lambertian::new(Box::new(SolidColor::new(albedo)))),
    ))
}

fn create_lambertian_checker(center: Vec3, radius: f64) -> Box<Object> {
    Box::new(Object::new(
        Box::new(Sphere::new(center, radius, None)),
        Box::new(Lambertian::new(Box::new(CheckerTexture::from_color(
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
            0.32,
        )))),
    ))
}

fn main() {
    let mut hittable_vec = Vec::<Box<dyn Hittable>>::default();
    hittable_vec.push(create_lambertian_checker(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
    ));
    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    hittable_vec.push(create_lambertian_moving(
                        center,
                        0.2,
                        Color::random(0.0, 1.0).blend(Color::random(0.0, 1.0), BlendMode::Mul),
                        Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0),
                    ));
                } else if choose_mat < 0.95 {
                    hittable_vec.push(create_metal(
                        center,
                        0.2,
                        Color::random(0.5, 1.0),
                        rng.gen_range(0.0..0.5),
                    ));
                } else {
                    hittable_vec.push(create_dielectric(center, 0.2, 1.5));
                }
            }
        }
    }
    hittable_vec.push(create_dielectric(Vec3::new(0.0, 1.0, 0.0), 1.0, 1.5));
    hittable_vec.push(create_lambertian(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Color::new(0.4, 0.2, 0.1),
    ));
    hittable_vec.push(create_metal(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Color::new(0.7, 0.6, 0.5),
        0.0,
    ));
    let hittable_list = HittableList::new(hittable_vec);

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let mut image_height = (image_width as f64 / aspect_ratio) as u32;
    if image_height < 1 {
        image_height = 1;
    }

    let camera = Camera::new(
        PerspectiveParam {
            look_from: Vec3::new(13.0, 2.0, 3.0),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            view_up: Vec3::new(0.0, 1.0, 0.0),
        },
        LensParam {
            fov: 20.0,
            filter: Color::WHITE,
            defocus_angle: 0.6,
            focus_dist: 10.0,
        },
        ImageParam {
            image_width,
            image_height,
            sample_per_pixel: 500,
        },
    );
    let picture = raytracer::canvas::Canvas::new(image_width, image_height);
    let mut raytracer = RayTracer::new(camera, picture, hittable_list, 50);
    raytracer.render(true);
    raytracer.save("output/book1/image25.png");
}
