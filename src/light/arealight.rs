use crate::{scene::FloatSize, utils::vector::Vec3};

use super::Light;

#[derive(Debug, Clone)]
pub struct Arealight {}

impl Light for Arealight {
    fn illuminate(&self) -> Vec3<FloatSize> {
        todo!()
    }

    fn position(&self) -> Vec3<FloatSize> {
        todo!()
    }

    fn intensity(&self) -> FloatSize {
        todo!()
    }

    fn color(&self) -> Vec3<FloatSize> {
        todo!()
    }
}
