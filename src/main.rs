#![feature(generic_const_exprs)]

use crate::experiments::clock::render_clock;
use crate::experiments::first_sphere_image::{render_basic_sphere_multithreaded, render_basic_sphere_singlethreaded};
use crate::experiments::projectile::run_projectile_simulation;

mod tuple;
mod colour;
mod experiments;
mod util;
mod canvas;
mod matrix;
mod ray;
mod objects;

fn main() {
    render_basic_sphere_singlethreaded();
    render_basic_sphere_multithreaded(8);
}

#[cfg(test)]
mod tests {
    use crate::tuple::{cross_product, dot_product, vector_i};

    mod rays {
        use crate::matrix::{IDENTITY_MATRIX, scaling_i, translation_i};
        use crate::objects::{Object, sphere};
        use crate::objects::Object::Sphere;
        use crate::ray::{intersection, Intersections, ray};
        use crate::tuple::{point, point_i, vector_i};

        #[test]
        fn intersecting_a_translated_sphere_with_a_ray() {
            let r = ray(point_i(0, 0, -5), vector_i(0, 0, 1));
            let s = sphere().set_transform(translation_i(5, 0, 0));
            let intersects = r.intersect(&s);

            assert_eq!(intersects, Intersections::None);
        }

        #[test]
        fn intersecting_a_scaled_sphere_with_a_ray() {
            let r = ray(point_i(0, 0, -5), vector_i(0, 0, 1));
            let s = sphere().set_transform(scaling_i(2, 2, 2));
            let intersects = r.intersect(&s);

            match intersects {
                Intersections::Some(intersections) => {
                    assert_eq!(intersections.len(), 2);
                    assert_eq!(intersections[0].time, 3.0);
                    assert_eq!(intersections[1].time, 7.0);
                }
                _ => assert!(false),
            }
        }

        #[test]
        fn changing_a_spheres_transformation() {
            let s = sphere();
            let t = translation_i(2, 3, 4);
            let s = s.set_transform(t);

            match s {
                Sphere { position: _, radius: _, transformation } => {
                    assert_eq!(transformation, t);
                }
            }
        }

        #[test]
        fn sphere_default_transformation() {
            let s = sphere();
            match s {
                Sphere { position: _, radius: _, transformation } => {
                    assert_eq!(transformation, IDENTITY_MATRIX);
                }
            }
        }

        #[test]
        fn scaling_a_ray() {
            let r = ray(point_i(1, 2, 3), vector_i(0, 1, 0));
            let translation = scaling_i(2, 3, 4);
            let r2 = r.transform(translation);

            assert_eq!(r2.origin, point_i(2, 6, 12));
            assert_eq!(r2.direction, vector_i(0, 3, 0));
        }

        #[test]
        fn translating_a_ray() {
            let r = ray(point_i(1, 2, 3), vector_i(0, 1, 0));
            let translation = translation_i(3, 4, 5);
            let r2 = r.transform(translation);

            assert_eq!(r2.origin, point_i(4, 6, 8));
            assert_eq!(r2.direction, vector_i(0, 1, 0));
        }

        #[test]
        fn hit_when_all_intersects_have_negative_t() {
            let s = sphere();
            let i1 = intersection(-2.0, s);
            let i2 = intersection(-1.0, s);
            let intersects = Intersections::Some([i2, i1]);

            let hit = intersects.hit();
            assert_eq!(hit, None);
        }

        #[test]
        fn hit_when_some_intersects_have_negative_t() {
            let s = sphere();
            let i1 = intersection(1.0, s);
            let i2 = intersection(-1.0, s);
            let intersects = Intersections::Some([i2, i1]);

            let hit = intersects.hit().unwrap();
            assert_eq!(hit, i1);
        }

        #[test]
        fn hit_when_all_intersects_have_positive_t() {
            let s = sphere();
            let i1 = intersection(1.0, s);
            let i2 = intersection(2.0, s);
            let intersects = Intersections::Some([i2, i1]);

            let hit = intersects.hit().unwrap();
            assert_eq!(hit, i1);
        }

        #[test]
        fn intersect_sets_object_on_intersections() {
            let r = ray(point_i(0, 0, -5), vector_i(0, 0, 1));
            let s = sphere();
            let intersects = r.intersect(&s);

            match intersects {
                Intersections::Some(intersections) => {
                    assert_eq!(intersections.len(), 2);
                    assert_eq!(intersections[0].object, s);
                    assert_eq!(intersections[1].object, s);
                }
                _ => assert!(false),
            }
        }

