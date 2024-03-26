use crate::{object::HitRecord, scene::FloatSize, utils::vector::Vec3};

pub mod arealight;
pub mod pointlight;

// pub trait Light: Sync + Serialize + for<'a> Deserialize<'a> {
pub trait Light: Sync + std::fmt::Debug {
    fn illuminate(&self, hit_record: &HitRecord) -> Vec3<FloatSize>;
}
