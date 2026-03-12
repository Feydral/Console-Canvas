#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    pub fn from_tuple(color: (u8, u8, u8)) -> Self {
        Self {
            r: color.0,
            g: color.1,
            b: color.2,
        }
    }

    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255 };
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };
    pub const YELLOW: Self = Self { r: 255, g: 255, b: 0 };
    pub const CYAN: Self = Self { r: 0, g: 255, b: 255 };
    pub const MAGENTA: Self = Self { r: 255, g: 0, b: 255 };
    pub const GRAY: Self = Self { r: 128, g: 128, b: 128 };
    pub const LIGHT_RED: Self = Self { r: 255, g: 100, b: 100 };
    pub const LIGHT_GREEN: Self = Self { r: 100, g: 255, b: 100 };
    pub const LIGHT_BLUE: Self = Self { r: 100, g: 100, b: 255 };
}