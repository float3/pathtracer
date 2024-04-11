use crate::{material::Material, ray::Ray, scene::Flooat, utils::vector::Float3};

use super::{HitRecord, Hittable};

#[derive(Debug)]
pub struct Plane {
    point: Float3,
    normal: Float3,
    material: Material,
}

impl Plane {
    pub fn new(point: Float3, normal: Float3, material: Material) -> Self {
        Plane {
            point,
            normal: normal.normalize(),
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: Flooat, t_max: Flooat) -> Option<HitRecord> {
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
                    uv: None,
                });
            }
        }
        None
    }
}
