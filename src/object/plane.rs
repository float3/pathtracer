use crate::{material::Material, ray::Ray, scene::FloatSize, utils::vector::Vec3};

use super::{HitRecord, Hittable};

pub struct Plane {
    point: Vec3<FloatSize>,
    normal: Vec3<FloatSize>,
    material: Material,
}

impl Plane {
    pub fn new(point: Vec3<FloatSize>, normal: Vec3<FloatSize>, material: Material) -> Self {
        Plane {
            point,
            normal,
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: FloatSize, t_max: FloatSize) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction);
        if denom.abs() > 1e-6 {
            let v = self.point - ray.origin;
            let distance = v.dot(&self.normal) / denom;
            if distance >= t_min && distance <= t_max {
                return Some(HitRecord {
                    point: ray.at(distance),
                    normal: self.normal,
                    t: distance,
                    front_face: true,
                    material: &self.material,
                });
            }
        }
        None
    }
}
