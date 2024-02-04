pub mod color;
pub mod entities;

use std::{fmt::Debug, ops::Mul};

use crate::color::Color;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    pub fn new(height: i32, width: i32) -> Size {
        Size { width, height }
    }
    fn position_to_index(&self, position: &Position) -> usize {
        (position.y * self.width + position.x) as usize
    }
}

impl Mul<i32> for Size {
    type Output = Size;

    fn mul(self, rhs: i32) -> Self::Output {
        Size {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

pub trait Drawable {
    fn draw(&self, buffer: &mut [Color], canvas_size: &Size);

    fn color_at(&self, position: &Position) -> Option<Color>;
}

#[derive(PartialEq, Eq)]
pub struct Canvas {
    size: Size,
    buffer: Vec<Color>,
    pub background_color: Color,
}

impl Canvas {
    const DEFAULT_BACKGROUND_COLOR: Color = Color::BLACK;
    const DEFAULT_CANVAS_WIDTH: i32 = 640;
    const DEFAULT_CANVAS_HEIGHT: i32 = 480;

    pub fn with_size(size: Size) -> Self {
        let buffer_size = size.width * size.height;

        let mut canvas = Canvas {
            size,
            buffer: Vec::with_capacity(buffer_size as usize),
            background_color: Canvas::DEFAULT_BACKGROUND_COLOR,
        };

        canvas.resize(size.width, size.height);

        canvas
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.buffer
            .resize((width * height) as usize, self.background_color)
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.fill(self.background_color);
    }

    pub fn render(&mut self, objects: &[impl Drawable]) {
        for object in objects {
            object.draw(self.buffer.as_mut_slice(), &self.size)
        }
    }

    pub fn size_ref(&self) -> &Size {
        &self.size
    }

    pub fn buffer_ref(&self) -> &[Color] {
        &self.buffer
    }
}

impl Debug for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chunk in self.buffer.chunks_exact(self.size.width as usize) {
            for color in chunk {
                write!(f, "{:?}", color)?
            }
            writeln!(f)?
        }

        Ok(())
    }
}

impl Default for Canvas {
    fn default() -> Self {
        let buffer_size = Canvas::DEFAULT_CANVAS_WIDTH * Canvas::DEFAULT_CANVAS_HEIGHT;

        let mut canvas = Canvas {
            size: Size {
                width: Canvas::DEFAULT_CANVAS_WIDTH,
                height: Canvas::DEFAULT_CANVAS_HEIGHT,
            },
            buffer: Vec::with_capacity(buffer_size as usize),
            background_color: Canvas::DEFAULT_BACKGROUND_COLOR,
        };

        canvas.resize(Canvas::DEFAULT_CANVAS_WIDTH, Canvas::DEFAULT_CANVAS_HEIGHT);

        canvas
    }
}
