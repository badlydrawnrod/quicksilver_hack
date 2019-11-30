use quicksilver::geom::{Rectangle, Shape, Transform, Vector};

use crate::collision_lines::{CollisionLines, CollisionModel};
use crate::line_renderer::{LineRenderer, RenderModel};

use super::world_pos::WorldPos;
use crate::constants::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH};
use crate::playing::health::Health;

pub struct Player {
    pos: Vector,
    angle: f32,
    velocity: Vector,
    bounds: Rectangle,
    render_model: RenderModel,
    collision_model: CollisionModel,
    collision_lines: CollisionLines,
    health: Health,
}

impl WorldPos for Player {
    fn world_pos(&self) -> Vector {
        self.pos
    }
    fn angle(&self) -> f32 {
        self.angle
    }
}

impl AsRef<Health> for Player {
    fn as_ref(&self) -> &Health {
        &self.health
    }
}

impl AsMut<Health> for Player {
    fn as_mut(&mut self) -> &mut Health {
        &mut self.health
    }
}

impl Player {
    pub fn new(render_model: RenderModel, collision_model: CollisionModel, pos: Vector) -> Self {
        Player {
            pos,
            angle: 90.0,
            velocity: Vector::ZERO,
            bounds: Rectangle::new((0, 0), (2 * VIRTUAL_WIDTH / 3, VIRTUAL_HEIGHT)),
            render_model: render_model,
            collision_model: collision_model,
            collision_lines: CollisionLines::new(),
            health: Health::new(),
        }
    }

    pub(crate) fn advance(&mut self, forward_velocity: Vector) {
        self.pos = self.pos.translate(forward_velocity);
        self.bounds = self.bounds.translate(forward_velocity);
    }

    pub(crate) fn control(&mut self, dx: f32, dy: f32) {
        if self.health.is_dead() {
            return;
        }

        // Apply movement due to input.
        self.velocity = Vector::new(dx * 2.0, dy * 2.0);
        if dx != 0.0 || dy != 0.0 {
            self.pos += self.velocity;
        }
        // Clamp the player to the bounds. This isn't ideal, because the bounds are updated with
        // the camera when drawing, so there's some slight but discernible stutter if you know
        // where to look.
        self.pos = self
            .pos
            .clamp(self.bounds.pos, self.bounds.pos + self.bounds.size);

        // Update the transformed model from the original model.
        let transform = Transform::translate(self.world_pos()) * Transform::rotate(self.angle);
        self.collision_lines.clear();
        self.collision_lines
            .add_model(self.collision_model.clone(), transform);
    }

    /// Draw the player's ship to the given line renderer.
    pub(crate) fn draw(&self, line_renderer: &mut LineRenderer, alpha: f64) {
        if self.health.is_alive() {
            let pos = self.world_pos() + self.velocity * alpha as f32;
            let transform = Transform::translate(pos) * Transform::rotate(self.angle);
            line_renderer.add_model(self.render_model.clone(), transform);
        }
    }
}

impl AsRef<CollisionLines> for Player {
    fn as_ref(&self) -> &CollisionLines {
        &self.collision_lines
    }
}
