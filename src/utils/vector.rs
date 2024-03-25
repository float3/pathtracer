use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use num_traits::Float;

#[derive(Debug, Clone, Copy)]
pub struct Vector<T, const N: usize>(pub [T; N]);

impl<T, const N: usize> Vector<T, N>
where
    T: Copy
        + Add<Output = T>
        + AddAssign<T>
        + Neg<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + MulAssign<T>
        + Div<Output = T>
        + DivAssign<T>
        + Default
        + From<T>
        + Into<T>
        + Float,
{
    pub fn new(elements: [T; N]) -> Self {
        Vector(elements)
    }

    pub fn dot(&self, other: &Self) -> T {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(T::default(), |acc, (&a, &b)| acc + a * b)
    }

    pub fn cross(&self, other: &Self) -> Self {
        assert_eq!(N, 3);
        let mut result = [T::default(); N];
        result[0] = self.0[1] * other.0[2] - self.0[2] * other.0[1];
        result[1] = self.0[2] * other.0[0] - self.0[0] * other.0[2];
        result[2] = self.0[0] * other.0[1] - self.0[1] * other.0[0];
        Vector(result)
    }

    pub fn length(&self) -> T {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len: T = self.length();
        let mut result = [T::default(); N];
        for i in 0..N {
            result[i] = self.0[i] / len;
        }
        Vector(result)
    }

    pub fn scale(&self, scalar: T) -> Self {
        let mut result = [T::default(); N];
        for i in 0..N {
            result[i] = self.0[i] * scalar;
        }
        Vector(result)
    }

    pub fn length_squared(&self) -> T {
        let x = self.length();
        x * x
    }

    pub fn near_zero(&self) -> bool {
        let s: T = num_traits::NumCast::from(1e-8).unwrap();
        self.0.iter().all(|&x| x.abs() < s)
    }
}

impl<T, const N: usize> Add for Vector<T, N>
where
    T: Add<Output = T> + Copy + Default + Float,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut result = [T::default(); N];
        for i in 0..N {
            result[i] = self.0[i] + other.0[i];
        }
        Vector(result)
    }
}

impl<T, const N: usize> AddAssign for Vector<T, N>
where
    T: Add<Output = T> + Copy + Default + Float,
{
    fn add_assign(&mut self, other: Self) {
        for i in 0..N {
            self.0[i] = self.0[i] + other.0[i];
        }
    }
}

impl<T, const N: usize> Neg for Vector<T, N>
where
    T: Neg<Output = T> + Copy + Default + Float,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut result = [T::default(); N];
        for i in 0..N {
            result[i] = -self.0[i];
        }
        Vector(result)
    }
}

impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Sub<Output = T> + Copy + Default + Float,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = [T::default(); N];
        for i in 0..N {
            result[i] = self.0[i] - other.0[i];
        }
        Vector(result)
    }
}

impl<T, const N: usize> Mul for Vector<T, N>
where
    T: Mul<Output = T> + Copy + Default + Float,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut result = [T::default(); N];
        for i in 0..N {
            result[i] = self.0[i] * other.0[i];
        }
        Vector(result)
    }
}

impl<T, const N: usize> MulAssign for Vector<T, N>
where
    T: Mul<Output = T> + Copy + Default + Float,
{
    fn mul_assign(&mut self, other: Self) {
        for i in 0..N {
            self.0[i] = self.0[i] * other.0[i];
        }
    }
}

impl<T, const N: usize> Div<T> for Vector<T, N>
where
    T: Div<Output = T> + Copy + Default + Float,
{
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        let mut result = [T::default(); N];
        for i in 0..N {
            result[i] = self.0[i] / scalar;
        }
        Vector(result)
    }
}

impl<T, const N: usize> DivAssign<T> for Vector<T, N>
where
    T: Div<Output = T> + Copy + Default + Float,
{
    fn div_assign(&mut self, scalar: T) {
        for i in 0..N {
            self.0[i] = self.0[i] / scalar;
        }
    }
}

pub type Vec1<T> = Vector<T, 1>;
pub type Vec2<T> = Vector<T, 2>;
pub type Vec3<T> = Vector<T, 3>;
pub type Vec4<T> = Vector<T, 4>;

impl<T> Vec1<T> {
    pub fn x(&self) -> &T {
        &self.0[0]
    }
}

impl<T> Vec2<T> {
    pub fn x(&self) -> &T {
        &self.0[0]
    }

    pub fn y(&self) -> &T {
        &self.0[1]
    }
}

impl<T> Vec3<T> {
    pub fn x(&self) -> &T {
        &self.0[0]
    }

    pub fn y(&self) -> &T {
        &self.0[1]
    }

    pub fn z(&self) -> &T {
        &self.0[2]
    }
}

impl<T> Vec4<T> {
    pub fn x(&self) -> &T {
        &self.0[0]
    }

    pub fn y(&self) -> &T {
        &self.0[1]
    }

    pub fn z(&self) -> &T {
        &self.0[2]
    }

    pub fn w(&self) -> &T {
        &self.0[3]
    }
}
