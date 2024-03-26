use rand::rngs::ThreadRng;

use crate::{
    camera::Camera,
    light::Light,
    object::{HitRecord, Hittable},
    ray::Ray,
    skybox::Skybox,
    utils::vector::Vec3,
};

pub type FloatSize = f64;

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Box<dyn Light>>,
    pub skybox: Skybox,
    pub camera: Camera,
}

impl Scene {
    pub fn illuminate(&self, hit_record: &HitRecord) -> Vec3<FloatSize> {
        let mut illumination = Vec3::new([0.0, 0.0, 0.0]);
        for light in self.lights.iter() {
            illumination += light.illuminate(hit_record);
        }
        illumination
    }

    pub fn hit(&self, ray: &Ray, arg: FloatSize) -> Option<HitRecord> {
        let mut closest_so_far = FloatSize::INFINITY;
        let mut hit_record = None;
        for object in self.objects.iter() {
            if let Some(record) = object.hit(ray, arg, closest_so_far) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }
        hit_record
    }

    pub fn trace_ray(
        &self,
        ray: &Ray,
        depth: u32,
        rand_state: &mut ThreadRng,
        is_left: bool,
    ) -> Vec3<FloatSize> {
        let mut throughput = Vec3::new([1.0, 1.0, 1.0]);
        let mut ray: Ray = *ray;
        for _bounce in 0..depth {
            if let Some(hit_record) = self.hit(&ray, 0.001) {
                let color: Vec3<FloatSize> = self.illuminate(&hit_record);

                ray = hit_record
                    .material
                    .scatter(&ray, &hit_record, rand_state, is_left);

                let brdf = hit_record
                    .material
                    .albedo
                    .scale(1.0 as FloatSize / std::f64::consts::PI as FloatSize);

                let cos_theta = ray.direction.dot(&hit_record.normal);

                let pdf = 1.0 / (2.0 * std::f64::consts::PI as FloatSize);

                throughput *= brdf.scale(cos_theta).scale(pdf.recip());
            } else {
                return throughput * self.skybox.color;
            }
        }
        Vec3::new([0.0, 0.0, 0.0])
    }
}
