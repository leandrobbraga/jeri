use crate::{color::Color, Drawable, Position, Size};

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
                buffer[screen_size.position_to_index(&Position { x, y })] = self.color
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
                    buffer[screen_size.position_to_index(&position)] = self.color
                }
            }
        }
    }
}
