use ::pathtracer::{pathtracer::PathTracer, scene::Scene};
use minifb::{Key, Scale, Window, WindowOptions};
use pathtracer::scene::FloatSize;

use toml::Value;

const MULTIPLIER: usize = 2;
const WIDTH: usize = 640 * MULTIPLIER;
const HEIGHT: usize = 360 * MULTIPLIER;
const SAMPLE_COUNT: usize = 256 * MULTIPLIER;
fn main() {
    let buffer = {
        /*let scene = Scene {
            objects: vec![
                Box::new(Sphere::new(
                    Vec3::new([0.0, 1.5, 0.0]),
                    1.0,
                    Material::reflective(),
                )),
                Box::new(Sphere::new(
                    Vec3::new([2.0, 1.0, 0.0]),
                    1.0,
                    Material::red(),
                )),
                Box::new(Sphere::new(
                    Vec3::new([-2.0, 1.0, 0.0]),
                    1.0,
                    Material::red(),
                )),
                Box::new(Sphere::new(
                    Vec3::new([0.0, 2000.0, 100.0]),
                    900.0,
                    Material::red(),
                )),
                Box::new(Quad::new(
                    Vec3::new([1.0, 0.0, 1.0]),
                    Vec3::new([1.0, 0.0, -1.0]),
                    Vec3::new([-1.0, 0.0, -1.0]),
                    Vec3::new([-1.0, 0.0, 1.0]),
                    Material::checkered(),
                )),
            ],
            lights: vec![Box::new(PointLight::new(
                Vec3::new([0.0, 5.0, 0.0]),
                Vec3::new([1.0, 1.0, 1.0]),
            ))],
            camera: Camera::new(
                Vec3::new([0.0, 0.5, 3.0]),
                Vec3::new([0.0, 0.0, -1.0]),
                // Vec3::new([0.0, 1.0, 0.0]),
                // 90.0,
                // WIDTH as f32 / HEIGHT as f32,
                // 0.1,
                // 100.0,
            ),
            skybox: pathtracer::skybox::Skybox {
                color: Vec3::new([0.1, 0.1, 0.1]),
            },
        }; */

        let toml_str = std::fs::read_to_string("scene.toml").expect("Failed to read scene.toml");
        let value = toml::from_str::<Value>(&toml_str).expect("Failed to parse TOML file");

        let scene = Scene::from_toml(&value);

        let pathtracer = PathTracer::new(WIDTH, HEIGHT, SAMPLE_COUNT);
        pathtracer.trace(&scene)
    };

    let mut encoder = png::Encoder::new(
        std::fs::File::create("output.png").unwrap(),
        WIDTH as u32,
        HEIGHT as u32,
    );
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let modbuffer = &buffer
        .iter()
        .flat_map(|color| {
            let color = color.scale(255.0 as FloatSize);
            vec![(color.0[0]) as u8, (color.0[1]) as u8, (color.0[2]) as u8]
        })
        .collect::<Vec<u8>>();

    match writer.write_image_data(modbuffer) {
        Ok(_) => println!("Image written to output.png"),
        Err(e) => eprintln!("Error writing image: {}", e),
    }

    let mut window = match Window::new(
        "Test Window",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    ) {
        Ok(window) => window,
        Err(e) => {
            eprintln!("Error creating window: {}", e);
            return;
        }
    };

    let packed_buffer = buffer
        .iter()
        .map(|color| {
            let r = (color.0[0].clamp(0.0, 1.0) * 255.0) as u32;
            let g = (color.0[1].clamp(0.0, 1.0) * 255.0) as u32;
            let b = (color.0[2].clamp(0.0, 1.0) * 255.0) as u32;

            (r << 16) | (g << 8) | b
        })
        .collect::<Vec<u32>>();

    match window.update_with_buffer(&packed_buffer, WIDTH, HEIGHT) {
        Ok(_) => println!("Window updated"),
        Err(e) => {
            eprintln!("Error updating window: {}", e);
            return;
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
