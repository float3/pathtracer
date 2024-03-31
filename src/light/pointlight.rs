use crate::{scene::FloatSize, utils::vector::Vec3};

use super::Light;
#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    position: Vec3<FloatSize>,
    color: Vec3<FloatSize>,
}

impl PointLight {
    pub fn new(position: Vec3<FloatSize>, color: Vec3<FloatSize>) -> Self {
        PointLight { position, color }
    }
}

impl Light for PointLight {
    fn illuminate(&self) -> Vec3<FloatSize> {
        self.color
    }

    fn position(&self) -> Vec3<FloatSize> {
        self.position
    }

    fn intensity(&self) -> FloatSize {
        self.color.length()
    }

    fn color(&self) -> Vec3<FloatSize> {
        self.color
    }
}
