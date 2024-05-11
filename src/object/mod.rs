use std::str::FromStr;

use crate::{
    material::Material,
    ray::Ray,
    scene::Float0,
    utils::vector::{Float2, Float3},
};

pub mod cube;
pub mod plane;
pub mod quad;
pub mod sphere;
#[allow(dead_code)]
pub mod triangle_mesh;

pub enum ObjectType {
    Sphere,
    Quad,
    Plane,
    Cube,
    TriangleMesh,
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
    fn hit(&self, ray: &Ray, t_min: Float0, t_max: Float0) -> Option<HitRecord>;
}
#[derive(Debug)]
pub struct HitRecord<'a> {
    pub point: Float3,
    pub normal: Float3,
    pub t: Float0,
    pub front_face: bool,
    pub material: &'a Material,
    pub uv: Option<Float2>,
}
