use crate::line_renderer::TintedLine;

use quicksilver::geom::{Line, Transform};

pub(crate) trait Transformable {
    fn transformed(&self, transform: Transform) -> Self
    where
        Self: Sized;

    fn rotated_by(&self, angle: f32) -> Self
    where
        Self: Sized,
    {
        self.transformed(Transform::rotate(angle))
    }
}

impl Transformable for Line {
    fn transformed(&self, transform: Transform) -> Self {
        Line {
            a: transform * self.a,
            b: transform * self.b,
            t: self.t,
        }
    }
}

impl Transformable for TintedLine {
    fn transformed(&self, transform: Transform) -> Self {
        TintedLine {
            line: self.line.transformed(transform),
            colour: self.colour,
        }
    }
}
