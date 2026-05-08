use crate::{
    scene::{Float0, RNGType},
    utils::vector::Float3,
};

use super::{Light, LightSample};
#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    position: Float3,
    color: Float3,
}

impl PointLight {
    pub fn new(position: Float3, color: Float3) -> Self {
        PointLight { position, color }
    }
}

impl Light for PointLight {
    fn illuminate(&self) -> Float3 {
        self.color
    }

    fn position(&self) -> Float3 {
        self.position
    }

    fn sample(&self, point: Float3, _rand_state: &mut RNGType) -> LightSample {
        let to_light = self.position - point;
        let distance = to_light.length();
        LightSample {
            direction: to_light.normalize(),
            distance,
            radiance: self.color.scale(1.0 / (distance * distance)),
            pdf: 1.0,
            delta: true,
        }
    }

    fn intensity(&self) -> Float0 {
        self.color.length()
    }

    fn color(&self) -> Float3 {
        self.color
    }
}
