use crate::canvas::canvas;
use crate::colour::colour;
use crate::tuple::{point_i, Tuple, vector, vector_i};
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
    write_file("projectile-simulation.ppm", &canvas.to_ppm()).expect("TODO: panic message");
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
