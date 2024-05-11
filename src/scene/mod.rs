use std::str::FromStr;

use toml::Value;

use crate::{
    camera::Camera,
    light::{pointlight::PointLight, Light, LightType},
    material::{Material, SamplingFunctions},
    object::{quad::Quad, sphere::Sphere, HitRecord, Hittable, ObjectType},
    ray::Ray,
    skybox::Skybox,
    utils::vector::{Float2, Float3},
};

pub type Float0 = f64;
pub type Int = i64;
pub const PI: Float0 = std::f64::consts::PI as Float0;

cfg_if! {
    if #[cfg(feature="small_rng")] {
        pub type RNGType = rand::rngs::SmallRng;
    } else {
        pub type RNGType = rand::rngs::ThreadRng;
    }
}

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Box<dyn Light>>,
    pub skybox: Skybox,
    pub camera: Camera,
}

impl Scene {
    pub fn illuminate(&self) -> Float3 {
        let mut illumination = Float3::new([0.0, 0.0, 0.0]);
        for light in self.lights.iter() {
            illumination += light.illuminate();
        }
        illumination
    }

    pub fn hit(&self, ray: &Ray, arg: Float0) -> Option<HitRecord> {
        let mut closest_so_far = Float0::INFINITY;
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
        rand_state: &mut RNGType,
        sample_type: &SamplingFunctions,
    ) -> Float3 {
        let mut throughput = Float3::new([1.0, 1.0, 1.0]);
        let mut ray: Ray = *ray;
        let mut emitted = Float3::new([0.0, 0.0, 0.0]);
        for _bounce in 0..depth {
            if let Some(hit_record) = self.hit(&ray, 0.001) {
                // return hit_record.material.color(&hit_record.uv);
                if hit_record.material.reflectivity == 1.0 {
                    let reflected =
                        Material::reflect(&ray.direction.normalize(), &hit_record.normal);
                    ray = Ray {
                        origin: hit_record.point + reflected.scale(0.001),
                        direction: reflected,
                    };
                } else {
                    let pdf;
                    (ray, pdf) = hit_record
                        .material
                        .scatter(&hit_record, rand_state, sample_type);

                    let brdf = hit_record
                        .material
                        .color(&hit_record.uv)
                        .scale(1.0 as Float0 / PI as Float0);

                    let cos_theta = ray.direction.dot(&hit_record.normal);

                    throughput *= brdf.scale(cos_theta).scale(pdf.recip());

                    for light in &self.lights {
                        let light_color = self.light_ray(&hit_record, &**light);
                        let light_direction = (light.position() - hit_record.point).normalize();
                        let n_dot_l = hit_record.normal.dot(&light_direction).max(0.0);
                        emitted += light_color.scale(n_dot_l) * throughput;
                    }
                }
            } else {
                return emitted + (throughput * self.skybox.color);
            }
        }
        Float3::new([0.0, 0.0, 0.0])
    }

    fn light_ray(&self, hit_record: &HitRecord, light: &dyn Light) -> Float3 {
        let light_direction = light.position() - hit_record.point;
        let distance_to_light = light_direction.magnitude();
        let shadow_ray = Ray::new(hit_record.point, light_direction.normalize());
        for object in self.objects.iter() {
            if let Some(_record) = object.hit(&shadow_ray, 0.001, distance_to_light) {
                return Float3::new([0.0, 0.0, 0.0]);
            }
        }

        let falloff = 1.0 / (distance_to_light * distance_to_light);
        light.color().scale(falloff)
    }

    pub fn from_toml(toml: &Value) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        let mut lights: Vec<Box<dyn Light>> = Vec::new();

        let camera = Camera {
            position: Float3::from_toml(&toml["camera"]["position"]),
            rotation: Float3::from_toml(&toml["camera"]["rotation"]),
        };

        let skybox = Skybox {
            color: Float3::from_toml(&toml["skybox"]["color"]),
        };

        for object in toml["objects"].as_array().unwrap() {
            let object_type = object["type"].as_str().unwrap();
            let material = match &object.get("material") {
                Some(material) => Material::from_toml(material),
                None => match &object.get("color") {
                    Some(color) => Material::from_color(Float3::from_toml(color)),
                    None => Material::default(),
                },
            };

            match ObjectType::from_str(object_type) {
                Ok(object_type) => match object_type {
                    ObjectType::Sphere => {
                        objects.push(Box::new(Sphere::new(
                            Float3::from_toml(&object["position"]),
                            object["radius"].as_float().unwrap(),
                            material,
                        )));
                    }
                    ObjectType::Quad => {
                        let infinite = object["infinite"].as_bool().unwrap_or(false);
                        let scale_vec = match &object.get("scale") {
                            Some(scale) => Float2::from_toml(scale),
                            None => Float2::new([1.0, 1.0]),
                        };
                        objects.push(Box::new(Quad {
                            a: Float3::from_toml(&object["point1"]),
                            b: Float3::from_toml(&object["point2"]),
                            c: Float3::from_toml(&object["point3"]),
                            d: Float3::from_toml(&object["point4"]),
                            scale: scale_vec,
                            material,
                            infinite,
                        }));
                    }
                    ObjectType::Plane => {
                        objects.push(Box::new(crate::object::plane::Plane::new(
                            Float3::from_toml(&object["point"]),
                            Float3::from_toml(&object["normal"]),
                            material,
                        )));
                    }
                    ObjectType::Cube => {
                        objects.push(Box::new(crate::object::cube::Cube::new(
                            Float3::from_toml(&object["min"]),
                            Float3::from_toml(&object["max"]),
                            material,
                        )));
                    }
                    ObjectType::TriangleMesh => {
                        todo!()
                    }
                },
                Err(_) => {
                    panic!("Invalid object type: {}", object_type);
                }
            }
        }

        if let Some(lights_array) = toml.get("lights").and_then(|lights| lights.as_array()) {
            for light in lights_array {
                let light_type = light["type"].as_str().unwrap();
                match LightType::from_str(light_type) {
                    Ok(light_type_enum) => match light_type_enum {
                        LightType::PointLight => {
                            lights.push(Box::new(PointLight::new(
                                Float3::from_toml(&light["position"]),
                                Float3::from_toml(&light["color"]),
                            )));
                        }
                        LightType::AreaLight => {
                            todo!()
                        }
                        LightType::ObjectLight => todo!(),
                    },
                    Err(_) => todo!(),
                }
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
