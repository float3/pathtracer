use crate::{material::Material, ray::Ray, scene::Flooat, utils::vector::Float3};

use super::{HitRecord, Hittable};

#[derive(Debug)]
pub struct Cube {
    min: Float3,
    max: Float3,
    material: Material,
}

impl Cube {
    pub fn new(min: Float3, max: Float3, material: Material) -> Self {
        Cube { min, max, material }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: Flooat, t_max: Flooat) -> Option<HitRecord> {
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
        let outward_normal = Float3::new([
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
