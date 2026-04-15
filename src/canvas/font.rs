use std::collections::HashMap;

pub struct Font {
    pub glyphs: HashMap<char, (&'static [u8], u8)>,
    glyph_height: u8,
}

impl Font {
    pub fn new(glyph_height: u8) -> Self {
        Self {
            glyphs: HashMap::new(),
            glyph_height,
        }
    }

    pub fn load_default() -> Self {
        let mut font = Self::new(8);

        font.glyphs.insert('0', (&ZERO, 5));
        font.glyphs.insert('1', (&ONE, 5));
        font.glyphs.insert('2', (&TWO, 5));
        font.glyphs.insert('3', (&THREE, 5));
        font.glyphs.insert('4', (&FOUR, 5));
        font.glyphs.insert('5', (&FIVE, 5));
        font.glyphs.insert('6', (&SIX, 5));
        font.glyphs.insert('7', (&SEVEN, 5));
        font.glyphs.insert('8', (&EIGHT, 5));
        font.glyphs.insert('9', (&NINE, 5));

        font.glyphs.insert('A', (&UPPERCASE_A, 5));
        font.glyphs.insert('B', (&UPPERCASE_B, 5));
        font.glyphs.insert('C', (&UPPERCASE_C, 5));
        font.glyphs.insert('D', (&UPPERCASE_D, 5));
        font.glyphs.insert('E', (&UPPERCASE_E, 5));
        font.glyphs.insert('F', (&UPPERCASE_F, 5));
        font.glyphs.insert('G', (&UPPERCASE_G, 5));
        font.glyphs.insert('H', (&UPPERCASE_H, 5));
        font.glyphs.insert('I', (&UPPERCASE_I, 3));
        font.glyphs.insert('J', (&UPPERCASE_J, 5));
        font.glyphs.insert('K', (&UPPERCASE_K, 5));
        font.glyphs.insert('L', (&UPPERCASE_L, 5));
        font.glyphs.insert('M', (&UPPERCASE_M, 5));
        font.glyphs.insert('N', (&UPPERCASE_N, 5));
        font.glyphs.insert('O', (&UPPERCASE_O, 5));
        font.glyphs.insert('P', (&UPPERCASE_P, 5));
        font.glyphs.insert('Q', (&UPPERCASE_Q, 5));
        font.glyphs.insert('R', (&UPPERCASE_R, 5));
        font.glyphs.insert('S', (&UPPERCASE_S, 5));
        font.glyphs.insert('T', (&UPPERCASE_T, 5));
        font.glyphs.insert('U', (&UPPERCASE_U, 5));
        font.glyphs.insert('V', (&UPPERCASE_V, 5));
        font.glyphs.insert('W', (&UPPERCASE_W, 5));
        font.glyphs.insert('X', (&UPPERCASE_X, 5));
        font.glyphs.insert('Y', (&UPPERCASE_Y, 5));
        font.glyphs.insert('Z', (&UPPERCASE_Z, 5));

        font.glyphs.insert('a', (&LOWERCASE_A, 5));
        font.glyphs.insert('b', (&LOWERCASE_B, 4));
        font.glyphs.insert('c', (&LOWERCASE_C, 4));
        font.glyphs.insert('d', (&LOWERCASE_D, 4));
        font.glyphs.insert('e', (&LOWERCASE_E, 5));
        font.glyphs.insert('f', (&LOWERCASE_F, 4));
        font.glyphs.insert('g', (&LOWERCASE_G, 5));
        font.glyphs.insert('h', (&LOWERCASE_H, 4));
        font.glyphs.insert('i', (&LOWERCASE_I, 1));
        font.glyphs.insert('j', (&LOWERCASE_J, 4));
        font.glyphs.insert('k', (&LOWERCASE_K, 4));
        font.glyphs.insert('l', (&LOWERCASE_L, 3));
        font.glyphs.insert('m', (&LOWERCASE_M, 5));
        font.glyphs.insert('n', (&LOWERCASE_N, 5));
        font.glyphs.insert('o', (&LOWERCASE_O, 5));
        font.glyphs.insert('p', (&LOWERCASE_P, 5));
        font.glyphs.insert('q', (&LOWERCASE_Q, 5));
        font.glyphs.insert('r', (&LOWERCASE_R, 5));
        font.glyphs.insert('s', (&LOWERCASE_S, 5));
        font.glyphs.insert('t', (&LOWERCASE_T, 3));
        font.glyphs.insert('u', (&LOWERCASE_U, 5));
        font.glyphs.insert('v', (&LOWERCASE_V, 5));
        font.glyphs.insert('w', (&LOWERCASE_W, 5));
        font.glyphs.insert('x', (&LOWERCASE_X, 5));
        font.glyphs.insert('y', (&LOWERCASE_Y, 5));
        font.glyphs.insert('z', (&LOWERCASE_Z, 5));

        font.glyphs.insert(' ', (&SPACE, 1));
        font.glyphs.insert('.', (&DOT, 2));
        font.glyphs.insert(',', (&COMMA, 2));
        font.glyphs.insert(';', (&SEMICOLON, 2));
        font.glyphs.insert(':', (&COLON, 2));
        font.glyphs.insert('-', (&DASH, 3));
        font.glyphs.insert('_', (&UNDERSCORE, 5));
        font.glyphs.insert('#', (&HASHTAG, 5));
        font.glyphs.insert('!', (&EXCLAMATION, 1));
        font.glyphs.insert('?', (&QUESTION, 5));

        font
    }

