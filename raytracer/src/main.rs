use rand::Rng;

use raytracer::camera::{Camera, ImageParam, LensParam, PerspectiveParam};
use raytracer::color::{BlendMode, Color};
use raytracer::hittable::{HittableList, Object};
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::perlin::Perlin;
use raytracer::raytracer::RayTracer;
use raytracer::shape::{Moving, Sphere};
use raytracer::texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor};
use raytracer::vec3::Vec3;

fn create_lambertian(
    center: Vec3,
    radius: f64,
    albedo: Color,
) -> Object<Sphere, SolidColor<Lambertian>> {
    Object::new(
        Sphere::new(center, radius),
        SolidColor::new(albedo, Lambertian),
    )
}

fn create_metal(
    center: Vec3,
    radius: f64,
    albedo: Color,
    fuzz: f64,
) -> Object<Sphere, SolidColor<Metal>> {
    Object::new(
        Sphere::new(center, radius),
        SolidColor::new(albedo, Metal::new(fuzz)),
    )
}

fn create_dielectric(
    center: Vec3,
    radius: f64,
    refraction_index: f64,
) -> Object<Sphere, Dielectric> {
    Object::new(
        Sphere::new(center, radius),
        Dielectric::new(refraction_index),
    )
}

fn create_lambertian_moving(
    center: Vec3,
    radius: f64,
    albedo: Color,
    direction: Vec3,
) -> Object<Moving<Sphere>, SolidColor<Lambertian>> {
    Object::new(
        Moving::new(direction, Sphere::new(center, radius)),
        SolidColor::new(albedo, Lambertian),
    )
}

fn create_lambertian_checker(
    center: Vec3,
    radius: f64,
) -> Object<Sphere, CheckerTexture<Lambertian>> {
    Object::new(
        Sphere::new(center, radius),
        CheckerTexture::new(
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
            0.32,
            Lambertian,
        ),
    )
}

fn create_lambertian_texture(
    center: Vec3,
    radius: f64,
    texture: &str,
) -> Object<Sphere, ImageTexture<Lambertian>> {
    Object::new(
        Sphere::new(center, radius),
        ImageTexture::new(texture, Lambertian),
    )
}

fn create_lambertian_noise(center: Vec3, radius: f64) -> Object<Sphere, NoiseTexture<Lambertian>> {
    Object::new(
        Sphere::new(center, radius),
        NoiseTexture::new(Perlin::default(), Lambertian),
    )
}

fn bouncing_spheres() {
    let mut hittable_list = HittableList::default();
    hittable_list.push(create_lambertian_checker(
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
                    hittable_list.push(create_lambertian_moving(
                        center,
                        0.2,
                        Color::random(0.0, 1.0).blend(Color::random(0.0, 1.0), BlendMode::Mul),
                        Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0),
                    ));
                } else if choose_mat < 0.95 {
                    hittable_list.push(create_metal(
                        center,
                        0.2,
                        Color::random(0.5, 1.0),
                        rng.gen_range(0.0..0.5),
                    ));
                } else {
                    hittable_list.push(create_dielectric(center, 0.2, 1.5));
                }
            }
        }
    }
    hittable_list.push(create_dielectric(Vec3::new(0.0, 1.0, 0.0), 1.0, 1.5));
    hittable_list.push(create_lambertian(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Color::new(0.4, 0.2, 0.1),
    ));
    hittable_list.push(create_metal(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Color::new(0.7, 0.6, 0.5),
        0.0,
    ));

    let image_width = 1200;
    let image_height = 675;
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
    let picture = raytracer::canvas::Canvas::empty(image_width, image_height);
    let raytracer = RayTracer::new(camera, picture, hittable_list.build(), 50);
    raytracer.render().save("output/book2/image2.png");
}

fn checkered_spheres() {
    let mut hittable_list = HittableList::default();
    hittable_list.push(create_lambertian_checker(Vec3::new(0.0, -10.0, 0.0), 10.0));
    hittable_list.push(create_lambertian_checker(Vec3::new(0.0, 10.0, 0.0), 10.0));

    let image_width = 400;
    let image_height = 225;
    let camera = Camera::new(
        PerspectiveParam {
            look_from: Vec3::new(13.0, 2.0, 3.0),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            view_up: Vec3::new(0.0, 1.0, 0.0),
        },
        LensParam {
            fov: 20.0,
            filter: Color::WHITE,
            defocus_angle: 0.0,
            focus_dist: 10.0,
        },
        ImageParam {
            image_width,
            image_height,
            sample_per_pixel: 100,
        },
    );
    let picture = raytracer::canvas::Canvas::empty(image_width, image_height);
    let raytracer = RayTracer::new(camera, picture, hittable_list.build(), 50);
    raytracer.render().save("output/book2/image3.png");
}

fn earth() {
    let mut hittable_list = HittableList::default();
    hittable_list.push(create_lambertian_texture(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        "assets/earth_map.jpg",
    ));

    let image_width = 400;
    let image_height = 225;
    let camera = Camera::new(
        PerspectiveParam {
            look_from: Vec3::new(0.0, 0.0, 12.0),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            view_up: Vec3::new(0.0, 1.0, 0.0),
        },
        LensParam {
            fov: 20.0,
            filter: Color::WHITE,
            defocus_angle: 0.0,
            focus_dist: 10.0,
        },
        ImageParam {
            image_width,
            image_height,
            sample_per_pixel: 100,
        },
    );
    let picture = raytracer::canvas::Canvas::empty(image_width, image_height);
    let raytracer = RayTracer::new(camera, picture, hittable_list.build(), 50);
    raytracer.render().save("output/book2/image5.png");
}

fn perlin_spheres() {
    let mut hittable_list = HittableList::default();
    hittable_list.push(create_lambertian_noise(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
    ));
    hittable_list.push(create_lambertian_noise(Vec3::new(0.0, 2.0, 0.0), 2.0));

    let image_width = 400;
    let image_height = 225;
    let camera = Camera::new(
        PerspectiveParam {
            look_from: Vec3::new(13.0, 2.0, 3.0),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            view_up: Vec3::new(0.0, 1.0, 0.0),
        },
        LensParam {
            fov: 20.0,
            filter: Color::WHITE,
            defocus_angle: 0.0,
            focus_dist: 10.0,
        },
        ImageParam {
            image_width,
            image_height,
            sample_per_pixel: 100,
        },
    );
    let picture = raytracer::canvas::Canvas::empty(image_width, image_height);
    let raytracer = RayTracer::new(camera, picture, hittable_list.build(), 50);
    raytracer.render().save("output/book2/image11.png");
}

fn main() {
    let x = 4;
    match x {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        _ => {}
    }
}
