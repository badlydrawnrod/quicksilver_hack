use crate::collision_lines::{CollisionLines, CollisionModel};
use crate::line_renderer::{LineRenderer, RenderModel};

use super::world_pos::WorldPos;

use quicksilver::geom::{Transform, Vector};

pub struct Shot {
    pos: Vector,
    angle: f32,
    velocity: Vector,
    render_model: RenderModel,
    collision_model: CollisionModel,
    collision_lines: CollisionLines,
    pub(crate) alive: bool, // TODO: make private
}

killable!(Shot);

impl WorldPos for Shot {
    fn world_pos(&self) -> Vector {
        self.pos
    }
    fn angle(&self) -> f32 {
        self.angle
    }
}

impl Shot {
    pub fn new(
        render_model: RenderModel,
        collision_model: CollisionModel,
        pos: Vector,
        forced_scroll: Vector,
        angle: f32,
    ) -> Self {
        Shot {
            pos,
            angle,
            velocity: Transform::translate(forced_scroll)
                * Transform::rotate(angle)
                * Vector::new(0.0, -8.0),
            render_model: render_model,
            collision_model: collision_model,
            collision_lines: CollisionLines::new(),
            alive: true,
        }
    }

    pub fn control(&mut self) {
        self.pos += self.velocity;
        let transform = Transform::translate(self.pos) * Transform::rotate(self.angle);
        self.collision_lines.clear();
        self.collision_lines
            .add_model(self.collision_model.clone(), transform);
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
