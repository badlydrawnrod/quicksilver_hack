use quicksilver::geom::{Shape, Transform, Vector};

use crate::collision_lines::{CollisionLines, CollisionModel};
use crate::line_renderer::{LineRenderer, RenderModel};

use super::world_pos::WorldPos;

pub struct Player {
    pos: Vector,
    pub(crate) angle: f32,
    render_model: RenderModel,
    collision_model: CollisionModel,
    pub(crate) collision_lines: CollisionLines,
    pub(crate) alive: bool,
}

killable!(Player);

impl WorldPos for Player {
    fn world_pos(&self) -> Vector {
        self.pos
    }
}

impl Player {
    pub fn new(
        render_model: RenderModel,
        collision_model: CollisionModel,
        pos: Vector,
        angle: f32,
    ) -> Self {
        Player {
            pos,
            angle,
            render_model: render_model,
            collision_model: collision_model,
            collision_lines: CollisionLines::new(),
            alive: true,
        }
    }

    pub(crate) fn control(&mut self, forced_scroll: Vector, dx: f32, dy: f32, rotate_by: f32) {
        if !self.alive {
            return;
        }

        // The player always moves forward at a steady rate.
        self.pos = self.pos.translate(forced_scroll);

        // Apply movement due to input.
        if dx != 0.0 || dy != 0.0 {
            let movement = Vector::new(dx * 2.0, dy * 2.0);
            self.pos += movement;
        }

        // Update the rotation.
        if rotate_by != 0.0 {
            self.angle += rotate_by * 4.0;
        }

        // Update the transformed model from the original model.
        let transform = Transform::translate(self.world_pos()) * Transform::rotate(self.angle);
        self.collision_lines.clear();
        self.collision_lines
            .add_model(self.collision_model.clone(), transform);
    }

    /// Draw the player's ship to the given line renderer.
    pub(crate) fn draw(&self, line_renderer: &mut LineRenderer) {
        if self.alive {
            let transform = Transform::translate(self.world_pos()) * Transform::rotate(self.angle);
            line_renderer.add_model(self.render_model.clone(), transform);
        }
    }
}
