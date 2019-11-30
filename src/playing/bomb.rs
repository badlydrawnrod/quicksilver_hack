use crate::collision_lines::{CollisionLines, CollisionModel};
use crate::line_renderer::{LineRenderer, RenderModel};
use crate::playing::health::Health;

use super::world_pos::WorldPos;

use quicksilver::geom::{Rectangle, Shape, Transform, Vector};

pub struct Bomb {
    pos: Vector,
    angle: f32,
    velocity: Vector,
    render_model: RenderModel,
    collision_model: CollisionModel,
    collision_lines: CollisionLines,
    health: Health,
}

impl WorldPos for Bomb {
    fn world_pos(&self) -> Vector {
        self.pos
    }
    fn angle(&self) -> f32 {
        self.angle
    }
}

impl AsRef<Health> for Bomb {
    fn as_ref(&self) -> &Health {
        &self.health
    }
}

impl AsMut<Health> for Bomb {
    fn as_mut(&mut self) -> &mut Health {
        &mut self.health
    }
}

impl Bomb {
    pub fn new(
        render_model: RenderModel,
        collision_model: CollisionModel,
        pos: Vector,
        forward_velocity: Vector,
    ) -> Self {
        let velocity = Transform::translate(forward_velocity) * Vector::new(0.0, 4.0);
        Bomb {
            pos,
            angle: forward_velocity.angle() + 90.0,
            velocity: velocity,
            render_model: render_model,
            collision_model: collision_model,
            collision_lines: CollisionLines::new(),
            health: Health::new(),
        }
    }

    pub fn control(&mut self, playfield: &Rectangle) {
        self.pos += self.velocity;
        self.angle = 180.0f32.min(self.angle + 1.0);
        let transform = Transform::translate(self.pos) * Transform::rotate(self.angle);
        self.collision_lines.clear();
        self.collision_lines
            .add_model(self.collision_model.clone(), transform);

        // Check for the bomb going out of bounds.
        if self.health.is_alive() && !playfield.contains(self.world_pos()) {
            self.health.kill();
        }
    }

    /// Draw the bomb to the given line renderer.
    pub(crate) fn draw(&self, line_renderer: &mut LineRenderer, alpha: f64) {
        let pos = self.pos + self.velocity * alpha as f32;
        let transform = Transform::translate(pos) * Transform::rotate(self.angle);
        line_renderer.add_model(self.render_model.clone(), transform);
    }
}

impl AsRef<CollisionLines> for Bomb {
    fn as_ref(&self) -> &CollisionLines {
        &self.collision_lines
    }
}
