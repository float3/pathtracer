use crate::object::HitRecord;
use crate::ray::Ray;
use crate::scene::{FloatSize, Scene};
use crate::utils::vector::Vec3;

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

        for y in 0..self.height {
            for x in 0..self.width {
                let ray = scene.camera.get_ray(x as FloatSize, y as FloatSize);
                let color = self.trace_ray(scene, &ray, 10);
                buffer[y * self.width + x] = color;
            }
        }

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

    fn trace_ray(&self, scene: &Scene, ray: &Ray, depth: u32) -> Vec3<FloatSize> {
        if depth == 5 {
            return Vec3::new([0.0, 0.0, 0.0]);
        }

        if let Some(hit_record) = scene.hit(&ray, 0.001) {
            let color: Vec3<FloatSize> = scene.illuminate(&hit_record);

            let scattered = self.scatter(&ray, &hit_record);

            if let Some(scattered) = scattered {
                return color * self.trace_ray(scene, &scattered, depth + 1);
            }

            color
        } else {
            Vec3::new([0.0, 0.0, 0.0])
        }
    }

    fn scatter(&self, ray: &&Ray, hit_record: &HitRecord) -> Option<Ray> {
        hit_record.material.scatter(ray, hit_record)
    }
}
