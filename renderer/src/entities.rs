use crate::{color::Color, Drawable, Position, Size};

// TODO: Add triangle
// TODO: Add text
// TODO: Deal with entities that fall completely or partially outside the canvas

pub struct Rectangle {
    pub center: Position,
    pub size: Size,
    pub color: Color,
}

pub struct Circle {
    pub center: Position,
    pub radius: i32,
    pub color: Color,
}

pub struct Line {
    pub start: Position,
    pub end: Position,
    pub color: Color,
    pub width: i32,
}

impl Drawable for Rectangle {
    fn draw(&self, buffer: &mut [Color], canvas_size: &Size) {
        for x in self.center.x - self.size.width / 2..=self.center.x + self.size.width / 2 {
            for y in self.center.y - self.size.height / 2..=self.center.y + self.size.height / 2 {
                let position = Position { x, y };

                buffer[canvas_size.position_to_index(&position)] += self.color
            }
        }
    }
}

impl Circle {
    /// Defines which color should go to the desired position.
    ///
    /// This algorithm performs anti-aliasing by upscaling the resolution and then calculating how
    /// many subpixels in each pixel are within bounds of the circle. The calculation is obscured by
    /// a float to integer transformation.
    ///
    /// This algorithm was heavily inspired by Tsoding's work in 'olive.c'
    /// <https://github.com/tsoding/olive.c/blob/master/olive.c#L524-L548>.
    /// See the video <https://www.youtube.com/watch?v=SoaXLQh3UQo> to understand how it was
    /// developed.
    fn color_at(&self, pos: &Position) -> Option<Color> {
        let aa = 2;
        let w = aa + 1;

        let mut subpixel_count = 0;

        for sx in 0..aa {
            for sy in 0..aa {
                // We cast everything to i64 to avoid overflowing
                let x = pos.x as i64;
                let y = pos.y as i64;
                let cx = self.center.x as i64;
                let cy = self.center.y as i64;
                let r = self.radius as i64;

                let dx = 2 * (w * (x - cx) + sx + 1) - w;
                let dy = 2 * (w * (y - cy) + sy + 1) - w;

                let sr = 2 * w * r;

                if dx * dx + dy * dy <= sr * sr {
                    subpixel_count += 1;
                }
            }
        }

        if subpixel_count == 0 {
            return None;
        }

        let alpha = (self.color.a as i64 * subpixel_count / (aa * aa)) as u8;

        Some(self.color.with_alpha(alpha))
    }
}

impl Drawable for Circle {
    fn draw(&self, buffer: &mut [Color], canvas_size: &Size) {
        // The loops reduce the "search area" to a square that inscribes the circle
        for x in self.center.x - self.radius..=self.center.x + self.radius {
            for y in self.center.y - self.radius..=self.center.y + self.radius {
                let position = Position { x, y };

                if let Some(color) = self.color_at(&position) {
                    buffer[canvas_size.position_to_index(&position)] += color
                }
            }
        }
    }
}

impl Drawable for Line {
    // TODO: Add Anti-aliasing
    // TODO: Render width better, currently we only add width horizontally or vertically, but we
    //       should add it in the normal direction of the line.
    fn draw(&self, buffer: &mut [Color], canvas_size: &Size) {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;

        // We render in the longest direction to have a better resolution, since the amount of steps
        // is determined by one chosen axis.
        if i32::abs(dx) >= i32::abs(dy) {
            // The line equation is 'y = slope*x + intercept'
            let slope = dy as f64 / dx as f64;
            let intercept = self.start.y as f64 - self.start.x as f64 * slope;

            let start = self.start.x.min(self.end.x);
            let end = self.start.x.max(self.end.x);

            for x in start..=end {
                let y = f64::round(slope * x as f64 + intercept) as i32;

                for offset in -self.width..=self.width {
                    let position = Position { x, y: y + offset };

                    buffer[canvas_size.position_to_index(&position)] = self.color
                }
            }
        } else {
            // The line equation is 'x = slope*y + intercept'
            let slope = dx as f64 / dy as f64;
            let intercept = self.start.x as f64 - self.start.y as f64 * slope;

            let start = self.start.y.min(self.end.y);
            let end = self.start.y.max(self.end.y);

            for y in start..=end {
                let x = f64::round(slope * y as f64 + intercept) as i32;

                for offset in -self.width..=self.width {
                    let position = Position { x: x + offset, y };

                    buffer[canvas_size.position_to_index(&position)] = self.color
                }
            }
        }
    }
}
