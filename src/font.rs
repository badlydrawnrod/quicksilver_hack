// Ported from: https://github.com/osresearch/vst/blob/master/teensyv/asteroids_font.c

use std::collections::HashMap;

pub enum DrawCommand {
    P(i8, i8),
    FontUp,
    FontLast,
}

use crate::line_renderer::{LineRenderer, TintedLine};
use quicksilver::geom::Vector;
use quicksilver::graphics::Color;
use DrawCommand::{FontLast, FontUp, P};

pub struct Glyph(pub Vec<DrawCommand>);

pub struct VectorFont(pub HashMap<char, Glyph>);

impl VectorFont {
    // Note that the origin in these glyphs is at the bottom left.
    pub fn new() -> Self {
        let mut font: HashMap<char, Glyph> = HashMap::new();
        font.insert(
            '0',
            Glyph(vec![
                P(0, 0),
                P(8, 0),
                P(8, 12),
                P(0, 12),
                P(0, 0),
                P(8, 12),
                FontLast,
            ]),
        );
        font.insert('1', Glyph(vec![P(4, 0), P(4, 12), P(3, 10), FontLast]));
        font.insert(
            '2',
            Glyph(vec![
                P(0, 12),
                P(8, 12),
                P(8, 7),
                P(0, 5),
                P(0, 0),
                P(8, 0),
                FontLast,
            ]),
        );
        font.insert(
            '3',
            Glyph(vec![
                P(0, 12),
                P(8, 12),
                P(8, 0),
                P(0, 0),
                FontUp,
                P(0, 6),
                P(8, 6),
                FontLast,
            ]),
        );
        font.insert(
            '4',
            Glyph(vec![
                P(0, 12),
                P(0, 6),
                P(8, 6),
                FontUp,
                P(8, 12),
                P(8, 0),
                FontLast,
            ]),
        );
        font.insert(
            '5',
            Glyph(vec![
                P(0, 0),
                P(8, 0),
                P(8, 6),
                P(0, 7),
                P(0, 12),
                P(8, 12),
                FontLast,
            ]),
        );
        font.insert(
            '6',
            Glyph(vec![P(0, 12), P(0, 0), P(8, 0), P(8, 5), P(0, 7), FontLast]),
        );
        font.insert(
            '7',
            Glyph(vec![P(0, 12), P(8, 12), P(8, 6), P(4, 0), FontLast]),
        );
        font.insert(
            '8',
            Glyph(vec![
                P(0, 0),
                P(8, 0),
                P(8, 12),
                P(0, 12),
                P(0, 0),
                FontUp,
                P(0, 6),
                P(8, 6),
            ]),
        );
        font.insert(
            '9',
            Glyph(vec![
                P(8, 0),
                P(8, 12),
                P(0, 12),
                P(0, 7),
                P(8, 5),
                FontLast,
            ]),
        );
        font.insert(' ', Glyph(vec![FontLast]));
        font.insert('.', Glyph(vec![P(3, 0), P(4, 0), FontLast]));
        font.insert(',', Glyph(vec![P(2, 0), P(4, 2), FontLast]));
        font.insert('-', Glyph(vec![P(2, 6), P(6, 6), FontLast]));
        font.insert(
            '+',
            Glyph(vec![P(1, 6), P(7, 6), FontUp, P(4, 9), P(4, 3), FontLast]),
        );
        font.insert(
            '!',
            Glyph(vec![
                P(4, 0),
                P(3, 2),
                P(5, 2),
                P(4, 0),
                FontUp,
                P(4, 4),
                P(4, 12),
                FontLast,
            ]),
        );
        font.insert(
            '#',
            Glyph(vec![
                P(0, 4),
                P(8, 4),
                P(6, 2),
                P(6, 10),
                P(8, 8),
                P(0, 8),
                P(2, 10),
                P(2, 2),
            ]),
        );
        font.insert('^', Glyph(vec![P(2, 6), P(4, 12), P(6, 6), FontLast]));
        font.insert(
            '=',
            Glyph(vec![P(1, 4), P(7, 4), FontUp, P(1, 8), P(7, 8), FontLast]),
        );
        font.insert(
            '*',
            Glyph(vec![
                P(0, 0),
                P(4, 12),
                P(8, 0),
                P(0, 8),
                P(8, 8),
                P(0, 0),
                FontLast,
            ]),
        );
        font.insert('_', Glyph(vec![P(0, 0), P(8, 0), FontLast]));
        font.insert('/', Glyph(vec![P(0, 0), P(8, 12), FontLast]));
        font.insert('\\', Glyph(vec![P(0, 12), P(8, 0), FontLast]));
        font.insert(
            '@',
            Glyph(vec![
                P(8, 4),
                P(4, 0),
                P(0, 4),
                P(0, 8),
                P(4, 12),
                P(8, 8),
                P(4, 4),
                P(3, 6),
            ]),
        );
        font.insert(
            '$',
            Glyph(vec![
                P(6, 2),
                P(2, 6),
                P(6, 10),
                FontUp,
                P(4, 12),
                P(4, 0),
                FontLast,
            ]),
        );
        font.insert(
            '&',
            Glyph(vec![
                P(8, 0),
                P(4, 12),
                P(8, 8),
                P(0, 4),
                P(4, 0),
                P(8, 4),
                FontLast,
            ]),
        );
        font.insert(
            '[',
            Glyph(vec![P(6, 0), P(2, 0), P(2, 12), P(6, 12), FontLast]),
        );
        font.insert(
            ']',
            Glyph(vec![P(2, 0), P(6, 0), P(6, 12), P(2, 12), FontLast]),
        );
        font.insert(
            '(',
            Glyph(vec![P(6, 0), P(2, 4), P(2, 8), P(6, 12), FontLast]),
        );
        font.insert(
            ')',
            Glyph(vec![P(2, 0), P(6, 4), P(6, 8), P(2, 12), FontLast]),
        );
        font.insert(
            '{',
            Glyph(vec![
                P(6, 0),
                P(4, 2),
                P(4, 10),
                P(6, 12),
                FontUp,
                P(2, 6),
                P(4, 6),
                FontLast,
            ]),
        );
        font.insert(
            '}',
            Glyph(vec![
                P(4, 0),
                P(6, 2),
                P(6, 10),
                P(4, 12),
                FontUp,
                P(6, 6),
                P(8, 6),
                FontLast,
            ]),
        );
        font.insert(
            '%',
            Glyph(vec![
                P(0, 0),
                P(8, 12),
                FontUp,
                P(2, 10),
                P(2, 8),
                FontUp,
                P(6, 4),
                P(6, 2),
            ]),
        );
        font.insert('<', Glyph(vec![P(6, 0), P(2, 6), P(6, 12), FontLast]));
        font.insert('>', Glyph(vec![P(2, 0), P(6, 6), P(2, 12), FontLast]));
        font.insert(
            '|',
            Glyph(vec![P(4, 0), P(4, 5), FontUp, P(4, 6), P(4, 12), FontLast]),
        );
        font.insert(
            ':',
            Glyph(vec![P(4, 9), P(4, 7), FontUp, P(4, 5), P(4, 3), FontLast]),
        );
        font.insert(
            ';',
            Glyph(vec![P(4, 9), P(4, 7), FontUp, P(4, 5), P(1, 2), FontLast]),
        );
        font.insert(
            '"',
            Glyph(vec![P(2, 10), P(2, 6), FontUp, P(6, 10), P(6, 6), FontLast]),
        );
        font.insert('\'', Glyph(vec![P(2, 6), P(6, 10), FontLast]));
        font.insert('`', Glyph(vec![P(2, 10), P(6, 6), FontLast]));
        font.insert(
            '~',
            Glyph(vec![P(0, 4), P(2, 8), P(6, 4), P(8, 8), FontLast]),
        );
        font.insert(
            '?',
            Glyph(vec![
                P(0, 8),
                P(4, 12),
                P(8, 8),
                P(4, 4),
                FontUp,
                P(4, 1),
                P(4, 0),
                FontLast,
            ]),
        );
        font.insert(
            'A',
            Glyph(vec![
                P(0, 0),
                P(0, 8),
                P(4, 12),
                P(8, 8),
                P(8, 0),
                FontUp,
                P(0, 4),
                P(8, 4),
            ]),
        );
        font.insert(
            'B',
            Glyph(vec![
                P(0, 0),
                P(0, 12),
                P(4, 12),
                P(8, 10),
                P(4, 6),
                P(8, 2),
                P(4, 0),
                P(0, 0),
            ]),
        );
        font.insert(
            'C',
            Glyph(vec![P(8, 0), P(0, 0), P(0, 12), P(8, 12), FontLast]),
        );
        font.insert(
            'D',
            Glyph(vec![
                P(0, 0),
                P(0, 12),
                P(4, 12),
                P(8, 8),
                P(8, 4),
                P(4, 0),
                P(0, 0),
                FontLast,
            ]),
        );
        font.insert(
            'E',
            Glyph(vec![
                P(8, 0),
                P(0, 0),
                P(0, 12),
                P(8, 12),
                FontUp,
                P(0, 6),
                P(6, 6),
                FontLast,
            ]),
        );
        font.insert(
            'F',
            Glyph(vec![
                P(0, 0),
                P(0, 12),
                P(8, 12),
                FontUp,
                P(0, 6),
                P(6, 6),
                FontLast,
            ]),
        );
        font.insert(
            'G',
            Glyph(vec![
                P(6, 6),
                P(8, 4),
                P(8, 0),
                P(0, 0),
                P(0, 12),
                P(8, 12),
                FontLast,
            ]),
        );
        font.insert(
            'H',
            Glyph(vec![
                P(0, 0),
                P(0, 12),
                FontUp,
                P(0, 6),
                P(8, 6),
                FontUp,
                P(8, 12),
                P(8, 0),
            ]),
        );
        font.insert(
            'I',
            Glyph(vec![
                P(0, 0),
                P(8, 0),
                FontUp,
                P(4, 0),
                P(4, 12),
                FontUp,
                P(0, 12),
                P(8, 12),
            ]),
        );
        font.insert(
            'J',
            Glyph(vec![P(0, 4), P(4, 0), P(8, 0), P(8, 12), FontLast]),
        );
        font.insert(
            'K',
            Glyph(vec![
                P(0, 0),
                P(0, 12),
                FontUp,
                P(8, 12),
                P(0, 6),
                P(6, 0),
                FontLast,
            ]),
        );
        font.insert('L', Glyph(vec![P(8, 0), P(0, 0), P(0, 12), FontLast]));
        font.insert(
            'M',
            Glyph(vec![
                P(0, 0),
                P(0, 12),
                P(4, 8),
                P(8, 12),
                P(8, 0),
                FontLast,
            ]),
        );
        font.insert(
            'N',
            Glyph(vec![P(0, 0), P(0, 12), P(8, 0), P(8, 12), FontLast]),
        );
        font.insert(
            'O',
            Glyph(vec![
                P(0, 0),
                P(0, 12),
                P(8, 12),
                P(8, 0),
                P(0, 0),
                FontLast,
            ]),
        );
        font.insert(
            'P',
            Glyph(vec![
                P(0, 0),
                P(0, 12),
                P(8, 12),
                P(8, 6),
                P(0, 5),
                FontLast,
            ]),
        );
        font.insert(
            'Q',
            Glyph(vec![
                P(0, 0),
                P(0, 12),
                P(8, 12),
                P(8, 4),
                P(0, 0),
                FontUp,
                P(4, 4),
                P(8, 0),
            ]),
        );
        font.insert(
            'R',
            Glyph(vec![
                P(0, 0),
                P(0, 12),
                P(8, 12),
                P(8, 6),
                P(0, 5),
                FontUp,
                P(4, 5),
                P(8, 0),
            ]),
        );
        font.insert(
            'S',
            Glyph(vec![
                P(0, 2),
                P(2, 0),
                P(8, 0),
                P(8, 5),
                P(0, 7),
                P(0, 12),
                P(6, 12),
                P(8, 10),
            ]),
        );
        font.insert(
            'T',
            Glyph(vec![
                P(0, 12),
                P(8, 12),
                FontUp,
                P(4, 12),
                P(4, 0),
                FontLast,
            ]),
        );
        font.insert(
            'U',
            Glyph(vec![
                P(0, 12),
                P(0, 2),
                P(4, 0),
                P(8, 2),
                P(8, 12),
                FontLast,
            ]),
        );
        font.insert('V', Glyph(vec![P(0, 12), P(4, 0), P(8, 12), FontLast]));
        font.insert(
            'W',
            Glyph(vec![
                P(0, 12),
                P(2, 0),
                P(4, 4),
                P(6, 0),
                P(8, 12),
                FontLast,
            ]),
        );
        font.insert(
            'X',
            Glyph(vec![P(0, 0), P(8, 12), FontUp, P(0, 12), P(8, 0), FontLast]),
        );
        font.insert(
            'Y',
            Glyph(vec![
                P(0, 12),
                P(4, 6),
                P(8, 12),
                FontUp,
                P(4, 6),
                P(4, 0),
                FontLast,
            ]),
        );
        font.insert(
            'Z',
            Glyph(vec![
                P(0, 12),
                P(8, 12),
                P(0, 0),
                P(8, 0),
                FontUp,
                P(2, 6),
                P(6, 6),
                FontLast,
            ]),
        );

        VectorFont(font)
    }
}

pub trait RenderFont {
    fn add_text(&mut self, pos: Vector, font: &VectorFont, text: &str);
}

impl RenderFont for LineRenderer {
    fn add_text(&mut self, pos: Vector, font: &VectorFont, text: &str) {
        let mut pos = pos;
        let mut lines: Vec<TintedLine> = Vec::new();
        for c in text.chars() {
            if let Some(strokes) = font.0.get(&c) {
                let mut last_coord: Option<Vector> = None;
                for stroke in strokes.0.iter() {
                    match stroke {
                        P(x, y) => {
                            let new_coord = pos + Vector::new(*x as f32 * 2.0, -*y as f32 * 2.0);
                            if let Some(coord) = last_coord {
                                lines.push(TintedLine::new(coord, new_coord, Color::GREEN));
                            }
                            last_coord = Some(new_coord);
                        }
                        FontUp => {
                            last_coord = None;
                        }
                        FontLast => break,
                    }
                }
            }
            pos.x += 10.0 * 2.0;
        }
        self.add_lines(lines.into_iter());
    }
}
