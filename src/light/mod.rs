use std::str::FromStr;

use crate::{
    scene::{Float0, RNGType},
    utils::vector::Float3,
};

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
    fn sample(&self, point: Float3, rand_state: &mut RNGType) -> LightSample;
    fn illuminate(&self) -> Float3;
    fn intensity(&self) -> Float0;
    fn color(&self) -> Float3;
    // fn clone_box(&self) -> Box<dyn Light>;
}

#[derive(Debug, Clone, Copy)]
pub struct LightSample {
    pub direction: Float3,
    pub distance: Float0,
    pub radiance: Float3,
    pub pdf: Float0,
    pub delta: bool,
}
