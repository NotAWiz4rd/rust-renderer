use std::ops::{Add, Mul};
use std::ptr::eq;

use crate::tuple::Tuple;
use crate::util::a_equal_b;

pub const IDENTITY_MATRIX: Matrix<4> = matrix::<4>([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
]);

#[derive(Debug, Copy, Clone)]
pub struct Matrix<const SIZE: usize> {
    pub data: [[f64; SIZE]; SIZE],
}

pub fn matrix_empty<const SIZE: usize>() -> Matrix<SIZE> {
    Matrix {
        data: [[0.0; SIZE]; SIZE]
    }
}

pub const fn matrix<const SIZE: usize>(rows: [[f64; SIZE]; SIZE]) -> Matrix<SIZE> {
    Matrix {
        data: rows
    }
}

impl<const SIZE: usize> Matrix<SIZE> {
    pub fn transpose(self) -> Self {
        let mut rows = [[0.0; SIZE]; SIZE];

        for i in 0..SIZE {
            let mut row = [0.0; SIZE];
            for j in 0..SIZE {
                row[j] = self.data[j][i];
            }
            rows[i] = row;
        }
        matrix::<SIZE>(rows)
    }

    pub fn submatrix(&self, delete_row: usize, delete_column: usize) -> Matrix<{ SIZE - 1 }> {
        let mut submatrix = matrix_empty::<{ SIZE - 1 }>();
        let mut column_modifier = 0;
        let mut row_modifier = 0;
        for row in 0..SIZE {
            if delete_row != row {
                let mut new_row = [0.0; SIZE - 1];
                for column in 0..SIZE {
                    if delete_column != column {
                        new_row[column - column_modifier] = self.data[row][column]
                    } else {
                        column_modifier = 1;
                    }
                }
                column_modifier = 0;
                submatrix.data[row - row_modifier] = new_row;
            } else {
                row_modifier = 1;
            }
        }
        submatrix
    }
}

impl Matrix<3> {
    pub fn minor(&self, delete_row: usize, delete_column: usize) -> f64 {
        let submatrix = self.submatrix(delete_row, delete_column);
        submatrix.determinant()
    }
}

impl Matrix<2> {
    pub fn determinant(&self) -> f64 {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }
}

impl<const SIZE: usize> Add for Matrix<SIZE> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut m = matrix_empty::<SIZE>();
        for row in 0..SIZE {
            for column in 0..SIZE {
                m.data[row][column] = self.data[row][column] + rhs.data[row][column];
            }
        }
        return m;
    }
}

impl<const SIZE: usize> PartialEq for Matrix<SIZE> {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..SIZE {
            for column in 0..SIZE {
                if !a_equal_b(self.data[row][column], other.data[row][column]) {
                    return false;
                }
            }
        }
        true
    }

    fn ne(&self, other: &Self) -> bool {
        !eq(self, other)
    }
}

impl<const SIZE: usize> Mul for Matrix<SIZE> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = matrix_empty::<SIZE>();
        for row in 0..SIZE {
            for column in 0..SIZE {
                for i in 0..SIZE {
                    m.data[row][column] = m.data[row][column] + self.data[row][i] * rhs.data[i][column]
                }
            }
        }
        return m;
    }
}

impl Mul<Tuple> for Matrix<4> {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.data[0][0] * rhs.x + self.data[0][1] * rhs.y + self.data[0][2] * rhs.z + self.data[0][3] * rhs.x,
            y: self.data[1][0] * rhs.x + self.data[1][1] * rhs.y + self.data[1][2] * rhs.z + self.data[1][3] * rhs.x,
            z: self.data[2][0] * rhs.x + self.data[2][1] * rhs.y + self.data[2][2] * rhs.z + self.data[2][3] * rhs.x,
            w: self.data[3][0] * rhs.x + self.data[3][1] * rhs.y + self.data[3][2] * rhs.z + self.data[3][3] * rhs.x,
        }
    }
}
