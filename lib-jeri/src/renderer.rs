use crate::{color::Color, Position, Size};

#[derive(PartialEq, Eq)]
pub struct Screen {
    size: Size,
    buffer: Vec<Color>,
    background_color: Color,
}

impl std::fmt::Debug for Screen {
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

pub trait Drawable {
    fn draw(&self, buffer: &mut [Color], screen_size: &Size);
}

fn position_to_index(position: &Position, screen_size: &Size) -> usize {
    (position.y * screen_size.width + position.x) as usize
}

pub struct Rectangle {
    center: Position,
    size: Size,
    color: Color,
}

pub struct Circle {
    center: Position,
    radius: i32,
    color: Color,
}

impl Circle {
    fn within_bound(&self, position: &Position) -> bool {
        (position.x - self.center.x) * (position.x - self.center.x)
            + (position.y - self.center.y) * (position.y - self.center.y)
            <= self.radius * self.radius
    }
}

impl Drawable for Rectangle {
    fn draw(&self, buffer: &mut [Color], screen_size: &Size) {
        for x in self.center.x - self.size.width / 2..=self.center.x + self.size.width / 2 {
            for y in self.center.y - self.size.height / 2..=self.center.y + self.size.height / 2 {
                buffer[position_to_index(&Position { x, y }, &screen_size)] = self.color
            }
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, buffer: &mut [Color], screen_size: &Size) {
        // The loops reduce the "search area" to a square that inscribes the circle
        for x in self.center.x - self.radius..=self.center.x + self.radius {
            for y in self.center.y - self.radius..=self.center.y + self.radius {
                let position = Position { x, y };

                if self.within_bound(&position) {
                    buffer[position_to_index(&position, &screen_size)] = self.color
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
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
