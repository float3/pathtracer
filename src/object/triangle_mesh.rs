use crate::{material::Material, scene::Float0, utils::vector::Float3};

use super::{HitRecord, Hittable};

#[derive(Debug)]
pub struct TriangleMesh {
    vertices: Vec<Float3>,
    transform: Float3,
    indices: Vec<usize>,
    material: Material,
}

impl Hittable for TriangleMesh {
    fn hit(
        &self,
        _ray: &crate::ray::Ray,
        _arg: Float0,
        _closest_so_far: Float0,
    ) -> Option<HitRecord> {
        unimplemented!()
    }
}
