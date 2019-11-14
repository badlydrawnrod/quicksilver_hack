// Ported from: https://github.com/osresearch/vst/blob/master/teensyv/asteroids_font.c

use std::collections::HashMap;

enum DrawCommand {
    P(i8, i8),
    FontUp,
    FontLast
}

use DrawCommand::{P, FontUp, FontLast};

struct Glyph(Vec<DrawCommand>);

struct VectorFont(HashMap<u8, Glyph>);


impl VectorFont {
    fn new() -> Self {
        let mut font: HashMap<u8, Glyph> = HashMap::new();
        font.insert(b'0', Glyph(vec![P(0, 0), P(8, 0), P(8, 12), P(0, 12), P(0, 0), P(8, 12), FontLast]));
        font.insert(b'1', Glyph(vec![P(4, 0), P(4, 12), P(3, 10), FontLast]));
        font.insert(b'2', Glyph(vec![P(0, 12), P(8, 12), P(8, 7), P(0, 5), P(0, 0), P(8, 0), FontLast]));
        font.insert(b'3', Glyph(vec![P(0, 12), P(8, 12), P(8, 0), P(0, 0), FontUp, P(0, 6), P(8, 6), FontLast]));
        font.insert(b'4', Glyph(vec![P(0, 12), P(0, 6), P(8, 6), FontUp, P(8, 12), P(8, 0), FontLast]));
        font.insert(b'5', Glyph(vec![P(0, 0), P(8, 0), P(8, 6), P(0, 7), P(0, 12), P(8, 12), FontLast]));
        font.insert(b'6', Glyph(vec![P(0, 12), P(0, 0), P(8, 0), P(8, 5), P(0, 7), FontLast]));
        font.insert(b'7', Glyph(vec![P(0, 12), P(8, 12), P(8, 6), P(4, 0), FontLast]));
        font.insert(b'8', Glyph(vec![P(0, 0), P(8, 0), P(8, 12), P(0, 12), P(0, 0), FontUp, P(0, 6), P(8, 6),]));
        font.insert(b'9', Glyph(vec![P(8, 0), P(8, 12), P(0, 12), P(0, 7), P(8, 5), FontLast]));
        font.insert(b' ', Glyph(vec![FontLast]));
        font.insert(b'.', Glyph(vec![P(3, 0), P(4, 0), FontLast]));
        font.insert(b',', Glyph(vec![P(2, 0), P(4, 2), FontLast]));
        font.insert(b'-', Glyph(vec![P(2, 6), P(6, 6), FontLast]));
        font.insert(b'+', Glyph(vec![P(1, 6), P(7, 6), FontUp, P(4, 9), P(4, 3), FontLast]));
        font.insert(b'!', Glyph(vec![P(4, 0), P(3, 2), P(5, 2), P(4, 0), FontUp, P(4, 4), P(4, 12), FontLast]));
        font.insert(b'#', Glyph(vec![P(0, 4), P(8, 4), P(6, 2), P(6, 10), P(8, 8), P(0, 8), P(2, 10), P(2, 2)]));
        font.insert(b'^', Glyph(vec![P(2, 6), P(4, 12), P(6, 6), FontLast]));
        font.insert(b'=', Glyph(vec![P(1, 4), P(7, 4), FontUp, P(1, 8), P(7, 8), FontLast]));
        font.insert(b'*', Glyph(vec![P(0, 0), P(4, 12), P(8, 0), P(0, 8), P(8, 8), P(0, 0), FontLast]));
        font.insert(b'_', Glyph(vec![P(0, 0), P(8, 0), FontLast]));
        font.insert(b'/', Glyph(vec![P(0, 0), P(8, 12), FontLast]));
        font.insert(b'\\', Glyph(vec![P(0, 12), P(8, 0), FontLast]));
        font.insert(b'@', Glyph(vec![P(8, 4), P(4, 0), P(0, 4), P(0, 8), P(4, 12), P(8, 8), P(4, 4), P(3, 6)]));
        font.insert(b'$', Glyph(vec![P(6, 2), P(2, 6), P(6, 10), FontUp, P(4, 12), P(4, 0), FontLast]));
        font.insert(b'&', Glyph(vec![P(8, 0), P(4, 12), P(8, 8), P(0, 4), P(4, 0), P(8, 4), FontLast]));
        font.insert(b'[', Glyph(vec![P(6, 0), P(2, 0), P(2, 12), P(6, 12), FontLast]));
        font.insert(b']', Glyph(vec![P(2, 0), P(6, 0), P(6, 12), P(2, 12), FontLast]));
        font.insert(b'(', Glyph(vec![P(6, 0), P(2, 4), P(2, 8), P(6, 12), FontLast]));
        font.insert(b')', Glyph(vec![P(2, 0), P(6, 4), P(6, 8), P(2, 12), FontLast]));
        font.insert(b'{', Glyph(vec![P(6, 0), P(4, 2), P(4, 10), P(6, 12), FontUp, P(2, 6), P(4, 6), FontLast]));
        font.insert(b'}', Glyph(vec![P(4, 0), P(6, 2), P(6, 10), P(4, 12), FontUp, P(6, 6), P(8, 6), FontLast]));
        font.insert(b'%', Glyph(vec![P(0, 0), P(8, 12), FontUp, P(2, 10), P(2, 8), FontUp, P(6, 4), P(6, 2)]));
        font.insert(b'<', Glyph(vec![P(6, 0), P(2, 6), P(6, 12), FontLast]));
        font.insert(b'>', Glyph(vec![P(2, 0), P(6, 6), P(2, 12), FontLast]));
        font.insert(b'|', Glyph(vec![P(4, 0), P(4, 5), FontUp, P(4, 6), P(4, 12), FontLast]));
        font.insert(b':', Glyph(vec![P(4, 9), P(4, 7), FontUp, P(4, 5), P(4, 3), FontLast]));
        font.insert(b';', Glyph(vec![P(4, 9), P(4, 7), FontUp, P(4, 5), P(1, 2), FontLast]));
        font.insert(b'"', Glyph(vec![P(2, 10), P(2, 6), FontUp, P(6, 10), P(6, 6), FontLast]));
        font.insert(b'\'', Glyph(vec![P(2, 6), P(6, 10), FontLast]));
        font.insert(b'`', Glyph(vec![P(2, 10), P(6, 6), FontLast]));
        font.insert(b'~', Glyph(vec![P(0, 4), P(2, 8), P(6, 4), P(8, 8), FontLast]));
        font.insert(b'?', Glyph(vec![P(0, 8), P(4, 12), P(8, 8), P(4, 4), FontUp, P(4, 1), P(4, 0), FontLast]));
        font.insert(b'A', Glyph(vec![P(0, 0), P(0, 8), P(4, 12), P(8, 8), P(8, 0), FontUp, P(0, 4), P(8, 4)]));
        font.insert(b'B', Glyph(vec![P(0, 0), P(0, 12), P(4, 12), P(8, 10), P(4, 6), P(8, 2), P(4, 0), P(0, 0)]));
        font.insert(b'C', Glyph(vec![P(8, 0), P(0, 0), P(0, 12), P(8, 12), FontLast]));
        font.insert(b'D', Glyph(vec![P(0, 0), P(0, 12), P(4, 12), P(8, 8), P(8, 4), P(4, 0), P(0, 0), FontLast]));
        font.insert(b'E', Glyph(vec![P(8, 0), P(0, 0), P(0, 12), P(8, 12), FontUp, P(0, 6), P(6, 6), FontLast]));
        font.insert(b'F', Glyph(vec![P(0, 0), P(0, 12), P(8, 12), FontUp, P(0, 6), P(6, 6), FontLast]));
        font.insert(b'G', Glyph(vec![P(6, 6), P(8, 4), P(8, 0), P(0, 0), P(0, 12), P(8, 12), FontLast]));
        font.insert(b'H', Glyph(vec![P(0, 0), P(0, 12), FontUp, P(0, 6), P(8, 6), FontUp, P(8, 12), P(8, 0)]));
        font.insert(b'I', Glyph(vec![P(0, 0), P(8, 0), FontUp, P(4, 0), P(4, 12), FontUp, P(0, 12), P(8, 12)]));
        font.insert(b'J', Glyph(vec![P(0, 4), P(4, 0), P(8, 0), P(8, 12), FontLast]));
        font.insert(b'K', Glyph(vec![P(0, 0), P(0, 12), FontUp, P(8, 12), P(0, 6), P(6, 0), FontLast]));
        font.insert(b'L', Glyph(vec![P(8, 0), P(0, 0), P(0, 12), FontLast]));
        font.insert(b'M', Glyph(vec![P(0, 0), P(0, 12), P(4, 8), P(8, 12), P(8, 0), FontLast]));
        font.insert(b'N', Glyph(vec![P(0, 0), P(0, 12), P(8, 0), P(8, 12), FontLast]));
        font.insert(b'O', Glyph(vec![P(0, 0), P(0, 12), P(8, 12), P(8, 0), P(0, 0), FontLast]));
        font.insert(b'P', Glyph(vec![P(0, 0), P(0, 12), P(8, 12), P(8, 6), P(0, 5), FontLast]));
        font.insert(b'Q', Glyph(vec![P(0, 0), P(0, 12), P(8, 12), P(8, 4), P(0, 0), FontUp, P(4, 4), P(8, 0)]));
        font.insert(b'R', Glyph(vec![P(0, 0), P(0, 12), P(8, 12), P(8, 6), P(0, 5), FontUp, P(4, 5), P(8, 0)]));
        font.insert(b'S', Glyph(vec![P(0, 2), P(2, 0), P(8, 0), P(8, 5), P(0, 7), P(0, 12), P(6, 12), P(8, 10)]));
        font.insert(b'T', Glyph(vec![P(0, 12), P(8, 12), FontUp, P(4, 12), P(4, 0), FontLast]));
        font.insert(b'U', Glyph(vec![P(0, 12), P(0, 2), P(4, 0), P(8, 2), P(8, 12), FontLast]));
        font.insert(b'V', Glyph(vec![P(0, 12), P(4, 0), P(8, 12), FontLast]));
        font.insert(b'W', Glyph(vec![P(0, 12), P(2, 0), P(4, 4), P(6, 0), P(8, 12), FontLast]));
        font.insert(b'X', Glyph(vec![P(0, 0), P(8, 12), FontUp, P(0, 12), P(8, 0), FontLast]));
        font.insert(b'Y', Glyph(vec![P(0, 12), P(4, 6), P(8, 12), FontUp, P(4, 6), P(4, 0), FontLast]));
        font.insert(b'Z', Glyph(vec![P(0, 12), P(8, 12), P(0, 0), P(8, 0), FontUp, P(2, 6), P(6, 6), FontLast]));

        VectorFont(font)
    }
}
