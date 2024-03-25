use rand::{rngs::ThreadRng, Rng};

use crate::{ray::Ray, scene::FloatSize, utils::vector::Vec3};

pub struct Camera {
    position: Vec3<FloatSize>,
    direction: Vec3<FloatSize>,
    // up: Vec3<FloatSize>,
    // right: Vec3<FloatSize>,
    // fov: f32,
    // aspect_ratio: f32,
    // near: f32,
    // far: f32,
}

impl Camera {
    pub fn new(
        position: Vec3<FloatSize>,
        direction: Vec3<FloatSize>,
        // up: Vec3<FloatSize>,
        // fov: f32,
        // aspect_ratio: f32,
        // near: f32,
        // far: f32,
    ) -> Camera {
        let direction = direction.normalize();
        // let right = up.cross(&direction).normalize();
        // let up = direction.cross(&right).normalize();
        Camera {
            position,
            direction,
            // up,
            // right,
            // fov,
            // aspect_ratio,
            // near,
            // far,
        }
    }

    pub fn get_ray(
        &self,
        x: FloatSize,
        y: FloatSize,
        width: FloatSize,
        height: FloatSize,
        rand_state: &mut ThreadRng,
    ) -> Ray {
        // let x = (2.0 * x - 1.0) * self.aspect_ratio * (self.fov.to_radians() / 2.0).tan();
        // let y = (1.0 - 2.0 * y) * (self.fov.to_radians() / 2.0).tan();
        // let direction = self.direction + self.right.scale(x) + self.up.scale(y);

        let x = x + rand_state.gen_range(0.0..1.0) as FloatSize;
        let y = y + rand_state.gen_range(0.0..1.0) as FloatSize;

        let x0 = (x / width) * 2.0 - 1.0;
        let y0 = (y / height) * 2.0 - 1.0;
        let direction = Vec3::new([x0 * width / height, -y0, -1.0]);

        Ray {
            direction: direction.normalize(),
            origin: self.position,
        }
    }
}