        #[test]
        fn aggregating_intersections() {
            let s = sphere();
            let i1 = intersection(1.0, s);
            let i2 = intersection(2.0, s);
            let intersections = Intersections::Some([i1, i2]);

            match intersections {
                Intersections::Some(intersects) => {
                    assert_eq!(intersects.len(), 2);
                    assert_eq!(intersects[0].time, 1.0);
                    assert_eq!(intersects[1].time, 2.0);
                }
                _ => assert!(false),
            }
        }

        #[test]
        fn intersection_encapsulates_t_and_object() {
            let s = sphere();
            let i = intersection(3.5, s);

            assert_eq!(i.time, 3.5);
            assert_eq!(i.object, s);
        }

        #[test]
        fn sphere_is_behind_ray() {
            let r = ray(point_i(0, 0, 5), vector_i(0, 0, 1));
            let s = sphere();
            let intersects = r.intersect(&s);

            match intersects {
                Intersections::Some(intersections) => {
                    assert_eq!(intersections[0].time, -6.0);
                    assert_eq!(intersections[1].time, -4.0);
                }
                _ => assert!(false),
            }
        }

        #[test]
        fn ray_originates_inside_sphere() {
            let r = ray(point_i(0, 0, 0), vector_i(0, 0, 1));
            let s = sphere();
            let intersects = r.intersect(&s);

            match intersects {
                Intersections::Some(intersections) => {
                    assert_eq!(intersections[0].time, -1.0);
                    assert_eq!(intersections[1].time, 1.0);
                }
                _ => assert!(false),
            }
        }

        #[test]
        fn ray_misses_sphere() {
            let r = ray(point_i(0, 2, -5), vector_i(0, 0, 1));
            let s = sphere();
            let intersects = r.intersect(&s);

            assert_eq!(intersects, Intersections::None);
        }

        #[test]
        fn ray_intersects_sphere_at_a_tangent() {
            let r = ray(point_i(0, 1, -5), vector_i(0, 0, 1));
            let s = sphere();
            let intersects = r.intersect(&s);

            match intersects {
                Intersections::Some(intersections) => {
                    assert_eq!(intersections[0].time, 5.0);
                    assert_eq!(intersections[1].time, 5.0);
                }
                _ => assert!(false),
            }
        }

        #[test]
        fn ray_intersects_sphere_at_two_points() {
            let r = ray(point_i(0, 0, -5), vector_i(0, 0, 1));
            let s = sphere();
            let intersects = r.intersect(&s);

            match intersects {
                Intersections::Some(intersections) => {
                    assert_eq!(intersections[0].time, 4.0);
                    assert_eq!(intersections[1].time, 6.0);
                }
                _ => assert!(false),
            }
        }

        #[test]
        fn computing_a_point_from_a_distance() {
            let r = ray(point_i(2, 3, 4), vector_i(1, 0, 0));

            assert_eq!(r.position(0.0), point_i(2, 3, 4));
            assert_eq!(r.position(1.0), point_i(3, 3, 4));
            assert_eq!(r.position(-1.0), point_i(1, 3, 4));
            assert_eq!(r.position(2.5), point(4.5, 3.0, 4.0));
        }

        #[test]
        fn creating_a_ray() {
            let origin = point_i(1, 2, 3);
            let direction = vector_i(4, 5, 6);
            let r = ray(origin, direction);

            assert_eq!(r.origin, origin);
            assert_eq!(r.direction, direction);
        }
    }

    mod transformations {
        use std::f64::consts::PI;

        use crate::matrix::{identity, rotation_x, scaling_i, translation_i};
        use crate::tuple::point_i;

        #[test]
        fn chained_transformations_must_be_applied_in_reverse_order() {
            let p = point_i(1, 0, 1);
            let rot = rotation_x(PI / 2.0);
            let scale = scaling_i(5, 5, 5);
            let translate = translation_i(10, 5, 7);
            let transform = identity().rotate_x(PI / 2.0).scale(5.0, 5.0, 5.0).translate(10.0, 5.0, 7.0);

            let chained_transformation = translate * scale * rot;
            assert_eq!(chained_transformation * p, point_i(15, 0, 7));
            assert_eq!(transform * p, point_i(15, 0, 7));
        }

