use crate::{scene::Flooat, utils::vector::Float3};

use super::Light;
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

    fn intensity(&self) -> Flooat {
        self.color.length()
    }

    fn color(&self) -> Float3 {
        self.color
    }
}
