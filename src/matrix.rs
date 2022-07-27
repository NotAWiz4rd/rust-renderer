use std::ops::{Add, Mul};
use std::ptr::eq;

use crate::util::a_equal_b;

pub struct Matrix<const SIZE: usize> {
    pub data: [[f64; SIZE]; SIZE],
}

pub fn matrix_empty<const SIZE: usize>() -> Matrix<SIZE> {
    Matrix {
        data: [[0.0; SIZE]; SIZE]
    }
}

pub fn matrix<const SIZE: usize>(rows: [[f64; SIZE]; SIZE]) -> Matrix<SIZE> {
    Matrix {
        data: rows
    }
}

impl<const SIZE: usize> Matrix<SIZE> {}

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
                todo!()
            }
        }
        return m;
    }
}
