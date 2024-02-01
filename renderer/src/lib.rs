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
    fn draw(&self, buffer: &mut [Color], screen_size: &Size);

    fn color_at(&self, position: &Position) -> Option<Color>;
}

#[derive(PartialEq, Eq)]
pub struct Screen {
    size: Size,
    buffer: Vec<Color>,
    background_color: Color,
}

impl Screen {
    const DEFAULT_BACKGROUND_COLOR: Color = Color::BLACK;
    const DEFAULT_SCREEN_WIDTH: i32 = 640;
    const DEFAULT_SCREEN_HEIGHT: i32 = 480;

    pub fn with_size(size: Size) -> Self {
        let buffer_size = size.width * size.height;

        let mut screen = Screen {
            size,
            buffer: Vec::with_capacity(buffer_size as usize),
            background_color: Screen::DEFAULT_BACKGROUND_COLOR,
        };

        screen.resize(size.width, size.height);

        screen
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

impl Debug for Screen {
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

impl Default for Screen {
    fn default() -> Self {
        let buffer_size = Screen::DEFAULT_SCREEN_WIDTH * Screen::DEFAULT_SCREEN_HEIGHT;

        let mut screen = Screen {
            size: Size {
                width: Screen::DEFAULT_SCREEN_WIDTH,
                height: Screen::DEFAULT_SCREEN_HEIGHT,
            },
            buffer: Vec::with_capacity(buffer_size as usize),
            background_color: Screen::DEFAULT_BACKGROUND_COLOR,
        };

        screen.resize(Screen::DEFAULT_SCREEN_WIDTH, Screen::DEFAULT_SCREEN_HEIGHT);

        screen
    }
}

#[cfg(test)]
mod test {
    use crate::entities::{Circle, Rectangle};

    use super::*;

    macro_rules! buffer {
        (.) => {Color::BLACK};
        (W) => {Color::WHITE};
        ($($s:tt)+) => {
            [$(buffer!($s)),+]
        };
    }

    impl Screen {
        fn assert_equal_buffers(&self, expected_buffer: &[Color]) {
            assert_eq!(self.buffer.len(), expected_buffer.len());

            let expected_screen = Screen {
                size: self.size,
                buffer: expected_buffer.to_vec(),
                background_color: self.background_color,
            };

            assert_eq!(self, &expected_screen);
        }
    }

    #[test]
    fn draw_circle() {
        let mut screen = Screen::with_size(Size {
            width: 9,
            height: 9,
        });

        let circle = Circle {
            center: Position { x: 4, y: 4 },
            radius: 2,
            color: Color::WHITE,
        };

        screen.render(&[circle]);

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

        screen.assert_equal_buffers(&expected_buffer);
    }

    #[test]
    fn draw_rectangle() {
        let mut screen = Screen::with_size(Size {
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

        screen.render(&[rectangle]);

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

        screen.assert_equal_buffers(&expected_buffer);
    }

    #[test]
    fn draw_two_objects() {
        let mut screen = Screen::with_size(Size {
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

        screen.render(&[rectangle]);
        screen.render(&[circle]);

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

        screen.assert_equal_buffers(&expected_buffer);
    }

    #[test]
    pub fn clear_buffer() {
        let mut screen = Screen::with_size(Size {
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

        screen.render(&[rectangle]);
        screen.clear_buffer();

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
        let expected_screen = Screen {
            size: screen.size,
            buffer: expected_buffer.to_vec(),
            background_color: screen.background_color,
        };

        assert_eq!(screen, expected_screen)
    }
}
