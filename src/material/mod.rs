use crate::{
    object::HitRecord,
    ray::Ray,
    scene::{Float0, RNGType, PI},
    utils::{
        matrix::Float3x3,
        vector::{Float2, Float3},
    },
};

use rand::prelude::*;

#[derive(Debug)]
pub struct Material {
    pub albedo: Float3,
    pub reflectivity: Float0,
    pub checkered: bool,
}

fn generate_coordinate_system(normal: &Float3) -> (Float3, Float3) {
    let w = *normal;
    let a = if w.x().abs() > 0.9 {
        Float3::new([0.0, 1.0, 0.0])
    } else {
        Float3::new([1.0, 0.0, 0.0])
    };
    // let a = Vec3::new([1.0, 1.0, 3.0]);
    let u = w.cross(&a).normalize();
    let v = w.cross(&u);
    (u, v)
}

#[allow(dead_code)]
fn random_unit_vector(rand_state: &mut RNGType) -> (Float3, Float0) {
    fn pdf() -> Float0 {
        1.0 / (4.0 * PI as Float0)
    }
    let theta: Float0 = rand_state.random_range(0.0..(PI as Float0));
    let phi: Float0 = rand_state.random_range(0.0..(2.0 * PI as Float0));
    (
        Float3::new([
            theta.sin() * phi.cos(),
            theta.sin() * phi.sin(),
            theta.cos(),
        ]),
        pdf(),
    )
}

pub enum SamplingFunctions {
    RandomUnitVector,
    CosineWeightedSample1,
    CosineWeightedSample2,
}

#[allow(dead_code)]
fn cosine_weighted_sample_1(normal: &Float3, rand_state: &mut RNGType) -> (Float3, Float0) {
    fn pdf(cos_theta: Float0) -> Float0 {
        cos_theta / PI as Float0
    }
    let (v, u) = generate_coordinate_system(normal);
    let r1: Float0 = rand_state.random_range(0.0..1.0);
    let r2: Float0 = rand_state.random_range(0.0..1.0);

    let phi = 2.0 * PI as Float0 * r1;
    let r = r2.sqrt();
    let x = r * phi.cos();
    let y = (1.0 - r2).sqrt(); // this is cos_theta
    let z = r * phi.sin();

    let local_sample = Float3::new([x, y, z]);
    let transformation_matrix = Float3x3::new_from_columns([u.0, normal.0, v.0]);
    (
        transformation_matrix.multiply_by_vector(&local_sample),
        pdf(y),
    )
}

#[allow(dead_code)]
fn cosine_weighted_sample_2(normal: &Float3, rand_state: &mut RNGType) -> (Float3, Float0) {
    fn pdf(cos_theta: Float0) -> Float0 {
        cos_theta / PI as Float0
    }
    let (v, u) = generate_coordinate_system(normal);
    let r1: Float0 = rand_state.random_range(0.0..1.0);
    let r2: Float0 = rand_state.random_range(0.0..1.0);
    let cos_theta = r1.sqrt();
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    let phi = r2 * 2.0 * PI as Float0;

    let local_sample = Float3::new([sin_theta * phi.cos(), cos_theta, sin_theta * phi.sin()]);
    let transformation_matrix = Float3x3::new_from_columns([u.0, normal.0, v.0]);
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
    ) -> (Ray, Float0) {
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

    pub fn color(&self, uv: &Option<Float2>) -> Float3 {
        if self.checkered {
            if let Some(uv) = uv {
                // Clamp coordinates to [0, 1)
                let u = uv.x().rem_euclid(1.0);
                let v = uv.y().rem_euclid(1.0);
                // Multiply to get grid cell index; these will be in a small range.
                let grid_u = (u * 10.0).floor() as i32;
                let grid_v = (v * 10.0).floor() as i32;
                if (grid_u + grid_v) % 2 == 0 {
                    Float3::new([0.0, 0.0, 0.0])
                } else {
                    Float3::new([1.0, 1.0, 1.0])
                }
            } else {
                self.albedo
            }
        } else {
            self.albedo
        }
    }

    pub fn reflect(v: &Float3, n: &Float3) -> Float3 {
        *v - n.scale(2.0 * v.dot(n))
    }

    pub fn reflective() -> Material {
        Material {
            albedo: Float3::new([1.0, 1.0, 1.0]),
            reflectivity: 1.0,
            checkered: false,
        }
    }

    pub fn red() -> Material {
        Material {
            albedo: Float3::new([1.0, 0.0, 0.0]),
            reflectivity: 0.0,
            checkered: false,
        }
    }

    pub fn green() -> Material {
        Material {
            albedo: Float3::new([0.0, 1.0, 0.0]),
            reflectivity: 0.0,
            checkered: false,
        }
    }

    pub fn blue() -> Material {
        Material {
            albedo: Float3::new([0.0, 0.0, 1.0]),
            reflectivity: 0.0,
            checkered: false,
        }
    }

    pub fn white() -> Material {
        Material {
            albedo: Float3::new([1.0, 1.0, 1.0]),
            reflectivity: 0.0,
            checkered: false,
        }
    }

    pub fn checkered() -> Material {
        Material {
            albedo: Float3::new([1.0, 1.0, 1.0]),
            reflectivity: 0.0,
            checkered: true,
        }
    }

    pub fn black() -> Material {
        Material {
            albedo: Float3::new([0.0, 0.0, 0.0]),
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

    pub fn from_color(color: crate::utils::vector::Float3) -> Material {
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
    fn avg_cosine(samples: Vec<Float3>, normal: Float3) -> Float0 {
        samples
            .iter()
            .map(|v| {
                let cos_theta = v.dot(&normal) / (v.length() * normal.length());
                cos_theta.max(0.0)
            })
            .sum::<Float0>()
            / samples.len() as Float0
    }

    #[test]
    fn test_cosine_weighted_sample_1_distribution() {
        let mut rng = get_rng();
        let normal = Float3::new([0.0, 1.0, 0.0]);
        let samples: Vec<Float3> = (0..1000)
            .map(|_| cosine_weighted_sample_1(&normal, &mut rng).0)
            .collect();

        let average_cosine: Float0 =
            samples.iter().map(|v| v.y()).sum::<Float0>() / samples.len() as Float0;
        let expected_average_cosine: Float0 = 2.0 / PI as Float0;

        assert!(
            (average_cosine - expected_average_cosine).abs() < 0.05,
            "Average cosine: {}",
            average_cosine
        );
    }

    #[test]
    fn test_cosine_weighted_sample_2_distribution() {
        let mut rng = get_rng();
        let normal = Float3::new([0.0, 1.0, 0.0]);
        let samples: Vec<Float3> = (0..1000)
            .map(|_| cosine_weighted_sample_2(&normal, &mut rng).0)
            .collect();

        let average_cosine: Float0 =
            samples.iter().map(|v| v.y()).sum::<Float0>() / samples.len() as Float0;
        let expected_average_cosine: Float0 = 2.0 / PI as Float0;

        assert!(
            (average_cosine - expected_average_cosine).abs() < 0.05,
            "Average cosine: {}",
            average_cosine
        );
    }
}
