use quicksilver::geom::{Rectangle, Transform, Vector};
use quicksilver::graphics::{Background::Blended, Color, Image};
use quicksilver::lifecycle::Window;
use rand::{prelude::*, Rng};

const PARTICLE_SIZE: f32 = 16.0;
const PARTICLE_LIFE: f32 = 60.0;
const HALF_LIFE: f32 = PARTICLE_LIFE / 2.0;
const MIN_SPEED: f32 = 2.0;
const MAX_SPEED: f32 = 6.0;

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
    next_particle: usize,
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
            next_particle: 0,
        }
    }

    pub fn draw(&mut self, window: &mut Window) {
        for particle in &mut self.particles {
            if particle.alive > 0.0 {
                let colour = Color::GREEN.with_alpha(particle.alive / HALF_LIFE);
                particle.position += particle.velocity;
                window.draw(
                    &Rectangle::new(
                        particle.position - Vector::new(PARTICLE_SIZE / 2.0, PARTICLE_SIZE / 2.0),
                        Vector::new(PARTICLE_SIZE, PARTICLE_SIZE),
                    ),
                    Blended(&self.image, colour),
                );
                particle.alive -= 1.0;
            }
        }
    }

    pub fn add(&mut self, number: usize, position: Vector, angle: f32, spread: f32) {
        for _ in 0..number {
            let mut particle = &mut self.particles[self.next_particle];
            let angle = angle + self.rng.gen_range(-spread, spread);
            let speed = self.rng.gen_range(MIN_SPEED, MAX_SPEED);
            particle.position = position;
            particle.velocity = Transform::rotate(angle) * Vector::new(0, -speed);
            particle.alive = PARTICLE_LIFE;
            self.next_particle = (self.next_particle + 1) % self.particles.len();
        }
    }
}
