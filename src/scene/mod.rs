use rand::rngs::ThreadRng;
use toml::Value;

use crate::{
    camera::Camera,
    light::{pointlight::PointLight, Light},
    material::Material,
    object::{quad::Quad, HitRecord, Hittable},
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
        let mut emitted = Vec3::new([0.0, 0.0, 0.0]);
        for _bounce in 0..depth {
            if let Some(hit_record) = self.hit(&ray, 0.001) {
                if hit_record.material.reflectivity == 1.0 {
                    let reflected =
                        Material::reflect(&ray.direction.normalize(), &hit_record.normal);
                    ray = Ray {
                        origin: hit_record.point + reflected.scale(0.001),
                        direction: reflected,
                    };
                } else {
                    ray = hit_record
                        .material
                        .scatter(&hit_record, rand_state, is_left);

                    let brdf = hit_record
                        .material
                        .color(&hit_record)
                        .scale(1.0 as FloatSize / std::f64::consts::PI as FloatSize);

                    let cos_theta = ray.direction.dot(&hit_record.normal);

                    let pdf = if is_left {
                        cos_theta / std::f64::consts::PI as FloatSize
                    } else {
                        cos_theta / std::f64::consts::PI as FloatSize
                        // 1.0 / (2.0 * std::f64::consts::pi as floatsize)
                    };

                    throughput *= brdf.scale(cos_theta).scale(pdf.recip());

                    for light in &self.lights {
                        let light_color = self.light_ray(&hit_record, light);
                        let light_direction = (light.position() - hit_record.point).normalize();
                        let n_dot_l = hit_record.normal.dot(&light_direction).max(0.0);
                        emitted += light_color.scale(n_dot_l) * throughput;
                    }
                }
            } else {
                return emitted + (throughput * self.skybox.color);
            }
        }
        Vec3::new([0.0, 0.0, 0.0])
    }

    fn light_ray(&self, hit_record: &HitRecord, light: &Box<dyn Light>) -> Vec3<FloatSize> {
        // trace a ray to the light source and return the color, if it's occluded return black
        let light_direction = light.position() - hit_record.point;
        let distance_to_light = light_direction.magnitude();
        let shadow_ray = Ray::new(hit_record.point, light_direction.normalize());
        for object in self.objects.iter() {
            if let Some(_record) = object.hit(&shadow_ray, 0.001, distance_to_light) {
                return Vec3::new([0.0, 0.0, 0.0]);
            }
        }
        light.color()
    }

    pub fn cornell_box() -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        let mut lights: Vec<Box<dyn Light>> = Vec::new();

        // Floor
        objects.push(Box::new(Quad::new(
            Vec3::new([1.0, 0.0, 1.0]),
            Vec3::new([1.0, 0.0, -1.0]),
            Vec3::new([-1.0, 0.0, -1.0]),
            Vec3::new([-1.0, 0.0, 1.0]),
            Material::white(),
        )));
        // Ceiling
        // objects.push(Box::new(Quad::new(
        //     Vec3::new([1.0, 2.0, 1.0]),
        //     Vec3::new([1.0, 2.0, -1.0]),
        //     Vec3::new([-1.0, 2.0, -1.0]),
        //     Vec3::new([-1.0, 2.0, 1.0]),
        //     Material::white(),
        // )));
        // // Back wall
        objects.push(Box::new(Quad::new(
            Vec3::new([-1.0, 0.0, -1.0]),
            Vec3::new([1.0, 0.0, -1.0]),
            Vec3::new([1.0, 2.0, -1.0]),
            Vec3::new([-1.0, 2.0, -1.0]),
            Material::white(),
        )));
        // // Right wall (Green)
        // objects.push(Box::new(Quad::new(
        //     Vec3::new([1.0, 0.0, -1.0]),
        //     Vec3::new([1.0, 0.0, 1.0]),
        //     Vec3::new([1.0, 2.0, 1.0]),
        //     Vec3::new([1.0, 2.0, -1.0]),
        //     Material::green(),
        // )));
        // // Left wall (Red)
        // objects.push(Box::new(Quad::new(
        //     Vec3::new([-1.0, 0.0, -1.0]),
        //     Vec3::new([-1.0, 0.0, 1.0]),
        //     Vec3::new([-1.0, 2.0, 1.0]),
        //     Vec3::new([-1.0, 2.0, -1.0]),
        //     Material::red(),
        // )));

        lights.push(Box::new(PointLight::new(
            Vec3::new([0.0, 1.9, 0.0]),
            Vec3::new([15.0, 15.0, 15.0]),
        )));

        let camera = Camera::new(Vec3::new([0.0, 1.0, 3.0]), Vec3::new([0.0, 0.0, -1.0]));
        let skybox = Skybox {
            color: Vec3::new([0.1, 0.1, 0.1]),
        };
        Scene {
            objects,
            lights,
            camera,
            skybox,
        }
    }

    pub fn from_toml(toml: &Value) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        let mut lights: Vec<Box<dyn Light>> = Vec::new();

        let camera = Camera::new(
            Vec3::from_toml(&toml["camera"]["position"]),
            Vec3::from_toml(&toml["camera"]["direction"]),
        );

        let skybox = Skybox {
            color: Vec3::from_toml(&toml["skybox"]["color"]),
        };

        for object in toml["objects"].as_array().unwrap() {
            let object_type = object["type"].as_str().unwrap();
            let material = Material::from_toml(&object["material"]);
            match object_type {
                "quad" => {
                    objects.push(Box::new(Quad::new(
                        Vec3::from_toml(&object["point1"]),
                        Vec3::from_toml(&object["point2"]),
                        Vec3::from_toml(&object["point3"]),
                        Vec3::from_toml(&object["point4"]),
                        material,
                    )));
                }
                "sphere" => {
                    objects.push(Box::new(crate::object::sphere::Sphere::new(
                        Vec3::from_toml(&object["position"]),
                        object["radius"].as_float().unwrap(),
                        material,
                    )));
                }
                "plane" => {
                    objects.push(Box::new(crate::object::plane::Plane::new(
                        Vec3::from_toml(&object["point"]),
                        Vec3::from_toml(&object["normal"]),
                        material,
                    )));
                }
                _ => panic!("Unknown object type: {}", object_type),
            }
        }

        for light in toml["lights"].as_array().unwrap() {
            let light_type = light["type"].as_str().unwrap();
            match light_type {
                "point" => {
                    lights.push(Box::new(PointLight::new(
                        Vec3::from_toml(&light["position"]),
                        Vec3::from_toml(&light["color"]),
                    )));
                }
                _ => panic!("Unknown light type: {}", light_type),
            }
        }

        Scene {
            objects,
            lights,
            camera,
            skybox,
        }
    }
}
