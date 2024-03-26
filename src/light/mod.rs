use crate::{object::HitRecord, scene::FloatSize, utils::vector::Vec3};

pub mod arealight;
pub mod pointlight;

pub trait Light: Sync + std::fmt::Debug {
    fn position(&self) -> Vec3<FloatSize>;
    fn illuminate(&self, hit_record: &HitRecord) -> Vec3<FloatSize>;
    fn intensity(&self) -> FloatSize;
    fn color(&self) -> Vec3<FloatSize>;
    // fn clone_box(&self) -> Box<dyn Light>;
}
