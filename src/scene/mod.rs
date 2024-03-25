use crate::{
    camera::Camera,
    light::Light,
    object::{HitRecord, Hittable},
    ray::Ray,
    utils::vector::Vec3,
};

pub type FloatSize = f32;

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Box<dyn Light>>,
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

    pub fn hit(&self, ray: &&Ray, arg: FloatSize) -> Option<HitRecord> {
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

    pub fn trace_ray(&self, ray: &Ray, depth: u32) -> Vec3<FloatSize> {
        println!("Tracing ray: {:?}", ray);
        if depth == 0 {
            return Vec3::new([0.0, 0.0, 0.0]);
        }

        if let Some(hit_record) = self.hit(&ray, 0.001) {
            let color: Vec3<FloatSize> = self.illuminate(&hit_record);

            let scattered = self.scatter(&ray, &hit_record);

            if let Some(scattered) = scattered {
                return color * self.trace_ray(&scattered, depth - 1);
            }

            color
        } else {
            Vec3::new([0.0, 0.0, 0.0])
        }
    }

    fn scatter(&self, ray: &&Ray, hit_record: &HitRecord) -> Option<Ray> {
        hit_record.material.scatter(ray, hit_record)
    }
}
