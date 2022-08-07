pub mod first_sphere_image {
    use std::f64::consts::PI;
    use std::sync::mpsc;
    use std::thread;
    use std::time::{Instant, SystemTime};

    use crate::canvas::{canvas, Canvas};
    use crate::colour::{GREEN, RED};
    use crate::matrix::identity;
    use crate::objects::{Object, sphere};
    use crate::ray::ray;
    use crate::tuple::{point, point_i, Tuple};
    use crate::util::write_file;

    pub fn render_basic_sphere_singlethreaded() {
        let start = Instant::now();
        let sphere = sphere().set_transform(identity().scale(1.0, 0.5, 1.0).rotate_z(PI / 4.0));
        let canvas_pixels = 1000;
        let mut canvas = canvas(canvas_pixels, canvas_pixels);
        let ray_origin = point_i(0, 0, -5);
        let wall_z = 10;
        let wall_size = 7.0;
        let pixel_size = wall_size / canvas_pixels as f64;
        let half = wall_size / 2.0;

        canvas = calculate_canvas(canvas_pixels, canvas_pixels, pixel_size, half, wall_z, ray_origin, sphere);

        let calculation_time = start.elapsed();
        let start2 = Instant::now();
        println!("Starting to write file.");
        write_file("basic-sphere-render-single.ppm", &canvas.to_ppm()).expect("Failure during file write.");
        println!("Total time: {:?}", start.elapsed());
        println!("Calculation time: {:?}", calculation_time);
        println!("Writing file time: {:?}", start2.elapsed());
    }

    pub fn render_basic_sphere_multithreaded(threads: u32) {
        let start = Instant::now();
        let sphere = sphere().set_transform(identity().scale(1.0, 0.5, 1.0).rotate_z(PI / 4.0));
        let canvas_pixels = 1000;
        let mut canvas = canvas(canvas_pixels, canvas_pixels);
        let ray_origin = point_i(0, 0, -5);
        let wall_z = 10;
        let wall_size = 7.0;
        let pixel_size = wall_size / canvas_pixels as f64;
        let half = wall_size / 2.0;

        let rows_per_thread = canvas_pixels / threads;
        let (tx, rx) = mpsc::channel();
        for i in 0..threads {
            let transmitter = tx.clone();
            thread::spawn(move || {
                let subcanvas = calculate_subcanvas(i, rows_per_thread, canvas_pixels, pixel_size, half, wall_z, ray_origin, sphere);
                transmitter.send(SubcanvasMessage {
                    subcanvas,
                    thread: i,
                }).unwrap();
            });
        }

        let mut threads_answered = 0;
        for received in rx {
            println!("Got result from thread {}", received.thread);
            for row in 0..received.subcanvas.height {
                for column in 0..received.subcanvas.width {
                    canvas.pixels[((rows_per_thread * received.thread) + row) as usize][column as usize] = received.subcanvas.pixels[row as usize][column as usize];
                }
            }
            threads_answered += 1;
            if threads_answered == threads {
                break;
            }
        }

        let calculation_time = start.elapsed();
        let start2 = Instant::now();
        println!("Starting to write file.");
        write_file("basic-sphere-render-multi.ppm", &canvas.to_ppm()).expect("Failure during file write.");
        println!("Total time: {:?}, using {} threads", start.elapsed(), threads);
        println!("Calculation time: {:?}", calculation_time);
        println!("Writing file time: {:?}", start2.elapsed());
    }

    struct SubcanvasMessage {
        subcanvas: Canvas,
        thread: u32,
    }

    fn calculate_canvas(rows: u32, columns: u32, pixel_size: f64, half: f64, wall_z: i32, ray_origin: Tuple, sphere: Object) -> Canvas {
        let mut canvas = canvas(columns, rows);
        for row in 0..rows {
            let world_y = half - pixel_size * row as f64;
            for column in 0..columns {
                let world_x = -half + pixel_size * column as f64;
                let wall_position_target = point(world_x, world_y, wall_z as f64);

                let ray = ray(ray_origin, (wall_position_target - ray_origin).normalize());
                let intersects = ray.intersect(&sphere);
                match intersects.hit() {
                    Some(..) => canvas.write_pixel(column, row, RED),
                    _ => ()
                }
            }
        }
        canvas
    }

    fn calculate_subcanvas(thread: u32, rows: u32, columns: u32, pixel_size: f64, half: f64, wall_z: i32, ray_origin: Tuple, sphere: Object) -> Canvas {
        let mut canvas = canvas(columns, rows);
        for row in (rows * thread)..(rows + (rows * thread)) {
            let world_y = half - pixel_size * row as f64;
            for column in 0..columns {
                let world_x = -half + pixel_size * column as f64;
                let wall_position_target = point(world_x, world_y, wall_z as f64);

                let ray = ray(ray_origin, (wall_position_target - ray_origin).normalize());
                let intersects = ray.intersect(&sphere);
                match intersects.hit() {
                    Some(..) => canvas.write_pixel(column, row - (rows * thread), GREEN),
                    _ => ()
                }
            }
        }
        canvas
    }
}

pub mod clock {
    use crate::canvas::canvas;
    use crate::colour::WHITE;
    use crate::matrix::identity;
    use crate::tuple::point_i;
    use crate::util::{normalize_coordinate, radians, write_file};

    pub fn render_clock() {
        let mut canvas = canvas(800, 800);
        let p = point_i(0, 1, 0);
        for time in 0..12 {
            let transform = identity().rotate_z(radians(30.0 * time as f64));
            let transformed_point = transform * p;
            normalize_coordinate(transformed_point.x, canvas.width);
            let x = normalize_coordinate(transformed_point.x, canvas.width);
            let y = normalize_coordinate(transformed_point.y, canvas.height);
            canvas.write_pixel(x, y, WHITE);
        }
        write_file("clock-face.ppm", &canvas.to_ppm()).expect("Failure during file write.")
    }
}

pub mod projectile {
    use crate::canvas::canvas;
    use crate::colour::colour;
    use crate::tuple::{point_i, Tuple, vector};
    use crate::util::write_file;

    pub fn run_projectile_simulation() {
        let mut proj = Projectile { position: point_i(0, 2, 0), velocity: vector(1.0, 1.8, 0.0).normalize() * 11.25 };
        let environ = Environment { gravity: vector(0.0, -0.1, 0.0), wind: vector(-0.01, 0.0, 0.0) };
        let mut canvas = canvas(900, 550);

        let mut tick_counter = 0;
        while proj.position.y > 0.0 {
            tick_counter += 1;
            tick(&environ, &mut proj);
            let x = proj.position.x.round() as u32;
            let y = 550 - proj.position.y.round() as u32;
            if x < 900 && y < 550 {
                canvas.write_pixel(proj.position.x.round() as u32, 550 - proj.position.y.round() as u32, colour(1.0, 0.0, 0.0));
            }
            println!("{:?}", proj)
        }
        write_file("projectile-simulation.ppm", &canvas.to_ppm()).expect("Failure during file write.");
        println!("Projectile flew for {} ticks.", tick_counter)
    }

    #[derive(Debug)]
    struct Projectile {
        // point
        position: Tuple,
        // vector
        velocity: Tuple,
    }

    struct Environment {
        // vector
        gravity: Tuple,
        // vector
        wind: Tuple,
    }

    fn tick(environment: &Environment, projectile: &mut Projectile) {
        let position = projectile.position + projectile.velocity;
        let velocity = projectile.velocity + environment.gravity + environment.wind;
        projectile.position = position;
        projectile.velocity = velocity;
    }
}

