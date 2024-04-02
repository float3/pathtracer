use crate::{material::Material, scene::FloatSize, utils::vector::Vec3};

use super::{HitRecord, Hittable};

#[derive(Debug)]
pub struct TriangleMesh {
    vertices: Vec<Vec3<FloatSize>>,
    transform: Vec3<FloatSize>,
    indices: Vec<usize>,
    material: Material,
}

impl Hittable for TriangleMesh {
    fn hit(
        &self,
        _ray: &crate::ray::Ray,
        _arg: FloatSize,
        _closest_so_far: FloatSize,
    ) -> Option<HitRecord> {
        unimplemented!()
    }
}
