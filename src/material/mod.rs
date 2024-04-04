use crate::{
    object::HitRecord,
    ray::Ray,
    scene::{FloatSize, RNGType, PI},
    utils::{
        matrix::Matrix,
        vector::{Vec2, Vec3},
    },
};

use rand::prelude::*;

#[derive(Debug)]
pub struct Material {
    pub albedo: Vec3<FloatSize>,
    pub reflectivity: FloatSize,
    pub checkered: bool,
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

#[allow(dead_code)]
fn random_unit_vector(rand_state: &mut RNGType) -> (Vec3<FloatSize>, FloatSize) {
    fn pdf() -> FloatSize {
        1.0 / (4.0 * PI as FloatSize)
    }
    let theta: FloatSize = rand_state.gen_range(0.0..(2.0 * PI as FloatSize));
    let phi_cos: FloatSize = rand_state.gen_range(-1.0..=1.0);
    let phi_sin: FloatSize = (1.0 - phi_cos * phi_cos).sqrt();
    (
        Vec3::new([phi_sin * theta.cos(), phi_cos, phi_sin * theta.sin()]),
        pdf(),
    )
}

pub enum SamplingFunctions {
    RandomUnitVector,
    CosineWeightedSample1,
    CosineWeightedSample2,
}

#[allow(dead_code)]
fn cosine_weighted_sample_1(
    normal: &Vec3<FloatSize>,
    rand_state: &mut RNGType,
) -> (Vec3<FloatSize>, FloatSize) {
    fn pdf(cos_theta: FloatSize) -> FloatSize {
        cos_theta / PI as FloatSize
    }
    let (v, u) = generate_coordinate_system(normal);
    let r1: FloatSize = rand_state.gen_range(0.0..1.0);
    let r2: FloatSize = rand_state.gen_range(0.0..1.0);

    let phi = 2.0 * PI as FloatSize * r1;
    let r = r2.sqrt();
    let x = r * phi.cos();
    let y = (1.0 - r2).sqrt(); // this is cos_theta
    let z = r * phi.sin();

    let local_sample = Vec3::new([x, y, z]);
    let transformation_matrix = Matrix::<FloatSize, 3, 3>::new_from_columns([u, *normal, v]);
    (
        transformation_matrix.multiply_by_vector(&local_sample),
        pdf(y),
    )
}

#[allow(dead_code)]
fn cosine_weighted_sample_2(
    normal: &Vec3<FloatSize>,
    rand_state: &mut RNGType,
) -> (Vec3<FloatSize>, FloatSize) {
    fn pdf(cos_theta: FloatSize) -> FloatSize {
        cos_theta / PI as FloatSize
    }
    let (v, u) = generate_coordinate_system(normal);
    let r1: FloatSize = rand_state.gen_range(0.0..1.0);
    let r2: FloatSize = rand_state.gen_range(0.0..1.0);
    let cos_theta = r1.sqrt();
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    let phi = r2 * 2.0 * PI as FloatSize;

    let local_sample = Vec3::new([sin_theta * phi.cos(), cos_theta, sin_theta * phi.sin()]);
    let transformation_matrix = Matrix::<FloatSize, 3, 3>::new_from_columns([u, *normal, v]);

    (
        transformation_matrix.multiply_by_vector(&local_sample),
        pdf(cos_theta),
    )
}

impl Material {
    pub fn scatter(
        &self,
        hit_record: &HitRecord,
        rand_state: &mut RNGType,
        sampletype: &SamplingFunctions,
    ) -> (Ray, FloatSize) {
        let mut random = match sampletype {
            SamplingFunctions::RandomUnitVector => random_unit_vector(rand_state),
            SamplingFunctions::CosineWeightedSample1 => {
                cosine_weighted_sample_1(&hit_record.normal, rand_state)
            }
            SamplingFunctions::CosineWeightedSample2 => {
                cosine_weighted_sample_2(&hit_record.normal, rand_state)
            }
        };

        if random.0.dot(&hit_record.normal) < 0.0 {
            random.0 = random.0.scale(-1.0);
        }

        (
            Ray {
                origin: hit_record.point + random.0.scale(0.001),
                direction: random.0,
            },
            random.1,
        )
    }

    pub fn color(&self, uv: &Option<Vec2<FloatSize>>) -> Vec3<FloatSize> {
        if self.checkered {
            match uv {
                Some(uv) => {
                    let u = uv.x();
                    let v = uv.y();

                    // return Vec3::new([*u, *v, 0.0]);
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

    pub fn from_color(color: crate::utils::vector::Vec3<FloatSize>) -> Material {
        Material {
            albedo: color,
            reflectivity: 0.0,
            checkered: false,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::white()
    }
}

#[cfg(test)]
mod tests {
    use crate::pathtracer::get_rng;

    use super::*;

    #[allow(dead_code)]
    fn avg_cosine(samples: Vec<Vec3<FloatSize>>, normal: Vec3<FloatSize>) -> FloatSize {
        samples
            .iter()
            .map(|v| {
                let cos_theta = v.dot(&normal) / (v.length() * normal.length());
                cos_theta.max(0.0)
            })
            .sum::<FloatSize>()
            / samples.len() as FloatSize
    }

    #[test]
    fn test_cosine_weighted_sample_1_distribution() {
        let mut rng = get_rng();
        let normal = Vec3::new([0.0, 1.0, 0.0]);
        let samples: Vec<Vec3<FloatSize>> = (0..1000)
            .map(|_| cosine_weighted_sample_1(&normal, &mut rng).0)
            .collect();

        let average_cosine: FloatSize =
            samples.iter().map(|v| v.y()).sum::<FloatSize>() / samples.len() as FloatSize;
        let expected_average_cosine: FloatSize = 2.0 / PI as FloatSize;

        assert!(
            (average_cosine - expected_average_cosine).abs() < 0.05,
            "Average cosine: {}",
            average_cosine
        );
    }

    #[test]
    fn test_cosine_weighted_sample_2_distribution() {
        let mut rng = get_rng();
        let normal = Vec3::new([0.0, 1.0, 0.0]);
        let samples: Vec<Vec3<FloatSize>> = (0..1000)
            .map(|_| cosine_weighted_sample_2(&normal, &mut rng).0)
            .collect();

        let average_cosine: FloatSize =
            samples.iter().map(|v| v.y()).sum::<FloatSize>() / samples.len() as FloatSize;
        let expected_average_cosine: FloatSize = 2.0 / PI as FloatSize;

        assert!(
            (average_cosine - expected_average_cosine).abs() < 0.05,
            "Average cosine: {}",
            average_cosine
        );
    }
}
