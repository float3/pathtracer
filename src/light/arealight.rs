use crate::{scene::Flooat, utils::vector::Float3};

use super::Light;

#[derive(Debug, Clone)]
pub struct Arealight {}

impl Light for Arealight {
    fn illuminate(&self) -> Float3 {
        todo!()
    }

    fn position(&self) -> Float3 {
        todo!()
    }

    fn intensity(&self) -> Flooat {
        todo!()
    }

    fn color(&self) -> Float3 {
        todo!()
    }
}
