use rand::RngExt;

use crate::{
    scene::{Float0, RNGType},
    utils::vector::Float3,
};

use super::{Light, LightSample};

#[derive(Debug, Clone)]
pub struct Arealight {
    a: Float3,
    b: Float3,
    d: Float3,
    color: Float3,
}

impl Arealight {
    pub fn new(a: Float3, b: Float3, d: Float3, color: Float3) -> Self {
        Self { a, b, d, color }
    }

    fn edge_u(&self) -> Float3 {
        self.b - self.a
    }

    fn edge_v(&self) -> Float3 {
        self.d - self.a
    }

    fn area(&self) -> Float0 {
        self.edge_u().cross(&self.edge_v()).length()
    }

    fn normal(&self) -> Float3 {
        self.edge_u().cross(&self.edge_v()).normalize()
    }
}

impl Light for Arealight {
    fn illuminate(&self) -> Float3 {
        self.color
    }

    fn position(&self) -> Float3 {
        self.a + (self.edge_u() + self.edge_v()).scale(0.5)
    }

    fn sample(&self, point: Float3, rand_state: &mut RNGType) -> LightSample {
        let u = rand_state.random_range(0.0..1.0);
        let v = rand_state.random_range(0.0..1.0);
        let sample_point = self.a + self.edge_u().scale(u) + self.edge_v().scale(v);
        let to_light = sample_point - point;
        let distance = to_light.length();
        let direction = to_light.normalize();
        let cos_light = self.normal().dot(&-direction).max(0.0);
        let area = self.area();

        if area <= 0.0 || cos_light <= 0.0 {
            return LightSample {
                direction,
                distance,
                radiance: Float3::new([0.0, 0.0, 0.0]),
                pdf: 1.0,
                delta: false,
            };
        }

        LightSample {
            direction,
            distance,
            radiance: self.color,
            pdf: distance * distance / (cos_light * area),
            delta: false,
        }
    }

    fn intensity(&self) -> Float0 {
        self.color.length() * self.area()
    }

    fn color(&self) -> Float3 {
        self.color
    }
}
