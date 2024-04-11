use crate::{scene::Flooat, utils::vector::Float3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Float3,
    pub direction: Float3,
}

impl Ray {
    // pub(crate) fn create(x: usize, y: usize, width: usize, height: usize) -> Ray {
    //     let aspect_ratio = width as f32 / height as f32;
    //     let fov = 90.0;
    //     let angle = (fov * 0.5).to_radians();
    //     let scale = (angle.tan(), angle.tan() / aspect_ratio);

    //     let x = (2.0 * (x as f32 + 0.5) / width as f32 - 1.0) * scale.0;
    //     let y = (1.0 - 2.0 * (y as f32 + 0.5) / height as f32) * scale.1;

    //     Ray {
    //         origin: Vec3::new([0.0, 0.0, 0.0]),
    //         direction: Vec3::new([x, y, -1.0]).normalize(),
    //     }
    // }

    pub fn new(origin: Float3, direction: Float3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, root: Flooat) -> Float3 {
        self.origin + self.direction.scale(root)
    }
}
