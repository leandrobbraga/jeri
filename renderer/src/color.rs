use std::{cmp::min, fmt::Debug, ops::AddAssign};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color::from_rgba_u32(0x000000FF);
    pub const WHITE: Color = Color::from_rgba_u32(0xFFFFFFFF);
    pub const GRAY: Color = Color::from_rgba_u32(0x181818FF);
    pub const RED: Color = Color::from_rgba_u32(0xFF0000FF);
    pub const GREEN: Color = Color::from_rgba_u32(0x00FF00FF);
    pub const BLUE: Color = Color::from_rgba_u32(0x0000FFFF);

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub const fn from_rgba_slice(rgba: [u8; 4]) -> Self {
        Color {
            r: rgba[0],
            g: rgba[1],
            b: rgba[2],
            a: rgba[3],
        }
    }

    pub const fn from_rgba_u32(rgba: u32) -> Self {
        Color {
            r: ((rgba >> 24) & 0xff) as u8,
            g: ((rgba >> 16) & 0xff) as u8,
            b: ((rgba >> 8) & 0xff) as u8,
            a: ((rgba >> 0) & 0xff) as u8,
        }
    }

    pub const fn to_u32(&self) -> u32 {
        ((self.r as u32) << 24)
            | ((self.g as u32) << 16 as u32)
            | ((self.b as u32) << 8 as u32)
            | ((self.a as u32) << 0 as u32)
    }

    pub fn with_alpha(&self, alpha: u8) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }
}

impl AddAssign<Color> for Color {
    /// Blends the two color together scaled by the right color alpha channel
    fn add_assign(&mut self, rhs: Color) {
        let r = (self.r as u32 * (255 - rhs.a as u32) + rhs.r as u32 * rhs.a as u32) / 255;
        let g = (self.g as u32 * (255 - rhs.a as u32) + rhs.g as u32 * rhs.a as u32) / 255;
        let b = (self.b as u32 * (255 - rhs.a as u32) + rhs.b as u32 * rhs.a as u32) / 255;

        self.r = min(255, r) as u8;
        self.g = min(255, g) as u8;
        self.b = min(255, b) as u8;
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " {:10} ", self.to_u32())
    }
}