    pub fn glyph_height(&self) -> u8 {
        self.glyph_height
    }
}

const ZERO: [u8; 40] = [
    1,1,1,1,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,1,1,1,1,
    0,0,0,0,0
];

const ONE: [u8; 40] = [
    0,0,1,0,0,
    0,1,1,0,0,
    1,0,1,0,0,
    0,0,1,0,0,
    0,0,1,0,0,
    0,0,1,0,0,
    1,1,1,1,1,
    0,0,0,0,0
];

const TWO: [u8; 40] = [
    1,1,1,1,1,
    0,0,0,0,1,
    0,0,0,0,1,
    1,1,1,1,1,
    1,0,0,0,0,
    1,0,0,0,0,
    1,1,1,1,1,
    0,0,0,0,0
];

const THREE: [u8; 40] = [
    1,1,1,1,1,
    0,0,0,0,1,
    0,0,0,0,1,
    0,1,1,1,1,
    0,0,0,0,1,
    0,0,0,0,1,
    1,1,1,1,1,
    0,0,0,0,0
];

const FOUR: [u8; 40] = [
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,1,1,1,1,
    0,0,0,0,1,
    0,0,0,0,1,
    0,0,0,0,1,
    0,0,0,0,0
];

const FIVE: [u8; 40] = [
    1,1,1,1,1,
    1,0,0,0,0,
    1,0,0,0,0,
    1,1,1,1,1,
    0,0,0,0,1,
    0,0,0,0,1,
    1,1,1,1,1,
    0,0,0,0,0
];

const SIX: [u8; 40] = [
    1,1,1,1,1,
    1,0,0,0,0,
    1,0,0,0,0,
    1,1,1,1,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,1,1,1,1,
    0,0,0,0,0
];

const SEVEN: [u8; 40] = [
    1,1,1,1,1,
    0,0,0,0,1,
    0,0,0,1,0,
    0,0,1,0,0,
    0,1,0,0,0,
    0,1,0,0,0,
    0,1,0,0,0,
    0,0,0,0,0
];

const EIGHT: [u8; 40] = [
    1,1,1,1,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,1,1,1,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,1,1,1,1,
    0,0,0,0,0
];

const NINE: [u8; 40] = [
    1,1,1,1,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,1,1,1,1,
    0,0,0,0,1,
    0,0,0,0,1,
    1,1,1,1,1,
    0,0,0,0,0
];

const UPPERCASE_A: [u8; 40] = [
    0,1,1,1,0,
    1,0,0,0,1,
    1,0,0,0,1,
    1,1,1,1,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    0,0,0,0,0
];

const UPPERCASE_B: [u8; 40] = [
    1,1,1,1,0,
    1,0,0,0,1,
    1,0,0,0,1,
    1,1,1,1,0,
    1,0,0,0,1,
    1,0,0,0,1,
    1,1,1,1,0,
    0,0,0,0,0
];

const UPPERCASE_C: [u8; 40] = [
    0,1,1,1,1,
    1,0,0,0,0,
    1,0,0,0,0,
    1,0,0,0,0,
    1,0,0,0,0,
    1,0,0,0,0,
    0,1,1,1,1,
    0,0,0,0,0
];

const UPPERCASE_D: [u8; 40] = [
    1,1,1,1,0,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,1,1,1,0,
    0,0,0,0,0
];

const UPPERCASE_E: [u8; 40] = [
    1,1,1,1,1,
    1,0,0,0,0,
    1,0,0,0,0,
    1,1,1,1,0,
    1,0,0,0,0,
    1,0,0,0,0,
    1,1,1,1,1,
    0,0,0,0,0
];

const UPPERCASE_F: [u8; 40] = [
    1,1,1,1,1,
    1,0,0,0,0,
    1,0,0,0,0,
    1,1,1,1,0,
    1,0,0,0,0,
    1,0,0,0,0,
    1,0,0,0,0,
    0,0,0,0,0
];

const UPPERCASE_G: [u8; 40] = [
    0,1,1,1,1,
    1,0,0,0,0,
    1,0,0,0,0,
    1,0,1,1,1,
    1,0,0,0,1,
    1,0,0,0,1,
    0,1,1,1,1,
    0,0,0,0,0
];

const UPPERCASE_H: [u8; 40] = [
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,1,1,1,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    0,0,0,0,0
];

const UPPERCASE_I: [u8; 24] = [
    1,1,1,
    0,1,0,
    0,1,0,
    0,1,0,
    0,1,0,
    0,1,0,
    1,1,1,
    0,0,0,
];

const UPPERCASE_J: [u8; 40] = [
    0,0,0,1,1,
    0,0,0,0,1,
    0,0,0,0,1,
    0,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    0,1,1,1,0,
    0,0,0,0,0
];

const UPPERCASE_K: [u8; 40] = [
    1,0,0,0,1,
    1,0,0,1,0,
    1,0,1,0,0,
    1,1,0,0,0,
    1,0,1,0,0,
    1,0,0,1,0,
    1,0,0,0,1,
    0,0,0,0,0
];

const UPPERCASE_L: [u8; 40] = [
    1,0,0,0,0,
    1,0,0,0,0,
    1,0,0,0,0,
    1,0,0,0,0,
    1,0,0,0,0,
    1,0,0,0,0,
    1,1,1,1,1,
    0,0,0,0,0
];

const UPPERCASE_M: [u8; 40] = [
    1,0,0,0,1,
    1,1,0,1,1,
    1,0,1,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    0,0,0,0,0
];

const UPPERCASE_N: [u8; 40] = [
    1,0,0,0,1,
    1,1,0,0,1,
    1,0,1,0,1,
    1,0,0,1,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    0,0,0,0,0
];

const UPPERCASE_O: [u8; 40] = [
    0,1,1,1,0,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    0,1,1,1,0,
    0,0,0,0,0
];

const UPPERCASE_P: [u8; 40] = [
    1,1,1,1,0,
    1,0,0,0,1,
    1,0,0,0,1,
    1,1,1,1,0,
    1,0,0,0,0,
    1,0,0,0,0,
    1,0,0,0,0,
    0,0,0,0,0
];

const UPPERCASE_Q: [u8; 40] = [
    0,1,1,1,0,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,1,0,1,
    1,0,0,1,0,
    0,1,1,0,1,
    0,0,0,0,0
];

const UPPERCASE_R: [u8; 40] = [
    1,1,1,1,0,
    1,0,0,0,1,
    1,0,0,0,1,
    1,1,1,1,0,
    1,0,1,0,0,
    1,0,0,1,0,
    1,0,0,0,1,
    0,0,0,0,0
];

const UPPERCASE_S: [u8; 40] = [
    0,1,1,1,1,
    1,0,0,0,0,
    1,0,0,0,0,
    0,1,1,1,0,
    0,0,0,0,1,
    0,0,0,0,1,
    1,1,1,1,0,
    0,0,0,0,0
];

const UPPERCASE_T: [u8; 40] = [
    1,1,1,1,1,
    0,0,1,0,0,
    0,0,1,0,0,
    0,0,1,0,0,
    0,0,1,0,0,
    0,0,1,0,0,
    0,0,1,0,0,
    0,0,0,0,0
];

const UPPERCASE_U: [u8; 40] = [
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    0,1,1,1,0,
    0,0,0,0,0
];

const UPPERCASE_V: [u8; 40] = [
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    0,1,0,1,0,
    0,0,1,0,0,
    0,0,0,0,0
];

const UPPERCASE_W: [u8; 40] = [
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,1,0,1,
    1,1,0,1,1,
    1,0,0,0,1,
    0,0,0,0,0
];

const UPPERCASE_X: [u8; 40] = [
    1,0,0,0,1,
    1,0,0,0,1,
    0,1,0,1,0,
    0,0,1,0,0,
    0,1,0,1,0,
    1,0,0,0,1,
    1,0,0,0,1,
    0,0,0,0,0
];

const UPPERCASE_Y: [u8; 40] = [
    1,0,0,0,1,
    1,0,0,0,1,
    0,1,0,1,0,
    0,0,1,0,0,
    0,0,1,0,0,
    0,0,1,0,0,
    0,0,1,0,0,
    0,0,0,0,0
];

const UPPERCASE_Z: [u8; 40] = [
    1,1,1,1,1,
    0,0,0,0,1,
    0,0,0,1,0,
    0,0,1,0,0,
    0,1,0,0,0,
    1,0,0,0,0,
    1,1,1,1,1,
    0,0,0,0,0
];

const LOWERCASE_A: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    0,1,1,1,0,
    0,0,0,0,1,
    0,1,1,1,1,
    1,0,0,0,1,
    0,1,1,1,1,
    0,0,0,0,0
];

