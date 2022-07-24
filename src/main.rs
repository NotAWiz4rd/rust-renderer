mod tuple;

fn main() {
    println!("Hello, world!");
}

fn a_larger_b(a: f64, b: f64) -> bool {
    a.abs() - b.abs() < 1e-10
}

#[cfg(test)]
mod tests {
    use crate::tuple::{cross_product, dot_product, vector_i};

    #[test]
    fn cross_product_test() {
        let v1 = vector_i(1, 2, 3);
        let v2 = vector_i(2, 3, 4);
        assert_eq!(cross_product(v1, v2), vector_i(-1, 2, -1));
        assert_eq!(cross_product(v2, v1), vector_i(1, -2, 1));
    }

    #[test]
    fn dot_product_test() {
        let v1 = vector_i(1, 2, 3);
        let v2 = vector_i(2, 3, 4);
        assert_eq!(dot_product(v1, v2), 20.0);
    }

    mod vector_normalizing {
        use crate::tuple::{vector, vector_i};

        #[test]
        fn normalizing_vector1() {
            let v = vector_i(4, 0, 0);
            assert_eq!(v.normalize(), vector_i(1, 0, 0));
        }

        #[test]
        fn normalizing_vector2() {
            let v = vector_i(1, 2, 3);
            assert_eq!(v.normalize(), vector(1.0 / 14.0_f64.sqrt(), 2.0 / 14.0_f64.sqrt(), 3.0 / 14.0_f64.sqrt()));
        }

        #[test]
        fn magnitude_of_normalized_vector() {
            let v = vector_i(1, 2, 3);
            let v = v.normalize();
            assert_eq!(v.magnitude(), 1.0);
        }
    }

    mod vector_magnitudes {
        use crate::tuple::vector_i;

        #[test]
        fn magnitude_of_vector1() {
            let v = vector_i(1, 0, 0);
            assert_eq!(v.magnitude(), 1.0);
        }

        #[test]
        fn magnitude_of_vector2() {
            let v = vector_i(0, 1, 0);
            assert_eq!(v.magnitude(), 1.0);
        }

        #[test]
        fn magnitude_of_vector3() {
            let v = vector_i(0, 0, 1);
            assert_eq!(v.magnitude(), 1.0);
        }

        #[test]
        fn magnitude_of_vector4() {
            let v = vector_i(1, 2, 3);
            assert_eq!(v.magnitude(), 14.0_f64.sqrt());
        }

        #[test]
        fn magnitude_of_vector5() {
            let v = vector_i(-1, -2, -3);
            assert_eq!(v.magnitude(), 14.0_f64.sqrt());
        }
    }


    mod operations {
        use crate::tuple::{point_i, Tuple, vector_i, ZERO_VECTOR};

        #[test]
        fn dividing_tuple_by_scalar() {
            let t = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
            assert_eq!(t / 2, Tuple { x: 0.5, y: -1.0, z: 1.5, w: -2.0 });
        }

        #[test]
        fn multiplying_tuple_with_fraction() {
            let t = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
            assert_eq!(t * 0.5, Tuple { x: 0.5, y: -1.0, z: 1.5, w: -2.0 });
        }

        #[test]
        fn multiplying_tuple_with_scalar() {
            let t = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
            assert_eq!(t * 3.5, Tuple { x: 3.5, y: -7.0, z: 10.5, w: -14.0 });
        }

        #[test]
        fn multiplying_tuple_with_scalar_int() {
            let t = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
            assert_eq!(t * 3, Tuple { x: 3.0, y: -6.0, z: 9.0, w: -12.0 });
        }

        #[test]
        fn negating_a_tuple() {
            let t = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
            assert_eq!(-t, Tuple { x: -1.0, y: 2.0, z: -3.0, w: 4.0 });
        }

        #[test]
        fn subtracting_a_vector_from_zero_vector() {
            let v = vector_i(1, -2, 3);
            assert_eq!(ZERO_VECTOR - v, vector_i(-1, 2, -3));
        }

        #[test]
        fn subtracting_a_vector_from_a_point() {
            let p1 = point_i(3, 2, 1);
            let p2 = vector_i(5, 6, 7);
            assert_eq!(p1 - p2, point_i(-2, -4, -6));
        }

        #[test]
        fn subtracting_two_points() {
            let p1 = point_i(3, 2, 1);
            let p2 = point_i(5, 6, 7);
            assert_eq!(p1 - p2, vector_i(-2, -4, -6));
        }

        #[test]
        fn subtracting_two_vectors() {
            let p1 = vector_i(3, 2, 1);
            let p2 = vector_i(5, 6, 7);
            assert_eq!(p1 - p2, vector_i(-2, -4, -6));
        }

        #[test]
        fn adding_two_tuples() {
            let t1 = point_i(3, -2, 5);
            let t2 = vector_i(-2, 3, 1);
            assert_eq!(t1 + t2, Tuple { x: 1.0, y: 1.0, z: 6.0, w: 1.0 });
        }
    }

    mod creations {
        use crate::tuple::{point, Tuple, vector};

        #[test]
        fn point_creates_tuple_with_w_equals_1() {
            let tuple = point(4 as f64, -4 as f64, 3 as f64);
            assert_eq!(tuple.x, 4 as f64);
            assert_eq!(tuple.y, -4 as f64);
            assert_eq!(tuple.z, 3 as f64);
            assert_eq!(tuple.w, 1.0);
        }

        #[test]
        fn vector_creates_tuple_with_w_equals_0() {
            let tuple = vector(4 as f64, -4 as f64, 3 as f64);
            assert_eq!(tuple.x, 4 as f64);
            assert_eq!(tuple.y, -4 as f64);
            assert_eq!(tuple.z, 3 as f64);
            assert_eq!(tuple.w, 0.0);
        }

        #[test]
        fn tuple_with_w_is_zero_is_vector() {
            let tuple = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 0.0 };
            assert_eq!(tuple.x, 4.3);
            assert_eq!(tuple.y, -4.2);
            assert_eq!(tuple.z, 3.1);
            assert_eq!(tuple.w, 0.0);
            assert!(!tuple.is_point());
            assert!(tuple.is_vector());
        }

        #[test]
        fn tuple_with_w_is_one_is_point() {
            let tuple = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1.0 };
            assert_eq!(tuple.x, 4.3);
            assert_eq!(tuple.y, -4.2);
            assert_eq!(tuple.z, 3.1);
            assert_eq!(tuple.w, 1.0);
            assert!(tuple.is_point());
            assert!(!tuple.is_vector());
        }
    }
}
