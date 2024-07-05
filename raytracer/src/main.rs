use rand::Rng;

use raytracer::camera::{Camera, ImageParam, LensParam, PerspectiveParam};
use raytracer::color::{BlendMode, Color};
use raytracer::hittable::{HittableList, Object};
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::noise::Noise;
use raytracer::raytracer::RayTracer;
use raytracer::shape::{Moving, Quad, Sphere};
use raytracer::texture::{
    CheckerTexture, Emissive, ImageTexture, NoiseTexture, SolidColor, TexturedMaterial,
};
use raytracer::vec3::Vec3;

fn create_lambertian(
    center: Vec3,
    radius: f64,
    albedo: Color,
) -> Object<Sphere, TexturedMaterial<SolidColor, Lambertian>> {
    Object::new(
        Sphere::new(center, radius),
        TexturedMaterial::new(SolidColor::new(albedo), Lambertian),
    )
}

fn create_metal(
    center: Vec3,
    radius: f64,
    albedo: Color,
    fuzz: f64,
) -> Object<Sphere, TexturedMaterial<SolidColor, Metal>> {
    Object::new(
        Sphere::new(center, radius),
        TexturedMaterial::new(SolidColor::new(albedo), Metal::new(fuzz)),
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
) -> Object<Moving<Sphere>, TexturedMaterial<SolidColor, Lambertian>> {
    Object::new(
        Moving::new(direction, Sphere::new(center, radius)),
        TexturedMaterial::new(SolidColor::new(albedo), Lambertian),
    )
}

fn create_lambertian_checker(
    center: Vec3,
    radius: f64,
) -> Object<Sphere, TexturedMaterial<CheckerTexture, Lambertian>> {
    Object::new(
        Sphere::new(center, radius),
        TexturedMaterial::new(
            CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9), 0.32),
            Lambertian,
        ),
    )
}

fn create_lambertian_texture(
    center: Vec3,
    radius: f64,
    texture: &str,
) -> Object<Sphere, TexturedMaterial<ImageTexture, Lambertian>> {
    Object::new(
        Sphere::new(center, radius),
        TexturedMaterial::new(ImageTexture::new(texture), Lambertian),
    )
}

fn create_lambertian_noise(
    center: Vec3,
    radius: f64,
) -> Object<Sphere, TexturedMaterial<NoiseTexture, Lambertian>> {
    Object::new(
        Sphere::new(center, radius),
        TexturedMaterial::new(NoiseTexture::new(Noise::default(), 4.0), Lambertian),
    )
}

fn create_quad(
    q: Vec3,
    u: Vec3,
    v: Vec3,
    albedo: Color,
) -> Object<Quad, TexturedMaterial<SolidColor, Lambertian>> {
    Object::new(
        Quad::new(q, u, v),
        TexturedMaterial::new(SolidColor::new(albedo), Lambertian),
    )
}

fn create_quad_light(
    q: Vec3,
    u: Vec3,
    v: Vec3,
    light: Color,
) -> Object<Quad, Emissive<SolidColor>> {
    Object::new(Quad::new(q, u, v), Emissive::new(SolidColor::new(light)))
}

fn create_sphere_light(
    center: Vec3,
    radius: f64,
    light: Color,
) -> Object<Sphere, Emissive<SolidColor>> {
    Object::new(
        Sphere::new(center, radius),
        Emissive::new(SolidColor::new(light)),
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
    let raytracer = RayTracer::new(
        camera,
        picture,
        hittable_list.build(),
        50,
        Color::new(0.7, 0.8, 1.0),
    );
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
    let raytracer = RayTracer::new(
        camera,
        picture,
        hittable_list.build(),
        50,
        Color::new(0.7, 0.8, 1.0),
    );
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
    let raytracer = RayTracer::new(
        camera,
        picture,
        hittable_list.build(),
        50,
        Color::new(0.7, 0.8, 1.0),
    );
    raytracer.render().save("output/book2/image5.png");
}

fn noise_spheres() {
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
    let raytracer = RayTracer::new(
        camera,
        picture,
        hittable_list.build(),
        50,
        Color::new(0.7, 0.8, 1.0),
    );
    raytracer.render().save("output/book2/image15.png");
}

fn quads() {
    let mut hittable_list = HittableList::default();
    hittable_list.push(create_quad(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Color::new(1.0, 0.2, 0.2),
    ));
    hittable_list.push(create_quad(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        Color::new(0.2, 1.0, 0.2),
    ));
    hittable_list.push(create_quad(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Color::new(0.2, 0.2, 1.0),
    ));
    hittable_list.push(create_quad(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        Color::new(1.0, 0.5, 0.0),
    ));
    hittable_list.push(create_quad(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        Color::new(0.2, 0.8, 0.8),
    ));

    let image_width = 400;
    let image_height = 400;
    let camera = Camera::new(
        PerspectiveParam {
            look_from: Vec3::new(0.0, 0.0, 9.0),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            view_up: Vec3::new(0.0, 1.0, 0.0),
        },
        LensParam {
            fov: 80.0,
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
    let raytracer = RayTracer::new(
        camera,
        picture,
        hittable_list.build(),
        50,
        Color::new(0.7, 0.8, 1.0),
    );
    raytracer.render().save("output/book2/image16.png");
}

fn simple_light() {
    let mut hittable_list = HittableList::default();
    hittable_list.push(create_lambertian_noise(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
    ));
    hittable_list.push(create_lambertian_noise(Vec3::new(0.0, 2.0, 0.0), 2.0));
    hittable_list.push(create_quad_light(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Color::new(4.0, 4.0, 4.0),
    ));
    hittable_list.push(create_sphere_light(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        Color::new(4.0, 4.0, 4.0),
    ));

    let image_width = 400;
    let image_height = 225;
    let camera = Camera::new(
        PerspectiveParam {
            look_from: Vec3::new(26.0, 3.0, 6.0),
            look_at: Vec3::new(0.0, 2.0, 0.0),
            view_up: Vec3::new(0.0, 1.0, 0.0),
        },
        LensParam {
            fov: 20.0,
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
    let raytracer = RayTracer::new(camera, picture, hittable_list.build(), 50, Color::BLACK);
    raytracer.render().save("output/book2/image18.png");
}

fn main() {
    let x = 6;
    match x {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => noise_spheres(),
        5 => quads(),
        6 => simple_light(),
        _ => {}
    }
}
