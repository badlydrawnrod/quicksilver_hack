use crate::collision_lines::{CollisionLines, CollisionModel};
use crate::line_renderer::{LineRenderer, RenderModel};
use crate::playing::health::Health;

use super::world_pos::WorldPos;

use quicksilver::geom::{Rectangle, Shape, Transform, Vector};

pub struct Shot {
    pos: Vector,
    angle: f32,
    velocity: Vector,
    render_model: RenderModel,
    collision_model: CollisionModel,
    collision_lines: CollisionLines,
    health: Health,
}

impl WorldPos for Shot {
    fn world_pos(&self) -> Vector {
        self.pos
    }
    fn angle(&self) -> f32 {
        self.angle
    }
}

impl AsRef<Health> for Shot {
    fn as_ref(&self) -> &Health {
        &self.health
    }
}

impl AsMut<Health> for Shot {
    fn as_mut(&mut self) -> &mut Health {
        &mut self.health
    }
}

impl Shot {
    pub fn new(
        render_model: RenderModel,
        collision_model: CollisionModel,
        pos: Vector,
        forward_velocity: Vector,
        angle: f32,
    ) -> Self {
        Shot {
            pos,
            angle,
            velocity: Transform::translate(forward_velocity)
                * Transform::rotate(angle)
                * Vector::new(0.0, -8.0),
            render_model: render_model,
            collision_model: collision_model,
            collision_lines: CollisionLines::new(),
            health: Health::new(),
        }
    }

    pub fn control(&mut self, playfield: &Rectangle) {
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

    /// Draw the shot to the given line renderer.
    pub(crate) fn draw(&self, line_renderer: &mut LineRenderer) {
        let transform = Transform::translate(self.pos) * Transform::rotate(self.angle);
        line_renderer.add_model(self.render_model.clone(), transform);
    }
}

impl AsRef<CollisionLines> for Shot {
    fn as_ref(&self) -> &CollisionLines {
        &self.collision_lines
    }
}
