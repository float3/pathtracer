use crate::{scene::FloatSize, utils::vector::Vec3};

pub struct Camera {
    position: Vec3<FloatSize>,
    direction: Vec3<FloatSize>,
    up: Vec3<FloatSize>,
    right: Vec3<FloatSize>,
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new(
        position: Vec3<FloatSize>,
        direction: Vec3<FloatSize>,
        up: Vec3<FloatSize>,
        fov: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) -> Camera {
        let direction = direction.normalize();
        let right = up.cross(&direction).normalize();
        let up = direction.cross(&right).normalize();
        Camera {
            position,
            direction,
            up,
            right,
            fov,
            aspect_ratio,
            near,
            far,
        }
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Vec3<FloatSize> {
        let x = (2.0 * x - 1.0) * self.aspect_ratio * (self.fov.to_radians() / 2.0).tan();
        let y = (1.0 - 2.0 * y) * (self.fov.to_radians() / 2.0).tan();
        let direction = self.direction + self.right.scale(x) + self.up.scale(y);
        direction.normalize()
    }
}
