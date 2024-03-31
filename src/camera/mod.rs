use rand::{rngs::ThreadRng, Rng};

use crate::{
    ray::Ray,
    scene::FloatSize,
    utils::{matrix::Matrix, vector::Vec3},
};

#[derive(Debug)]
pub struct Camera {
    pub position: Vec3<FloatSize>,
    pub rotation: Vec3<FloatSize>,
}

impl Camera {
    pub fn get_ray(
        &self,
        x: FloatSize,
        y: FloatSize,
        width: FloatSize,
        height: FloatSize,
        rand_state: &mut ThreadRng,
    ) -> Ray {
        let x = x + rand_state.gen_range(0.0..1.0) as FloatSize;
        let y = y + rand_state.gen_range(0.0..1.0) as FloatSize;

        let x0 = (x / width) * 2.0 - 1.0;
        let y0 = (y / height) * 2.0 - 1.0;
        let mut direction = Vec3::new([x0 * width / height, -y0, -1.0]);

        // Create rotation matrix from Euler angles
        let rotation_matrix = self.get_rotation_matrix();

        // Apply the rotation matrix to the direction vector
        direction = rotation_matrix * direction;

        Ray {
            direction: direction.normalize(),
            origin: self.position,
        }
    }

    fn get_rotation_matrix(&self) -> Matrix<FloatSize, 3, 3> {
        let yaw = self.rotation.x().to_radians();
        let pitch = self.rotation.y().to_radians();
        let roll = self.rotation.z().to_radians();

        // Create rotation matrices for each Euler angle
        let rotation_z = Matrix::new([
            [yaw.cos(), -yaw.sin(), 0.0],
            [yaw.sin(), yaw.cos(), 0.0],
            [0.0, 0.0, 1.0],
        ]);
        let rotation_y = Matrix::new([
            [pitch.cos(), 0.0, pitch.sin()],
            [0.0, 1.0, 0.0],
            [-pitch.sin(), 0.0, pitch.cos()],
        ]);
        let rotation_x = Matrix::new([
            [1.0, 0.0, 0.0],
            [0.0, roll.cos(), -roll.sin()],
            [0.0, roll.sin(), roll.cos()],
        ]);

        rotation_z * rotation_y * rotation_x
    }
}
