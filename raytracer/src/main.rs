use rand::Rng;
use raytracer::camera::{Camera, ImageParam, LensParam, PerspectiveParam};
use raytracer::color::{BlendMode, Color};
use raytracer::hittable::{HittableList, Object};
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::raytracer::RayTracer;
use raytracer::shape::Sphere;
use raytracer::vec3d::Vec3d;

fn create_lambertian(center: Vec3d, radius: f64, albedo: Color) -> Box<Object> {
    Box::new(Object::new(
        Box::new(Sphere::new(center, radius, Default::default())),
        Box::new(Lambertian::new(albedo)),
    ))
}

fn create_metal(center: Vec3d, radius: f64, albedo: Color, fuzz: f64) -> Box<Object> {
    Box::new(Object::new(
        Box::new(Sphere::new(center, radius, Default::default())),
        Box::new(Metal::new(albedo, fuzz)),
    ))
}

fn create_dielectric(center: Vec3d, radius: f64, refraction_index: f64) -> Box<Object> {
    Box::new(Object::new(
        Box::new(Sphere::new(center, radius, Default::default())),
        Box::new(Dielectric::new(refraction_index)),
    ))
}

fn create_lambertian_moving(
    center: Vec3d,
    radius: f64,
    albedo: Color,
    direction: Vec3d,
) -> Box<Object> {
    Box::new(Object::new(
        Box::new(Sphere::new(center, radius, direction)),
        Box::new(Lambertian::new(albedo)),
    ))
}

fn main() {
    let mut hittable_list = HittableList::default();
    hittable_list.add(create_lambertian(
        Vec3d::new(0.0, -1000.0, 0.0),
        1000.0,
        Color::new(0.5, 0.5, 0.5),
    ));
    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3d::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3d::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    hittable_list.add(create_lambertian_moving(
                        center,
                        0.2,
                        Color::random(0.0, 1.0).blend(Color::random(0.0, 1.0), BlendMode::Mul),
                        Vec3d::new(0.0, rng.gen_range(0.0..0.5), 0.0),
                    ));
                } else if choose_mat < 0.95 {
                    hittable_list.add(create_metal(
                        center,
                        0.2,
                        Color::random(0.5, 1.0),
                        rng.gen_range(0.0..0.5),
                    ));
                } else {
                    hittable_list.add(create_dielectric(center, 0.2, 1.5));
                }
            }
        }
    }
    hittable_list.add(create_dielectric(Vec3d::new(0.0, 1.0, 0.0), 1.0, 1.5));
    hittable_list.add(create_lambertian(
        Vec3d::new(-4.0, 1.0, 0.0),
        1.0,
        Color::new(0.4, 0.2, 0.1),
    ));
    hittable_list.add(create_metal(
        Vec3d::new(4.0, 1.0, 0.0),
        1.0,
        Color::new(0.7, 0.6, 0.5),
        0.0,
    ));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as u32;
    if image_height < 1 {
        image_height = 1;
    }

    let camera = Camera::new(
        PerspectiveParam {
            look_from: Vec3d::new(13.0, 2.0, 3.0),
            look_at: Vec3d::new(0.0, 0.0, 0.0),
            view_up: Vec3d::new(0.0, 1.0, 0.0),
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
            sample_per_pixel: 100,
        },
    );
    let picture = raytracer::canvas::Canvas::new(image_width, image_height);
    let mut raytracer = RayTracer::new(camera, picture, hittable_list, 50);
    raytracer.render(true);
    raytracer.save("output/book1/image24.png");
}
