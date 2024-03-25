use crate::{object::HitRecord, ray::Ray, scene::FloatSize, utils::vector::Vec3};

use rand::prelude::*;

type Color = Vec3<FloatSize>;

pub struct Material {
    albedo: Color,
    reflectivity: FloatSize,
}

impl Material {
    pub(crate) fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Ray> {
        let reflected = Self::reflect(&ray.direction.normalize(), &hit_record.normal);
        let scatter_direction = reflected + Material::random_unit_vector().scale(self.reflectivity);

        let scatter_direction = if scatter_direction.near_zero() {
            hit_record.normal
        } else {
            scatter_direction
        };
        Some(Ray {
            origin: hit_record.point,
            direction: scatter_direction,
        })
    }

    fn reflect(v: &Vec3<FloatSize>, n: &Vec3<FloatSize>) -> Vec3<FloatSize> {
        *v - n.scale(2.0 * v.dot(n))
    }

    fn random_unit_vector() -> Vec3<FloatSize> {
        let mut rng = thread_rng();
        let a: FloatSize = rng.gen_range(0.0..(2.0 * std::f64::consts::PI as FloatSize));
        let z: FloatSize = rng.gen_range(-1.0..=1.0);
        let r: FloatSize = (1.0 - z * z).sqrt();
        Vec3::new([r * a.cos(), r * a.sin(), z])
    }

    pub fn diffuse() -> Material {
        Material {
            albedo: Vec3::new([1.0, 1.0, 1.0]),
            reflectivity: 0.0,
        }
    }

    pub fn reflective() -> Material {
        Material {
            albedo: Vec3::new([1.0, 1.0, 1.0]),
            reflectivity: 1.0,
        }
    }

    pub fn mirror() -> Material {
        Material {
            albedo: Vec3::new([1.0, 1.0, 1.0]),
            reflectivity: 1.0,
        }
    }
}
