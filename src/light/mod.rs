use std::str::FromStr;

use crate::{scene::Float0, utils::vector::Float3};

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
    fn position(&self) -> Float3;
    fn illuminate(&self) -> Float3;
    fn intensity(&self) -> Float0;
    fn color(&self) -> Float3;
    // fn clone_box(&self) -> Box<dyn Light>;
}
