use crate::{
    material::Material,
    ray::Ray,
    scene::FloatSize,
    utils::vector::{Vec2, Vec3},
};

use super::{HitRecord, Hittable};

#[derive(Debug)]
pub struct Quad {
    pub a: Vec3<FloatSize>,
    pub b: Vec3<FloatSize>,
    pub c: Vec3<FloatSize>,
    pub d: Vec3<FloatSize>,
    pub infinite: bool,
    pub scale: Vec2<FloatSize>,
    pub material: Material,
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, t_min: FloatSize, t_max: FloatSize) -> Option<HitRecord> {
        let normal = (self.b - self.a).cross(&(self.c - self.a)).normalize();
        let denom = ray.direction.dot(&normal);

        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.a - ray.origin).dot(&normal) / denom;
        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.at(t);
        let front_face = ray.direction.dot(&normal) < 0.0;
        let normal = if front_face { normal } else { -normal };

        let ad = self.d - self.a;
        let ap = point - self.a;
        let u = ad.dot(&ap) / ad.length_squared();
        let ab = self.b - self.a;
        let bp = point - self.b;
        let v = ab.dot(&bp) / ab.length_squared();

        if !self.infinite && (!(0.0..=1.0).contains(&u) || !(0.0..=1.0).contains(&v)) {
            return None;
        }

        let uv = Some(Vec2::new([u, v]) * self.scale);

        Some(HitRecord {
            point,
            normal,
            t,
            front_face,
            material: &self.material,
            uv,
        })
    }
}
