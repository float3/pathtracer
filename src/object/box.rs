#[derive(Debug)]
pub struct Box {
    min: Vec3<FloatSize>,
    max: Vec3<FloatSize>,
    material: Material,
}

impl Box {
    pub fn new(min: Vec3<FloatSize>, max: Vec3<FloatSize>, material: Material) -> Self {
        Box { min, max, material }
    }
}

impl Hittable for Box {
    fn hit(&self, ray: &Ray, t_min: FloatSize, t_max: FloatSize) -> Option<HitRecord> {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv_d;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return None;
            }
        }
        let point = ray.at(t_min);
        let outward_normal = Vec3::new([
            (point[0] - self.min[0]).min(self.max[0] - point[0]),
            (point[1] - self.min[1]).min(self.max[1] - point[1]),
            (point[2] - self.min[2]).min(self.max[2] - point[2]),
        ]);
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal.normalize()
        } else {
            -outward_normal.normalize()
        };
        Some(HitRecord {
            point,
            normal,
            t: t_min,
            front_face,
            material: &self.material,
            uv: None,
        })
    }
}
