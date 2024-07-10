use rand::Rng;

use raytracer::bvh::ShapeList;
use raytracer::camera::{Camera, ImageParam, LensParam, PerspectiveParam};
use raytracer::color::{BlendMode, Color};
use raytracer::hittable::{Object, WorldBuilder};
use raytracer::material::{Dielectric, Isotropic, Lambertian, Metal};
use raytracer::noise::Noise;
use raytracer::raytracer::RayTracer;
use raytracer::shape::{ConstantMedium, Moving, Quad, Shape, Sphere};
use raytracer::texture::{
    CheckerTexture, Emissive, ImageTexture, NoiseTexture, SolidColor, TexturedMaterial,
};
use raytracer::transform::Transform;
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
    scale: f64,
) -> Object<Sphere, TexturedMaterial<NoiseTexture, Lambertian>> {
    Object::new(
        Sphere::new(center, radius),
        TexturedMaterial::new(NoiseTexture::new(Noise::default(), scale), Lambertian),
    )
}

fn create_light(center: Vec3, radius: f64, light: Color) -> Object<Sphere, Emissive<SolidColor>> {
    Object::new(
        Sphere::new(center, radius),
        Emissive::new(SolidColor::new(light)),
    )
}

fn create_smoke(
    center: Vec3,
    radius: f64,
    albedo: Color,
    density: f64,
) -> Object<ConstantMedium<Sphere>, TexturedMaterial<SolidColor, Isotropic>> {
    Object::new(
        ConstantMedium::new(density, Sphere::new(center, radius)),
        TexturedMaterial::new(SolidColor::new(albedo), Isotropic),
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

fn create_cube_rotated(
    a: Vec3,
    albedo: Color,
    translate: Vec3,
    angle: f64,
) -> Object<ShapeList, TexturedMaterial<SolidColor, Lambertian>> {
    let mut cube = ShapeList::cube(Vec3::default(), a);
    cube.transform(Transform::rotate_y(angle.to_radians()));
    cube.transform(Transform::translate(translate));
    Object::new(
        cube,
        TexturedMaterial::new(SolidColor::new(albedo), Lambertian),
    )
}

fn create_cube_rotated_smoke(
    a: Vec3,
    albedo: Color,
    translate: Vec3,
    angle: f64,
) -> Object<ConstantMedium<ShapeList>, TexturedMaterial<SolidColor, Isotropic>> {
    let mut cube = ShapeList::cube(Vec3::default(), a);
    cube.transform(Transform::rotate_y(angle.to_radians()));
    cube.transform(Transform::translate(translate));
    Object::new(
        ConstantMedium::new(0.01, cube),
        TexturedMaterial::new(SolidColor::new(albedo), Isotropic),
    )
}

fn bouncing_spheres() {
    let mut world = WorldBuilder::default();
    world.add_object(create_lambertian_checker(
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
                    world.add_object(create_lambertian_moving(
                        center,
                        0.2,
                        Color::random(0.0, 1.0).blend(Color::random(0.0, 1.0), BlendMode::Mul),
                        Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0),
                    ));
                } else if choose_mat < 0.95 {
                    world.add_object(create_metal(
                        center,
                        0.2,
                        Color::random(0.5, 1.0),
                        rng.gen_range(0.0..0.5),
                    ));
                } else {
                    world.add_object(create_dielectric(center, 0.2, 1.5));
                }
            }
        }
    }
    world.add_object(create_dielectric(Vec3::new(0.0, 1.0, 0.0), 1.0, 1.5));
    world.add_object(create_lambertian(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Color::new(0.4, 0.2, 0.1),
    ));
    world.add_object(create_metal(
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
    let raytracer = RayTracer::new(camera, picture, world.build(), 50);
    raytracer.render().save("output/book2/image2.png");
}

fn checkered_spheres() {
    let mut world = WorldBuilder::default();
    world.add_object(create_lambertian_checker(Vec3::new(0.0, -10.0, 0.0), 10.0));
    world.add_object(create_lambertian_checker(Vec3::new(0.0, 10.0, 0.0), 10.0));

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
    let raytracer = RayTracer::new(camera, picture, world.build(), 50);
    raytracer.render().save("output/book2/image3.png");
}

fn earth() {
    let mut world = WorldBuilder::default();
    world.add_object(create_lambertian_texture(
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
    let raytracer = RayTracer::new(camera, picture, world.build(), 50);
    raytracer.render().save("output/book2/image5.png");
}

fn noise_spheres() {
    let mut world = WorldBuilder::default();
    world.add_object(create_lambertian_noise(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        4.0,
    ));
    world.add_object(create_lambertian_noise(Vec3::new(0.0, 2.0, 0.0), 2.0, 4.0));

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
    let raytracer = RayTracer::new(camera, picture, world.build(), 50);
    raytracer.render().save("output/book2/image15.png");
}

fn quads() {
    let mut world = WorldBuilder::default();
    world.add_object(create_quad(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Color::new(1.0, 0.2, 0.2),
    ));
    world.add_object(create_quad(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        Color::new(0.2, 1.0, 0.2),
    ));
    world.add_object(create_quad(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Color::new(0.2, 0.2, 1.0),
    ));
    world.add_object(create_quad(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        Color::new(1.0, 0.5, 0.0),
    ));
    world.add_object(create_quad(
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
    let raytracer = RayTracer::new(camera, picture, world.build(), 50);
    raytracer.render().save("output/book2/image16.png");
}

fn simple_light() {
    let mut world = WorldBuilder::default();
    world.add_object(create_lambertian_noise(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        4.0,
    ));
    world.add_object(create_lambertian_noise(Vec3::new(0.0, 2.0, 0.0), 2.0, 4.0));
    world.add_object(create_quad_light(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Color::new(4.0, 4.0, 4.0),
    ));
    world.add_object(create_light(
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
    let raytracer = RayTracer::new(camera, picture, world.build(), 50);
    raytracer.render().save("output/book2/image18.png");
}

fn cornell_box() {
    let mut world = WorldBuilder::default();
    world.add_object(create_quad(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Color::new(0.12, 0.45, 0.15),
    ));
    world.add_object(create_quad(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Color::new(0.65, 0.05, 0.05),
    ));
    world.add_object(create_quad_light(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        Color::new(15.0, 15.0, 15.0),
    ));
    world.add_object(create_quad(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Color::new(0.73, 0.73, 0.73),
    ));
    world.add_object(create_quad(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        Color::new(0.73, 0.73, 0.73),
    ));
    world.add_object(create_quad(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Color::new(0.73, 0.73, 0.73),
    ));
    world.add_object(create_cube_rotated(
        Vec3::new(165.0, 330.0, 165.0),
        Color::new(0.73, 0.73, 0.73),
        Vec3::new(265.0, 0.0, 295.0),
        15.0,
    ));
    world.add_object(create_cube_rotated(
        Vec3::new(165.0, 165.0, 165.0),
        Color::new(0.73, 0.73, 0.73),
        Vec3::new(130.0, 0.0, 65.0),
        -18.0,
    ));
    world.add_light(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
    ));

    let image_width = 600;
    let image_height = 600;
    let camera = Camera::new(
        PerspectiveParam {
            look_from: Vec3::new(278.0, 278.0, -800.0),
            look_at: Vec3::new(278.0, 278.0, 0.0),
            view_up: Vec3::new(0.0, 1.0, 0.0),
        },
        LensParam {
            fov: 40.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
        },
        ImageParam {
            image_width,
            image_height,
            sample_per_pixel: 1000,
        },
    );
    let picture = raytracer::canvas::Canvas::empty(image_width, image_height);
    let raytracer = RayTracer::new(camera, picture, world.build(), 50);
    raytracer.render().save("output/book3/image3.png");
}

fn cornell_smoke() {
    let mut world = WorldBuilder::default();
    world.add_object(create_quad(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Color::new(0.12, 0.45, 0.15),
    ));
    world.add_object(create_quad(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Color::new(0.65, 0.05, 0.05),
    ));
    world.add_object(create_quad_light(
        Vec3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        Color::new(7.0, 7.0, 7.0),
    ));
    world.add_object(create_quad(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Color::new(0.73, 0.73, 0.73),
    ));
    world.add_object(create_quad(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        Color::new(0.73, 0.73, 0.73),
    ));
    world.add_object(create_quad(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Color::new(0.73, 0.73, 0.73),
    ));
    world.add_object(create_cube_rotated_smoke(
        Vec3::new(165.0, 330.0, 165.0),
        Color::BLACK,
        Vec3::new(265.0, 0.0, 295.0),
        15.0,
    ));
    world.add_object(create_cube_rotated_smoke(
        Vec3::new(165.0, 165.0, 165.0),
        Color::WHITE,
        Vec3::new(130.0, 0.0, 65.0),
        -18.0,
    ));

    let image_width = 600;
    let image_height = 600;
    let camera = Camera::new(
        PerspectiveParam {
            look_from: Vec3::new(278.0, 278.0, -800.0),
            look_at: Vec3::new(278.0, 278.0, 0.0),
            view_up: Vec3::new(0.0, 1.0, 0.0),
        },
        LensParam {
            fov: 40.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
        },
        ImageParam {
            image_width,
            image_height,
            sample_per_pixel: 200,
        },
    );
    let picture = raytracer::canvas::Canvas::empty(image_width, image_height);
    let raytracer = RayTracer::new(camera, picture, world.build(), 50);
    raytracer.render().save("output/book2/image22.png");
}

fn final_scene(image_width: u32, sample_per_pixel: u32, max_depth: u32) {
    let mut world = WorldBuilder::default();
    let mut rng = rand::thread_rng();
    let mut box1 = ShapeList::default();
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;
            box1.push(ShapeList::cube(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
            ));
        }
    }
    world.add_object(Object::new(
        box1.tree(),
        TexturedMaterial::new(SolidColor::new(Color::new(0.48, 0.83, 0.53)), Lambertian),
    ));
    world.add_object(create_quad_light(
        Vec3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        Color::new(7.0, 7.0, 7.0),
    ));
    world.add_object(create_lambertian_moving(
        Vec3::new(400.0, 400.0, 200.0),
        50.0,
        Color::new(0.7, 0.3, 0.1),
        Vec3::new(30.0, 0.0, 0.0),
    ));
    world.add_object(create_dielectric(Vec3::new(260.0, 150.0, 45.0), 50.0, 1.5));
    world.add_object(create_metal(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Color::new(0.8, 0.8, 0.9),
        1.0,
    ));
    world.add_object(create_dielectric(Vec3::new(360.0, 150.0, 145.0), 70.0, 1.5));
    world.add_object(create_smoke(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Color::new(0.2, 0.4, 0.9),
        0.2,
    ));
    world.add_object(create_smoke(
        Vec3::new(0.0, 0.0, 0.0),
        5000.0,
        Color::WHITE,
        0.0001,
    ));
    world.add_object(create_lambertian_texture(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        "assets/earth_map.jpg",
    ));
    world.add_object(create_lambertian_noise(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        0.2,
    ));
    let ns = 1000;
    let mut spheres = ShapeList::default();
    for _ in 0..ns {
        let mut sphere = Sphere::new(Vec3::random(0.0, 165.0), 10.0);
        sphere.transform(Transform::rotate_y(15.0f64.to_radians()));
        sphere.transform(Transform::translate(Vec3::new(-100.0, 270.0, 395.0)));
        spheres.push(sphere);
    }
    world.add_object(Object::new(
        spheres.tree(),
        TexturedMaterial::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)), Lambertian),
    ));

    let image_height = image_width;
    let camera = Camera::new(
        PerspectiveParam {
            look_from: Vec3::new(478.0, 278.0, -600.0),
            look_at: Vec3::new(278.0, 278.0, 0.0),
            view_up: Vec3::new(0.0, 1.0, 0.0),
        },
        LensParam {
            fov: 40.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
        },
        ImageParam {
            image_width,
            image_height,
            sample_per_pixel,
        },
    );
    let picture = raytracer::canvas::Canvas::empty(image_width, image_height);
    let raytracer = RayTracer::new(camera, picture, world.build(), max_depth);
    raytracer.render().save("output/book2/image23.png");
}

fn main() {
    let x = 7;
    match x {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => noise_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800, 10000, 40),
        _ => final_scene(400, 1000, 4),
    }
}
