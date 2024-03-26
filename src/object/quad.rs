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
    pub material: Material,
}

impl Quad {
    pub fn new(
        a: Vec3<FloatSize>,
        b: Vec3<FloatSize>,
        c: Vec3<FloatSize>,
        d: Vec3<FloatSize>,
        material: Material,
    ) -> Quad {
        Quad {
            a,
            b,
            c,
            d,
            material,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, t_min: FloatSize, t_max: FloatSize) -> Option<HitRecord> {
        let normal = (self.b - self.a).cross(&(self.c - self.a)).normalize();
        let t = (self.a - ray.origin).dot(&normal) / ray.direction.dot(&normal);
        if t < t_min || t > t_max {
            return None;
        }
        let point = ray.at(t);
        let p = point - self.a;
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let area_abc = ab.cross(&ac).length();
        let area_pbc = p.cross(&ac).length();
        let area_pca = ab.cross(&p).length();

        let u = area_pbc / area_abc;
        let v = area_pca / area_abc;

        let front_face = ray.direction.dot(&normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        Some(HitRecord {
            point,
            normal,
            t,
            front_face,
            material: &self.material,
            uv: Some(Vec2::new([u, v])),
        })
    }
}
