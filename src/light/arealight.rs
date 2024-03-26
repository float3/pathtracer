use super::Light;

#[derive(Debug)]
pub struct Arealight {}

impl Light for Arealight {
    fn illuminate(
        &self,
        hit_record: &crate::object::HitRecord,
    ) -> crate::utils::vector::Vec3<crate::scene::FloatSize> {
        todo!()
    }
}
