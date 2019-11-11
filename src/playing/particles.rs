use quicksilver::geom::{Rectangle, Vector};
use quicksilver::graphics::{Background::Blended, Color, Image};
use quicksilver::lifecycle::Window;

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
}

impl Particles {
    pub fn new(n: usize, image: Image) -> Self {
        let mut particles = Vec::with_capacity(n);
        for _ in 0..n {
            particles.push(Particle::new());
        }
        Particles { particles, image }
    }

    pub fn draw(&mut self, window: &mut Window) {
        for particle in &mut self.particles {
            if particle.alive > 0.0 {
                particle.position += particle.velocity;
                window.draw(
                    &Rectangle::new(particle.position - Vector::new(4, 4), Vector::new(8, 8)),
                    Blended(&self.image, Color::RED),
                );
                particle.alive -= 1.0;
            }
        }
    }

    pub fn add(&mut self, position: Vector) {
        for particle in &mut self.particles {
            if particle.alive <= 0.0 {
                particle.position = position;
                particle.alive = 120.0;
                break;
            }
        }
    }
}
