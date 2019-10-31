use quicksilver::{
    geom::{Shape, Transform, Vector},
    graphics::Color,
};

use crate::collision_lines::CollisionLines;
use crate::line_renderer::{LineRenderer, TintedLine};
use crate::transformed::Transformable;

use super::world_pos::WorldPos;

pub struct Player {
    pos: Vector,
    pub(crate) angle: f32,
    model_lines: Vec<TintedLine>,
    render_lines: Vec<TintedLine>,
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
    pub fn new(pos: Vector, angle: f32) -> Self {
        let lines = vec![
            TintedLine::new((-16, 16), (16, 16), Color::GREEN),
            TintedLine::new((16, 16), (0, -16), Color::GREEN),
            TintedLine::new((0, -16), (-16, 16), Color::GREEN),
        ];
        let length = lines.len();
        let collision_lines = lines.iter().map(|line| line.line).collect::<Vec<_>>();
        Player {
            pos,
            angle,
            model_lines: lines,
            render_lines: Vec::with_capacity(length),
            collision_lines: CollisionLines::new(collision_lines),
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
        self.render_lines.clear();
        self.render_lines.extend(
            self.model_lines
                .iter()
                .map(|line| line.transformed(transform)),
        );

        self.collision_lines.update(transform);
    }

    /// Draw the player's ship to the given line renderer.
    pub(crate) fn draw(&self, line_renderer: &mut LineRenderer) {
        if self.alive {
            line_renderer.add_lines(self.render_lines.iter());
        }
    }
}
