use super::camera::Camera;
use super::killable::Kill;
use crate::collision_lines::CollisionLines;
use crate::line_renderer::{LineRenderer, TintedLine};
use crate::transformed::Transformable;

use super::world_pos::WorldPos;

use quicksilver::{
    geom::{Transform, Vector},
    graphics::Color,
};

use rand::{prelude::*, Rng};

pub struct Turret {
    pos: Vector,
    pub(crate) angle: f32,
    model_lines: Vec<TintedLine>,
    render_lines: Vec<TintedLine>,
    pub(crate) collision_lines: CollisionLines,
    pub(crate) alive: bool,
    pub(crate) is_firing: bool,
    rng: ThreadRng,
}

killable!(Turret);

impl WorldPos for Turret {
    fn world_pos(&self) -> Vector {
        self.pos
    }
}

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
        let collision_lines = lines.iter().map(|line| line.line).collect::<Vec<_>>();
        let length = lines.len();
        Turret {
            pos,
            angle,
            model_lines: lines,
            render_lines: Vec::with_capacity(length),
            collision_lines: CollisionLines::new(collision_lines),
            alive: true,
            is_firing: false,
            rng: rand::thread_rng(),
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
        self.collision_lines.update(transform);

        self.is_firing = self.rng.gen_range(0, 1000) < 10;

        if self.pos.x < camera.pos.x - 16.0 {
            self.kill();
        }
    }

    /// Draw the turret to the given line renderer.
    pub(crate) fn draw(&self, line_renderer: &mut LineRenderer) {
        line_renderer.add_lines(self.render_lines.iter());
    }
}
