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

    pub fn normal_at(&self, point: Tuple) -> Tuple {
        match self {
            Sphere { position, radius: _, transformation } => {
                let object_point = transformation.invert().unwrap() * point;
                let object_normal = object_point - *position;
                let world_normal = transformation.invert().unwrap().transpose() * object_normal;
                let world_normal = Tuple {
                    x: world_normal.x,
                    y: world_normal.y,
                    z: world_normal.z,
                    w: 0.0,
                };
                world_normal.normalize()
            }
        }
    }
}