const LOWERCASE_B: [u8; 32] = [
    1,0,0,0,
    1,0,0,0,
    1,1,1,0,
    1,0,0,1,
    1,0,0,1,
    1,0,0,1,
    1,1,1,0,
    0,0,0,0
];

const LOWERCASE_C: [u8; 32] = [
    0,0,0,0,
    0,0,0,0,
    0,1,1,1,
    1,0,0,0,
    1,0,0,0,
    1,0,0,0,
    0,1,1,1,
    0,0,0,0
];

const LOWERCASE_D: [u8; 32] = [
    0,0,0,1,
    0,0,0,1,
    0,1,1,1,
    1,0,0,1,
    1,0,0,1,
    1,0,0,1,
    0,1,1,1,
    0,0,0,0
];

const LOWERCASE_E: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    0,1,1,1,0,
    1,0,0,0,1,
    1,1,1,1,1,
    1,0,0,0,0,
    0,1,1,1,0,
    0,0,0,0,0
];

const LOWERCASE_F: [u8; 32] = [
    0,0,1,1,
    0,1,0,0,
    1,1,1,1,
    0,1,0,0,
    0,1,0,0,
    0,1,0,0,
    0,1,0,0,
    0,1,0,0
];

const LOWERCASE_G: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    0,1,1,1,1,
    1,0,0,0,1,
    1,0,0,0,1,
    0,1,1,1,1,
    0,0,0,0,1,
    1,1,1,1,0
];

