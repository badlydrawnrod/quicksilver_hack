use crate::collision_lines::CollisionModel;
use quicksilver::geom::Line;

pub struct CollisionAssets {
    shot: CollisionModel,
    turret: CollisionModel,
    player: CollisionModel,
    rocket: CollisionModel,
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

        let rocket_lines = vec![
            Line::new((-12, 16), (0, -32)),
            Line::new((0, -32), (12, 16)),
            Line::new((12, 16), (0, 0)),
            Line::new((0, 0), (-12, 16)),
        ];

        CollisionAssets {
            shot: CollisionModel::new(shot_lines),
            turret: CollisionModel::new(turret_lines),
            player: CollisionModel::new(player_lines),
            rocket: CollisionModel::new(rocket_lines),
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

    pub fn rocket(&self) -> CollisionModel {
        self.rocket.clone()
    }
}
