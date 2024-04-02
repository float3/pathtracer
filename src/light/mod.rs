use std::str::FromStr;

use crate::{scene::FloatSize, utils::vector::Vec3};

pub mod arealight;
pub mod pointlight;

pub enum LightType {
    PointLight,
    AreaLight,
    ObjectLight,
}

impl FromStr for LightType {
    type Err = ();

    fn from_str(s: &str) -> Result<LightType, ()> {
        match s {
            "point" => Ok(Self::PointLight),
            "area" => Ok(Self::AreaLight),
            "object" => Ok(Self::ObjectLight),
            _ => Err(()),
        }
    }
}

pub trait Light: Sync + std::fmt::Debug {
    fn position(&self) -> Vec3<FloatSize>;
    fn illuminate(&self) -> Vec3<FloatSize>;
    fn intensity(&self) -> FloatSize;
    fn color(&self) -> Vec3<FloatSize>;
    // fn clone_box(&self) -> Box<dyn Light>;
}
