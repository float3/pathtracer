extern crate pathtracer;

#[cfg(test)]
#[test]
fn sample_test() {
    use std::fs::File;
    use std::path::Path;

    use pathtracer::scene::Float0;
    use png::{BitDepth, ColorType, Encoder, text_metadata::ITXtChunk};

    let pathtracer = pathtracer::pathtracer::PathTracer::new_seeded(1280, 720, 256, 0);
    let toml_str = std::fs::read_to_string("tests/sample_test.toml").unwrap();
    let value = toml::from_str::<toml::Value>(&toml_str).unwrap();

    let scene = pathtracer::scene::Scene::from_toml(&value);
    let buffer = pathtracer.trace(&scene, true);

    let output_file = Path::new("target")
        .join("test-output")
        .join("sample_test.png");
    std::fs::create_dir_all(output_file.parent().unwrap()).unwrap();

    let mut encoder = Encoder::new(
        File::create(&output_file).unwrap(),
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
        Ok(_) => println!("Image written to {}", output_file.display()),
        Err(e) => eprintln!("Error writing image: {e}"),
    }

    let tail = ITXtChunk::new("scene", &toml_str);
    writer.write_text_chunk(&tail).unwrap();
}
