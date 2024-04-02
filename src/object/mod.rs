use std::str::FromStr;

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
    Cube,
}

impl FromStr for ObjectType {
    type Err = ();

    fn from_str(s: &str) -> Result<ObjectType, ()> {
        match s {
            "sphere" => Ok(Self::Sphere),
            "quad" => Ok(Self::Quad),
            "plane" => Ok(Self::Plane),
            "cube" => Ok(Self::Cube),
            _ => panic!("Unknown object type: {}", s),
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
