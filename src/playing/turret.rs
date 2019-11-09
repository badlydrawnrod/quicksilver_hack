use crate::collision_lines::{CollisionLines, CollisionModel};
use crate::line_renderer::{LineRenderer, RenderModel};

use super::world_pos::WorldPos;

use quicksilver::geom::{Rectangle, Shape, Transform, Vector};

use crate::playing::health::Health;
use rand::{prelude::*, Rng};

pub struct Turret {
    pos: Vector,
    angle: f32,
    render_model: RenderModel,
    collision_model: CollisionModel,
    collision_lines: CollisionLines,
    health: Health,
    rng: ThreadRng,
}

pub enum TurretAction {
    None,
    MakeShot(Vector, f32),
}

impl WorldPos for Turret {
    fn world_pos(&self) -> Vector {
        self.pos
    }
    fn angle(&self) -> f32 {
        self.angle
    }
}

impl AsRef<Health> for Turret {
    fn as_ref(&self) -> &Health {
        &self.health
    }
}

impl AsMut<Health> for Turret {
    fn as_mut(&mut self) -> &mut Health {
        &mut self.health
    }
}

impl Turret {
    pub(crate) fn new(
        render_model: RenderModel,
        collision_model: CollisionModel,
        pos: Vector,
        angle: f32,
    ) -> Self {
        Turret {
            pos,
            angle,
            render_model: render_model,
            collision_model: collision_model,
            collision_lines: CollisionLines::new(),
            health: Health::new(),
            rng: rand::thread_rng(),
        }
    }

    pub(crate) fn control(&mut self, playfield: &Rectangle) -> TurretAction {
        let transform = Transform::translate(self.pos) * Transform::rotate(self.angle);
        self.collision_lines.clear();
        self.collision_lines
            .add_model(self.collision_model.clone(), transform);

        // Check for the turret going out of bounds.
        if self.health.is_alive() && !playfield.contains(self.world_pos()) {
            self.health.kill();
            return TurretAction::None;
        }

        let is_firing = self.rng.gen_range(0, 1000) < 10;
        if is_firing {
            TurretAction::MakeShot(self.world_pos(), self.angle)
        } else {
            TurretAction::None
        }
    }

    /// Draw the turret to the given line renderer.
    pub(crate) fn draw(&self, line_renderer: &mut LineRenderer) {
        let transform = Transform::translate(self.pos) * Transform::rotate(self.angle);
        line_renderer.add_model(self.render_model.clone(), transform);
    }
}

impl AsRef<CollisionLines> for Turret {
    fn as_ref(&self) -> &CollisionLines {
        &self.collision_lines
    }
}
