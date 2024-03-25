use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub};

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

    pub fn length(&self) -> T {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len: T = self.length();
        let mut result = [T::default(); N];
        for (i, item) in result.iter_mut().enumerate().take(N) {
            *item = self.0[i] / len;
        }
        Vector(result)
    }

    pub fn scale(&self, scalar: T) -> Self {
        let mut result = [T::default(); N];
        for (i, item) in result.iter_mut().enumerate().take(N) {
            *item = self.0[i] * scalar;
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

impl<T> Vector<T, 3>
where
    T: Float,
{
    pub fn cross(&self, other: &Self) -> Self {
        let result = [
            self.0[1] * other.0[2] - self.0[2] * other.0[1],
            self.0[2] * other.0[0] - self.0[0] * other.0[2],
            self.0[0] * other.0[1] - self.0[1] * other.0[0],
        ];
        Vector(result)
    }
}

impl<T, const N: usize> Add for Vector<T, N>
where
    T: Add<Output = T> + Copy + Default + Float,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut result = [T::default(); N];
        for (i, item) in result.iter_mut().enumerate().take(N) {
            *item = self.0[i] + other.0[i];
        }
        Vector(result)
    }
}

impl<T, const N: usize> AddAssign for Vector<T, N>
where
    T: Add<Output = T> + Copy + Default + Float,
{
    fn add_assign(&mut self, other: Self) {
        for (i, item) in self.0.iter_mut().enumerate().take(N) {
            *item = *item + other.0[i];
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
        for (i, item) in result.iter_mut().enumerate().take(N) {
            *item = -self.0[i];
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
        for (i, item) in result.iter_mut().enumerate().take(N) {
            *item = self.0[i] - other.0[i];
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
        for (i, item) in result.iter_mut().enumerate().take(N) {
            *item = self.0[i] * other.0[i];
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
#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_new() {
        let v = Vector::new([1.0, 2.0, 3.0]);
        assert_eq!(v.0, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_dot() {
        let v1 = Vector::new([1.0, 2.0, 3.0]);
        let v2 = Vector::new([4.0, 5.0, 6.0]);
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn test_length() {
        let v = Vector::new([1.0, 2.0, 2.0]);
        assert_eq!(v.length(), 3.0);
    }

    #[test]
    fn test_normalize() {
        let v = Vector::new([1.0, 2.0, 2.0]);
        let normalized = v.normalize();
        assert!(approx_eq!(
            f64,
            normalized.length(),
            1.0,
            epsilon = Float::epsilon(),
            ulps = 2
        ));
    }

    #[test]
    fn test_scale() {
        let v = Vector::new([1.0, 2.0, 3.0]);
        let scaled = v.scale(2.0);
        assert_eq!(scaled.0, [2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_length_squared() {
        let v = Vector::new([1.0, 2.0, 2.0]);
        assert_eq!(v.length_squared(), 9.0);
    }

    #[test]
    fn test_near_zero() {
        let v = Vector::new([1e-9, 1e-9, 1e-9]);
        assert!(v.near_zero());
    }

    #[test]
    fn test_cross() {
        let v1 = Vector::new([1.0, 0.0, 0.0]);
        let v2 = Vector::new([0.0, 1.0, 0.0]);
        let cross = v1.cross(&v2);
        assert_eq!(cross.0, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_add() {
        let v1 = Vector::new([1.0, 2.0, 3.0]);
        let v2 = Vector::new([4.0, 5.0, 6.0]);
        let sum = v1 + v2;
        assert_eq!(sum.0, [5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vector::new([1.0, 2.0, 3.0]);
        let v2 = Vector::new([4.0, 5.0, 6.0]);
        v1 += v2;
        assert_eq!(v1.0, [5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_neg() {
        let v = Vector::new([1.0, 2.0, 3.0]);
        let neg = -v;
        assert_eq!(neg.0, [-1.0, -2.0, -3.0]);
    }

    #[test]
    fn test_sub() {
        let v1 = Vector::new([1.0, 2.0, 3.0]);
        let v2 = Vector::new([4.0, 5.0, 6.0]);
        let diff = v1 - v2;
        assert_eq!(diff.0, [-3.0, -3.0, -3.0]);
    }

    #[test]
    fn test_mul() {
        let v1 = Vector::new([1.0, 2.0, 3.0]);
        let v2 = Vector::new([4.0, 5.0, 6.0]);
        let product = v1 * v2;
        assert_eq!(product.0, [4.0, 10.0, 18.0]);
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Vector::new([1.0, 2.0, 3.0]);
        let v2 = Vector::new([4.0, 5.0, 6.0]);
        v1 *= v2;
        assert_eq!(v1.0, [4.0, 10.0, 18.0]);
    }
}
