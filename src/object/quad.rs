use crate::{
    material::Material,
    ray::Ray,
    scene::Float0,
    utils::vector::{Float2, Float3},
};

use super::{HitRecord, Hittable, aabb::Aabb};

#[derive(Debug)]
pub struct Quad {
    pub a: Float3,
    pub b: Float3,
    pub c: Float3,
    pub d: Float3,
    pub infinite: bool,
    pub scale: Float2,
    pub material: Material,
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, t_min: Float0, t_max: Float0) -> Option<HitRecord<'_>> {
        let edge_u = self.b - self.a;
        let edge_v = self.d - self.a;
        let normal = edge_u.cross(&edge_v);
        if normal.length_squared() < 1e-16 {
            return None;
        }
        let normal = normal.normalize();
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

        let ap = point - self.a;
        let uu = edge_u.dot(&edge_u);
        let uv = edge_u.dot(&edge_v);
        let vv = edge_v.dot(&edge_v);
        let wu = ap.dot(&edge_u);
        let wv = ap.dot(&edge_v);
        let denominator = uu * vv - uv * uv;
        if denominator.abs() < 1e-16 {
            return None;
        }
        let u = (wu * vv - wv * uv) / denominator;
        let v = (wv * uu - wu * uv) / denominator;

        if !self.infinite && (!(0.0..=1.0).contains(&u) || !(0.0..=1.0).contains(&v)) {
            return None;
        }

        let uv = Some(Float2::new([u, v]) * self.scale);

        Some(HitRecord {
            point,
            normal,
            t,
            front_face,
            material: &self.material,
            uv,
        })
    }

    fn bounding_box(&self) -> Option<Aabb> {
        if self.infinite {
            None
        } else {
            Aabb::from_points(&[self.a, self.b, self.c, self.d])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        material::Material,
        ray::Ray,
        utils::vector::{Float2, Float3},
    };

    use super::*;

    fn left_wall() -> Quad {
        Quad {
            a: Float3::new([-1.0, -1.0, -1.0]),
            b: Float3::new([-1.0, -1.0, 1.0]),
            c: Float3::new([-1.0, 1.0, 1.0]),
            d: Float3::new([-1.0, 1.0, -1.0]),
            infinite: false,
            scale: Float2::new([1.0, 1.0]),
            material: Material::red(),
        }
    }

    fn assert_close(actual: Float0, expected: Float0) {
        assert!(
            (actual - expected).abs() < 1e-9,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn finite_quad_accepts_center_hit() {
        let ray = Ray::new(Float3::new([0.0, 0.0, 0.0]), Float3::new([-1.0, 0.0, 0.0]));
        let wall = left_wall();

        let record = wall
            .hit(&ray, 0.001, Float0::INFINITY)
            .expect("ray should hit the center of the wall");

        assert_close(record.point.0[0], -1.0);
        assert_close(record.point.0[1], 0.0);
        assert_close(record.point.0[2], 0.0);
        assert_eq!(record.normal.0, [1.0, 0.0, 0.0]);

        let uv = record.uv.expect("quad hits should include uv coordinates");
        assert_close(uv.0[0], 0.5);
        assert_close(uv.0[1], 0.5);
    }

    #[test]
    fn finite_quad_rejects_points_outside_edges() {
        let ray = Ray::new(Float3::new([0.0, 1.5, 0.0]), Float3::new([-1.0, 0.0, 0.0]));

        assert!(left_wall().hit(&ray, 0.001, Float0::INFINITY).is_none());
    }
}
