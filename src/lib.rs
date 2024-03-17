pub mod color;
pub mod entities;
mod threadpool;

use std::fmt::Debug;
use std::ops::{Mul, Sub};

use threadpool::ThreadPool;

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

    pub fn pixel_count(&self) -> i32 {
        self.width * self.height
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
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
where
    T: Mul<T, Output = T> + Sub<T, Output = T> + Copy,
{
    #[inline(always)]
    fn cross_product_sign(p1: Vector2<T>, p2: Vector2<T>) -> T {
        (p1.x * p2.y) - (p2.x * p1.y)
    }
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }
}

impl<T> Sub<Vector2<T>> for Vector2<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Vector2<T>;

    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub trait Drawable: Send + Sync {
    fn color_at(&self, position: Vector2<i32>) -> Option<Color>;
}

pub struct Canvas {
    size: Size,
    buffer: Vec<u8>,
    pub background_color: Color,
    threadpool: ThreadPool,
}

impl PartialEq for Canvas {
    fn eq(&self, other: &Self) -> bool {
        (self.size == other.size)
            & (self.buffer == other.buffer)
            & (self.background_color == other.background_color)
    }
}

impl Eq for Canvas {}

impl Canvas {
    const DEFAULT_BACKGROUND_COLOR: Color = Color::BLACK;
    const DEFAULT_CANVAS_WIDTH: i32 = 640;
    const DEFAULT_CANVAS_HEIGHT: i32 = 480;

    pub fn with_size(size: Size) -> Self {
        let buffer_size = Color::CHANNELS as i32 * (size.width * size.height);
        let threadpool = ThreadPool::default();

        let mut canvas = Canvas {
            size,
            buffer: Vec::with_capacity(buffer_size as usize),
            background_color: Canvas::DEFAULT_BACKGROUND_COLOR,
            threadpool,
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
            chunk.swap_with_slice(color.to_rgba_array().as_mut_slice())
        }
    }
    pub fn clear_buffer(&mut self) {
        self.fill_buffer(self.background_color)
    }

    pub fn render(&mut self, entities: &[impl Drawable]) {
        let pixels_per_chunk = self.size.pixel_count() as usize
            / (std::thread::available_parallelism().unwrap().get() * 5);
        let canvas_width = self.size.width;

        // Ensure that we have multiple of 64 bytes since it's the size of a cache line to avoid
        // false sharing. See: https://en.wikipedia.org/wiki/False_sharing
        let bytes_per_chunk = usize::max(1, 64 * (Color::CHANNELS * pixels_per_chunk) / 64);

        self.threadpool.with_scope(|scope| {
            for (chunk_index, pixel_chunk) in self.buffer.chunks_mut(bytes_per_chunk).enumerate() {
                scope.enqueue_task(move || {
                    // Each thread will process the whole chunk, pixel by pixel.
                    for (pixel_index, pixel) in
                        pixel_chunk.chunks_exact_mut(Color::CHANNELS).enumerate()
                    {
                        let mut pixel: &mut [u8; Color::CHANNELS] = pixel.try_into().unwrap();
                        let position_index = (chunk_index * pixels_per_chunk) + pixel_index;

                        let position = Vector2 {
                            x: position_index as i32 % canvas_width,
                            y: position_index as i32 / canvas_width,
                        };

                        for entity in entities {
                            if let Some(color) = entity.color_at(position) {
                                pixel += color;
                            }
                        }
                    }
                })
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
        let threadpool = ThreadPool::default();

        let mut canvas = Canvas {
            size: Size {
                width: Canvas::DEFAULT_CANVAS_WIDTH,
                height: Canvas::DEFAULT_CANVAS_HEIGHT,
            },
            buffer: Vec::with_capacity(buffer_size as usize),
            background_color: Canvas::DEFAULT_BACKGROUND_COLOR,
            threadpool,
        };

        canvas.resize(Canvas::DEFAULT_CANVAS_WIDTH, Canvas::DEFAULT_CANVAS_HEIGHT);

        canvas
    }
}