const LOWERCASE_H: [u8; 32] = [
    1,0,0,0,
    1,0,0,0,
    1,1,1,0,
    1,0,0,1,
    1,0,0,1,
    1,0,0,1,
    1,0,0,1,
    0,0,0,0
];

const LOWERCASE_I: [u8; 8] = [
    0,
    1,
    0,
    1,
    1,
    1,
    1,
    0
];

const LOWERCASE_J: [u8; 32] = [
    0,0,0,0,
    0,0,0,1,
    0,0,0,0,
    0,0,0,1,
    0,0,0,1,
    0,0,0,1,
    1,0,0,1,
    0,1,1,0
];

const LOWERCASE_K: [u8; 32] = [
    1,0,0,0,
    1,0,0,0,
    1,0,0,1,
    1,0,1,0,
    1,1,0,0,
    1,0,1,0,
    1,0,0,1,
    0,0,0,0
];

const LOWERCASE_L: [u8; 24] = [
    1,1,0,
    0,1,0,
    0,1,0,
    0,1,0,
    0,1,0,
    0,1,0,
    1,1,1,
    0,0,0
];

const LOWERCASE_M: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    1,1,0,1,0,
    1,0,1,0,1,
    1,0,1,0,1,
    1,0,1,0,1,
    1,0,1,0,1,
    0,0,0,0,0
];

