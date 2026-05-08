use crate::{ray::Ray, scene::Float0, utils::vector::Float3};

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Float3,
    pub max: Float3,
}

impl Aabb {
    pub fn new(min: Float3, max: Float3) -> Self {
        Self { min, max }.padded()
    }

    pub fn from_points(points: &[Float3]) -> Option<Self> {
        let first = *points.first()?;
        let mut min = first;
        let mut max = first;
        for point in points.iter().skip(1) {
            for axis in 0..3 {
                min.0[axis] = min.0[axis].min(point.0[axis]);
                max.0[axis] = max.0[axis].max(point.0[axis]);
            }
        }
        Some(Self::new(min, max))
    }

    pub fn surrounding(a: Self, b: Self) -> Self {
        let mut min = a.min;
        let mut max = a.max;
        for axis in 0..3 {
            min.0[axis] = min.0[axis].min(b.min.0[axis]);
            max.0[axis] = max.0[axis].max(b.max.0[axis]);
        }
        Self::new(min, max)
    }

    pub fn centroid_axis(&self, axis: usize) -> Float0 {
        (self.min.0[axis] + self.max.0[axis]) * 0.5
    }

    pub fn longest_axis(&self) -> usize {
        let extents = self.max - self.min;
        if extents.0[0] > extents.0[1] && extents.0[0] > extents.0[2] {
            0
        } else if extents.0[1] > extents.0[2] {
            1
        } else {
            2
        }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: Float0, mut t_max: Float0) -> bool {
        for axis in 0..3 {
            let origin = ray.origin.0[axis];
            let direction = ray.direction.0[axis];
            if direction.abs() < 1e-12 {
                if origin < self.min.0[axis] || origin > self.max.0[axis] {
                    return false;
                }
                continue;
            }

            let inv_d = 1.0 / direction;
            let mut t0 = (self.min.0[axis] - origin) * inv_d;
            let mut t1 = (self.max.0[axis] - origin) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    fn padded(mut self) -> Self {
        for axis in 0..3 {
            if (self.max.0[axis] - self.min.0[axis]).abs() < 1e-6 {
                self.min.0[axis] -= 1e-6;
                self.max.0[axis] += 1e-6;
            }
        }
        self
    }
}
