use renderer::color::Color;
use renderer::entities::{Circle, Line, Rectangle};
use renderer::{Canvas, Position, Size};

// TODO: Evolve the tests from macros to expectations.
//
//       With expectations, everytime the developer run the tests, the program generates a
//       file which is compared to the previously approved expectation. If the difference is
//       desirable the developer can approve it as the new expectation, otherwise they can keep
//       working to meet the expectation.
//
//       This way is much easier to evolve the renderer, specially subtle things like
//       anti-aliasing
macro_rules! buffer {
        (.) => {Color::BLACK};
        (W) => {Color::WHITE};
        ($($s:tt)+) => {
            [$(buffer!($s)),+]
        };
    }

#[test]
fn draw_circle() {
    let mut canvas = Canvas::with_size(Size {
        width: 9,
        height: 9,
    });

    let circle = Circle {
        center: Position { x: 4, y: 4 },
        radius: 2,
        color: Color::WHITE,
    };

    canvas.render(&[circle]);

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

    assert_eq!(canvas.buffer_ref(), &expected_buffer);
}

#[test]
fn draw_line() {
    let mut canvas = Canvas::with_size(Size {
        width: 9,
        height: 9,
    });

    let circle = Line {
        end: Position { x: 5, y: 2 },
        start: Position { x: 1, y: 5 },
        color: Color::WHITE,
    };

    canvas.render(&[circle]);

    let expected_buffer = buffer![
        . . . . . . . . .
        . . . . . . . . .
        . . . . . W . . .
        . . . . W . . . .
        . . W W . . . . .
        . W . . . . . . .
        . . . . . . . . .
        . . . . . . . . .
        . . . . . . . . .
    ];

    assert_eq!(canvas.buffer_ref(), &expected_buffer);
}

#[test]
fn draw_rectangle() {
    let mut canvas = Canvas::with_size(Size {
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

    canvas.render(&[rectangle]);

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

    assert_eq!(canvas.buffer_ref(), &expected_buffer);
}

#[test]
fn draw_two_objects() {
    let mut canvas = Canvas::with_size(Size {
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

    canvas.render(&[rectangle]);
    canvas.render(&[circle]);

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

    assert_eq!(canvas.buffer_ref(), &expected_buffer);
}

#[test]
pub fn clear_buffer() {
    let mut canvas = Canvas::with_size(Size {
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

    canvas.render(&[rectangle]);
    canvas.clear_buffer();

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

    assert_eq!(canvas.buffer_ref(), &expected_buffer);
}
