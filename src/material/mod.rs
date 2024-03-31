use crate::{
    object::HitRecord,
    ray::Ray,
    scene::{FloatSize, RNGType},
    utils::vector::{Vec2, Vec3},
};

use rand::prelude::*;

#[derive(Debug)]
pub struct Material {
    pub albedo: Vec3<FloatSize>,
    pub reflectivity: FloatSize,
    pub checkered: bool,
}

#[allow(dead_code)]
fn random_unit_vector(rand_state: &mut RNGType) -> Vec3<FloatSize> {
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

fn cosine_weighted_sample_1(normal: &Vec3<FloatSize>, rand_state: &mut RNGType) -> Vec3<FloatSize> {
    let (v, u) = generate_coordinate_system(normal);
    let r1: FloatSize = rand_state.gen_range(0.0..1.0);
    let r2: FloatSize = rand_state.gen_range(0.0..1.0);

    let phi = 2.0 * std::f64::consts::PI as FloatSize * r1;
    let r = r2.sqrt();
    let x = r * phi.cos();
    // let y = (1.0 - x * x - y * y).sqrt();
    let y = (1.0 - r2).sqrt();
    let z = r * phi.sin();

    let a = Vec3::new([x, y, z]);
    Vec3::new([
        a.x() * u.x() + a.y() * normal.x() + a.z() * v.x(),
        a.x() * u.y() + a.y() * normal.y() + a.z() * v.y(),
        a.x() * u.z() + a.y() * normal.z() + a.z() * v.z(),
    ])
}

fn cosine_weighted_sample_2(normal: &Vec3<FloatSize>, rand_state: &mut RNGType) -> Vec3<FloatSize> {
    let (v, u) = generate_coordinate_system(normal);
    let r1: FloatSize = rand_state.gen_range(0.0..1.0);
    let r2: FloatSize = rand_state.gen_range(0.0..1.0);
    let theta = r1.sqrt().acos();
    let phi = r2 * 2.0 * std::f64::consts::PI as FloatSize;
    let a = Vec3::new([phi.sin() * theta.cos(), phi.cos(), phi.sin() * theta.sin()]);
    Vec3::new([
        a.x() * u.x() + a.y() * normal.x() + a.z() * v.x(),
        a.x() * u.y() + a.y() * normal.y() + a.z() * v.y(),
        a.x() * u.z() + a.y() * normal.z() + a.z() * v.z(),
    ])
}

impl Material {
    pub fn scatter(&self, hit_record: &HitRecord, rand_state: &mut RNGType, is_left: bool) -> Ray {
        let mut random = if is_left {
            cosine_weighted_sample_2(&hit_record.normal, rand_state)
        } else {
            cosine_weighted_sample_1(&hit_record.normal, rand_state)
            // random_unit_vector(rand_state)
        };
        if random.dot(&hit_record.normal) < 0.0 {
            random = random.scale(-1.0);
        }

        Ray {
            origin: hit_record.point + random.scale(0.001),
            direction: random,
        }
    }

    pub fn color(&self, uv: &Option<Vec2<FloatSize>>) -> Vec3<FloatSize> {
        if self.checkered {
            match uv {
                Some(uv) => {
                    let u = uv.x();
                    let v = uv.y();

                    // return Vec3::new([*u, *v, 0.0]);
                    // checkered pattern
                    if (((u * 10.0).floor() as i32) + ((v * 10.0).floor() as i32)) % 2 == 0 {
                        Vec3::new([0.0, 0.0, 0.0])
                    } else {
                        Vec3::new([1.0, 1.0, 1.0])
                    }
                }
                None => self.albedo,
            }
        } else {
            self.albedo
        }
    }

    pub fn reflect(v: &Vec3<FloatSize>, n: &Vec3<FloatSize>) -> Vec3<FloatSize> {
        *v - n.scale(2.0 * v.dot(n))
    }

    pub fn reflective() -> Material {
        Material {
            albedo: Vec3::new([1.0, 1.0, 1.0]),
            reflectivity: 1.0,
            checkered: false,
        }
    }

    pub fn red() -> Material {
        Material {
            albedo: Vec3::new([1.0, 0.0, 0.0]),
            reflectivity: 0.0,
            checkered: false,
        }
    }

    pub fn green() -> Material {
        Material {
            albedo: Vec3::new([0.0, 1.0, 0.0]),
            reflectivity: 0.0,
            checkered: false,
        }
    }

    pub fn blue() -> Material {
        Material {
            albedo: Vec3::new([0.0, 0.0, 1.0]),
            reflectivity: 0.0,
            checkered: false,
        }
    }

    pub fn white() -> Material {
        Material {
            albedo: Vec3::new([1.0, 1.0, 1.0]),
            reflectivity: 0.0,
            checkered: false,
        }
    }

    pub fn checkered() -> Material {
        Material {
            albedo: Vec3::new([1.0, 1.0, 1.0]),
            reflectivity: 0.0,
            checkered: true,
        }
    }

    pub fn black() -> Material {
        Material {
            albedo: Vec3::new([0.0, 0.0, 0.0]),
            reflectivity: 0.0,
            checkered: false,
        }
    }

    pub(crate) fn from_toml(object: &toml::Value) -> Material {
        match object.as_str().unwrap() {
            "reflective" => Material::reflective(),
            "red" => Material::red(),
            "green" => Material::green(),
            "blue" => Material::blue(),
            "white" => Material::white(),
            "checkered" => Material::checkered(),
            "black" => Material::black(),
            _ => todo!(),
        }
    }

    pub fn default() -> Material {
        Material::white()
    }

    pub fn from_color(color: crate::utils::vector::Vec3<FloatSize>) -> Material {
        Material {
            albedo: color,
            reflectivity: 0.0,
            checkered: false,
        }
    }
}
