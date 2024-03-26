// trace the cornell box and write it to conrell_box.png
extern crate pathtracer;

const MULTIPLIER: usize = 2;
const WIDTH: usize = 640 * MULTIPLIER;
const HEIGHT: usize = 360 * MULTIPLIER;
const SAMPLE_COUNT: usize = 256 * MULTIPLIER;
#[cfg(test)]
#[test]
fn cornell_box() {
    use pathtracer::scene::{FloatSize, Scene};

    let scene = Scene::cornell_box();
    let pathtracer = pathtracer::pathtracer::PathTracer::new(WIDTH, HEIGHT, SAMPLE_COUNT);
    let buffer = pathtracer.trace(&scene);

    let mut encoder = png::Encoder::new(
        std::fs::File::create("cornell_box.png").unwrap(),
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
}
