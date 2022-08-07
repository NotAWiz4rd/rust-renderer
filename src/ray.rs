use crate::matrix::Matrix;
use crate::objects::Object;
use crate::tuple::{dot_product, ORIGIN, Tuple};

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn transform(&self, translation: Matrix<4>) -> Ray {
        let origin = translation * self.origin;
        let direction = translation * self.direction;
        ray(origin, direction)
    }
}

pub fn ray(origin: Tuple, direction: Tuple) -> Ray {
    Ray {
        origin,
        direction,
    }
}

impl Ray {
    pub fn position(&self, time: f64) -> Tuple {
        self.origin + (self.direction * time)
    }

    pub fn intersect(&self, object: &Object) -> Intersections {
        let ray = match object {
            Object::Sphere { transformation, .. } => self.transform(transformation.invert().unwrap()),
        };

        let object_position = match object {
            Object::Sphere { position, .. } => *position,
        };

        let sphere_to_ray = ray.origin - object_position;
        let a = dot_product(ray.direction, ray.direction);
        let b = 2.0 * dot_product(ray.direction, sphere_to_ray);
        let c = dot_product(sphere_to_ray, sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Intersections::None;
        }
        let intersect1 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
        let intersect2 = (-b + f64::sqrt(discriminant)) / (2.0 * a);
        Intersections::Some([intersection(intersect1, *object), intersection(intersect2, *object)])
    }
}

#[derive(Debug, PartialEq)]
pub enum Intersections {
    None,
    Some([Intersection; 2]),
}

impl Intersections {
    pub fn hit(&self) -> Option<Intersection> {
        match self {
            Intersections::Some(intersects) => {
                let mut smallest_i = usize::MAX;
                let mut smallest_time = f64::MAX;
                for i in 0..intersects.len() {
                    if intersects[i].time > 0.0 && (intersects[i].time < smallest_time) {
                        smallest_time = intersects[i].time;
                        smallest_i = i;
                    }
                }
                if smallest_i > 2 {
                    None
                } else {
                    Some(intersects[smallest_i])
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection {
    pub time: f64,
    pub object: Object,
}

pub fn intersection(t: f64, object: Object) -> Intersection {
    Intersection {
        time: t,
        object,
    }
}
