use ::pathtracer::{pathtracer::PathTracer, scene::Scene};
use pathtracer::scene::FloatSize;

use png::text_metadata::ITXtChunk;
use toml::Value;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (multiplier, args_offset) = if args.len() > 1 && args[1].starts_with("--multiplier=") {
        let multiplier_str = &args[1]["--multiplier=".len()..];
        let multiplier: usize = multiplier_str
            .parse()
            .expect("Multiplier must be a positive integer");
        (multiplier, 2)
    } else {
        (1, 1)
    };

    let width = 1280 * multiplier;
    let height = 720 * multiplier;
    let sample_count = 512 * multiplier;

    let pathtracer = PathTracer::new(width, height, sample_count);

    match args.len() - args_offset {
        0 => {
            trace_scene_file("scene.toml", "output.png", &pathtracer);
        }
        1 if args[args_offset] == "--all" => {
            trace_all_scenes(&pathtracer);
        }
        1 => {
            let scene_file = &args[args_offset];
            let output_file = format!("{}.png", scene_file.trim_end_matches(".toml"));
            trace_scene_file(scene_file, &output_file, &pathtracer);
        }
        _ => {
            println!("Usage: pathtracer [--multiplier=N] [scene_file.toml] or --all");
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
        pathtracer.width as u32,
        pathtracer.height as u32,
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
                    let output_file = if path.file_name().unwrap() != "scene.toml" {
                        format!("{}.png", path.file_stem().unwrap().to_str().unwrap())
                    } else {
                        "output.png".to_string()
                    };
                    trace_scene_file(scene_file, &output_file, pathtracer);
                }
            }
        }
    }
}
