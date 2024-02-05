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
                buffer[canvas_size.position_to_index(Position { x, y })] += self.color
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
    fn color_at(&self, pos: Position) -> Option<Color> {
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

                if let Some(color) = self.color_at(position) {
                    buffer[canvas_size.position_to_index(position)] += color
                }
            }
        }
    }
}

impl Drawable for Line {
    // TODO: Render width better, currently we only add width horizontally or vertically, but we
    //       should add it in the normal direction of the line.

    fn draw(&self, buffer: &mut [Color], canvas_size: &Size) {
        let dx = (self.start.x - self.end.x) as f64;
        let dy = (self.start.y - self.end.y) as f64;

        // We render in the longest direction to have a better resolution, since the amount of steps
        // is determined by one chosen axis.
        if f64::abs(dx) >= f64::abs(dy) {
            // The line equation is 'y = slope*x + intercept'
            let slope = dy / dx;
            let intercept = self.start.y as f64 - self.start.x as f64 * slope;

            let start = self.start.x.min(self.end.x);
            let end = self.start.x.max(self.end.x);

            for x in start..=end {
                // Center of the pixel x
                let cpx = x as f64 + 0.5;
                let y = slope * cpx + intercept;

                // The 'y', after flooring, is the main pixel that we're painting. The fraction
                // determines the opacity of the neighboor pixel for anti-aliasing
                let fract = y.fract();
                let y = y.floor() as i32;

                // We add some width in the 'y-axis', centered in the 'yth' pixel
                // FIXME: This algorithm adds an extra pixel. For example, if the user asks for
                //        width=2 it will run from (-1..=1) which are 3 pixels wide instead of 2.
                //        It was done like that to cleanly add the anti-aliasing pixel later due
                //        to the symmetry. Ideally, this interval should be (-width/2..width/2) or
                //        (width/2+1..width/2+1).
                for width_offset in -self.width / 2..=self.width / 2 {
                    buffer[canvas_size.position_to_index(Position {
                        x,
                        y: y + width_offset,
                    })] += self.color;
                }

                // An 'y' that landed in a '.5' means that it's dead center on the pixel, so all the
                // opacity should to go the main pixel, any difference in the fraction will go to
                // the AA pixel
                // TODO: The closest pixel to the AA-pixel should have the complementary opacity
                let aa_alpha_percentage = f64::abs(fract - 0.5);
                let offset_signal = if fract > 0.5 { 1 } else { -1 };
                let alpha = (self.color.a as f64 * aa_alpha_percentage) as u8;
                buffer[canvas_size.position_to_index(Position {
                    x,
                    y: y + offset_signal * (1 + self.width / 2),
                })] += self.color.with_alpha(alpha);
            }
        } else {
            // The line equation is 'x = slope*y + intercept'
            let slope = dx as f64 / dy as f64;
            let intercept = self.start.x as f64 - self.start.y as f64 * slope;

            let start = self.start.y.min(self.end.y);
            let end = self.start.y.max(self.end.y);

            for y in start..=end {
                // Center of the pixel x
                let cpy = y as f64 + 0.5;
                let x = slope * cpy + intercept;

                // The 'x', after flooring, is the main pixel that we're painting. The fraction
                // determines the opacity of the neighboor pixel for anti-aliasing
                let fract = x.fract();
                let x = x.floor() as i32;

                // We add some width in the 'x-axis', centered in the 'xth' pixel
                // FIXME: This algorithm adds an extra pixel. For example, if the user asks for
                //        width=2 it will run from (-1..=1) which are 3 pixels wide instead of 2.
                //        It was done like that to cleanly add the anti-aliasing pixel later due
                //        to the symmetry. Ideally, this interval should be (-width/2..width/2) or
                //        (width/2+1..width/2+1).
                for width_offset in -self.width / 2..=self.width / 2 {
                    buffer[canvas_size.position_to_index(Position {
                        x: x + width_offset,
                        y,
                    })] += self.color;
                }

                // An 'x' that landed in a '.5' means that it's dead center on the pixel, so all the
                // opacity should to go the main pixel, any difference in the fraction will go to
                // the AA pixel
                // TODO: The closest pixel to the AA-pixel should have the complementary opacity
                let aa_alpha_percentage = f64::abs(fract - 0.5);
                let offset_signal = if fract > 0.5 { 1 } else { -1 };
                let alpha = (self.color.a as f64 * aa_alpha_percentage) as u8;
                buffer[canvas_size.position_to_index(Position {
                    x: x + offset_signal * (1 + self.width / 2),
                    y,
                })] += self.color.with_alpha(alpha);
            }
        }
    }
}
