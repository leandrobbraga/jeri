#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
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
}
