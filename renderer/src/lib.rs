pub mod color;
pub mod entities;

use std::{fmt::Debug, ops::Mul};

use crate::color::Color;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Size {
    width: i32,
    height: i32,
}

impl Size {
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

pub struct Position {
    x: i32,
    y: i32,
}

pub trait Drawable {
    fn draw(&self, buffer: &mut [Color], canvas_size: &Size);

    fn color_at(&self, position: &Position) -> Option<Color>;
}

#[derive(PartialEq, Eq)]
pub struct Canvas {
    size: Size,
    buffer: Vec<Color>,
    background_color: Color,
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

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color
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

#[cfg(test)]
mod test {
    use crate::entities::{Circle, Line, Rectangle};

    use super::*;

    // TODO: Evolve the tests from macros to expectations.
    //
    //       With expectations, everytime the developer run the tests, the program generates a
    //       file which is compared to the previously approved expectation. If the difference is
    //       desirable the developer can approve it as the new expectation, otherwise they can keep
    //       working to meet the expectation.
    //
    //       This way is much easier to evolve the renderer, specially subtle things like
    //       anti-aliasing
    macro_rules! buffer {
        (.) => {Color::BLACK};
        (W) => {Color::WHITE};
        ($($s:tt)+) => {
            [$(buffer!($s)),+]
        };
    }

    impl Canvas {
        fn assert_equal_buffers(&self, expected_buffer: &[Color]) {
            assert_eq!(self.buffer.len(), expected_buffer.len());

            let expected_canvas = Canvas {
                size: self.size,
                buffer: expected_buffer.to_vec(),
                background_color: self.background_color,
            };

            assert_eq!(self, &expected_canvas);
        }
    }

    #[test]
    fn draw_circle() {
        let mut canvas = Canvas::with_size(Size {
            width: 9,
            height: 9,
        });

        let circle = Circle {
            center: Position { x: 4, y: 4 },
            radius: 2,
            color: Color::WHITE,
        };

        canvas.render(&[circle]);

        let expected_buffer = buffer![
            . . . . . . . . .
            . . . . . . . . .
            . . . . W . . . .
            . . . W W W . . .
            . . W W W W W . .
            . . . W W W . . .
            . . . . W . . . .
            . . . . . . . . .
            . . . . . . . . .
        ];

        canvas.assert_equal_buffers(&expected_buffer);
    }

    #[test]
    fn draw_line() {
        let mut canvas = Canvas::with_size(Size {
            width: 9,
            height: 9,
        });

        let circle = Line {
            end: Position { x: 5, y: 2 },
            start: Position { x: 1, y: 5 },
            color: Color::WHITE,
        };

        canvas.render(&[circle]);

        let expected_buffer = buffer![
            . . . . . . . . .
            . . . . . . . . .
            . . . . . W . . .
            . . . . W . . . .
            . . W W . . . . .
            . W . . . . . . .
            . . . . . . . . .
            . . . . . . . . .
            . . . . . . . . .
        ];

        canvas.assert_equal_buffers(&expected_buffer);
    }

    #[test]
    fn draw_rectangle() {
        let mut canvas = Canvas::with_size(Size {
            width: 9,
            height: 9,
        });

        let rectangle = Rectangle {
            center: Position { x: 4, y: 4 },
            size: Size {
                width: 3,
                height: 3,
            },
            color: Color::WHITE,
        };

        canvas.render(&[rectangle]);

        let expected_buffer = buffer![
            . . . . . . . . .
            . . . . . . . . .
            . . . . . . . . .
            . . . W W W . . .
            . . . W W W . . .
            . . . W W W . . .
            . . . . . . . . .
            . . . . . . . . .
            . . . . . . . . .
        ];

        canvas.assert_equal_buffers(&expected_buffer);
    }

    #[test]
    fn draw_two_objects() {
        let mut canvas = Canvas::with_size(Size {
            width: 15,
            height: 15,
        });

        let rectangle = Rectangle {
            center: Position { x: 4, y: 4 },
            size: Size {
                width: 3,
                height: 3,
            },
            color: Color::WHITE,
        };

        let circle = Circle {
            center: Position { x: 7, y: 7 },
            radius: 3,
            color: Color::WHITE,
        };

        canvas.render(&[rectangle]);
        canvas.render(&[circle]);

        let expected_buffer = buffer![
            . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . .
            . . . W W W . . . . . . . . .
            . . . W W W . W . . . . . . .
            . . . W W W W W W W . . . . .
            . . . . . W W W W W . . . . .
            . . . . W W W W W W W . . . .
            . . . . . W W W W W . . . . .
            . . . . . W W W W W . . . . .
            . . . . . . . W . . . . . . .
            . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . .
            . . . . . . . . . . . . . . .
        ];

        canvas.assert_equal_buffers(&expected_buffer);
    }

    #[test]
    pub fn clear_buffer() {
        let mut canvas = Canvas::with_size(Size {
            width: 10,
            height: 10,
        });

        let rectangle = Rectangle {
            center: Position { x: 4, y: 4 },
            size: Size {
                width: 3,
                height: 3,
            },
            color: Color::WHITE,
        };

        canvas.render(&[rectangle]);
        canvas.clear_buffer();

        let expected_buffer = buffer![
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
            . . . . . . . . . .
        ];

        canvas.assert_equal_buffers(&expected_buffer);
    }
}
