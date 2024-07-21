use raytracer::bvh::ShapeList;
use raytracer::camera::{Camera, ImageParam, LensParam, PerspectiveParam};
use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::hittable::WorldBuilder;
use raytracer::material::{Dielectric, Emissive, Isotropic, Lambertian, Translucent};
use raytracer::mesh::Mesh;
use raytracer::raytracer::RayTracer;
use raytracer::shape::{ConstantMedium, Sphere};
use raytracer::texture::{Atlas, ImageTexture, SolidColor};
use raytracer::vec3::Vec3;

fn main() {
    let mut world = WorldBuilder::default();
    let mut mesh = Mesh::load_obj("assets/test.obj");
    println!("Mesh: {:?}", mesh.get_names());
    world.add_object(
        mesh.remove_shape("key1_up"),
        Translucent::new(1.5),
        Atlas::default().set_transparency(ImageTexture::new("assets/key1_alpha.png")),
    );
    world.add_object(
        mesh.remove_shape("key1_down"),
        Translucent::new(1.5),
        Atlas::default().set_transparency(ImageTexture::new("assets/key1_alpha.png")),
    );
    world.add_object(
        mesh.remove_shape("key1_mid"),
        Lambertian,
        Atlas::default()
            .set_transparency(ImageTexture::new("assets/key1_icon_alpha.png"))
            .set_attenuation(SolidColor::new(Color::BLACK)),
    );
    world.add_object(
        mesh.remove_shape("key1_side"),
        Dielectric::new(1.5),
        Atlas::default(),
    );
    world.add_object(
        mesh.remove_shape("key2_up"),
        Dielectric::new(1.5),
        Atlas::default().set_transparency(ImageTexture::new("assets/key2_alpha.png")),
    );
    world.add_object(
        mesh.remove_shape("key2_down"),
        Dielectric::new(1.5),
        Atlas::default().set_transparency(ImageTexture::new("assets/key2_alpha.png")),
    );
    world.add_object(
        mesh.remove_shape("key2_mid"),
        Lambertian,
        Atlas::default()
            .set_transparency(ImageTexture::new("assets/key2_icon_alpha.png"))
            .set_attenuation(ImageTexture::new("assets/key2_icon.png")),
    );
    world.add_object(
        mesh.remove_shape("key2_side"),
        Dielectric::new(1.5),
        Atlas::default(),
    );
    world.add_object(
        mesh.remove_shape("key3_up"),
        Dielectric::new(1.5),
        Atlas::default()
            .set_transparency(ImageTexture::new("assets/key3_alpha.png")),
    );
    world.add_object(
        mesh.remove_shape("key3_down"),
        Dielectric::new(1.5),
        Atlas::default()
            .set_transparency(ImageTexture::new("assets/key3_alpha.png")),
    );
    world.add_object(
        mesh.remove_shape("key3_front"),
        Lambertian,
        Atlas::default()
            .set_transparency(ImageTexture::new("assets/key3_icon_alpha.png"))
            .set_attenuation(ImageTexture::new("assets/key3_icon.png")),
    );
    world.add_object(
        mesh.remove_shape("key3_side"),
        Dielectric::new(1.5),
        Atlas::default()
    );
    let mut boundary = ShapeList::default();
    boundary.push(mesh.remove_shape("key3_up_boundary"));
    boundary.push(mesh.remove_shape("key3_down_boundary"));
    boundary.push(mesh.remove_shape("key3_side_boundary"));
    world.add_object(
        ConstantMedium::new(5.0, boundary),
        Isotropic,
        Atlas::default()
            .set_transparency(ImageTexture::new("assets/key3_alpha.png"))
            .set_attenuation(SolidColor::new(Color::new_u8(51,102,153))),
    );
    world.add_object(
        mesh.remove_shape("box_back"),
        Lambertian,
        Atlas::default().set_attenuation(SolidColor::new(Color::WHITE)),
    );
    world.add_object(
        mesh.remove_shape("box_down"),
        Lambertian,
        Atlas::default().set_attenuation(SolidColor::new(Color::WHITE)),
    );
    world.add_object(
        Sphere::new(Vec3::new(0.0, 0.0, 3.0), 1.0),
        Emissive,
        Atlas::default().set_emission(SolidColor::new(Color::new(40.0, 30.0, 20.0))),
    );
    world.add_light(
        Sphere::new(Vec3::new(0.0, 0.0, 3.0), 1.0),
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
            sample_per_pixel: 500,
        },
    );
    let picture = Canvas::empty(image_width, image_height);
    let raytracer = RayTracer::new(camera, picture, world.build(), 50);
    raytracer.render().save("output/final/test5.png");
}
