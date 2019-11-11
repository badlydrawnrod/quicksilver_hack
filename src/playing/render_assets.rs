use crate::line_renderer::{RenderModel, TintedLine};
use quicksilver::prelude::Color;

pub struct RenderAssets {
    shot: RenderModel,
    turret: RenderModel,
    player: RenderModel,
    rocket: RenderModel,
}

impl RenderAssets {
    pub fn new() -> Self {
        let shot_lines = vec![
            TintedLine::new((-4, 4), (4, 4), Color::GREEN),
            TintedLine::new((4, 4), (0, -4), Color::GREEN),
            TintedLine::new((0, -4), (-4, 4), Color::GREEN),
            TintedLine::new((0, 0), (0, -8), Color::GREEN),
        ];

        let turret_lines = vec![
            TintedLine::new((-16, 0), (16, 0), Color::GREEN),
            TintedLine::new((-16, 0), (-12, 16), Color::GREEN),
            TintedLine::new((-12, 16), (-4, 16), Color::GREEN),
            TintedLine::new((-4, 16), (0, 24), Color::GREEN),
            TintedLine::new((0, 24), (4, 16), Color::GREEN),
            TintedLine::new((-4, 16), (12, 16), Color::GREEN),
            TintedLine::new((12, 16), (16, 0), Color::GREEN),
        ];

        let player_lines = vec![
            TintedLine::new((-16, 16), (16, 16), Color::GREEN),
            TintedLine::new((16, 16), (0, -16), Color::GREEN),
            TintedLine::new((0, -16), (-16, 16), Color::GREEN),
        ];

        let rocket_lines = vec![
            TintedLine::new((-6, 4), (-12, 16), Color::GREEN),
            TintedLine::new((-12, 16), (-12, 0), Color::GREEN),
            TintedLine::new((-12, 0), (0, 8), Color::GREEN),
            TintedLine::new((0, 8), (12, 0), Color::GREEN),
            TintedLine::new((12, 0), (12, 16), Color::GREEN),
            TintedLine::new((12, 16), (6, 4), Color::GREEN),
            TintedLine::new((-12, 0), (0, -28), Color::GREEN),
            TintedLine::new((0, -28), (12, 0), Color::GREEN),
        ];

        RenderAssets {
            shot: RenderModel::new(shot_lines),
            turret: RenderModel::new(turret_lines),
            player: RenderModel::new(player_lines),
            rocket: RenderModel::new(rocket_lines),
        }
    }

    pub fn shot(&self) -> RenderModel {
        self.shot.clone()
    }

    pub fn turret(&self) -> RenderModel {
        self.turret.clone()
    }

    pub fn player(&self) -> RenderModel {
        self.player.clone()
    }

    pub fn rocket(&self) -> RenderModel {
        self.rocket.clone()
    }
}
