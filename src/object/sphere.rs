use crate::{material::Material, ray::Ray, scene::Float0, utils::vector::Float3};

use super::{HitRecord, Hittable};

#[derive(Debug)]
pub struct Sphere {
    center: Float3,
    radius: Float0,
    material: Material,
}

impl Sphere {
    pub fn new(center: Float3, radius: Float0, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: Float0, t_max: Float0) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center).scale(1.0 / self.radius);
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal.normalize()
        } else {
            -outward_normal.normalize()
        };

        Some(HitRecord {
            point,
            normal,
            t: root,
            front_face,
            material: &self.material,
            uv: None,
        })
    }
}
