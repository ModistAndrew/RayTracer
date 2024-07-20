use raytracer::camera::{Camera, ImageParam, LensParam, PerspectiveParam};
use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::hittable::WorldBuilder;
use raytracer::material::{Dielectric, Emissive, Lambertian, Translucent};
use raytracer::mesh::load_obj;
use raytracer::raytracer::RayTracer;
use raytracer::shape::Sphere;
use raytracer::texture::{Atlas, ImageTexture, SolidColor};
use raytracer::vec3::Vec3;

fn main() {
    let mut world = WorldBuilder::default();
    let mut mesh = load_obj("assets/test.obj");
    println!("Mesh: {:?}", mesh.keys());
    world.add_object(
        mesh.remove("key1_up").unwrap(),
        Translucent::new(1.5),
        Atlas::default().set_transparency(ImageTexture::new("assets/key1_alpha.png")),
    );
    world.add_object(
        mesh.remove("key1_down").unwrap(),
        Translucent::new(1.5),
        Atlas::default().set_transparency(ImageTexture::new("assets/key1_alpha.png")),
    );
    world.add_object(
        mesh.remove("key1_mid").unwrap(),
        Lambertian,
        Atlas::default()
            .set_transparency(ImageTexture::new("assets/key1_icon_alpha.png"))
            .set_attenuation(SolidColor::new(Color::BLACK)),
    );
    world.add_object(
        mesh.remove("key1_curve").unwrap(),
        Dielectric::new(1.5),
        Atlas::default(),
    );
    world.add_object(
        mesh.remove("box_back").unwrap(),
        Lambertian,
        Atlas::default().set_attenuation(SolidColor::new(Color::WHITE)),
    );
    world.add_object(
        mesh.remove("box_down").unwrap(),
        Lambertian,
        Atlas::default().set_attenuation(SolidColor::new(Color::WHITE)),
    );
    world.add_object(
        Sphere::new(Vec3::new(1.0, 3.0, -1.0), 0.5),
        Emissive,
        Atlas::default().set_emission(SolidColor::new(Color::new(40.0, 30.0, 20.0))),
    );
    world.add_light(
        Sphere::new(Vec3::new(1.0, 3.0, -1.0), 0.5),
    );

    let image_width = 1200;
    let image_height = 675;
    let camera = Camera::new(
        PerspectiveParam {
            look_from: Vec3::new(0.0, 20.0, -20.0),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            view_up: Vec3::new(0.0, 1.0, 0.0), // y-axis is up
        },
        LensParam {
            fov: 40.0,
            defocus_angle: 0.0,
            focus_dist: 10.0,
        },
        ImageParam {
            image_width,
            image_height,
            sample_per_pixel: 100,
        },
    );
    let picture = Canvas::empty(image_width, image_height);
    let raytracer = RayTracer::new(camera, picture, world.build(), 50);
    raytracer.render().save("output/final/test.png");
}