        #[test]
        fn individual_transformations_are_applied_in_sequence() {
            let p = point_i(1, 0, 1);
            let rot = rotation_x(PI / 2.0);
            let scale = scaling_i(5, 5, 5);
            let translate = translation_i(10, 5, 7);

            // apply rotation first
            let p = rot * p;
            assert_eq!(p, point_i(1, -1, 0));

            // then the scaling
            let p = scale * p;
            assert_eq!(p, point_i(5, -5, 0));

            // then the translation
            let p = translate * p;
            assert_eq!(p, point_i(15, 0, 7));
        }

        mod shearing {
            use crate::matrix::shearing_i;
            use crate::tuple::point_i;

            #[test]
            fn shearing_transformation_moves_z_in_proportion_to_y() {
                let transform = shearing_i(0, 0, 0, 0, 0, 1);
                let p = point_i(2, 3, 4);

                assert_eq!(transform * p, point_i(2, 3, 7));
            }

            #[test]
            fn shearing_transformation_moves_z_in_proportion_to_x() {
                let transform = shearing_i(0, 0, 0, 0, 1, 0);
                let p = point_i(2, 3, 4);

                assert_eq!(transform * p, point_i(2, 3, 6));
            }

            #[test]
            fn shearing_transformation_moves_y_in_proportion_to_z() {
                let transform = shearing_i(0, 0, 0, 1, 0, 0);
                let p = point_i(2, 3, 4);

                assert_eq!(transform * p, point_i(2, 7, 4));
            }

            #[test]
            fn shearing_transformation_moves_y_in_proportion_to_x() {
                let transform = shearing_i(0, 0, 1, 0, 0, 0);
                let p = point_i(2, 3, 4);

                assert_eq!(transform * p, point_i(2, 5, 4));
            }

            #[test]
            fn shearing_transformation_moves_x_in_proportion_to_z() {
                let transform = shearing_i(0, 1, 0, 0, 0, 0);
                let p = point_i(2, 3, 4);

                assert_eq!(transform * p, point_i(6, 3, 4));
            }

            #[test]
            fn shearing_transformation_moves_x_in_proportion_to_y() {
                let transform = shearing_i(1, 0, 0, 0, 0, 0);
                let p = point_i(2, 3, 4);

                assert_eq!(transform * p, point_i(5, 3, 4));
            }
        }

        mod rotation {
            use std::f64::consts::PI;

            use crate::matrix::{rotation_x, rotation_y, rotation_z};
            use crate::tuple::{point, point_i};

            #[test]
            fn rotating_point_around_z_axis() {
                let p = point_i(0, 1, 0);
                let half_quarter = rotation_z(PI / 4.0);
                let full_quarter = rotation_z(PI / 2.0);

                assert_eq!(half_quarter * p, point(-f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0));
                assert_eq!(full_quarter * p, point_i(-1, 0, 0));
            }

            #[test]
            fn rotating_point_around_y_axis() {
                let p = point_i(0, 0, 1);
                let half_quarter = rotation_y(PI / 4.0);
                let full_quarter = rotation_y(PI / 2.0);

                assert_eq!(half_quarter * p, point(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0));
                assert_eq!(full_quarter * p, point_i(1, 0, 0));
            }

            #[test]
            fn rotating_point_around_x_axis() {
                let p = point_i(0, 1, 0);
                let half_quarter = rotation_x(PI / 4.0);
                let full_quarter = rotation_x(PI / 2.0);

                assert_eq!(half_quarter * p, point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0));
                assert_eq!(full_quarter * p, point_i(0, 0, 1));
            }

            #[test]
            fn inverse_of_x_rotation_rotates_in_opposite_direction() {
                let p = point_i(0, 1, 0);
                let half_quarter = rotation_x(PI / 4.0);
                let inverse = half_quarter.invert().unwrap();

                assert_eq!(inverse * p, point(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0));
            }
        }

        mod scaling {
            use crate::matrix::scaling_i;
            use crate::tuple::{point_i, vector_i};

            #[test]
            fn reflection_is_scaling_by_negative_value() {
                let transform = scaling_i(-1, 1, 1);
                let p = point_i(2, 3, 4);
                assert_eq!(transform * p, point_i(-2, 3, 4))
            }

            #[test]
            fn multiplying_by_inverse_of_scaling_matrix() {
                let transform = scaling_i(2, 3, 4);
                let inverted = transform.invert().unwrap();
                let v = vector_i(-4, 6, 8);
                assert_eq!(inverted * v, vector_i(-2, 2, 2))
            }

