use crate::{
    material::Material,
    ray::Ray,
    scene::FloatSize,
    utils::vector::{Vec2, Vec3},
};

pub mod cube;
pub mod plane;
pub mod quad;
pub mod sphere;

pub enum ObjectType {
    Sphere,
    Quad,
    Plane,
    Box,
}

impl ObjectType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "sphere" => Some(Self::Sphere),
            "quad" => Some(Self::Quad),
            "plane" => Some(Self::Plane),
            "box" => Some(Self::Box),
            _ => None,
        }
    }
}

pub trait Hittable: Sync + std::fmt::Debug {
    fn hit(&self, ray: &Ray, t_min: FloatSize, t_max: FloatSize) -> Option<HitRecord>;
}
#[derive(Debug)]
pub struct HitRecord<'a> {
    pub point: Vec3<FloatSize>,
    pub normal: Vec3<FloatSize>,
    pub t: FloatSize,
    pub front_face: bool,
    pub material: &'a Material,
    pub uv: Option<Vec2<FloatSize>>,
}
