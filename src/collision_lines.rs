use crate::transformed::Transformable;
use quicksilver::geom::{Line, Shape, Transform};

pub struct CollisionLines {
    model: Vec<Line>,
    transformed: Vec<Line>,
}

impl CollisionLines {
    pub(crate) fn new(lines: Vec<Line>) -> Self {
        CollisionLines {
            model: lines,
            transformed: Vec::new(),
        }
    }

    pub(crate) fn reset(&mut self, transform: Transform, lines: impl Iterator<Item = Line>) {
        self.transformed.clear();
        self.transformed
            .extend(lines.map(|line| line.transformed(transform)));
    }

    pub(crate) fn update(&mut self, transform: Transform) {
        self.transformed.clear();
        self.transformed
            .extend(self.model.iter().map(|line| line.transformed(transform)));
    }

    pub fn intersects(&self, other: &CollisionLines) -> bool {
        for line_a in &self.transformed {
            for line_b in &other.transformed {
                if line_a.intersects(&line_b) {
                    return true;
                }
            }
        }
        false
    }
}
