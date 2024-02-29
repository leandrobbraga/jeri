use std::fmt::Debug;
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const CHANNELS: usize = 4;

    pub const BLACK: Color = Color::from_rgba_u32(0x000000FF);
    pub const WHITE: Color = Color::from_rgba_u32(0xFFFFFFFF);
    pub const GRAY: Color = Color::from_rgba_u32(0x181818FF);
    pub const RED: Color = Color::from_rgba_u32(0xFF0000FF);
    pub const GREEN: Color = Color::from_rgba_u32(0x00FF00FF);
    pub const BLUE: Color = Color::from_rgba_u32(0x0000FFFF);
    pub const LIGHT_PINK: Color = Color::from_rgba_u32(0xFFC0CBFF);
    pub const BRIGHT_PINK: Color = Color::from_rgba_u32(0xFC0FC0FF);

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub const fn from_rgba_array(rgba: [u8; 4]) -> Self {
        Color {
            r: rgba[0],
            g: rgba[1],
            b: rgba[2],
            a: rgba[3],
        }
    }

    pub const fn to_rgba_array(&self) -> [u8; Self::CHANNELS] {
        [self.r, self.g, self.b, self.a]
    }

    #[allow(clippy::identity_op)]
    pub const fn from_rgba_u32(rgba: u32) -> Self {
        Color {
            r: (rgba >> 24) as u8,
            g: (rgba >> 16) as u8,
            b: (rgba >> 8) as u8,
            a: (rgba >> 0) as u8,
        }
    }

    #[allow(clippy::identity_op)]
    pub const fn to_u32(&self) -> u32 {
        ((self.r as u32) << 24)
            | ((self.g as u32) << 16)
            | ((self.b as u32) << 8)
            | ((self.a as u32) << 0)
    }

    pub const fn with_alpha(&self, alpha: u8) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }
}

type ColorBytes = [u8; Color::CHANNELS];

impl AddAssign<Color> for &mut ColorBytes {
    /// Blends the two color together scaled by the right color alpha channel
    #[inline(always)]
    fn add_assign(&mut self, rhs: Color) {
        let r = (self[0] as u32 * (255 - rhs.a as u32) + rhs.r as u32 * rhs.a as u32) / 255;
        let g = (self[1] as u32 * (255 - rhs.a as u32) + rhs.g as u32 * rhs.a as u32) / 255;
        let b = (self[2] as u32 * (255 - rhs.a as u32) + rhs.b as u32 * rhs.a as u32) / 255;
        let a = u8::max(self[3], rhs.a);

        self[0] = r as u8;
        self[1] = g as u8;
        self[2] = b as u8;
        self[3] = a;
    }
}

impl Add<Color> for Color {
    type Output = Color;

    #[inline(always)]
    fn add(self, rhs: Color) -> Self::Output {
        let r = (self.r as u32 * (255 - rhs.a as u32) + rhs.r as u32 * rhs.a as u32) / 255;
        let g = (self.g as u32 * (255 - rhs.a as u32) + rhs.g as u32 * rhs.a as u32) / 255;
        let b = (self.b as u32 * (255 - rhs.a as u32) + rhs.b as u32 * rhs.a as u32) / 255;
        let a = u8::max(self.a, rhs.a);

        Color {
            r: r as u8,
            g: g as u8,
            b: b as u8,
            a,
        }
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " {:10} ", self.to_u32())
    }
}
