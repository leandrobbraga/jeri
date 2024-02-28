mod text;

use crate::{color::Color, Drawable, Position, Size};
pub use text::Text;

pub struct Rectangle {
    pub center: Position<i32>,
    pub size: Size,
    pub color: Color,
}

pub struct Circle {
    pub center: Position<i32>,
    pub radius: i32,
    pub color: Color,
}

pub struct Line {
    pub start: Position<i32>,
    pub end: Position<i32>,
    pub color: Color,
    pub width: i32,
}

pub struct Triangle {
    pub p1: Position<i32>,
    pub p2: Position<i32>,
    pub p3: Position<i32>,
    pub color: Color,
}

impl Drawable for Rectangle {
    #[inline(always)]
    fn color_at(&self, position: Position<i32>) -> Option<Color> {
        if !((position.x >= self.center.x - self.size.width / 2)
            & (position.x <= self.center.x + self.size.width / 2)
            & (position.y >= self.center.y - self.size.width / 2)
            & (position.y <= self.center.y + self.size.width / 2))
        {
            return None;
        }

        Some(self.color)
    }
}

impl Drawable for Circle {
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
    #[inline(always)]
    fn color_at(&self, position: Position<i32>) -> Option<Color> {
        // Fast path, this defines the square that inscribes the circle
        if !((position.x >= self.center.x - self.radius)
            & (position.x <= self.center.x + self.radius)
            & (position.y >= self.center.y - self.radius)
            & (position.y <= self.center.y + self.radius))
        {
            return None;
        }

        let aa = 2;
        let w = aa + 1;

        let mut subpixel_count = 0;

        for sx in 0..aa {
            for sy in 0..aa {
                // We cast everything to i64 to avoid overflowing
                let x = position.x as i64;
                let y = position.y as i64;
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

impl Drawable for Line {
    // TODO: Render width better, currently we only add width horizontally or vertically, but we
    //       should add it in the normal direction of the line.

    #[inline(always)]
    fn color_at(&self, position: Position<i32>) -> Option<Color> {
        // Fast path
        let x_start = self.start.x.min(self.end.x);
        let x_end = self.start.x.max(self.end.x);
        let y_start = self.start.y.min(self.end.y);
        let y_end = self.start.y.max(self.end.y);
        // We need to include the extra width
        if !((position.x as f64 >= x_start as f64 - self.width as f64 / 2.)
            & (position.x as f64 <= x_end as f64 + self.width as f64 / 2.)
            & (position.y as f64 >= y_start as f64 - self.width as f64 / 2.)
            & (position.y as f64 <= y_end as f64 + self.width as f64 / 2.))
        {
            return None;
        }

        let dx = (self.start.x - self.end.x) as f64;
        let dy = (self.start.y - self.end.y) as f64;

        let px = position.x as f64;
        let py = position.y as f64;

        // We render in the longest direction to have a better resolution, since the amount of steps
        // is determined by one chosen axis.
        let distance = if f64::abs(dx) >= f64::abs(dy) {
            // Fast path, we only need the extra pixels for the extra width, which does not happen
            // in the longer direction
            if (px < x_start as f64) | (px > x_end as f64) {
                return None;
            }

            // The line equation is 'y = slope*x + intercept'
            let slope = dy / dx;
            let intercept = self.start.y as f64 - self.start.x as f64 * slope;

            let y = slope * px + intercept;

            f64::abs(y - py)
        } else {
            // Fast path, we only need the extra pixels for the extra width, which does not happen
            // in the longer direction
            if (py < y_start as f64) | (py > y_end as f64) {
                return None;
            }

            // The line equation is 'x = slope*y + intercept'
            let slope = dx as f64 / dy as f64;
            let intercept = self.start.x as f64 - self.start.y as f64 * slope;

            let x = slope * py + intercept;

            f64::abs(x - px)
        };

        if distance > (self.width as f64) / 2. + 0.5 {
            return None;
        }

        // An 'y' that landed in a '.5' means that it's dead center on the pixel, so all the
        // opacity should to go the main pixel, any difference in the fraction will go to
        // the AA pixel
        let aa_alpha_percentage = f64::min(1.0, self.width as f64 / 2. - distance + 0.5);
        let alpha = (self.color.a as f64 * aa_alpha_percentage) as u8;

        Some(self.color.with_alpha(alpha))
    }
}

impl Drawable for Triangle {
    // NOTE: This technique obscures the calculation by upscaling the numbers to transform floats
    //       into integers.
    #[inline(always)]
    fn color_at(&self, position: Position<i32>) -> Option<Color> {
        // Fast path
        let x_start = self.p1.x.min(self.p2.x).min(self.p3.x);
        let x_end = self.p1.x.max(self.p2.x).max(self.p3.x);
        let y_start = self.p1.y.min(self.p2.y).min(self.p3.y);
        let y_end = self.p1.y.max(self.p2.y).max(self.p3.y);
        if !((position.x >= x_start)
            & (position.x <= x_end)
            & (position.y >= y_start)
            & (position.y <= y_end))
        {
            return None;
        }

        let aa = 2;
        let w = aa + 1;

        let mut subpixel_count = 0;

        for sxo in 1..=aa {
            for syo in 1..=aa {
                let p = Position {
                    x: 2 * (w * position.x as i64 + sxo),
                    y: 2 * (w * position.y as i64 + syo),
                };

                if self.is_point_inside(p, w) {
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

impl Triangle {
    /// Check if a given point is inside the triangle.
    /// Reference: <https://math.stackexchange.com/a/51328>
    #[inline(always)]
    fn is_point_inside(&self, p: Position<i64>, w: i64) -> bool {
        let p1 = Position {
            x: w * (2 * self.p1.x as i64 + 1),
            y: w * (2 * self.p1.y as i64 + 1),
        };
        let p2 = Position {
            x: w * (2 * self.p2.x as i64 + 1),
            y: w * (2 * self.p2.y as i64 + 1),
        };
        let p3 = Position {
            x: w * (2 * self.p3.x as i64 + 1),
            y: w * (2 * self.p3.y as i64 + 1),
        };
        let ab = p1 - p2;
        let ap = p1 - p;

        let bc = p2 - p3;
        let bp = p2 - p;

        let ca = p3 - p1;
        let cp = p3 - p;

        let abp = Triangle::cross_product_sign(ab, ap);
        let bcp = Triangle::cross_product_sign(bc, bp);
        let cap = Triangle::cross_product_sign(ca, cp);

        ((abp >= 0) & (bcp >= 0) & (cap >= 0)) | ((abp <= 0) & (bcp <= 0) & (cap <= 0))
    }

    /// Since this only work with 2d plane, we only need the third component of the cross product
    #[inline(always)]
    fn cross_product_sign(p1: Position<i64>, p2: Position<i64>) -> i64 {
        (p1.x * p2.y) - (p2.x * p1.y)
    }
}
