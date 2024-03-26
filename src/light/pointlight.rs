use crate::{object::HitRecord, scene::FloatSize, utils::vector::Vec3};

use super::Light;
#[derive(Debug)]
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
    fn illuminate(&self, hit_record: &HitRecord) -> Vec3<FloatSize> {
        // check if in shadow

        self.color
    }
}
