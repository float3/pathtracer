use std::str::FromStr;

use toml::Value;

use crate::{
    camera::Camera,
    light::{Light, LightType, arealight::Arealight, pointlight::PointLight},
    material::{Material, SamplingFunctions},
    object::{
        HitRecord, Hittable, ObjectType, bvh::Bvh, cube::Cube, plane::Plane, quad::Quad,
        sphere::Sphere, triangle_mesh::TriangleMesh,
    },
    ray::Ray,
    skybox::Skybox,
    utils::vector::{Float2, Float3},
};

pub type Float0 = f64;
pub type Int = i64;
pub const PI: Float0 = std::f64::consts::PI as Float0;

pub type RNGType = rand::rngs::StdRng;

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Box<dyn Light>>,
    pub skybox: Skybox,
    pub camera: Camera,
    bvh: Bvh,
    unbounded_objects: Vec<usize>,
}

impl Scene {
    pub fn illuminate(&self) -> Float3 {
        let mut illumination = Float3::new([0.0, 0.0, 0.0]);
        for light in self.lights.iter() {
            illumination += light.illuminate();
        }
        illumination
    }

    pub fn hit(&self, ray: &Ray, arg: Float0) -> Option<HitRecord<'_>> {
        let mut hit_record = self.bvh.hit(&self.objects, ray, arg, Float0::INFINITY);
        let mut closest_so_far = hit_record
            .as_ref()
            .map_or(Float0::INFINITY, |record| record.t);
        for object_index in &self.unbounded_objects {
            if let Some(record) = self.objects[*object_index].hit(ray, arg, closest_so_far) {
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
                if hit_record.material.reflectivity == 1.0 {
                    let reflected =
                        Material::reflect(&ray.direction.normalize(), &hit_record.normal);
                    ray = Ray {
                        origin: hit_record.point + reflected.scale(0.001),
                        direction: reflected,
                    };
                } else {
                    emitted +=
                        throughput * self.direct_lighting(&hit_record, rand_state, sample_type);

                    let pdf;
                    (ray, pdf) = hit_record
                        .material
                        .scatter(&hit_record, rand_state, sample_type);

                    let brdf = hit_record.material.color(&hit_record.uv).scale(1.0 / PI);

                    let cos_theta = ray.direction.dot(&hit_record.normal).max(0.0);
                    if pdf <= 0.0 || cos_theta <= 0.0 {
                        return emitted;
                    }

                    throughput *= brdf.scale(cos_theta).scale(pdf.recip());
                }
            } else {
                return emitted + (throughput * self.skybox.color);
            }
        }
        Float3::new([0.0, 0.0, 0.0])
    }

    fn direct_lighting(
        &self,
        hit_record: &HitRecord,
        rand_state: &mut RNGType,
        sample_type: &SamplingFunctions,
    ) -> Float3 {
        let mut contribution = Float3::new([0.0, 0.0, 0.0]);
        let brdf = hit_record.material.color(&hit_record.uv).scale(1.0 / PI);

        for light in &self.lights {
            let sample = light.sample(hit_record.point, rand_state);
            if sample.pdf <= 0.0 || sample.radiance.length_squared() == 0.0 {
                continue;
            }

            let cos_theta = hit_record.normal.dot(&sample.direction).max(0.0);
            if cos_theta <= 0.0
                || !self.visible_to_light(hit_record.point, sample.direction, sample.distance)
            {
                continue;
            }

            let weight = if sample.delta {
                1.0
            } else {
                let bsdf_pdf =
                    Material::sample_pdf(&hit_record.normal, &sample.direction, sample_type);
                Self::power_heuristic(sample.pdf, bsdf_pdf)
            };
            contribution += brdf
                * sample
                    .radiance
                    .scale(cos_theta * weight * sample.pdf.recip());
        }

        contribution
    }

    fn visible_to_light(&self, point: Float3, direction: Float3, distance: Float0) -> bool {
        let shadow_ray = Ray::new(point + direction.scale(0.001), direction);
        self.hit(&shadow_ray, 0.001)
            .is_none_or(|record| record.t >= distance - 0.001)
    }

    fn power_heuristic(a_pdf: Float0, b_pdf: Float0) -> Float0 {
        let a = a_pdf * a_pdf;
        let b = b_pdf * b_pdf;
        a / (a + b)
    }

    pub fn try_from_toml(toml: &Value) -> Result<Self, String> {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        let mut lights: Vec<Box<dyn Light>> = Vec::new();

        let camera_value = required(toml, "camera", "scene")?;
        let camera = Camera {
            position: float3_field(camera_value, "position", "camera")?,
            rotation: float3_field(camera_value, "rotation", "camera")?,
        };

        let skybox_value = required(toml, "skybox", "scene")?;
        let skybox = Skybox {
            color: float3_field(skybox_value, "color", "skybox")?,
        };

        for (index, object) in array_field(toml, "objects", "scene")?.iter().enumerate() {
            let path = format!("objects[{index}]");
            let object_type = str_field(object, "type", &path)?;
            let material = material_from_object(object, &path)?;

            match ObjectType::from_str(object_type) {
                Ok(object_type) => match object_type {
                    ObjectType::Sphere => {
                        objects.push(Box::new(Sphere::new(
                            float3_field(object, "position", &path)?,
                            float_field(object, "radius", &path)?,
                            material,
                        )));
                    }
                    ObjectType::Quad => {
                        let infinite = bool_field(object, "infinite", &path)?.unwrap_or(false);
                        let scale_vec = optional_float2_field(object, "scale", &path)?
                            .unwrap_or_else(|| Float2::new([1.0, 1.0]));
                        objects.push(Box::new(Quad {
                            a: float3_field(object, "point1", &path)?,
                            b: float3_field(object, "point2", &path)?,
                            c: float3_field(object, "point3", &path)?,
                            d: float3_field(object, "point4", &path)?,
                            scale: scale_vec,
                            material,
                            infinite,
                        }));
                    }
                    ObjectType::Plane => {
                        objects.push(Box::new(Plane::new(
                            float3_field(object, "point", &path)?,
                            float3_field(object, "normal", &path)?,
                            material,
                        )));
                    }
                    ObjectType::Cube => {
                        objects.push(Box::new(Cube::new(
                            float3_field(object, "min", &path)?,
                            float3_field(object, "max", &path)?,
                            material,
                        )));
                    }
                    ObjectType::TriangleMesh => {
                        objects.push(Box::new(TriangleMesh::new(
                            vertices_field(object, "vertices", &path)?,
                            indices_field(object, "indices", &path)?,
                            optional_float3_field(object, "position", &path)?
                                .or(optional_float3_field(object, "transform", &path)?)
                                .unwrap_or_else(|| Float3::new([0.0, 0.0, 0.0])),
                            material,
                        )));
                    }
                },
                Err(_) => {
                    return Err(format!(
                        "{path}.type has unknown object type `{object_type}`"
                    ));
                }
            }
        }

        if let Some(lights_array) = toml.get("lights").and_then(|lights| lights.as_array()) {
            for (index, light) in lights_array.iter().enumerate() {
                let path = format!("lights[{index}]");
                let light_type = str_field(light, "type", &path)?;
                match LightType::from_str(light_type) {
                    Ok(light_type_enum) => match light_type_enum {
                        LightType::PointLight => {
                            lights.push(Box::new(PointLight::new(
                                float3_field(light, "position", &path)?,
                                float3_field(light, "color", &path)?,
                            )));
                        }
                        LightType::AreaLight => {
                            let _c = float3_field(light, "point3", &path)?;
                            lights.push(Box::new(Arealight::new(
                                float3_field(light, "point1", &path)?,
                                float3_field(light, "point2", &path)?,
                                float3_field(light, "point4", &path)?,
                                float3_field(light, "color", &path)?,
                            )));
                        }
                        LightType::ObjectLight => {
                            return Err(format!("{path}.type object lights are not implemented"));
                        }
                    },
                    Err(_) => {
                        return Err(format!("{path}.type has unknown light type `{light_type}`"));
                    }
                }
            }
        }

        let (bvh, unbounded_objects) = Bvh::build(&objects);

        Ok(Scene {
            objects,
            lights,
            camera,
            skybox,
            bvh,
            unbounded_objects,
        })
    }

    pub fn from_toml(toml: &Value) -> Self {
        Self::try_from_toml(toml).unwrap_or_else(|err| panic!("invalid scene TOML: {err}"))
    }
}

