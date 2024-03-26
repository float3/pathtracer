use crate::{object::HitRecord, scene::FloatSize, utils::vector::Vec3};

pub mod arealight;
pub mod pointlight;

pub enum LightType {
    PointLight,
    AreaLight,
}

impl LightType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "point" => Some(Self::PointLight),
            "area" => Some(Self::AreaLight),
            _ => None,
        }
    }
}

pub trait Light: Sync + std::fmt::Debug {
    fn position(&self) -> Vec3<FloatSize>;
    fn illuminate(&self, hit_record: &HitRecord) -> Vec3<FloatSize>;
    fn intensity(&self) -> FloatSize;
    fn color(&self) -> Vec3<FloatSize>;
    // fn clone_box(&self) -> Box<dyn Light>;
}
