use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use toml::Value;

use crate::scene::{FloatSize, IntSize};

trait Vector:
    Add
    + AddAssign
    + Div
    + DivAssign
    + Mul
    + MulAssign
    + Neg
    + Sub
    + SubAssign
    + PartialEq
    + PartialOrd
    + Eq
    + Ord
    + Sized
    + Copy
    + Clone
    + Debug
{
    fn new(x: [FloatSize]) -> Self;
    fn from_toml(value: &Value) -> Self;
    fn to_toml(&self) -> Value;
    fn dot(&self, other: &Self) -> FloatSize;
    fn length(&self) -> FloatSize;
    fn length_squared(&self) -> FloatSize;
    fn normalize(&self) -> Self;
    fn scale(&self, scalar: FloatSize) -> Self;
    fn cross(&self, other: &Self) -> Self;
    fn near_zero(&self) -> bool;
    fn as_array(&self) -> [FloatSize];
}

#[derive(Debug, Copy, Clone)]
struct Float2 {
    x: FloatSize,
    y: FloatSize,
}

#[derive(Debug, Copy, Clone)]
struct Float3 {
    x: FloatSize,
    y: FloatSize,
    z: FloatSize,
}

#[derive(Debug, Copy, Clone)]
struct Float4 {
    x: FloatSize,
    y: FloatSize,
    z: FloatSize,
    w: FloatSize,
}

impl Vector for Float1 {
    fn new(x: [FloatSize]) -> Self {
        Float1 { x: x[0] }
    }

    fn from_toml(value: &Value) -> Self {}

    fn to_toml(&self) -> Value {
        todo!()
    }

    fn dot(&self, other: &Self) -> FloatSize {
        self.x * other.x
    }

    fn length(&self) -> FloatSize {
        self.dot(self).sqrt()
    }

    fn length_squared(&self) -> FloatSize {
        self.dot(self)
    }

    fn normalize(&self) -> Self {
        Float1 {
            x: self.x / self.length(),
        }
    }

    fn scale(&self, scalar: FloatSize) -> Self {
        Float1 { x: self.x * scalar }
    }

    fn cross(&self, other: &Self) -> Self {
        todo!()
    }

    fn near_zero(&self) -> bool {
        let s: FloatSize = 1e-8;
        self.x.abs() < s
    }

    fn as_array(&self) -> [FloatSize] {
        todo!()
    }
}

impl Ord for Float1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.partial_cmp(&other.x).unwrap()
    }
}

impl Eq for Float1 {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for Float1 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl PartialOrd for Float1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl Sub for Float1 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Float1 { x: self.x - rhs.x }
    }
}

impl SubAssign for Float1 {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub(rhs);
    }
}

impl Add for Float1 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Float1 { x: self.x + rhs.x }
    }
}

impl AddAssign for Float1 {
    fn add_assign(&mut self, rhs: Self) {
        self.add(rhs);
    }
}

impl Mul for Float1 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Float1 { x: self.x * rhs.x }
    }
}

impl MulAssign for Float1 {
    fn mul_assign(&mut self, rhs: Self) {
        self.mul(rhs);
    }
}

impl Div for Float1 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Float1 { x: self.x / rhs.x }
    }
}

impl DivAssign for Float1 {
    fn div_assign(&mut self, rhs: Self) {
        self.div(rhs);
    }
}

impl Neg for Float1 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Float1 { x: -self.x }
    }
}

impl Vector for Float2 {
    fn new(x: [FloatSize]) -> Self {
        todo!()
    }

    fn from_toml(value: &Value) -> Self {
        todo!()
    }

    fn to_toml(&self) -> Value {
        todo!()
    }

    fn dot(&self, other: &Self) -> FloatSize {
        todo!()
    }

    fn length(&self) -> FloatSize {
        todo!()
    }

    fn length_squared(&self) -> FloatSize {
        todo!()
    }

    fn normalize(&self) -> Self {
        todo!()
    }

    fn scale(&self, scalar: FloatSize) -> Self {
        todo!()
    }

    fn cross(&self, other: &Self) -> Self {
        todo!()
    }

    fn near_zero(&self) -> bool {
        todo!()
    }

    fn as_array(&self) -> [FloatSize] {
        todo!()
    }
}

impl Vector for Float3 {
    fn new(x: [FloatSize]) -> Self {
        todo!()
    }

    fn from_toml(value: &Value) -> Self {
        todo!()
    }

    fn to_toml(&self) -> Value {
        todo!()
    }

    fn dot(&self, other: &Self) -> FloatSize {
        todo!()
    }

    fn length(&self) -> FloatSize {
        todo!()
    }

    fn length_squared(&self) -> FloatSize {
        todo!()
    }

    fn normalize(&self) -> Self {
        todo!()
    }

    fn scale(&self, scalar: FloatSize) -> Self {
        todo!()
    }

    fn cross(&self, other: &Self) -> Self {
        todo!()
    }

    fn near_zero(&self) -> bool {
        todo!()
    }

    fn as_array(&self) -> [FloatSize] {
        todo!()
    }
}

impl Vector for Float4 {
    fn new(x: [FloatSize]) -> Self {
        todo!()
    }

    fn from_toml(value: &Value) -> Self {
        todo!()
    }

    fn to_toml(&self) -> Value {
        todo!()
    }

    fn dot(&self, other: &Self) -> FloatSize {
        todo!()
    }

    fn length(&self) -> FloatSize {
        todo!()
    }

    fn normalize(&self) -> Self {
        todo!()
    }

    fn length_squared(&self) -> FloatSize {
        todo!()
    }

    fn scale(&self, scalar: FloatSize) -> Self {
        todo!()
    }

    fn cross(&self, other: &Self) -> Self {
        todo!()
    }

    fn near_zero(&self) -> bool {
        todo!()
    }

    fn as_array(&self) -> [FloatSize] {
        todo!()
    }
}