fn material_from_object(object: &Value, path: &str) -> Result<Material, String> {
    if let Some(material) = object.get("material") {
        Material::try_from_toml(material).map_err(|err| format!("{path}.material: {err}"))
    } else if let Some(color) = object.get("color") {
        Ok(Material::from_color(float3(
            color,
            &format!("{path}.color"),
        )?))
    } else {
        Ok(Material::default())
    }
}

fn required<'a>(value: &'a Value, key: &str, path: &str) -> Result<&'a Value, String> {
    value
        .get(key)
        .ok_or_else(|| format!("{path}.{key} is required"))
}

fn array_field<'a>(value: &'a Value, key: &str, path: &str) -> Result<&'a [Value], String> {
    required(value, key, path)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| format!("{path}.{key} must be an array"))
}

fn str_field<'a>(value: &'a Value, key: &str, path: &str) -> Result<&'a str, String> {
    required(value, key, path)?
        .as_str()
        .ok_or_else(|| format!("{path}.{key} must be a string"))
}

fn float_field(value: &Value, key: &str, path: &str) -> Result<Float0, String> {
    number(required(value, key, path)?, &format!("{path}.{key}"))
}

fn bool_field(value: &Value, key: &str, path: &str) -> Result<Option<bool>, String> {
    value
        .get(key)
        .map(|value| {
            value
                .as_bool()
                .ok_or_else(|| format!("{path}.{key} must be a boolean"))
        })
        .transpose()
}