            #[test]
            fn scaling_matrix_applied_to_vector() {
                let transform = scaling_i(2, 3, 4);
                let v = vector_i(-4, 6, 8);
                assert_eq!(transform * v, vector_i(-8, 18, 32))
            }

            #[test]
            fn scaling_matrix_applied_to_point() {
                let transform = scaling_i(2, 3, 4);
                let p = point_i(-4, 6, 8);
                assert_eq!(transform * p, point_i(-8, 18, 32))
            }
        }

        mod translation {
            use crate::matrix::translation_i;
            use crate::tuple::{point_i, vector_i};

            #[test]
            fn translation_does_not_affect_vectors() {
                let transform = translation_i(5, -3, 2);
                let v = vector_i(-3, 4, 5);
                assert_eq!(transform * v, v);
            }

            #[test]
            fn multiplying_by_inverse_of_translation_matrix() {
                let transform = translation_i(5, -3, 2);
                let inverse = transform.invert().unwrap();
                let p = point_i(-3, 4, 5);
                assert_eq!(inverse * p, point_i(-8, 7, 3));
            }

            #[test]
            fn multiplying_by_a_translation_matrix() {
                let transform = translation_i(5, -3, 2);
                let p = point_i(-3, 4, 5);
                assert_eq!(transform * p, point_i(2, 1, 7));
            }
        }
    }

    mod matrix_tests {
        use crate::matrix::{IDENTITY_MATRIX, matrix};
        use crate::tuple::Tuple;

        #[test]
        fn multiplying_product_by_its_inverse() {
            let m1 = matrix::<4>([
                [3.0, -9.0, 7.0, 3.0],
                [3.0, -8.0, 2.0, -9.0],
                [-4.0, 4.0, 4.0, 1.0],
                [-6.0, 5.0, -1.0, 1.0],
            ]);
            let m2 = matrix::<4>([
                [8.0, 2.0, 2.0, 2.0],
                [3.0, -1.0, 7.0, 0.0],
                [7.0, 0.0, 5.0, 4.0, ],
                [6.0, -2.0, 0.0, 5.0],
            ]);

            let product = m1 * m2;
            assert_eq!(product * m2.invert().unwrap(), m1)
        }

        #[test]
        fn calculating_inverse_of_a_matrix() {
            let m = matrix::<4>([
                [-5.0, 2.0, 6.0, -8.0],
                [1.0, -5.0, 1.0, 8.0],
                [7.0, 7.0, -6.0, -7.0],
                [1.0, -3.0, 7.0, 4.0],
            ]);
            let m_inverted = m.invert().unwrap();

            assert_eq!(m.determinant(), 532.0);
            assert_eq!(m.cofactor(2, 3), -160.0);
            assert_eq!(m_inverted.data[3][2], (-160.0 / 532.0));
            assert_eq!(m.cofactor(3, 2), 105.0);
            assert_eq!(m_inverted.data[2][3], (105.0 / 532.0));
            assert_eq!(m_inverted, matrix::<4>([
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639],
            ]));

            let m2 = matrix::<4>([
                [8.0, -5.0, 9.0, 2.0],
                [7.0, 5.0, 6.0, 1.0],
                [-6.0, 0.0, 9.0, 6.0],
                [-3.0, 0.0, -9.0, -4.0],
            ]);
            let m2_inverted = m2.invert().unwrap();

            assert_eq!(m2_inverted, matrix::<4>([
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692, 0.12308, 0.02564, 0.03077],
                [0.35897, 0.35897, 0.43590, 0.92308],
                [-0.69231, -0.69231, -0.76923, -1.92308],
            ]));

            let m3 = matrix::<4>([
                [9.0, 3.0, 0.0, 9.0],
                [-5.0, -2.0, -6.0, -3.0],
                [-4.0, 9.0, 6.0, 4.0],
                [-7.0, 6.0, 6.0, 2.0],
            ]);
            let m3_inverted = m3.invert().unwrap();

            assert_eq!(m3_inverted, matrix::<4>([
                [-0.04074, -0.07778, 0.14444, -0.22222],
                [-0.07778, 0.03333, 0.36667, -0.33333],
                [-0.02901, -0.14630, -0.10926, 0.12963],
                [0.17778, 0.06667, -0.26667, 0.33333],
            ]));
        }

        #[test]
        fn invertibility_of_noninvertible_matrix() {
            let m = matrix::<4>([
                [-4.0, 2.0, -2.0, -3.0],
                [9.0, 6.0, 2.0, 6.0],
                [0.0, -5.0, 1.0, -5.0],
                [0.0, 0.0, 0.0, 0.0],
            ]);
            assert_eq!(m.determinant(), 0.0);
            assert_eq!(m.invert(), None)
        }

        #[test]
        fn invertibility_of_invertible_matrix() {
            let m = matrix::<4>([
                [6.0, 4.0, 4.0, 4.0],
                [5.0, 5.0, 7.0, 6.0],
                [4.0, -9.0, 3.0, -7.0],
                [9.0, 1.0, 7.0, -6.0],
            ]);

            assert_eq!(m.determinant(), -2120.0);
            assert_ne!(m.invert(), None)
        }

        #[test]
        fn calculating_determinant_of_4_by_4() {
            let m = matrix::<4>([
                [-2.0, -8.0, 3.0, 5.0],
                [-3.0, 1.0, 7.0, 3.0],
                [1.0, 2.0, -9.0, 6.0],
                [-6.0, 7.0, 7.0, -9.0],
            ]);
            assert_eq!(m.cofactor(0, 0), 690.0);
            assert_eq!(m.cofactor(0, 1), 447.0);
            assert_eq!(m.cofactor(0, 2), 210.0);
            assert_eq!(m.cofactor(0, 3), 51.0);
            assert_eq!(m.determinant(), -4071.0);
        }

        #[test]
        fn calculating_determinant_of_3_by_3() {
            let m = matrix::<3>([
                [1.0, 2.0, 6.0],
                [-5.0, 8.0, -4.0],
                [2.0, 6.0, 4.0],
            ]);
            assert_eq!(m.cofactor(0, 0), 56.0);
            assert_eq!(m.cofactor(0, 1), 12.0);
            assert_eq!(m.cofactor(0, 2), -46.0);
            assert_eq!(m.determinant(), -196.0);
        }

        #[test]
        fn calculating_cofactor() {
            let m = matrix::<3>([
                [3.0, 5.0, 0.0],
                [2.0, -1.0, -7.0],
                [6.0, -1.0, 5.0],
            ]);

            assert_eq!(m.minor(0, 0), -12.0);
            assert_eq!(m.cofactor(0, 0), -12.0);
            assert_eq!(m.minor(1, 0), 25.0);
            assert_eq!(m.cofactor(1, 0), -25.0);
        }

        #[test]
        fn calculating_minor() {
            let m1 = matrix::<3>([
                [3.0, 5.0, 0.0],
                [2.0, -1.0, -7.0],
                [6.0, -1.0, 5.0],
            ]);
            let m2 = m1.submatrix(1, 0);

            assert_eq!(m2.determinant(), 25.0);
            assert_eq!(m1.minor(1, 0), 25.0);
        }

        #[test]
        fn small_submatrix() {
            let m = matrix::<3>([
                [1.0, 5.0, 0.0],
                [-3.0, 2.0, 7.0],
                [0.0, 6.0, -3.0],
            ]);

            let expected_result = matrix::<2>([
                [-3.0, 2.0],
                [0.0, 6.0],
            ]);
            assert_eq!(m.submatrix(0, 2), expected_result);
        }

        #[test]
        fn larger_submatrix() {
            let m = matrix::<4>([
                [-6.0, 1.0, 1.0, 6.0],
                [-8.0, 5.0, 8.0, 6.0],
                [-1.0, 0.0, 8.0, 2.0],
                [-7.0, 1.0, -1.0, 1.0],
            ]);

            let expected_result = matrix::<3>([
                [-6.0, 1.0, 6.0],
                [-8.0, 8.0, 6.0],
                [-7.0, -1.0, 1.0],
            ]);
            assert_eq!(m.submatrix(2, 1), expected_result);
        }

        #[test]
        fn determinant_of_matrix() {
            let m = matrix::<2>([
                [1.0, 5.0],
                [-3.0, 2.0]
            ]);
            assert_eq!(m.determinant(), 17.0)
        }

        #[test]
        fn transposing_matrix() {
            let m = matrix([
                [0.0, 9.0, 3.0, 0.0],
                [9.0, 8.0, 0.0, 8.0],
                [1.0, 8.0, 5.0, 3.0],
                [0.0, 0.0, 5.0, 8.0]
            ]);

            let expected_result = matrix([
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0]
            ]);
            assert_eq!(m.transpose(), expected_result);
        }

        #[test]
        fn transposing_identity_matrix() {
            let transposed = IDENTITY_MATRIX.transpose();
            assert_eq!(transposed, IDENTITY_MATRIX);
        }

        #[test]
        fn matrix_multiply_by_identity_matrix() {
            let m = matrix::<4>(
                [[0.0, 1.0, 2.0, 3.0],
                    [1.0, 2.0, 4.0, 8.0],
                    [2.0, 4.0, 8.0, 16.0],
                    [4.0, 8.0, 16.0, 32.0]]
            );

            assert_eq!(m * IDENTITY_MATRIX, m);
        }

        #[test]
        fn matrix_multiply_with_tuple() {
            let m = matrix::<4>(
                [[1.0, 2.0, 3.0, 4.0],
                    [2.0, 4.0, 4.0, 2.0],
                    [8.0, 6.0, 4.0, 1.0],
                    [0.0, 0.0, 0.0, 1.0]]
            );
            let t = Tuple {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                w: 1.0,
            };

            let expected_result = Tuple {
                x: 18.0,
                y: 24.0,
                z: 33.0,
                w: 1.0,
            };
            assert_eq!(m * t, expected_result);
        }

        #[test]
        fn matrix_multiplication() {
            let m1 = matrix::<4>(
                [[1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 8.0, 7.0, 6.0],
                    [5.0, 4.0, 3.0, 2.0]]
            );
            let m2 = matrix::<4>(
                [[-2.0, 1.0, 2.0, 3.0],
                    [3.0, 2.0, 1.0, -1.0],
                    [4.0, 3.0, 6.0, 5.0],
                    [1.0, 2.0, 7.0, 8.0]]
            );

            let expected_result = matrix::<4>(
                [[20.0, 22.0, 50.0, 48.0],
                    [44.0, 54.0, 114.0, 108.0],
                    [40.0, 58.0, 110.0, 102.0],
                    [16.0, 26.0, 46.0, 42.0]]
            );

            assert!(m1 * m2 == expected_result);
        }

        #[test]
        fn matrix_inequality() {
            let m1 = matrix::<4>(
                [[1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 8.0, 7.0, 6.0],
                    [5.0, 4.0, 3.0, 2.0]]
            );
            let m2 = matrix::<4>(
                [[2.0, 3.0, 4.0, 5.0],
                    [6.0, 7.0, 8.0, 9.0],
                    [8.0, 7.0, 6.0, 5.0],
                    [4.0, 3.0, 2.0, 1.0]]
            );

            assert!(m1 != m2)
        }

        #[test]
        fn matrix_equality() {
            let m1 = matrix::<4>(
                [[1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 8.0, 7.0, 6.0],
                    [5.0, 4.0, 3.0, 2.0]]
            );
            let m2 = matrix::<4>(
                [[1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 8.0, 7.0, 6.0],
                    [5.0, 4.0, 3.0, 2.0]]
            );

            assert!(m1 == m2)
        }

        #[test]
        fn three_by_three_matrix_works() {
            let matrix = matrix::<3>(
                [[-3.0, 5.0, 0.0],
                    [1.0, -2.0, -7.0],
                    [1.0, -2.0, 1.0]]
            );
            assert_eq!(matrix.data[0][0], -3.0);
            assert_eq!(matrix.data[1][1], -2.0);
            assert_eq!(matrix.data[2][2], 1.0);
        }

        #[test]
        fn two_by_two_matrix_works() {
            let matrix = matrix::<2>(
                [[-3.0, 5.0],
                    [1.0, -2.0]]
            );

            assert_eq!(matrix.data[0][0], -3.0);
            assert_eq!(matrix.data[0][1], 5.0);
            assert_eq!(matrix.data[1][0], 1.0);
            assert_eq!(matrix.data[1][1], -2.0);
        }

        #[test]
        fn constructing_and_inspecting_a_matrix() {
            let matrix = matrix::<4>(
                [
                    [1.0, 2.0, 3.0, 4.0],
                    [5.5, 6.5, 7.5, 8.5],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.5, 14.5, 15.5, 16.5],
                ]
            );

            assert_eq!(matrix.data[0][0], 1.0);
            assert_eq!(matrix.data[0][3], 4.0);
            assert_eq!(matrix.data[1][0], 5.5);
            assert_eq!(matrix.data[1][2], 7.5);
            assert_eq!(matrix.data[2][2], 11.0);
            assert_eq!(matrix.data[3][0], 13.5);
            assert_eq!(matrix.data[3][2], 15.5);
        }
    }

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
