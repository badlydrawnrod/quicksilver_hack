use crate::transformed::Transformable;
use quicksilver::geom::{Line, Shape, Transform};
use std::rc::Rc;

#[derive(Clone)]
pub struct CollisionModel {
    lines: Rc<Vec<Line>>,
}

impl CollisionModel {
    pub fn new(lines: Vec<Line>) -> Self {
        CollisionModel {
            lines: Rc::new(lines),
        }
    }
}

pub struct CollisionAssets {
    shot: CollisionModel,
    turret: CollisionModel,
    player: CollisionModel,
}

impl CollisionAssets {
    pub fn new() -> Self {
        let shot_lines = vec![
            Line::new((-4, 4), (4, 4)),
            Line::new((4, 4), (0, -4)),
            Line::new((0, -4), (-4, 4)),
            Line::new((0, 0), (0, -8)),
        ];

        let turret_lines = vec![
            Line::new((-16, 0), (16, 0)),
            Line::new((-16, 0), (-12, 16)),
            Line::new((-12, 16), (-4, 16)),
            Line::new((-4, 16), (0, 24)),
            Line::new((0, 24), (4, 16)),
            Line::new((-4, 16), (12, 16)),
            Line::new((12, 16), (16, 0)),
        ];

        let player_lines = vec![
            Line::new((-16, 16), (16, 16)),
            Line::new((16, 16), (0, -16)),
            Line::new((0, -16), (-16, 16)),
        ];

        CollisionAssets {
            shot: CollisionModel::new(shot_lines),
            turret: CollisionModel::new(turret_lines),
            player: CollisionModel::new(player_lines),
        }
    }

    pub fn shot(&self) -> CollisionModel {
        self.shot.clone()
    }

    pub fn turret(&self) -> CollisionModel {
        self.turret.clone()
    }

    pub fn player(&self) -> CollisionModel {
        self.player.clone()
    }
}

#[derive(Clone)]
pub struct CollisionLines {
    lines: Vec<Line>,
}

impl CollisionLines {
    pub(crate) fn new() -> Self {
        CollisionLines { lines: Vec::new() }
    }

    /// Clear all the lines from this collision object.
    pub(crate) fn clear(&mut self) {
        self.lines.clear();
    }

    pub(crate) fn add_lines(&mut self, transform: Transform, lines: impl Iterator<Item = Line>) {
        self.lines
            .extend(lines.map(|line| line.transformed(transform)));
    }

    pub(crate) fn add_model(&mut self, collision_model: CollisionModel, transform: Transform) {
        self.lines.extend(
            collision_model
                .lines
                .iter()
                .map(|line| line.transformed(transform)),
        );
    }

    pub fn intersects(&self, other: &CollisionLines) -> bool {
        for line_a in &self.lines {
            for line_b in &other.lines {
                if line_a.intersects(&line_b) {
                    return true;
                }
            }
        }
        false
    }
}
