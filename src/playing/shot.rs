use crate::collision_lines::CollisionLines;
use crate::line_renderer::{LineRenderer, TintedLine};
use crate::transformed::Transformable;

use super::world_pos::WorldPos;

use quicksilver::{
    geom::{Transform, Vector},
    graphics::Color,
};

pub struct Shot {
    pos: Vector,
    angle: f32,
    velocity: Vector,
    model_lines: Vec<TintedLine>,
    render_lines: Vec<TintedLine>,
    pub(crate) collision_lines: CollisionLines,
    pub(crate) alive: bool,
}

killable!(Shot);

impl WorldPos for Shot {
    fn world_pos(&self) -> Vector {
        self.pos
    }
}

impl Shot {
    pub fn new(pos: Vector, forced_scroll: Vector, angle: f32) -> Self {
        let lines = vec![
            TintedLine::new((-4, 4), (4, 4), Color::GREEN),
            TintedLine::new((4, 4), (0, -4), Color::GREEN),
            TintedLine::new((0, -4), (-4, 4), Color::GREEN),
            TintedLine::new((0, 0), (0, -8), Color::GREEN),
        ];
        let collision_lines = lines.iter().map(|line| line.line).collect::<Vec<_>>();
        let length = lines.len();
        Shot {
            pos,
            angle,
            velocity: Transform::translate(forced_scroll)
                * Transform::rotate(angle)
                * Vector::new(0.0, -8.0),
            model_lines: lines,
            render_lines: Vec::with_capacity(length),
            collision_lines: CollisionLines::new(collision_lines),
            alive: true,
        }
    }

    pub fn control(&mut self) {
        self.pos += self.velocity;

        let transform = Transform::translate(self.pos) * Transform::rotate(self.angle);
        self.render_lines.clear();
        self.render_lines.extend(
            self.model_lines
                .iter()
                .map(|line| line.transformed(transform)),
        );

        self.collision_lines.update(transform);
    }

    /// Draw the shot to the given line renderer.
    pub(crate) fn draw(&self, line_renderer: &mut LineRenderer) {
        line_renderer.add_lines(self.render_lines.iter());
    }
}