fn optional_float2_field(value: &Value, key: &str, path: &str) -> Result<Option<Float2>, String> {
    value
        .get(key)
        .map(|value| float2(value, &format!("{path}.{key}")))
        .transpose()
}

fn optional_float3_field(value: &Value, key: &str, path: &str) -> Result<Option<Float3>, String> {
    value
        .get(key)
        .map(|value| float3(value, &format!("{path}.{key}")))
        .transpose()
}

fn float3_field(value: &Value, key: &str, path: &str) -> Result<Float3, String> {
    float3(required(value, key, path)?, &format!("{path}.{key}"))
}

fn float2(value: &Value, path: &str) -> Result<Float2, String> {
    let values = number_array::<2>(value, path)?;
    Ok(Float2::new(values))
}

fn float3(value: &Value, path: &str) -> Result<Float3, String> {
    let values = number_array::<3>(value, path)?;
    Ok(Float3::new(values))
}

fn number_array<const N: usize>(value: &Value, path: &str) -> Result<[Float0; N], String> {
    let values = value
        .as_array()
        .ok_or_else(|| format!("{path} must be an array of {N} numbers"))?;
    if values.len() != N {
        return Err(format!("{path} must contain exactly {N} numbers"));
    }

    let mut out = [0.0; N];
    for (index, value) in values.iter().enumerate() {
        out[index] = number(value, &format!("{path}[{index}]"))?;
    }
    Ok(out)
}

fn number(value: &Value, path: &str) -> Result<Float0, String> {
    value
        .as_float()
        .or_else(|| value.as_integer().map(|integer| integer as Float0))
        .ok_or_else(|| format!("{path} must be a number"))
}