const LOWERCASE_N: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    1,1,1,1,0,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    0,0,0,0,0
];

const LOWERCASE_O: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    0,1,1,1,0,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    0,1,1,1,0,
    0,0,0,0,0
];

const LOWERCASE_P: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    1,1,1,1,0,
    1,0,0,0,1,
    1,0,0,0,1,
    1,1,1,1,0,
    1,0,0,0,0,
    1,0,0,0,0
];

const LOWERCASE_Q: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    0,1,1,1,1,
    1,0,0,0,1,
    1,0,0,0,1,
    0,1,1,1,1,
    0,0,0,0,1,
    0,0,0,0,1
];

const LOWERCASE_R: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    1,0,1,1,0,
    1,1,0,0,1,
    1,0,0,0,0,
    1,0,0,0,0,
    1,0,0,0,0,
    0,0,0,0,0
];

const LOWERCASE_S: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    0,1,1,1,1,
    1,0,0,0,0,
    0,1,1,1,0,
    0,0,0,0,1,
    1,1,1,1,0,
    0,0,0,0,0
];

const LOWERCASE_T: [u8; 24] = [
    0,1,0,
    1,1,1,
    0,1,0,
    0,1,0,
    0,1,0,
    0,1,0,
    0,0,1,
    0,0,0
];

const LOWERCASE_U: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    0,1,1,1,1,
    0,0,0,0,0
];

const LOWERCASE_V: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,0,0,1,
    0,1,0,1,0,
    0,0,1,0,0,
    0,0,0,0,0
];

const LOWERCASE_W: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    1,0,0,0,1,
    1,0,0,0,1,
    1,0,1,0,1,
    1,1,0,1,1,
    1,0,0,0,1,
    0,0,0,0,0
];

const LOWERCASE_X: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    1,0,0,0,1,
    0,1,0,1,0,
    0,0,1,0,0,
    0,1,0,1,0,
    1,0,0,0,1,
    0,0,0,0,0
];

const LOWERCASE_Y: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    1,0,0,0,1,
    1,0,0,0,1,
    0,1,0,1,0,
    0,0,1,0,0,
    0,0,1,0,0,
    1,1,0,0,0
];

const LOWERCASE_Z: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    1,1,1,1,1,
    0,0,0,1,0,
    0,0,1,0,0,
    0,1,0,0,0,
    1,1,1,1,1,
    0,0,0,0,0
];

const SPACE: [u8; 8] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0
];

const DOT: [u8; 16] = [
    0,0,
    0,0,
    0,0,
    0,0,
    0,0,
    1,1,
    1,1,
    0,0
];

const COMMA: [u8; 16] = [
    0,0,
    0,0,
    0,0,
    0,0,
    1,1,
    1,1,
    0,1,
    1,0
];

const SEMICOLON: [u8; 16] = [
    0,0,
    1,1,
    1,1,
    0,0,
    1,1,
    1,1,
    0,1,
    1,0
];

const COLON: [u8; 16] = [
    0,0,
    1,1,
    1,1,
    0,0,
    1,1,
    1,1,
    0,0,
    0,0
];

const DASH: [u8; 24] = [
    0,0,0,
    0,0,0,
    0,0,0,
    1,1,1,
    0,0,0,
    0,0,0,
    0,0,0,
    0,0,0
];

const UNDERSCORE: [u8; 40] = [
    0,0,0,0,0,
    0,0,0,0,0,
    0,0,0,0,0,
    0,0,0,0,0,
    0,0,0,0,0,
    0,0,0,0,0,
    0,0,0,0,0,
    1,1,1,1,1
];

const HASHTAG: [u8; 40] = [
    0,0,0,0,0,
    0,1,0,1,0,
    1,1,1,1,1,
    0,1,0,1,0,
    0,1,0,1,0,
    1,1,1,1,1,
    0,1,0,1,0,
    0,0,0,0,0
];

const EXCLAMATION: [u8; 8] = [
    1,
    1,
    1,
    1,
    1,
    0,
    1,
    0
];

const QUESTION: [u8; 40] = [
    1,1,1,1,0,
    0,0,0,0,1,
    0,0,0,0,1,
    0,0,1,1,0,
    0,1,0,0,0,
    0,0,0,0,0,
    0,1,0,0,0,
    0,0,0,0,0
];