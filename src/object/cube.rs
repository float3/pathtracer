use crate::{material::Material, ray::Ray, scene::FloatSize, utils::vector::Vec3};

use super::{HitRecord, Hittable};

#[derive(Debug)]
pub struct Cube {
    min: Vec3<FloatSize>,
    max: Vec3<FloatSize>,
    material: Material,
}

impl Cube {
    pub fn new(min: Vec3<FloatSize>, max: Vec3<FloatSize>, material: Material) -> Self {
        Cube { min, max, material }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: FloatSize, t_max: FloatSize) -> Option<HitRecord> {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction.0[i];
            let mut t0 = (self.min.0[i] - ray.origin.0[i]) * inv_d;
            let mut t1 = (self.max.0[i] - ray.origin.0[i]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return None;
            }
        }
        let point = ray.at(t_min);
        let outward_normal = Vec3::new([
            (point.0[0] - self.min.0[0]).min(self.max.0[0] - point.0[0]),
            (point.0[1] - self.min.0[1]).min(self.max.0[1] - point.0[1]),
            (point.0[2] - self.min.0[2]).min(self.max.0[2] - point.0[2]),
        ]);
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal.normalize()
        } else {
            -outward_normal.normalize()
        };
        Some(HitRecord {
            point,
            normal,
            t: t_min,
            front_face,
            material: &self.material,
            uv: None,
        })
    }
}
