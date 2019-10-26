use crate::transformed::Transformable;
use quicksilver::geom::{Line, Shape, Transform};

pub struct CollisionLines(Vec<Line>);

impl CollisionLines {
    pub(crate) fn new(lines: Vec<Line>) -> Self {
        CollisionLines(lines)
    }

    pub(crate) fn update(&mut self, transform: Transform, lines: impl Iterator<Item = Line>) {
        self.0.clear();
        self.0.extend(lines.map(|line| line.transformed(transform)));
    }

    pub(crate) fn intersects(&self, other: &CollisionLines) -> bool {
        for line_a in &self.0 {
            for line_b in &other.0 {
                if line_a.intersects(&line_b) {
                    return true;
                }
            }
        }
        false
    }
}
