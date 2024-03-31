use crate::scene::{FloatSize, Scene};
use crate::utils::vector::Vec3;
use rayon::prelude::*;

pub struct PathTracer {
    width: usize,
    height: usize,
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

    pub fn trace(&self, scene: &Scene) -> Vec<Vec3<FloatSize>> {
        let mut buffer = vec![Vec3::new([0.0, 0.0, 0.0]); self.width * self.height];
        buffer
            .par_chunks_mut(self.width)
            .enumerate()
            .for_each(|(y, row)| {
                (0..self.samples).for_each(|_sample| {
                    let mut rand_state = rand::thread_rng();
                    (0..self.width).for_each(|x| {
                        let ray = scene.camera.get_ray(
                            x as FloatSize,
                            y as FloatSize,
                            self.width as FloatSize,
                            self.height as FloatSize,
                            &mut rand_state,
                        );
                        let _is_left = x < self.width / 2;
                        let color = scene.trace_ray(&ray, 10, &mut rand_state, true);
                        row[x] += color.scale(1.0 / self.samples as FloatSize);
                    });
                });
            });
        buffer
    }
}
