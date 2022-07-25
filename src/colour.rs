use std::ops::{Add, Mul, Sub};

use crate::util::a_equal_b;

pub const BLACK: Colour = Colour { red: 0.0, green: 0.0, blue: 0.0 };
pub const WHITE: Colour = Colour { red: 1.0, green: 1.0, blue: 1.0 };

#[derive(Debug, Copy, Clone)]
pub struct Colour {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Colour {
    pub fn to_string(&self) -> String {
        let mut string_builder = Self::normalize_value(self.red).to_string();
        string_builder = string_builder.add(" ");
        string_builder = string_builder.add(&Self::normalize_value(self.green).to_string());
        string_builder = string_builder.add(" ");
        string_builder = string_builder.add(&Self::normalize_value(self.blue).to_string());
        string_builder.add(" ")
    }

    fn normalize_value(value: f64) -> u32 {
        let value = value * 255.0;
        let mut value = value.round().trunc();
        if value < 0.0 {
            value = 0.0
        }
        if value > 255.0 {
            value = 255.0;
        }
        return value as u32;
    }
}

pub fn colour(red: f64, green: f64, blue: f64) -> Colour {
    Colour {
        red,
        green,
        blue,
    }
}

impl Add for Colour {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for Colour {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul<i32> for Colour {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        let rhs = rhs as f64;
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul<f64> for Colour {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul<Colour> for Colour {
    type Output = Self;

    fn mul(self, rhs: Colour) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl PartialEq<Colour> for Colour {
    fn eq(&self, other: &Colour) -> bool {
        a_equal_b(self.red, other.red) &&
            a_equal_b(self.blue, other.blue) &&
            a_equal_b(self.green, other.green)
    }

    fn ne(&self, other: &Colour) -> bool {
        !a_equal_b(self.red, other.red) ||
            !a_equal_b(self.blue, other.blue) ||
            !a_equal_b(self.green, other.green)
    }
}
