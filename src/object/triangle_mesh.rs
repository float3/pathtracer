use crate::{material::Material, ray::Ray, scene::Float0, utils::vector::Float3};

use super::{HitRecord, Hittable, aabb::Aabb};

#[derive(Debug)]
pub struct TriangleMesh {
    vertices: Vec<Float3>,
    indices: Vec<[usize; 3]>,
    material: Material,
}

impl TriangleMesh {
    pub fn new(
        vertices: Vec<Float3>,
        indices: Vec<[usize; 3]>,
        transform: Float3,
        material: Material,
    ) -> Self {
        let vertices = vertices
            .into_iter()
            .map(|vertex| vertex + transform)
            .collect::<Vec<_>>();
        Self {
            vertices,
            indices,
            material,
        }
    }
}

impl Hittable for TriangleMesh {
    fn hit(&self, ray: &Ray, t_min: Float0, t_max: Float0) -> Option<HitRecord<'_>> {
        let mut closest = t_max;
        let mut hit_record = None;

        for [i0, i1, i2] in &self.indices {
            let Some(v0) = self.vertices.get(*i0) else {
                continue;
            };
            let Some(v1) = self.vertices.get(*i1) else {
                continue;
            };
            let Some(v2) = self.vertices.get(*i2) else {
                continue;
            };

            if let Some(record) = hit_triangle(ray, *v0, *v1, *v2, t_min, closest, &self.material) {
                closest = record.t;
                hit_record = Some(record);
            }
        }

        hit_record
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Aabb::from_points(&self.vertices)
    }
}

fn hit_triangle<'a>(
    ray: &Ray,
    v0: Float3,
    v1: Float3,
    v2: Float3,
    t_min: Float0,
    t_max: Float0,
    material: &'a Material,
) -> Option<HitRecord<'a>> {
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let h = ray.direction.cross(&edge2);
    let a = edge1.dot(&h);
    if a.abs() < 1e-8 {
        return None;
    }

    let f = 1.0 / a;
    let s = ray.origin - v0;
    let u = f * s.dot(&h);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }

    let q = s.cross(&edge1);
    let v = f * ray.direction.dot(&q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = f * edge2.dot(&q);
    if t < t_min || t > t_max {
        return None;
    }

    let outward_normal = edge1.cross(&edge2).normalize();
    let front_face = ray.direction.dot(&outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal
    } else {
        -outward_normal
    };

    Some(HitRecord {
        point: ray.at(t),
        normal,
        t,
        front_face,
        material,
        uv: None,
    })
}
