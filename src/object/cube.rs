use crate::{material::Material, ray::Ray, scene::Float0, utils::vector::Float3};

use super::{HitRecord, Hittable, aabb::Aabb};

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
    fn hit(&self, ray: &Ray, t_min: Float0, t_max: Float0) -> Option<HitRecord<'_>> {
        let mut t_near = Float0::NEG_INFINITY;
        let mut t_far = Float0::INFINITY;
        let mut near_normal = Float3::new([0.0, 0.0, 0.0]);
        let mut far_normal = Float3::new([0.0, 0.0, 0.0]);

        for axis in 0..3 {
            let origin = ray.origin.0[axis];
            let direction = ray.direction.0[axis];

            if direction.abs() < 1e-12 {
                if origin < self.min.0[axis] || origin > self.max.0[axis] {
                    return None;
                }
                continue;
            }

            let inv_d = 1.0 / direction;
            let mut t0 = (self.min.0[axis] - origin) * inv_d;
            let mut t1 = (self.max.0[axis] - origin) * inv_d;
            let mut normal0 = axis_normal(axis, -1.0);
            let mut normal1 = axis_normal(axis, 1.0);
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
                std::mem::swap(&mut normal0, &mut normal1);
            }

            if t0 > t_near {
                t_near = t0;
                near_normal = normal0;
            }
            if t1 < t_far {
                t_far = t1;
                far_normal = normal1;
            }

            if t_far <= t_near {
                return None;
            }
        }

        let (t, outward_normal) = if t_near >= t_min {
            (t_near, near_normal)
        } else {
            (t_far, far_normal)
        };

        if t < t_min || t > t_max || outward_normal.length_squared() == 0.0 {
            return None;
        }

        let point = ray.at(t);
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Some(HitRecord {
            point,
            normal,
            t,
            front_face,
            material: &self.material,
            uv: None,
        })
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb::new(self.min, self.max))
    }
}

fn axis_normal(axis: usize, value: Float0) -> Float3 {
    let mut normal = [0.0, 0.0, 0.0];
    normal[axis] = value;
    Float3::new(normal)
}

#[cfg(test)]
mod tests {
    use crate::{material::Material, ray::Ray};

    use super::*;

    fn assert_close(actual: Float0, expected: Float0) {
        assert!(
            (actual - expected).abs() < 1e-9,
            "expected {expected}, got {actual}"
        );
    }

    fn assert_vec_close(actual: Float3, expected: [Float0; 3]) {
        for (actual, expected) in actual.0.iter().zip(expected) {
            assert_close(*actual, expected);
        }
    }

    #[test]
    fn cube_hit_reports_axis_aligned_entry_normal() {
        let cube = Cube::new(
            Float3::new([-1.0, -1.0, -1.0]),
            Float3::new([1.0, 1.0, 1.0]),
            Material::white(),
        );
        let ray = Ray::new(Float3::new([0.0, 0.0, 3.0]), Float3::new([0.0, 0.0, -1.0]));

        let record = cube
            .hit(&ray, 0.001, Float0::INFINITY)
            .expect("ray should hit the front face");

        assert_close(record.t, 2.0);
        assert_vec_close(record.point, [0.0, 0.0, 1.0]);
        assert_vec_close(record.normal, [0.0, 0.0, 1.0]);
        assert!(record.front_face);
    }

    #[test]
    fn cube_hit_from_inside_reports_exit_face() {
        let cube = Cube::new(
            Float3::new([-1.0, -1.0, -1.0]),
            Float3::new([1.0, 1.0, 1.0]),
            Material::white(),
        );
        let ray = Ray::new(Float3::new([0.0, 0.0, 0.0]), Float3::new([1.0, 0.0, 0.0]));

        let record = cube
            .hit(&ray, 0.001, Float0::INFINITY)
            .expect("ray should hit the exit face");

        assert_close(record.t, 1.0);
        assert_vec_close(record.point, [1.0, 0.0, 0.0]);
        assert_vec_close(record.normal, [-1.0, 0.0, 0.0]);
        assert!(!record.front_face);
    }
}
