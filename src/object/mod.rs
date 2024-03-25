use crate::{material::Material, ray::Ray, scene::FloatSize, utils::vector::Vec3};

pub mod plane;
pub mod sphere;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: FloatSize, t_max: FloatSize) -> Option<HitRecord>;
}

pub struct HitRecord<'a> {
    pub point: Vec3<FloatSize>,
    pub normal: Vec3<FloatSize>,
    pub t: FloatSize,
    pub front_face: bool,
    pub material: &'a Material,
}
