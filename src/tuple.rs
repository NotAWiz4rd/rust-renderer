use std::ops::{Add, Div, Mul, Neg, Sub};

pub const ZERO_VECTOR: Tuple = Tuple { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

pub fn point_i(x: i32, y: i32, z: i32) -> Tuple {
    point(x as f64, y as f64, z as f64)
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

pub fn vector_i(x: i32, y: i32, z: i32) -> Tuple {
    vector(x as f64, y as f64, z as f64)
}

pub fn dot_product(v1: Tuple, v2: Tuple) -> f64 {
    (v1.x * v2.x) +
        (v1.y * v2.y) +
        (v1.z * v2.z) +
        (v1.w * v2.w)
}

pub fn cross_product(v1: Tuple, v2: Tuple) -> Tuple {
    vector(v1.y * v2.z - v1.z * v2.y,
           v1.z * v2.x - v1.x * v2.z,
           v1.x * v2.y - v1.y * v2.x)
}

impl Tuple {
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2))
    }

    pub fn normalize(&self) -> Self {
        Self {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude(),
            w: self.w / self.magnitude(),
        }
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<i32> for Tuple {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        let rhs = rhs as f64;
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl Div<i32> for Tuple {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        let rhs = rhs as f64;
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}
