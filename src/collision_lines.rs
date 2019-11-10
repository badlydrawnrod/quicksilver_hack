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
}

/// Collide one against one, returning true if a collision is detected.
pub fn collides_with(a: impl AsRef<CollisionLines>, b: impl AsRef<CollisionLines>) -> bool {
    let a_lines = &a.as_ref().lines;
    let b_lines = &b.as_ref().lines;
    for line_a in a_lines {
        for line_b in b_lines {
            if line_a.intersects(&line_b) {
                return true;
            }
        }
    }
    false
}

/// Collide many against one, invoking the given closure when a collision is detected.
pub fn collide_many_one<T, U, F>(xs: &mut [T], y: &mut U, on_collision: F)
where
    T: AsRef<CollisionLines>,
    U: AsRef<CollisionLines>,
    F: Fn(&mut T, &mut U) -> (),
{
    for x in xs.iter_mut() {
        if collides_with(&x, &y) {
            on_collision(x, y);
        }
    }
}

/// Collide many against many, invoking the given closure when a collision is detected.
pub fn collide_many_many<T, U, F>(xs: &mut [T], ys: &mut [U], on_collision: F)
where
    T: AsRef<CollisionLines>,
    U: AsRef<CollisionLines>,
    F: Fn(&mut T, &mut U) -> (),
{
    for x in xs.iter_mut() {
        for y in ys.iter_mut() {
            if collides_with(&x, &y) {
                on_collision(x, y);
            }
        }
    }
}
