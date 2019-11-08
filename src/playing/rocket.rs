use crate::collision_lines::{CollisionLines, CollisionModel};
use crate::line_renderer::{LineRenderer, RenderModel};

use super::world_pos::WorldPos;

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
    alive: bool,
    rng: ThreadRng,
}

killable!(Rocket);

impl WorldPos for Rocket {
    fn world_pos(&self) -> Vector {
        self.pos
    }
    fn angle(&self) -> f32 {
        self.angle
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
            flight_velocity: Transform::rotate(angle) * Vector::new(0.0, -8.0),
            render_model: render_model,
            collision_model: collision_model,
            collision_lines: CollisionLines::new(),
            alive: true,
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
        self.alive = self.alive && playfield.contains(self.world_pos());
    }

    /// Draw the rocket to the given line renderer.
    pub(crate) fn draw(&self, line_renderer: &mut LineRenderer) {
        let transform = Transform::translate(self.pos) * Transform::rotate(self.angle);
        line_renderer.add_model(self.render_model.clone(), transform);
    }
}

impl AsRef<CollisionLines> for Rocket {
    fn as_ref(&self) -> &CollisionLines {
        &self.collision_lines
    }
}
