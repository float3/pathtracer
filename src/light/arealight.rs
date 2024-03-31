use super::Light;

#[derive(Debug, Clone)]
pub struct Arealight {}

impl Light for Arealight {
    fn illuminate(
        &self,
        hit_record: &crate::object::HitRecord,
    ) -> crate::utils::vector::Vec3<crate::scene::FloatSize> {
        todo!()
    }

    fn position(&self) -> crate::utils::vector::Vec3<crate::scene::FloatSize> {
        todo!()
    }

    fn intensity(&self) -> crate::scene::FloatSize {
        todo!()
    }

    fn color(&self) -> crate::utils::vector::Vec3<crate::scene::FloatSize> {
        todo!()
    }
}