fn vertices_field(value: &Value, key: &str, path: &str) -> Result<Vec<Float3>, String> {
    array_field(value, key, path)?
        .iter()
        .enumerate()
        .map(|(index, value)| float3(value, &format!("{path}.{key}[{index}]")))
        .collect()
}

fn indices_field(value: &Value, key: &str, path: &str) -> Result<Vec<[usize; 3]>, String> {
    let values = array_field(value, key, path)?;
    if values.first().is_some_and(Value::is_array) {
        values
            .iter()
            .enumerate()
            .map(|(triangle_index, triangle)| {
                let indices = triangle.as_array().ok_or_else(|| {
                    format!("{path}.{key}[{triangle_index}] must be an index array")
                })?;
                if indices.len() != 3 {
                    return Err(format!(
                        "{path}.{key}[{triangle_index}] must contain exactly 3 indices"
                    ));
                }
                Ok([
                    usize_value(&indices[0], &format!("{path}.{key}[{triangle_index}][0]"))?,
                    usize_value(&indices[1], &format!("{path}.{key}[{triangle_index}][1]"))?,
                    usize_value(&indices[2], &format!("{path}.{key}[{triangle_index}][2]"))?,
                ])
            })
            .collect()
    } else {
        if !values.len().is_multiple_of(3) {
            return Err(format!(
                "{path}.{key} flat index array length must divide by 3"
            ));
        }
        values
            .chunks_exact(3)
            .enumerate()
            .map(|(triangle_index, chunk)| {
                Ok([
                    usize_value(&chunk[0], &format!("{path}.{key}[{}]", triangle_index * 3))?,
                    usize_value(
                        &chunk[1],
                        &format!("{path}.{key}[{}]", triangle_index * 3 + 1),
                    )?,
                    usize_value(
                        &chunk[2],
                        &format!("{path}.{key}[{}]", triangle_index * 3 + 2),
                    )?,
                ])
            })
            .collect()
    }
}

fn usize_value(value: &Value, path: &str) -> Result<usize, String> {
    let integer = value
        .as_integer()
        .ok_or_else(|| format!("{path} must be an integer index"))?;
    usize::try_from(integer).map_err(|_| format!("{path} must be non-negative"))
}

#[cfg(test)]
mod tests {
    use super::Scene;

    #[test]
    fn reports_missing_object_field_path() {
        let scene = toml::from_str(
            r#"
            [[objects]]
            type = "sphere"
            radius = 1.0

            [camera]
            position = [0.0, 0.0, 0.0]
            rotation = [0.0, 0.0, 0.0]

            [skybox]
            color = [0.0, 0.0, 0.0]
            "#,
        )
        .unwrap();

        let error = Scene::try_from_toml(&scene).unwrap_err();
        assert!(error.contains("objects[0].position"));
    }

    #[test]
    fn parses_triangle_meshes_and_area_lights() {
        let scene = toml::from_str(
            r#"
            [[objects]]
            type = "triangle_mesh"
            vertices = [
                [0.0, 0.0, -1.0],
                [1.0, 0.0, -1.0],
                [0.0, 1.0, -1.0],
            ]
            indices = [[0, 1, 2]]
            material = "white"

            [[lights]]
            type = "area"
            point1 = [-0.5, 1.0, -1.0]
            point2 = [0.5, 1.0, -1.0]
            point3 = [0.5, 1.0, 0.0]
            point4 = [-0.5, 1.0, 0.0]
            color = [2.0, 2.0, 2.0]

            [camera]
            position = [0.0, 0.0, 1.0]
            rotation = [0.0, 0.0, 0.0]

            [skybox]
            color = [0.0, 0.0, 0.0]
            "#,
        )
        .unwrap();

        let scene = Scene::try_from_toml(&scene).unwrap();
        assert_eq!(scene.objects.len(), 1);
        assert_eq!(scene.lights.len(), 1);
    }
}
