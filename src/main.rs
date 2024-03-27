use ::pathtracer::{pathtracer::PathTracer, scene::Scene};
use pathtracer::scene::FloatSize;

use png::text_metadata::ITXtChunk;
use toml::Value;

const MULTIPLIER: usize = 2;
const WIDTH: usize = 640 * MULTIPLIER;
const HEIGHT: usize = 360 * MULTIPLIER;
const SAMPLE_COUNT: usize = 256 * MULTIPLIER;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pathtracer = PathTracer::new(WIDTH, HEIGHT, SAMPLE_COUNT);

    match args.len() {
        1 => {
            trace_scene_file("scene.toml", "output.png", &pathtracer);
        }
        2 if args[1] == "--all" => {
            trace_all_scenes(&pathtracer);
        }
        2 => {
            let scene_file = &args[1];
            let output_file = format!("{}.png", scene_file.trim_end_matches(".toml"));
            trace_scene_file(scene_file, &output_file, &pathtracer);
        }
        _ => {
            println!("Usage: program_name [scene_file.toml] or --all");
        }
    }
}

fn trace_scene_file(scene_file: &str, output_file: &str, pathtracer: &PathTracer) {
    let toml_str = std::fs::read_to_string(scene_file).expect("Failed to read scene.toml");
    let value = toml::from_str::<Value>(&toml_str).expect("Failed to parse TOML file");
    let buffer = {
        let scene = Scene::from_toml(&value);
        pathtracer.trace(&scene)
    };

    let mut encoder = png::Encoder::new(
        std::fs::File::create(output_file).unwrap(),
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
        Ok(_) => println!("Image written to {}", output_file),
        Err(e) => eprintln!("Error writing image: {}", e),
    }

    let tail = ITXtChunk::new("scene", &toml_str);
    writer.write_text_chunk(&tail).unwrap();
}

fn trace_all_scenes(pathtracer: &PathTracer) {
    let entries = fs::read_dir(".").unwrap_or_else(|err| {
        panic!("Failed to read directory: {}", err);
    });

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "toml" && path.file_name().unwrap() != "Cargo.toml" {
                    let scene_file = path.to_str().unwrap();
                    let output_file =
                        format!("{}.png", path.file_stem().unwrap().to_str().unwrap());
                    trace_scene_file(scene_file, &output_file, pathtracer);
                }
            }
        }
    }
}
