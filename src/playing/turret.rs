use super::camera::Camera;
use super::killable::Kill;
use crate::collision_lines::CollisionLines;
use crate::line_renderer::{LineRenderer, TintedLine};
use crate::transformed::Transformable;

use quicksilver::{
    geom::{Transform, Vector},
    graphics::Color,
};

pub struct Turret {
    pos: Vector,
    angle: f32,
    model_lines: Vec<TintedLine>,
    render_lines: Vec<TintedLine>,
    pub(crate) collision_lines: CollisionLines,
    pub(crate) alive: bool,
}

killable!(Turret);

impl Turret {
    pub(crate) fn new(pos: Vector, angle: f32) -> Self {
        let lines = vec![
            TintedLine::new((-16, 0), (16, 0), Color::GREEN),
            TintedLine::new((-16, 0), (-12, 16), Color::GREEN),
            TintedLine::new((-12, 16), (-4, 16), Color::GREEN),
            TintedLine::new((-4, 16), (0, 24), Color::GREEN),
            TintedLine::new((0, 24), (4, 16), Color::GREEN),
            TintedLine::new((-4, 16), (12, 16), Color::GREEN),
            TintedLine::new((12, 16), (16, 0), Color::GREEN),
        ];
        let length = lines.len();
        Turret {
            pos,
            angle,
            model_lines: lines,
            render_lines: Vec::with_capacity(length),
            collision_lines: CollisionLines::new(Vec::with_capacity(length)),
            alive: true,
        }
    }

    pub(crate) fn control(&mut self, camera: &Camera) {
        let transform = Transform::translate(self.pos) * Transform::rotate(self.angle);
        self.render_lines.clear();
        self.render_lines.extend(
            self.model_lines
                .iter()
                .map(|line| line.transformed(transform)),
        );

        self.collision_lines
            .update(transform, self.model_lines.iter().map(|line| line.line));

        if self.pos.x < camera.pos.x - 16.0 {
            self.kill();
        }
    }

    /// Draw the turret to the given line renderer.
    pub(crate) fn draw(&self, line_renderer: &mut LineRenderer) {
        line_renderer.add_lines(self.render_lines.iter());
    }
}
