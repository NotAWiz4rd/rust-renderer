use crate::projectile::run_projectile_simulation;

mod tuple;
mod colour;
mod projectile;
mod util;
mod canvas;

fn main() {
    run_projectile_simulation();
}

#[cfg(test)]
mod tests {
    use crate::tuple::{cross_product, dot_product, vector_i};

    mod ppm_tests {
        use crate::canvas::canvas;
        use crate::colour::colour;

        #[test]
        fn ppm_is_terminated_by_newline() {
            let c = canvas(5, 3);
            let ppm = c.to_ppm();
            assert!(ppm.ends_with("\n"))
        }

        #[test]
        fn splitting_long_lines_in_ppm() {
            let mut c = canvas(10, 2);
            for y in 0..2 {
                for x in 0..10 {
                    c.write_pixel(x, y, colour(1.0, 0.8, 0.6));
                }
            }
            let ppm = c.to_ppm();

            let mut line_iterator = ppm.lines();
            let fourth_line = line_iterator.nth(3).unwrap();
            let fifth_line = line_iterator.nth(0).unwrap();
            let sixth_line = line_iterator.nth(0).unwrap();
            let seventh_line = line_iterator.nth(0).unwrap();

            assert_eq!(fourth_line, "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
            assert_eq!(fifth_line, "153 255 204 153 255 204 153 255 204 153 255 204 153");
            assert_eq!(sixth_line, "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
            assert_eq!(seventh_line, "153 255 204 153 255 204 153 255 204 153 255 204 153");
        }

        #[test]
        fn constructing_ppm_pixel_data() {
            let mut c = canvas(5, 3);
            let colour1 = colour(1.5, 0.0, 0.0);
            let colour2 = colour(0.0, 0.5, 0.0);
            let colour3 = colour(-0.5, 0.0, 1.0);
            c.write_pixel(0, 0, colour1);
            c.write_pixel(2, 1, colour2);
            c.write_pixel(4, 2, colour3);

            let ppm = c.to_ppm();
            let mut line_iterator = ppm.lines();
            let fourth_line = line_iterator.nth(3).unwrap();
            let fifth_line = line_iterator.nth(0).unwrap();
            let sixth_line = line_iterator.nth(0).unwrap();

            assert_eq!(fourth_line, "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
            assert_eq!(fifth_line, "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
            assert_eq!(sixth_line, "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
        }

        #[test]
        fn constructing_ppm_header() {
            let c = canvas(5, 3);
            let ppm = c.to_ppm();

            let mut line_iterator = ppm.lines();
            let first_line = line_iterator.nth(0).unwrap();
            let second_line = line_iterator.nth(0).unwrap();
            let third_line = line_iterator.nth(0).unwrap();
            assert_eq!(first_line, "P3");
            assert_eq!(second_line, "5 3");
            assert_eq!(third_line, "255");
        }
    }

    mod canvas_tests {
        use crate::canvas::canvas;
        use crate::colour::{BLACK, colour};

        #[test]
        fn creating_a_canvas() {
            let c = canvas(10, 20);
            assert_eq!(c.width, 10);
            assert_eq!(c.height, 20);

            for i in 0..10 {
                for j in 0..10 {
                    assert_eq!(c.pixels[i][j], BLACK);
                }
            }
        }

        #[test]
        fn writing_pixels_into_canvas() {
            let mut c = canvas(10, 20);
            let red = colour(1.0, 0.0, 0.0);
            c.write_pixel(2, 3, red);
            assert_eq!(c.pixels[2][3], red)
        }
    }

    mod colour_tests {
        use crate::colour::colour;

        #[test]
        fn creating_a_colour() {
            let c = colour(-0.5, 0.4, 1.7);
            assert_eq!(c.red, -0.5);
            assert_eq!(c.green, 0.4);
            assert_eq!(c.blue, 1.7);
        }

        #[test]
        fn adding_colours() {
            let c1 = colour(0.9, 0.6, 0.75);
            let c2 = colour(0.7, 0.1, 0.25);
            assert_eq!(c1 + c2, colour(1.6, 0.7, 1.0));
        }

        #[test]
        fn subtracting_colours() {
            let c1 = colour(0.9, 0.6, 0.75);
            let c2 = colour(0.7, 0.1, 0.25);
            assert_eq!(c1 - c2, colour(0.2, 0.5, 0.5));
        }

        #[test]
        fn multiplying_colours() {
            let c1 = colour(1.0, 0.2, 0.4);
            let c2 = colour(0.9, 1.0, 0.1);
            assert_eq!(c1 * c2, colour(0.9, 0.2, 0.04));
        }
    }

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
            let v1 = vector_i(5, 6, 7);
            assert_eq!(p1 - v1, point_i(-2, -4, -6));
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
