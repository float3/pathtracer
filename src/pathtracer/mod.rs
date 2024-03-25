use crate::scene::Scene;
use crate::utils::vector::Vec3;
use rayon::prelude::*;

pub struct PathTracer {
    width: usize,
    height: usize,
}

impl PathTracer {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn trace(&self, scene: &Scene) -> Vec<u32> {
        let mut buffer = vec![Vec3::new([0.0, 0.0, 0.0]); self.width * self.height];

        buffer
            .par_chunks_mut(self.width)
            .enumerate()
            .for_each(|(y, row)| {
                (0..self.width).for_each(|x| {
                    let ray = scene
                        .camera
                        .get_ray(x as f32 / self.width as f32, y as f32 / self.height as f32);
                    let color = scene.trace_ray(&ray, 10);
                    row[x] = color;
                });
            });

        let packed_buffer = buffer
            .iter()
            .map(|color| {
                let r = (color.0[0].clamp(0.0, 1.0) * 255.0) as u32;
                let g = (color.0[1].clamp(0.0, 1.0) * 255.0) as u32;
                let b = (color.0[2].clamp(0.0, 1.0) * 255.0) as u32;

                (r << 16) | (g << 8) | b
            })
            .collect::<Vec<u32>>();

        packed_buffer
    }
}
