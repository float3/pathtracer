use crate::{object::HitRecord, scene::FloatSize, utils::vector::Vec3};

pub mod pointlight;

pub trait Light {
    fn illuminate(&self, hit_record: &HitRecord) -> Vec3<FloatSize>;
}