use crate::material::SamplingFunctions;
use crate::scene::{FloatSize, RNGType, Scene};
use crate::utils::vector::Vec3;
use rayon::prelude::*;

pub struct PathTracer {
    pub width: usize,
    pub height: usize,
    samples: usize,
}

impl PathTracer {
    pub fn new(width: usize, height: usize, samples: usize) -> Self {
        Self {
            width,
            height,
            samples,
        }
    }

    pub fn trace(&self, scene: &Scene, debug: bool) -> Vec<Vec3<FloatSize>> {
        let mut buffer = vec![Vec3::new([0.0, 0.0, 0.0]); self.width * self.height];

        buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, pixel)| {
                let mut rand_state = get_rng();

                let x = index % self.width;
                let y = index / self.width;

                let mut color = Vec3::new([0.0, 0.0, 0.0]);

                for _sample in 0..self.samples {
                    let ray = scene.camera.get_ray(
                        x as FloatSize,
                        y as FloatSize,
                        self.width as FloatSize,
                        self.height as FloatSize,
                        &mut rand_state,
                    );
                    let is_left = x < self.width / 2;

                    let sample_type = if debug {
                        if is_left {
                            SamplingFunctions::RandomUnitVector
                        } else {
                            SamplingFunctions::CosineWeightedSample2
                        }
                    } else {
                        SamplingFunctions::CosineWeightedSample1
                    };
                    color += scene.trace_ray(&ray, 10, &mut rand_state, &sample_type);
                }

                *pixel = color.scale(1.0 / self.samples as FloatSize);
            });

        #[cfg(feature = "oidn")]
        self::denoise_image(self.width, self.height, &mut buffer);

        buffer
    }
}

#[cfg(feature = "oidn")]
fn denoise_image(width: usize, height: usize, buffer: &mut Vec<Vec3<FloatSize>>) {
    let mut binding: Vec<f32> = buffer
        .iter()
        .flat_map(|v| v.as_array().to_vec())
        .map(|x| x as f32)
        .collect();

    let input: &mut [f32] = binding.as_mut_slice();

    let device = oidn::Device::new();
    oidn::RayTracing::new(&device)
        .hdr(true)
        .srgb(false)
        .image_dimensions(width, height)
        .filter_in_place(input)
        .expect("Filter config error!");

    for (i, pixel) in buffer.iter_mut().enumerate() {
        let start = i * 3;
        let end = start + 3;
        let slice = &input[start..end];
        *pixel = Vec3::new([
            slice[0] as FloatSize,
            slice[1] as FloatSize,
            slice[2] as FloatSize,
        ]);
    }
}

pub fn get_rng() -> RNGType {
    #[cfg(not(feature = "small_rng"))]
    return rand::thread_rng();
    #[cfg(feature = "small_rng")]
    return <rand::rngs::SmallRng as rand::SeedableRng>::from_entropy();
}
