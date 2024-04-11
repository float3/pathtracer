use std::{
    iter::Sum,
    ops::{Add, AddAssign, Index, IndexMut, Mul},
};

use crate::scene::{Flooat, Int};

use super::vector::Vector;

#[derive(Debug, Clone)]
pub struct Matrix<T, const M: usize, const N: usize>(pub [[T; N]; M]);

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + Sum,
{
    pub fn new(elements: [[T; N]; M]) -> Self {
        Matrix(elements)
    }

    pub fn multiply_by_vector(&self, rhs: &Vector<T, N>) -> Vector<T, M> {
        let mut result = [T::default(); M];
        for (i, res) in result.iter_mut().enumerate() {
            *res = self.0[i]
                .iter()
                .zip(rhs.0.iter())
                .map(|(&a, &b)| a * b)
                .sum();
        }
        Vector(result)
    }

    pub fn new_from_columns(columns: [[T; M]; N]) -> Self {
        let mut elements = [[T::default(); N]; M];
        for (i, column) in columns.iter().enumerate() {
            for (j, elem) in column.iter().enumerate() {
                elements[j][i] = *elem;
            }
        }
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

impl<T, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, N, P>> for Matrix<T, M, N>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + AddAssign + Sum,
{
    type Output = Matrix<T, M, P>;

    fn mul(self, rhs: Matrix<T, N, P>) -> Self::Output {
        let mut result = [[T::default(); P]; M];
        for (i, row) in result.iter_mut().enumerate() {
            for (j, elem) in row.iter_mut().enumerate().take(P) {
                let mut sum = T::default();
                for k in 0..N {
                    sum += self.0[i][k] * rhs.0[k][j];
                }
                *elem = sum;
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
}

pub type Float1x1 = Matrix<Flooat, 1, 1>;
// pub type Float1x2 = Matrix<FloatSize, 1, 2>;
// pub type Float1x3 = Matrix<FloatSize, 1, 3>;
// pub type Float1x4 = Matrix<FloatSize, 1, 4>;
// pub type Float2x1 = Matrix<FloatSize, 2, 1>;
pub type Float2x2 = Matrix<Flooat, 2, 2>;
// pub type Float2x3 = Matrix<FloatSize, 2, 3>;
// pub type Float2x4 = Matrix<FloatSize, 2, 4>;
// pub type Float3x1 = Matrix<FloatSize, 3, 1>;
// pub type Float3x2 = Matrix<FloatSize, 3, 2>;
pub type Float3x3 = Matrix<Flooat, 3, 3>;
// pub type Float3x4 = Matrix<FloatSize, 3, 4>;
// pub type Float4x1 = Matrix<FloatSize, 4, 1>;
// pub type Float4x2 = Matrix<FloatSize, 4, 2>;
// pub type Float4x3 = Matrix<FloatSize, 4, 3>;
pub type Float4x4 = Matrix<Flooat, 4, 4>;
pub type Int1x1 = Matrix<Int, 1, 1>;
pub type Int2x2 = Matrix<Int, 2, 2>;
pub type Int3x3 = Matrix<Int, 3, 3>;
pub type Int4x4 = Matrix<Int, 4, 4>;

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
        let result = matrix.multiply_by_vector(&vector);
        assert_eq!(result, Vector::new([3.0, 7.0]));
    }

    #[test]
    fn test_matrix_matrix_multiplication() {
        let a = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let b = Matrix::new([[7, 8], [9, 10], [11, 12]]);

        let expected = Matrix::new([
            [58, 64],   // = 1*7 + 2*9 + 3*11, 1*8 + 2*10 + 3*12
            [139, 154], // = 4*7 + 5*9 + 6*11, 4*8 + 5*10 + 6*12
        ]);

        let result = a * b;
        assert_eq!(result, expected);
    }
}
