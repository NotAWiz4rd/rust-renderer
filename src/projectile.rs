use crate::tuple::{point_i, Tuple, vector, vector_i};

pub fn run_projectile_simulation() {
    let mut proj = Projectile { position: point_i(0, 2, 0), velocity: vector_i(1, 1, 0).normalize() };
    let environ = Environment { gravity: vector(0.0, -0.1, 0.0), wind: vector(-0.01, 0.0, 0.0) };

    let mut tick_counter = 0;
    while proj.position.y > 0.0 {
        tick_counter += 1;
        tick(&environ, &mut proj);
        println!("{:?}", proj)
    }
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
