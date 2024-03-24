use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

// #[tokio::main]
fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    // let buffer: Vec<u32> = task::spawn(async {
    //     // Here you would call your path tracing function that returns a Vec<u32>
    //     // For the sake of this example, we'll just create a dummy buffer
    //     let width = 800;
    //     let height = 600;
    //     vec![0u32; width * height]
    // })
    // .await
    // .unwrap();

    // let width = 800;
    // let height = 600;

    // let mut window = Window::new(
    //     "Test Window",
    //     width,
    //     height,
    //     WindowOptions {
    //         scale: Scale::X2,
    //         ..WindowOptions::default()
    //     },
    // )
    // .unwrap_or_else(|e| {
    //     panic!("{}", e);
    // });

    // while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
    //     window.update_with_buffer(&buffer, width, height).unwrap();
    // }
}
