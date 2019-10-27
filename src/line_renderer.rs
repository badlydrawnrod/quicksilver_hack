const LINE_THICKNESS: f32 = 8.0;

use quicksilver::{
    geom::{Line, Transform, Vector},
    graphics::{Color, Drawable, Image, Mesh},
    lifecycle::Window,
    prelude::Blended,
};

use std::borrow::BorrowMut;

#[derive(Copy, Clone)]
/// A line with a colour.
pub(crate) struct TintedLine {
    /// The line.
    pub line: Line,
    /// The line's colour.
    pub colour: Color,
}

impl TintedLine {
    /// Create a tinted line from a start and end vector, and a colour.
    pub(crate) fn new(
        start: impl Into<Vector>,
        end: impl Into<Vector>,
        colour: impl Into<Color>,
    ) -> Self {
        TintedLine {
            line: Line::new(start, end),
            colour: colour.into(),
        }
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

    /// Clear all lines from this renderer's mesh.
    pub(crate) fn clear(&mut self) {
        self.mesh.clear();
    }

    /// Add the given lines to this renderer's mesh.
    pub(crate) fn add_lines<'a>(&mut self, lines: impl Iterator<Item = &'a TintedLine>) {
        let identity = Transform::IDENTITY;
        for tinted_line in lines {
            let thick_line = tinted_line.line.with_thickness(LINE_THICKNESS);
            thick_line.draw(
                self.mesh.borrow_mut(),
                Blended(&self.image, tinted_line.colour),
                identity,
                0.0,
            );
        }
    }

    /// Render the mesh to a window.
    pub(crate) fn render(&self, window: &mut Window) {
        window.mesh().extend(&self.mesh);
    }
}
