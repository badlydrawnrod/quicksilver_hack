use super::camera::Camera;
use super::killable::Kill;
use crate::collision_lines::{CollisionLines, CollisionModel};
use crate::line_renderer::{LineRenderer, RenderModel};

use super::world_pos::WorldPos;

use quicksilver::geom::{Transform, Vector};

use rand::{prelude::*, Rng};

pub struct Turret {
    pos: Vector,
    angle: f32,
    render_model: RenderModel,
    collision_model: CollisionModel,
    collision_lines: CollisionLines,
    pub(crate) alive: bool, // TODO: make private
    rng: ThreadRng,
}

pub enum TurretAction {
    None,
    MakeShot(Vector, f32),
}

killable!(Turret);

impl WorldPos for Turret {
    fn world_pos(&self) -> Vector {
        self.pos
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
            alive: true,
            rng: rand::thread_rng(),
        }
    }

    pub(crate) fn control(&mut self, camera: &Camera) -> TurretAction {
        let transform = Transform::translate(self.pos) * Transform::rotate(self.angle);
        self.collision_lines.clear();
        self.collision_lines
            .add_model(self.collision_model.clone(), transform);

        if self.pos.x < camera.pos.x - 16.0 {
            self.kill();
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
