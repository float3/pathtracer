use pathtracer::{
    pathtracer::PathTracer,
    scene::{Float0, Scene},
    utils::vector::Vector,
};

use png::{text_metadata::ITXtChunk, BitDepth, ColorType, Encoder};
use toml::Value;

use std::fs::{self, File};
use std::{env, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let (multiplier, args_offset) = if args.len() > 1 && args[1].starts_with("--multiplier=") {
        let multiplier_str: &str = &args[1]["--multiplier=".len()..];
        let multiplier: usize = multiplier_str
            .parse()
            .expect("Multiplier must be a positive integer");
        (multiplier, 2)
    } else {
        (1, 1)
    };

    let width: usize = 1280 * multiplier;
    let height: usize = 720 * multiplier;
    let sample_count: usize = 256 * multiplier;

    let pathtracer = PathTracer::new(width, height, sample_count);

    match args.len() - args_offset {
        0 => {
            trace_scene_file("scenes/scene.toml", "renders/scene.png", &pathtracer);
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
    let toml_str: String = fs::read_to_string(scene_file).expect("Failed to read scene.toml");
    let value: Value = toml::from_str::<Value>(&toml_str).expect("Failed to parse TOML file");
    let buffer: Vec<Vector<f64, 3>> = {
        let scene = Scene::from_toml(&value);
        pathtracer.trace(&scene, false)
    };

    let mut encoder = Encoder::new(
        File::create(output_file).unwrap(),
        pathtracer.width as u32,
        pathtracer.height as u32,
    );
    encoder.set_color(ColorType::Rgb);
    encoder.set_depth(BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    let modbuffer = &buffer
        .iter()
        .flat_map(|color| {
            let color = color.scale(255.0 as Float0);
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
    let scenes_dir = Path::new("./scenes");

    let entries = match fs::read_dir(scenes_dir) {
        Ok(entries) => entries,
        Err(err) => panic!("Failed to read directory: {}", err),
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Failed to access directory entry: {}", err);
                continue;
            }
        };

        let path = entry.path();
        if path.is_file()
            && path.extension() == Some("toml".as_ref())
            && path.file_name() != Some("Cargo.toml".as_ref())
        {
            let scene_file = match path.to_str() {
                Some(path_str) => path_str,
                None => {
                    eprintln!("Invalid UTF-8 in file path: {:?}", path);
                    continue;
                }
            };

            let output_file = path.file_stem().map_or_else(
                || "renders/output.png".to_string(),
                |stem| format!("renders/{}.png", stem.to_string_lossy()),
            );

            trace_scene_file(scene_file, &output_file, pathtracer);
        }
    }
}
