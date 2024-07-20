use raytracer::camera::{Camera, ImageParam, LensParam, PerspectiveParam};
use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::material::{Dielectric, Translucent};
use raytracer::mesh::MeshObject;
use raytracer::raytracer::RayTracer;
use raytracer::shape::Sphere;
use raytracer::texture::{ImageTexture, SolidColor, TexturedMaterial};
use raytracer::vec3::Vec3;
use raytracer::world::WorldBuilder;

fn main() {
    let mut world = WorldBuilder::default();
    world.set_background(Color::new(0.2, 0.3, 0.4));
    let mut mesh = MeshObject::from_obj("assets/test.obj");
    println!("Mesh: {:?}", mesh.keys());
    world.add(
        mesh.remove("key1_up")
            .unwrap()
            .set_material(Translucent::new(1.1))
            .set_transparency(ImageTexture::new("assets/key1_alpha.png")),
    );

    world.add(
        mesh.remove("key1_down")
            .unwrap()
            .set_material(Translucent::new(1.1))
            .set_transparency(ImageTexture::new("assets/key1_alpha.png")),
    );

    world.add(
        mesh.remove("key1_mid")
            .unwrap()
            .set_transparency(ImageTexture::new("assets/key1_icon_alpha.png"))
            .set_attenuation(SolidColor::new(Color::BLACK)),
    );

    world.add(
        mesh.remove("key1_curve")
            .unwrap()
            .set_material(Dielectric::new(1.1)),
    );

    world.add_object(
        Sphere::new(Vec3::new(1.0, 3.0, -1.0), 0.5),
        TexturedMaterial::default().set_emission(SolidColor::new(Color::new(30.0, 15.0, 0.0))),
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
