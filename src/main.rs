use raytracer::bvh::ShapeList;
use raytracer::camera::{Camera, ImageParam, LensParam, PerspectiveParam};
use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::hittable::WorldBuilder;
use raytracer::material::{Dielectric, Emissive, Isotropic, Lambertian, Metal, Translucent};
use raytracer::mesh::Mesh;
use raytracer::raytracer::RayTracer;
use raytracer::shape::{ConstantMedium, Sphere};
use raytracer::texture::{Atlas, ImageTexture, SolidColor};
use raytracer::vec3::Vec3;

fn main() {
    let mut world = WorldBuilder::default();
    let mut mesh = Mesh::load_obj("assets/test2.obj");
    println!("Mesh: {:?}", mesh.get_names());
    world.set_background(Color::new_u8(0x7F, 0xFF, 0xD4) * 0.5);
    world.add_object(
        mesh.remove_shape("deco1"),
        Metal::new(0.1),
        Atlas::default().set_attenuation(SolidColor::new(Color::new_u8(0xC0, 0xC0, 0xC0))),
    );
    world.add_object(
        mesh.remove_shape("deco2"),
        Metal::new(0.1),
        Atlas::default().set_attenuation(SolidColor::new(Color::new_u8(236, 197, 192))),
    );
    world.add_object(
        mesh.remove_shape("key1_chain"),
        Dielectric::new(1.5),
        Atlas::default(),
    );
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
        mesh.remove_shape("key2_chain"),
        Lambertian,
        Atlas::default().set_attenuation(SolidColor::new(Color::WHITE)),
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
        mesh.remove_shape("key3_chain"),
        Lambertian,
        Atlas::default()
            .set_attenuation(SolidColor::new(Color::BLACK)),
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
        Atlas::default(),
    );
    let mut boundary = ShapeList::default();
    boundary.push(mesh.remove_shape("key3_up_boundary"));
    boundary.push(mesh.remove_shape("key3_down_boundary"));
    boundary.push(mesh.remove_shape("key3_side_boundary"));
    world.add_object(
        ConstantMedium::new(5.0, boundary),
        Isotropic::new(0.4),
        Atlas::default()
            .set_transparency(ImageTexture::new("assets/key3_alpha.png"))
            .set_attenuation(SolidColor::new(Color::new_u8(51, 102, 153)))
            .set_emission(SolidColor::new(Color::new_u8(51, 102, 153))),
    );
    world.add_object(
        mesh.remove_shape("key4_chain"),
        Metal::new(0.2),
        Atlas::default(),
    );
    world.add_object(
        mesh.remove_shape("key4_in_up"),
        Metal::new(0.5),
        Atlas::default().set_transparency(ImageTexture::new("assets/key4_in_alpha.png"))
            .set_normal(ImageTexture::new("assets/key4_in_normal.png"))
            .set_attenuation(SolidColor::new(Color::new(0.804, 0.498, 0.196))),
    );
    world.add_object(
        mesh.remove_shape("key4_in_down"),
        Metal::new(0.5),
        Atlas::default().set_transparency(ImageTexture::new("assets/key4_in_alpha.png"))
            .set_attenuation(SolidColor::new(Color::new(0.804, 0.498, 0.196))),
    );
    world.add_object(
        mesh.remove_shape("key4_in_side"),
        Metal::new(0.5),
        Atlas::default().set_attenuation(SolidColor::new(Color::new(0.804, 0.498, 0.196))),
    );
    world.add_object(
        mesh.remove_shape("key4_out_up"),
        Metal::new(0.5),
        Atlas::default().set_transparency(ImageTexture::new("assets/key4_out_alpha.png"))
            .set_normal(ImageTexture::new("assets/key4_out_normal.png"))
            .set_attenuation(SolidColor::new(Color::new(0.804, 0.498, 0.196))),
    );
    world.add_object(
        mesh.remove_shape("key4_out_down"),
        Metal::new(0.5),
        Atlas::default().set_transparency(ImageTexture::new("assets/key4_out_alpha.png"))
            .set_attenuation(SolidColor::new(Color::new(0.804, 0.498, 0.196))),
    );
    world.add_object(
        mesh.remove_shape("key4_out_side"),
        Metal::new(0.5),
        Atlas::default().set_attenuation(SolidColor::new(Color::new(0.804, 0.498, 0.196))),
    );
    world.add_object(
        mesh.remove_shape("key4_ring"),
        Metal::new(0.5),
        Atlas::default().set_attenuation(SolidColor::new(Color::new(0.804, 0.498, 0.196))),
    );
    world.add_object(
        mesh.remove_shape("key4_cylinder"),
        Metal::new(0.5),
        Atlas::default().set_attenuation(SolidColor::new(Color::new(0.804, 0.498, 0.196))),
    );
    world.add_object(
        mesh.remove_shape("key5_chain"),
        Metal::new(0.2),
        Atlas::default(),
    );
    world.add_object(
        mesh.remove_shape("key5"),
        Metal::new(0.2),
        Atlas::default().set_attenuation(SolidColor::new(Color::new(1.0, 0.84, 0.0))),
    );
    world.add_object(
        mesh.remove_shape("box_back"),
        Emissive::new(1.0),
        Atlas::default().set_emission(ImageTexture::new("assets/emission.png")),
    );
    world.add_object(
        mesh.remove_shape("box_down"),
        Metal::new(0.5),
        Atlas::default().set_attenuation(ImageTexture::new("assets/wood.png")),
    );
    world.add_object(
        Sphere::new(Vec3::new(6.5, 13.0, 0.0), 1.0),
        Emissive::new(20.0),
        Atlas::default().set_emission(SolidColor::new(Color::new_u8(253, 94, 83))),
    );
    world.add_object(
        ConstantMedium::new(0.03, Sphere::new(Vec3::new(0.0, 4.0, 0.0), 3.75)),
        Isotropic::new(0.2),
        Atlas::default().set_attenuation(SolidColor::new(Color::new_u8(255, 0, 255))),
    );
    world.add_object(
        ConstantMedium::new(0.02, Sphere::new(Vec3::new(0.0, 4.0, 0.0), 7.5)),
        Isotropic::new(0.2),
        Atlas::default().set_attenuation(SolidColor::new(Color::new_u8(255, 0, 255))),
    );

    let image_width = 4000;
    let image_height = 2000;
    let camera = Camera::new(
        PerspectiveParam {
            look_from: Vec3::new(45.0, 12.0, 0.0),
            look_at: Vec3::new(0.0, 2.95, 0.0),
            view_up: Vec3::new(0.0, 1.0, 0.0), // y-axis is up
        },
        LensParam {
            fov: 28.0,
            defocus_angle: 1.5,
            focus_dist: 40.0,
        },
        ImageParam {
            image_width,
            image_height,
            sample_per_pixel: 1000,
        },
    );
    let picture = Canvas::empty(image_width, image_height);
    let raytracer = RayTracer::new(camera, picture, world.build(), 50);
    raytracer.render().save("output/final/final_final_scene.png");
}
