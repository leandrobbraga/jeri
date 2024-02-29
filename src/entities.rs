mod text;

use crate::{color::Color, Drawable, Position, Size};
pub use text::Text;

const AA_FACTOR: i32 = 2;

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
        if !((2 * position.x >= 2 * self.center.x - self.size.width)
            & (2 * position.x <= 2 * self.center.x + self.size.width)
            & (2 * position.y >= 2 * self.center.y - self.size.width)
            & (2 * position.y <= 2 * self.center.y + self.size.width))
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
        // FIXME: We could assert this at creation / radius change
        assert!(
            (AA_FACTOR + 1) * self.radius < 16384,
            "The current Circle implementation as a radius limit to avoid integer overflow"
        );

        // Fast path, this defines the square that inscribes the circle
        if !((position.x >= self.center.x - self.radius)
            & (position.x <= self.center.x + self.radius)
            & (position.y >= self.center.y - self.radius)
            & (position.y <= self.center.y + self.radius))
        {
            return None;
        }

        let w = AA_FACTOR + 1;

        let mut subpixel_count = 0;

        for sx in 0..AA_FACTOR {
            for sy in 0..AA_FACTOR {
                let dx = 2 * (w * (position.x - self.center.x) + sx + 1) - w;
                let dy = 2 * (w * (position.y - self.center.y) + sy + 1) - w;

                let sr = 2 * w * self.radius;

                // WARNING: To avoid overflowing the 'i32' values we need to keep both 'dx*dx +
                // dy*dy' and 'sr*sr' below the value of i32::MAX, which is 2_147_483_647. Since
                // we're only searching in the space of a square inscribing the circle, the worst
                // case scenario is in the corners of the square, where we have max 'dx' and 'dy'
                // which is equal to 'sr'.
                //
                // Meaning that 'sr*sr + sr*sr' cannot surpass 2_147_483_647, 'sr' cannot surpass
                // '32_767' and the radius cannot surpass '16384 / (aa+1)'. For example, with an
                // 'aa' of 2 we have a max radius of '5461'.
                if dx * dx + dy * dy <= sr * sr {
                    subpixel_count += 1;
                }
            }
        }

        if subpixel_count == 0 {
            return None;
        }

        let alpha = (self.color.a as i32 * subpixel_count / (AA_FACTOR * AA_FACTOR)) as u8;

        Some(self.color.with_alpha(alpha))
    }
}

impl Drawable for Line {
    // TODO: Render width better, currently we only add width horizontally or vertically, but we
    //       should add it in the normal direction of the line.

    #[inline(always)]
    fn color_at(&self, position: Position<i32>) -> Option<Color> {
        // Fast path
        // FIXME: We could cache this
        let x_start = self.start.x.min(self.end.x);
        let x_end = self.start.x.max(self.end.x);
        let y_start = self.start.y.min(self.end.y);
        let y_end = self.start.y.max(self.end.y);

        // We need to include the extra width
        if !((2 * position.x >= 2 * x_start - self.width)
            & (2 * position.x <= 2 * x_end + self.width)
            & (2 * position.y >= 2 * y_start - self.width)
            & (2 * position.y <= 2 * y_end + self.width))
        {
            return None;
        }

        // FIXME: We could cache this
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;

        let px = position.x;
        let py = position.y;

        // We render in the longest direction to have a better resolution, since the amount of steps
        // is determined by one chosen axis.
        if i32::abs(dx) >= i32::abs(dy) {
            // Fast path, we only need the extra pixels for the extra width, which does not happen
            // in the longer direction
            if (px < x_start) | (px > x_end) {
                return None;
            }

            // The line equation is 'y = slope*x + intercept'
            let y = px * dy + dx * self.start.y - dy * self.start.x;

            if 2 * i32::abs(y - py * dx) > i32::abs(dx) * (self.width + 1) {
                return None;
            }
        } else {
            // Fast path, we only need the extra pixels for the extra width, which does not happen
            // in the longer direction
            if (py < y_start) | (py > y_end) {
                return None;
            }

            // The line equation is 'x = slope*y + intercept'
            let x = py * dx + dy * self.start.x - dx * self.start.y;

            if 2 * i32::abs(x - px * dy) > i32::abs(dy) * (self.width + 1) {
                return None;
            }
        };

        // FIXME: Implement AA
        Some(self.color)
    }
}

impl Drawable for Triangle {
    // NOTE: This technique obscures the calculation by upscaling the numbers to transform floats
    //       into integers.
    #[inline(always)]
    fn color_at(&self, position: Position<i32>) -> Option<Color> {
        // Fast path
        // FIXME: We could cache this
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

        let w = AA_FACTOR + 1;

        // 1. Since we're dealing with sub-pixels we also need to use the center of the pixel
        // 'p.x + 0.5' and 'p.y + 0.5'.
        // 2. To keep it as an integer, we multiply it by the scaling factor '2*w'
        // FIXME: We could cache this
        let p1 = Position {
            x: w * (2 * self.p1.x + 1),
            y: w * (2 * self.p1.y + 1),
        };
        let p2 = Position {
            x: w * (2 * self.p2.x + 1),
            y: w * (2 * self.p2.y + 1),
        };
        let p3 = Position {
            x: w * (2 * self.p3.x + 1),
            y: w * (2 * self.p3.y + 1),
        };

        let mut subpixel_count = 0;

        for sxo in 1..=AA_FACTOR {
            for syo in 1..=AA_FACTOR {
                // 1. We divide a pixel in 'aa + 1' parts, meaning that each sub-pixel is
                //    'p.x + i/(aa+1)' and 'p.y + i/(aa+1)'
                // 2. To keep it as an integer we multiply by the scaling factor '2*w'
                let p = Position {
                    x: 2 * (w * position.x + sxo),
                    y: 2 * (w * position.y + syo),
                };

                if self.is_point_inside(p, p1, p2, p3) {
                    subpixel_count += 1;
                }
            }
        }

        if subpixel_count == 0 {
            return None;
        }

        let alpha = (self.color.a as i32 * subpixel_count / (AA_FACTOR * AA_FACTOR)) as u8;

        Some(self.color.with_alpha(alpha))
    }
}

impl Triangle {
    /// Check if a given point is inside the triangle.
    /// Reference: <https://math.stackexchange.com/a/51328>
    #[inline(always)]
    fn is_point_inside(
        &self,
        p: Position<i32>,
        p1: Position<i32>,
        p2: Position<i32>,
        p3: Position<i32>,
    ) -> bool {
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
    fn cross_product_sign(p1: Position<i32>, p2: Position<i32>) -> i32 {
        (p1.x * p2.y) - (p2.x * p1.y)
    }
}
