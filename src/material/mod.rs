use crate::{object::HitRecord, ray::Ray, scene::FloatSize, utils::vector::Vec3};

use rand::prelude::*;

#[derive(Debug)]
pub struct Material {
    pub albedo: Vec3<FloatSize>,
    pub reflectivity: FloatSize,
}

fn random_unit_vector(rand_state: &mut ThreadRng) -> Vec3<FloatSize> {
    let theta: FloatSize = rand_state.gen_range(0.0..(2.0 * std::f64::consts::PI as FloatSize));
    let phi_cos: FloatSize = rand_state.gen_range(-1.0..=1.0);
    let phi_sin: FloatSize = (1.0 - phi_cos * phi_cos).sqrt();
    Vec3::new([phi_sin * theta.cos(), phi_cos, phi_sin * theta.sin()])
}

fn generate_coordinate_system(normal: &Vec3<FloatSize>) -> (Vec3<FloatSize>, Vec3<FloatSize>) {
    let w = *normal;
    let a = if w.x().abs() > 0.9 {
        Vec3::new([0.0, 1.0, 0.0])
    } else {
        Vec3::new([1.0, 0.0, 0.0])
    };
    // let a = Vec3::new([1.0, 1.0, 3.0]);
    let u = w.cross(&a).normalize();
    let v = w.cross(&u);
    (u, v)
}

fn cosine_weighted_sample(normal: &Vec3<FloatSize>, rand_state: &mut ThreadRng) -> Vec3<FloatSize> {
    let (v, u) = generate_coordinate_system(normal);
    let r1: FloatSize = rand_state.gen_range(0.0..1.0);
    let r2: FloatSize = rand_state.gen_range(0.0..1.0);

    let phi = 2.0 * std::f64::consts::PI * r1;
    let r = r2.sqrt();
    let x = r * phi.cos();
    let y = r * phi.sin();

    let z = (1.0 - x * x - y * y).sqrt();

    let a = Vec3::new([x, y, z]);
    Vec3::new([
        a.x() * u.x() + a.y() * normal.x() + a.z() * v.x(),
        a.x() * u.y() + a.y() * normal.y() + a.z() * v.y(),
        a.x() * u.z() + a.y() * normal.z() + a.z() * v.z(),
    ])
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rand_state: &mut ThreadRng) -> Ray {
        // let reflected = Self::reflect(&ray.direction.normalize(), &hit_record.normal);
        // let scatter_direction = reflected + Material::random_unit_vector().scale(self.reflectivity);

        // let scatter_direction = if scatter_direction.near_zero() {
        //     hit_record.normal
        // } else {
        //     scatter_direction
        // };
        // Ray {
        //     origin: hit_record.point,
        //     direction: scatter_direction,
        // }
        let random = cosine_weighted_sample(&hit_record.normal, rand_state);
        // if random.dot(&hit_record.normal) < 0.0 {
        //     random = random.scale(-1.0);
        // }

        Ray {
            origin: hit_record.point + random.scale(0.001),
            direction: random,
        }
    }

    fn reflect(v: &Vec3<FloatSize>, n: &Vec3<FloatSize>) -> Vec3<FloatSize> {
        *v - n.scale(2.0 * v.dot(n))
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

    pub fn checkered() -> Material {
        Material {
            albedo: Vec3::new([1.0, 1.0, 1.0]),
            reflectivity: 0.0,
        }
    }
}
