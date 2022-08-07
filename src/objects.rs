use crate::matrix::{IDENTITY_MATRIX, Matrix};
use crate::objects::Object::Sphere;
use crate::tuple::{ORIGIN, Tuple};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Object {
    Sphere {
        position: Tuple,
        radius: f64,
        transformation: Matrix<4>,
    },
}

pub fn sphere() -> Object {
    Sphere {
        position: ORIGIN,
        radius: 1.0,
        transformation: IDENTITY_MATRIX,
    }
}

impl Object {
    pub fn set_transform(self, transform: Matrix<4>) -> Object {
        match self {
            Sphere { position, radius, transformation: _ } => {
                Sphere {
                    position,
                    radius,
                    transformation: transform,
                }
            }
        }
    }
}
