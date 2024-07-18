use raytracer::camera::{Camera, ImageParam, LensParam, PerspectiveParam};
use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::hittable::WorldBuilder;
use raytracer::material::{Dielectric, Translucent};
use raytracer::mesh::MeshObject;
use raytracer::raytracer::RayTracer;
use raytracer::shape::Sphere;
use raytracer::texture::{Emissive, ImageTexture, SolidColor};
use raytracer::vec3::Vec3;

fn main() {
    let mut world = WorldBuilder::default();
    world.set_background(Color::new(0.2, 0.3, 0.4));
    let mut mesh = MeshObject::from_obj("assets/test.obj");
    println!("Mesh: {:?}", mesh.keys());
    let up = mesh.get_mut("key1_up").unwrap();
    up.set_material(Dielectric::new(1.0));
    // up.get_textures_mut().set_transparency(ImageTexture::new("assets/key1_alpha.png"));

    let down = mesh.get_mut("key1_down").unwrap();
    // down.get_textures_mut().set_transparency(ImageTexture::new("assets/key1_alpha.png"));
    down.set_material(Dielectric::new(1.0));

    let mid = mesh.get_mut("key1_mid").unwrap();
    mid.get_textures_mut()
        .set_transparency(ImageTexture::new("assets/key1_icon_alpha.png"));
    mid.get_textures_mut()
        .set_albedo(SolidColor::new(Color::new(1.0, 0.0, 0.0)));

    let curve = mesh.get_mut("key1_curve").unwrap();
    curve.set_material(Dielectric::new(1.0));
    // mesh.remove("key1_up");
    // mesh.remove("key1_down");
    mesh.remove("key1_curve");
    // mesh.remove("key1_mid");
    mesh.into_iter().for_each(|object| {
        println!("Object: {:?}", object.0);
        world.add(object.1);
    });
    world.add_object(
        Sphere::new(Vec3::new(1.0, 1.0, -1.0), 0.5),
        Emissive::new(SolidColor::new(Color::new(30.0, 15.0, 15.0))),
    );

    let image_width = 1200;
    let image_height = 675;
    let camera = Camera::new(
        PerspectiveParam {
            look_from: Vec3::new(12.0, 12.0, -6.0),
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
