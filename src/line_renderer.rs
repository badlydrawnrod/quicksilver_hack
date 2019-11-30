const LINE_THICKNESS: f32 = 9.0;

use quicksilver::{
    geom::{Line, Transform, Vector},
    graphics::{Color, Drawable, Image, Mesh},
    lifecycle::Window,
    prelude::Blended,
};

use crate::transformed::Transformable;
use std::borrow::BorrowMut;
use std::rc::Rc;

#[derive(Copy, Clone)]
/// A line with a colour.
pub struct TintedLine {
    /// The line.
    pub line: Line,
    /// The line's colour.
    pub colour: Color,
}

impl TintedLine {
    /// Create a tinted line from a start and end vector, and a colour.
    pub fn new(start: impl Into<Vector>, end: impl Into<Vector>, colour: impl Into<Color>) -> Self {
        TintedLine {
            line: Line::new(start, end).with_thickness(LINE_THICKNESS),
            colour: colour.into(),
        }
    }
}

#[derive(Clone)]
pub struct RenderModel {
    lines: Rc<Vec<TintedLine>>,
}

impl RenderModel {
    pub fn new(lines: Vec<TintedLine>) -> Self {
        RenderModel {
            lines: Rc::new(lines),
        }
    }
}

impl AsRef<RenderModel> for RenderModel {
    fn as_ref(&self) -> &RenderModel {
        &self
    }
}

/// A renderer for lines with a given image.
pub struct LineRenderer {
    /// The image used for all of the lines.
    image: Image,
    /// The mesh that all of the lines are drawn to.
    mesh: Mesh,
}

impl LineRenderer {
    /// Create a line renderer that will render lines with the given image.
    pub(crate) fn new(image: Image) -> Self {
        LineRenderer {
            image,
            mesh: Mesh::new(),
        }
    }

    /// Clear all lines from this line renderer's mesh.
    pub(crate) fn clear(&mut self) {
        self.mesh.clear();
    }

    /// Add the given lines to this renderer's mesh.
    pub(crate) fn add_lines(&mut self, lines: impl Iterator<Item = TintedLine>) {
        let identity = Transform::IDENTITY;
        for tinted_line in lines {
            let line = tinted_line.line;
            line.draw(
                self.mesh.borrow_mut(),
                Blended(&self.image, tinted_line.colour),
                identity,
                0.0,
            );
        }
    }

    /// Add the given model, transformed, to this line renderer's mesh.
    pub fn add_model<T: AsRef<RenderModel>>(&mut self, render_model: T, transform: Transform) {
        let render_model = render_model.as_ref();
        let transformed = render_model
            .lines
            .iter()
            .map(|line| line.transformed(transform));
        self.add_lines(transformed);
    }

    /// Render the mesh to a window.
    pub(crate) fn render(&self, window: &mut Window) {
        window.mesh().extend(&self.mesh);
    }
}
