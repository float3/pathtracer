use crate::material::SamplingFunctions;
use crate::scene::{Float0, RNGType, Scene};
use crate::utils::vector::Float3;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

pub struct PathTracer {
    pub width: usize,
    pub height: usize,
    samples: usize,
    seed: Option<u64>,
}

impl PathTracer {
    pub fn new(width: usize, height: usize, samples: usize) -> Self {
        Self::with_seed(width, height, samples, None)
    }

    pub fn new_seeded(width: usize, height: usize, samples: usize, seed: u64) -> Self {
        Self::with_seed(width, height, samples, Some(seed))
    }

    fn with_seed(width: usize, height: usize, samples: usize, seed: Option<u64>) -> Self {
        Self {
            width,
            height,
            samples,
            seed,
        }
    }

    pub fn trace(&self, scene: &Scene, debug: bool) -> Vec<Float3> {
        let mut buffer = vec![Float3::new([0.0, 0.0, 0.0]); self.width * self.height];

        buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, pixel)| {
                let mut rand_state = self.rng_for_pixel(index);

                let x = index % self.width;
                let y = index / self.width;

                let mut color = Float3::new([0.0, 0.0, 0.0]);

                for _sample in 0..self.samples {
                    let ray = scene.camera.get_ray(
                        x as Float0,
                        y as Float0,
                        self.width as Float0,
                        self.height as Float0,
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

                *pixel = color.scale(1.0 / self.samples as Float0);
            });

        #[cfg(feature = "oidn")]
        self::denoise_image(self.width, self.height, &mut buffer);

        buffer
    }

    fn rng_for_pixel(&self, index: usize) -> RNGType {
        match self.seed {
            Some(seed) => seeded_rng(seed ^ (index as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)),
            None => get_rng(),
        }
    }
}

#[cfg(feature = "oidn")]
fn denoise_image(width: usize, height: usize, buffer: &mut [Float3]) {
    let mut binding: Vec<f32> = buffer
        .iter()
        .flat_map(|v| v.as_array().to_vec())
        .map(|x| x as f32)
        .collect();

    let input: &mut [f32] = binding.as_mut_slice();

    let device = oidn::Device::cpu();
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
        *pixel = Float3::new([slice[0] as Float0, slice[1] as Float0, slice[2] as Float0]);
    }
}

pub fn get_rng() -> RNGType {
    let mut rng = rand::rng();
    seeded_rng(rng.random())
}

pub fn seeded_rng(seed: u64) -> RNGType {
    rand::rngs::StdRng::seed_from_u64(seed)
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::seeded_rng;

    #[test]
    fn seeded_rng_repeats() {
        let mut a = seeded_rng(42);
        let mut b = seeded_rng(42);

        assert_eq!(a.random::<u64>(), b.random::<u64>());
        assert_eq!(a.random::<u64>(), b.random::<u64>());
    }
}
