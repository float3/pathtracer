use ::pathtracer::{pathtracer::PathTracer, scene::Scene};
use minifb::{Key, Scale, Window, WindowOptions};
use pathtracer::{
    camera::Camera,
    light::pointlight::PointLight,
    material::Material,
    object::{plane::Plane, sphere::Sphere},
    utils::vector::Vec3,
};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut window = Window::new(
        "Test Window",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let buffer: Vec<u32> = {
        let scene = Scene {
            objects: vec![
                Box::new(Sphere::new(
                    Vec3::new([0.0, 1.0, 0.0]),
                    1.0,
                    Material::mirror(),
                )),
                Box::new(Plane::new(
                    Vec3::new([0.0, 0.0, 0.0]),
                    Vec3::new([0.0, 1.0, 0.0]),
                    Material::diffuse(),
                )),
            ],
            lights: vec![Box::new(PointLight::new(
                Vec3::new([0.0, 3.0, 0.0]),
                Vec3::new([1.0, 1.0, 1.0]),
            ))],
            camera: Camera::new(
                Vec3::new([0.0, 0.0, 3.0]),
                Vec3::new([0.0, 0.0, -1.0]),
                Vec3::new([0.0, 1.0, 0.0]),
                90.0,
                WIDTH as f32 / HEIGHT as f32,
                0.1,
                100.0,
            ),
        };

        let pathtracer = PathTracer::new(WIDTH, HEIGHT);
        pathtracer.trace(&scene)
    };
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
