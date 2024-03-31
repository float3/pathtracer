use std::{
    iter::Sum,
    ops::{Add, AddAssign, Index, IndexMut, Mul},
};

use super::vector::Vector;

#[derive(Debug, Clone)]
pub struct Matrix<T, const M: usize, const N: usize>(pub [[T; N]; M]);

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    pub fn new(elements: [[T; N]; M]) -> Self {
        Matrix(elements)
    }
}

impl<T, const M: usize, const N: usize> Index<usize> for Matrix<T, M, N> {
    type Output = [T; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const M: usize, const N: usize> IndexMut<usize> for Matrix<T, M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T, const M: usize, const N: usize> Mul<Vector<T, N>> for Matrix<T, M, N>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + Sum,
{
    type Output = Vector<T, M>;

    fn mul(self, rhs: Vector<T, N>) -> Self::Output {
        let mut result = [T::default(); M];
        for i in 0..M {
            result[i] = self.0[i]
                .iter()
                .zip(rhs.0.iter())
                .map(|(&a, &b)| a * b)
                .sum();
        }
        Vector(result)
    }
}

impl<T, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, N, P>> for Matrix<T, M, N>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + AddAssign + Sum,
{
    type Output = Matrix<T, M, P>;

    fn mul(self, rhs: Matrix<T, N, P>) -> Self::Output {
        let mut result = [[T::default(); P]; M];
        for i in 0..M {
            for j in 0..P {
                let mut sum = T::default();
                for k in 0..N {
                    sum += self.0[i][k] * rhs.0[k][j];
                }
                result[i][j] = sum;
            }
        }
        Matrix(result)
    }
}

impl<T, const M: usize, const N: usize> PartialEq for Matrix<T, M, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(self_row, other_row)| self_row.iter().zip(other_row.iter()).all(|(a, b)| a == b))
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_new() {
        let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(matrix[0], [1, 2, 3]);
        assert_eq!(matrix[1], [4, 5, 6]);
    }

    #[test]
    fn test_matrix_index() {
        let matrix = Matrix::new([[7, 8], [9, 10]]);
        assert_eq!(matrix[0][1], 8);
        assert_eq!(matrix[1][0], 9);
    }

    #[test]
    fn test_matrix_vector_multiplication() {
        let matrix = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
        let vector = Vector::new([1.0, 1.0]);
        let result = matrix * vector;
        assert_eq!(result, Vector::new([3.0, 7.0]));
    }

    #[test]
    fn test_matrix_matrix_multiplication() {
        // Define two matrices A (2x3) and B (3x2)
        let a = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let b = Matrix::new([[7, 8], [9, 10], [11, 12]]);

        // Expected result of A * B
        let expected = Matrix::new([
            [58, 64],   // = 1*7 + 2*9 + 3*11, 1*8 + 2*10 + 3*12
            [139, 154], // = 4*7 + 5*9 + 6*11, 4*8 + 5*10 + 6*12
        ]);

        // Perform the multiplication
        let result = a * b;

        // Check that each element matches the expected result
        assert_eq!(result, expected);
    }
}
