pub mod renderer;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Size {
    width: i32,
    height: i32,
}

pub struct Position {
    x: i32,
    y: i32,
}
