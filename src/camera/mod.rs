use rand::Rng;

use crate::{
    ray::Ray,
    scene::{Float0, RNGType},
    utils::{matrix::Float3x3, vector::Float3},
};

#[derive(Debug)]
pub struct Camera {
    pub position: Float3,
    pub rotation: Float3,
}

impl Camera {
    pub fn get_ray(
        &self,
        x: Float0,
        y: Float0,
        width: Float0,
        height: Float0,
        rand_state: &mut RNGType,
    ) -> Ray {
        let x = x + rand_state.random_range(0.0..1.0) as Float0;
        let y = y + rand_state.random_range(0.0..1.0) as Float0;

        let x0 = (x / width) * 2.0 - 1.0;
        let y0 = (y / height) * 2.0 - 1.0;
        let mut direction = Float3::new([x0 * width / height, -y0, -1.0]);

        let rotation_matrix = self.get_rotation_matrix();

        direction = rotation_matrix.multiply_by_vector(&direction);

        Ray {
            direction: direction.normalize(),
            origin: self.position,
        }
    }

    fn get_rotation_matrix(&self) -> Float3x3 {
        let yaw = self.rotation.x().to_radians();
        let pitch = self.rotation.y().to_radians();
        let roll = self.rotation.z().to_radians();

        let rotation_z = Float3x3::new([
            [yaw.cos(), -yaw.sin(), 0.0],
            [yaw.sin(), yaw.cos(), 0.0],
            [0.0, 0.0, 1.0],
        ]);
        let rotation_y = Float3x3::new([
            [pitch.cos(), 0.0, pitch.sin()],
            [0.0, 1.0, 0.0],
            [-pitch.sin(), 0.0, pitch.cos()],
        ]);
        let rotation_x = Float3x3::new([
            [1.0, 0.0, 0.0],
            [0.0, roll.cos(), -roll.sin()],
            [0.0, roll.sin(), roll.cos()],
        ]);

        rotation_z * rotation_y * rotation_x
    }
}
