pub mod color;
pub mod entities;

use std::fmt::Debug;
use std::ops::{Mul, Sub};

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

#[derive(Debug, Clone, Copy)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

impl<T> Position<T> {
    pub fn new(x: T, y: T) -> Position<T> {
        Position { x, y }
    }
}

impl<T> Sub<Position<T>> for Position<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Position<T>;

    fn sub(self, rhs: Position<T>) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub trait Drawable: Send + Sync {
    fn color_at(&self, position: Position<i32>) -> Option<Color>;
}

#[derive(PartialEq, Eq)]
pub struct Canvas {
    size: Size,
    buffer: Vec<u8>,
    pub background_color: Color,
}

impl Canvas {
    const DEFAULT_BACKGROUND_COLOR: Color = Color::BLACK;
    const DEFAULT_CANVAS_WIDTH: i32 = 640;
    const DEFAULT_CANVAS_HEIGHT: i32 = 480;

    pub fn with_size(size: Size) -> Self {
        let buffer_size = Color::CHANNELS as i32 * (size.width * size.height);

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
            .resize(Color::CHANNELS * (width * height) as usize, 0);
        self.clear_buffer();
    }

    pub fn fill_buffer(&mut self, color: Color) {
        for chunk in self.buffer.chunks_exact_mut(Color::CHANNELS) {
            chunk.swap_with_slice(color.to_rgba_slice().as_mut_slice())
        }
    }
    pub fn clear_buffer(&mut self) {
        self.fill_buffer(self.background_color)
    }

    pub fn render(&mut self, entities: &[impl Drawable]) {
        // TODO: Use a ThreadPool instead of spawning new threads everytime the user wants to render
        //       something.
        let available_paralellism = std::thread::available_parallelism().unwrap().get();

        // The last thread will potentially receive less work to do if the number of pixels is not
        // divisible by the number of threads.
        // FIXME: The threads will be better utilized with the ThreadPool and smaller chunks of work
        let pixels_per_chunk =
            f64::ceil((self.size.width * self.size.height) as f64 / available_paralellism as f64)
                as usize;

        let canvas_width = self.size.width;

        std::thread::scope(|s| {
            // Each thread will have a screen section assigned to it, the canvas section size is
            // determined from the number of pixels and the available parallelism on the system.
            for (chunk_index, pixel_chunk) in self
                .buffer
                .chunks_mut(Color::CHANNELS * pixels_per_chunk)
                .enumerate()
            {
                // Each thread will process the whole chunk, pixel by pixel.
                s.spawn(move || {
                    for (pixel_index, pixel) in
                        pixel_chunk.chunks_exact_mut(Color::CHANNELS).enumerate()
                    {
                        let mut pixel: &mut [u8; Color::CHANNELS] = pixel.try_into().unwrap();
                        let position_index = (chunk_index * pixels_per_chunk) + pixel_index;

                        let position = Position {
                            x: position_index as i32 % canvas_width,
                            y: position_index as i32 / canvas_width,
                        };

                        for entity in entities {
                            if let Some(color) = entity.color_at(position) {
                                pixel += color;
                            }
                        }
                    }
                });
            }
        });
    }

    pub fn size_ref(&self) -> &Size {
        &self.size
    }

    pub fn buffer_ref(&self) -> &[u8] {
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
