use crate::collision_lines::{CollisionLines, CollisionModel};
use crate::line_renderer::{LineRenderer, RenderModel};

use super::world_pos::WorldPos;

use crate::playing::health::Health;
use quicksilver::geom::{Rectangle, Shape, Transform, Vector};
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Rocket {
    pos: Vector,
    angle: f32,
    velocity: Vector,
    flight_velocity: Vector,
    render_model: RenderModel,
    collision_model: CollisionModel,
    collision_lines: CollisionLines,
    health: Health,
    rng: ThreadRng,
}

impl WorldPos for Rocket {
    fn world_pos(&self) -> Vector {
        self.pos
    }
    fn angle(&self) -> f32 {
        self.angle
    }
}

impl AsRef<Health> for Rocket {
    fn as_ref(&self) -> &Health {
        &self.health
    }
}
impl AsMut<Health> for Rocket {
    fn as_mut(&mut self) -> &mut Health {
        &mut self.health
    }
}

impl Rocket {
    pub fn new(
        render_model: RenderModel,
        collision_model: CollisionModel,
        pos: Vector,
        angle: f32,
    ) -> Self {
        Rocket {
            pos,
            angle,
            velocity: Vector::ZERO,
            flight_velocity: Transform::rotate(angle) * Vector::new(0.0, -4.0),
            render_model: render_model,
            collision_model: collision_model,
            collision_lines: CollisionLines::new(),
            health: Health::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn control(&mut self, playfield: &Rectangle) {
        if self.velocity == Vector::ZERO && self.rng.gen_range(0, 1000) < 5 {
            self.velocity = self.flight_velocity;
        }

        self.pos += self.velocity;
        let transform = Transform::translate(self.pos) * Transform::rotate(self.angle);
        self.collision_lines.clear();
        self.collision_lines
            .add_model(self.collision_model.clone(), transform);

        // Check for the rocket going out of bounds.
        if self.health.is_alive() && !playfield.contains(self.world_pos()) {
            self.health.kill();
        }
    }

    /// Draw the rocket to the given line renderer.
    pub(crate) fn draw(&self, line_renderer: &mut LineRenderer, alpha: f64) {
        let pos = self.pos + self.velocity * alpha as f32;
        let transform = Transform::translate(pos) * Transform::rotate(self.angle);
        line_renderer.add_model(self.render_model.clone(), transform);
    }
}

impl AsRef<CollisionLines> for Rocket {
    fn as_ref(&self) -> &CollisionLines {
        &self.collision_lines
    }
}
