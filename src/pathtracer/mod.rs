use crate::material::SamplingFunctions;
use crate::scene::{FloatSize, Scene};
use crate::utils::vector::Vec3;
use rayon::prelude::*;

use rand::{rngs::SmallRng, SeedableRng};

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
                let mut rand_state = SmallRng::from_entropy();

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
                    let _is_left = x < self.width / 2;

                    let sample_type = if debug {
                        if _is_left {
                            SamplingFunctions::CosineWeightedSample1
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

        buffer
    }
}
