use crate::collision_lines::CollisionLines;
use crate::constants::*;
use crate::line_renderer::{LineRenderer, TintedLine};

use super::camera::Camera;

use quicksilver::{
    geom::{Line, Transform, Vector},
    graphics::Color,
};

use rand::{prelude::*, Rng};

const LANDSCAPE_MIN_Y: f32 = VIRTUAL_HEIGHT as f32 / 4.0;
const LANDSCAPE_MAX_Y: f32 = VIRTUAL_HEIGHT as f32 - 8.0;
const LANDSCAPE_MAX_DY: f32 = 128.0;
const LANDSCAPE_STEP: f32 = 16.0;

pub struct Landscape {
    pub(crate) render_lines: Vec<TintedLine>,
    pub(crate) collision_lines: CollisionLines,
    rng: ThreadRng,
    pub(crate) want_turret: bool,
    flat: i32,
}

impl Landscape {
    pub fn new() -> Self {
        let mut render_lines = Vec::new();
        let mut collision_lines = Vec::new();
        let mut last_point = Vector::new(0.0, 15 * VIRTUAL_HEIGHT / 16);
        let mut x = 0.0;
        while x <= VIRTUAL_WIDTH as f32 + LANDSCAPE_STEP {
            let next_point = Vector::new(x, last_point.y);
            let line = Line::new(last_point, next_point);
            render_lines.push(TintedLine::new(line.a, line.b, Color::GREEN));
            collision_lines.push(line);
            last_point = next_point;
            x += LANDSCAPE_STEP;
        }
        Landscape {
            render_lines,
            collision_lines: CollisionLines::new(),
            rng: rand::thread_rng(),
            want_turret: false,
            flat: 0,
        }
    }

    pub fn update(&mut self, camera: &Camera) {
        // We need to add a new line to our landscape if the rightmost point of the rightmost line
        // is about to become visible.
        self.want_turret = false;
        let b = self.render_lines[self.render_lines.len() - 1].line.b;
        if b.x < LANDSCAPE_STEP + VIRTUAL_WIDTH as f32 + camera.pos.x {
            let new_y = if self.flat == 0 && self.rng.gen_range(0, 100) >= 25 {
                let mut new_y = b.y + self.rng.gen_range(-LANDSCAPE_MAX_DY, LANDSCAPE_MAX_DY);
                while new_y > LANDSCAPE_MAX_Y || new_y < LANDSCAPE_MIN_Y {
                    new_y = b.y + self.rng.gen_range(-64.0, 64.0);
                }
                new_y
            } else {
                if self.flat == 0 {
                    self.flat = 4;
                } else {
                    self.flat -= 1;
                }
                b.y
            };
            let next_point = Vector::new(b.x + LANDSCAPE_STEP, new_y);
            self.render_lines
                .push(TintedLine::new(b, next_point, Color::GREEN));

            // Randomly add a turret if the conditions are right.
            self.want_turret = self.flat == 2 && self.rng.gen_range(0, 100) >= 50;
        }

        // We need to remove the leftmost line from the landscape if it is no longer visible.
        let a = &self.render_lines[0].line.b;
        if a.x < camera.pos.x {
            self.render_lines.remove(0);
        }

        self.collision_lines.clear();
        self.collision_lines.add_lines(
            Transform::IDENTITY,
            self.render_lines.iter().map(|line| line.line),
        );
    }

    /// Draw the landscape to the given line renderer.
    pub fn draw(&self, line_renderer: &mut LineRenderer) {
        line_renderer.add_lines(self.render_lines.iter().cloned());
    }
}
