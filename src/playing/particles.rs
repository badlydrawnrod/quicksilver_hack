use quicksilver::geom::{Rectangle, Transform, Vector};
use quicksilver::graphics::{Background::Blended, Color, Image};
use quicksilver::lifecycle::Window;
use rand::{prelude::*, Rng};

struct Particle {
    alive: f32,
    position: Vector,
    velocity: Vector,
}

impl Particle {
    fn new() -> Self {
        Particle {
            alive: 0.0,
            position: Vector::ZERO,
            velocity: Vector::ZERO,
        }
    }
}

pub struct Particles {
    particles: Vec<Particle>,
    image: Image,
    rng: ThreadRng,
}

impl Particles {
    pub fn new(n: usize, image: Image) -> Self {
        let mut particles = Vec::with_capacity(n);
        for _ in 0..n {
            particles.push(Particle::new());
        }
        Particles {
            particles,
            image,
            rng: rand::thread_rng(),
        }
    }

    pub fn draw(&mut self, window: &mut Window) {
        for particle in &mut self.particles {
            if particle.alive > 0.0 {
                let colour = Color::GREEN.with_alpha(particle.alive / 30.0); // TODO: no magic.
                particle.position += particle.velocity;
                window.draw(
                    &Rectangle::new(particle.position - Vector::new(8, 8), Vector::new(16, 16)),
                    Blended(&self.image, colour),
                );
                particle.alive -= 1.0;
            }
        }
    }

    pub fn add(&mut self, number: usize, position: Vector, angle: f32) {
        let mut number = number;
        while number > 0 {
            for particle in &mut self.particles {
                if particle.alive <= 0.0 {
                    let angle = angle + self.rng.gen_range(-30.0, 30.0);
                    let speed = 2.0 + self.rng.gen_range(0.0, 4.0);
                    particle.position = position;
                    particle.velocity = Transform::rotate(angle) * Vector::new(0, -speed);
                    particle.alive = 60.0; // TODO: no magic.
                    break;
                }
            }
            number -= 1;
        }
    }
}
