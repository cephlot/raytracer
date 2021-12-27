use raylib::{math::fundamentals::Tuple, Environment, Projectile};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize(),
    };
    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };
    let mut i = 0;

    while p.position.y > 0.0 {
        p = e.tick(p);
        i += 1;
        println!("{}: {:?}", i, p.position);
    }

    Ok(())
}
