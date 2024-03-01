use jeri::color::Color;
use jeri::entities::{Circle, Line, Rectangle, Text, Triangle};
use jeri::{Canvas, Position, Size};

use std::fs::File;
use std::io::{BufReader, BufWriter};

type Result<T> = core::result::Result<T, ()>;

const ERROR_COLOR: Color = Color::BRIGHT_PINK;

struct Buffer {
    ibuf: Vec<u8>,
    height: u32,
    width: u32,
}

fn evaluate_test_case(actual_canvas: &Canvas, test: &str) {
    let actual_filepath = format!("./tests/resources/actual/{test}.png");
    let expected_filepath = format!("./tests/resources/expected/{test}.png");
    let diff_filepath = format!("./tests/resources/diff/{test}.png");

    let actual_buffer = Buffer {
        ibuf: actual_canvas.buffer_ref().iter().cloned().collect(),
        height: actual_canvas.size_ref().height as u32,
        width: actual_canvas.size_ref().width as u32,
    };

    let Ok(expected_buffer) = load_buffer_from_png(&expected_filepath) else {
        save_buffer_to_png(&actual_buffer, &actual_filepath);
        panic!(
            "There was no expectation for this test yet, evaluate the {actual_filepath} to \
            determine if it meets the expectation than copy it to {expected_filepath}."
        )
    };

    // The test passes, we end it here
    if actual_canvas.buffer_ref() == expected_buffer.ibuf {
        return;
    }

    let diff_buffer = compute_diff_buffer(&actual_buffer, &expected_buffer);

    save_buffer_to_png(&diff_buffer, &diff_filepath);
    save_buffer_to_png(&actual_buffer, &actual_filepath);

    panic!(
        "The test {test} failed, look at the {diff_filepath} to see the difference, if this \
        difference is desirable, copy the {actual_filepath} to {expected_filepath} to update the \
        expectation."
    );
}

fn load_buffer_from_png(filepath: &str) -> Result<Buffer> {
    let file = File::open(filepath).map_err(|err| match err.kind() {
        std::io::ErrorKind::NotFound => (),
        _ => panic!("Failed to load {filepath}: {err}"),
    })?;

    let file = BufReader::new(file);
    let decoder = png::Decoder::new(file);
    let mut reader = decoder.read_info().unwrap();
    let mut buffer = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buffer).unwrap();

    Ok(Buffer {
        ibuf: buffer,
        height: info.height,
        width: info.width,
    })
}

fn save_buffer_to_png(buffer: &Buffer, filepath: &str) {
    let file = File::create(filepath).unwrap();
    let file = BufWriter::new(file);

    let mut encoder = png::Encoder::new(file, buffer.width, buffer.height);
    encoder.set_color(png::ColorType::Rgba);

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&buffer.ibuf).unwrap();
}

fn compute_diff_buffer(lb: &Buffer, rb: &Buffer) -> Buffer {
    assert_eq!(lb.height, rb.height);
    assert_eq!(lb.width, rb.width);

    let mut result = Vec::with_capacity(lb.ibuf.len());

    for (l, r) in lb
        .ibuf
        .chunks_exact(Color::CHANNELS)
        .zip(rb.ibuf.chunks_exact(Color::CHANNELS))
    {
        if l == r {
            result.extend_from_slice(l);
        } else {
            result.extend_from_slice(
                (ERROR_COLOR + Color::from_rgba_array(l.try_into().unwrap()).with_alpha(0x99))
                    .to_rgba_array()
                    .as_slice(),
            );
        }
    }

    Buffer {
        ibuf: result,
        height: lb.height,
        width: lb.width,
    }
}

#[test]
fn circle() {
    let mut canvas = Canvas::with_size(Size {
        width: 100,
        height: 100,
    });

    let circle = Circle {
        center: Position { x: 35, y: 35 },
        radius: 20,
        color: Color::WHITE,
    };

    canvas.render(&[circle]);

    evaluate_test_case(&canvas, "circle")
}

#[test]
fn line() {
    let mut canvas = Canvas::with_size(Size {
        width: 100,
        height: 100,
    });

    let line_1 = Line {
        end: Position { x: 0, y: 20 },
        start: Position { x: 25, y: 85 },
        color: Color::WHITE,
        width: 2,
    };
    let line_2 = Line {
        end: Position { x: 70, y: 20 },
        start: Position { x: 73, y: 85 },
        color: Color::WHITE,
        width: 2,
    };
    let vertical_line = Line {
        end: Position { x: 35, y: 30 },
        start: Position { x: 35, y: 10 },
        color: Color::WHITE,
        width: 2,
    };
    let horizontal_line = Line {
        end: Position { x: 10, y: 70 },
        start: Position { x: 40, y: 70 },
        color: Color::WHITE,
        width: 2,
    };

    canvas.render(&[line_1, line_2, vertical_line, horizontal_line]);

    evaluate_test_case(&canvas, "line")
}

#[test]
fn rectangle() {
    let mut canvas = Canvas::with_size(Size {
        width: 100,
        height: 100,
    });

    let rectangle = Rectangle {
        center: Position { x: 40, y: 40 },
        size: Size {
            width: 30,
            height: 30,
        },
        color: Color::WHITE,
    };

    canvas.render(&[rectangle]);

    evaluate_test_case(&canvas, "rectangle")
}

#[test]
fn two_objects() {
    let mut canvas = Canvas::with_size(Size {
        width: 100,
        height: 100,
    });

    let rectangle = Rectangle {
        center: Position { x: 40, y: 40 },
        size: Size {
            width: 30,
            height: 30,
        },
        color: Color::WHITE,
    };

    let circle = Circle {
        center: Position { x: 60, y: 60 },
        radius: 20,
        color: Color::WHITE,
    };

    canvas.render(&[rectangle]);
    canvas.render(&[circle]);

    evaluate_test_case(&canvas, "two_objects")
}

#[test]
pub fn clear_buffer() {
    let mut canvas = Canvas::with_size(Size {
        width: 100,
        height: 100,
    });

    let rectangle = Rectangle {
        center: Position { x: 40, y: 40 },
        size: Size {
            width: 30,
            height: 30,
        },
        color: Color::WHITE,
    };

    canvas.render(&[rectangle]);
    canvas.clear_buffer();

    evaluate_test_case(&canvas, "clear_buffer")
}

#[test]
pub fn triangle() {
    let mut canvas = Canvas::with_size(Size {
        width: 100,
        height: 100,
    });

    let triangle = Triangle {
        p1: Position { x: 10, y: 10 },
        p2: Position { x: 95, y: 45 },
        p3: Position { x: 50, y: 10 },
        color: Color::WHITE,
    };

    canvas.render(&[triangle]);

    evaluate_test_case(&canvas, "triangle")
}

#[test]
pub fn text() {
    let mut canvas = Canvas::with_size(Size {
        width: 600,
        height: 100,
    });

    let text = Text::new(
        r#"abcdefghijklmnopqrstuvwxyz"' _.!?,()[]{}1234567890<>-*+=%;:|\/"#.into(),
        Position { x: 10, y: 10 },
        Color::WHITE,
        2,
    );

    canvas.render(&[text]);

    evaluate_test_case(&canvas, "text")
}

#[test]
pub fn complex_image() {
    let mut canvas = Canvas::with_size(Size {
        width: 1920,
        height: 1080,
    });

    let texts = vec![Text::new(
        r#"abcdefghijklmnopqrstuvwxyz"' _.!?,()[]{}1234567890<>-*+=%;:|\/"#.into(),
        Position { x: 10, y: 10 },
        Color::LIGHT_PINK,
        2,
    )];

    let circles = vec![
        Circle {
            center: Position { x: 1600, y: 60 },
            radius: 200,
            color: Color::WHITE,
        },
        Circle {
            center: Position { x: 1500, y: 600 },
            radius: 250,
            color: Color::GRAY,
        },
        Circle {
            center: Position { x: 1450, y: 450 },
            radius: 200,
            color: Color::BRIGHT_PINK,
        },
        Circle {
            center: Position { x: 1900, y: 900 },
            radius: 40,
            color: Color::GREEN,
        },
    ];
    let rectangles = vec![
        Rectangle {
            center: Position { x: 100, y: 100 },
            size: Size {
                width: 100,
                height: 60,
            },
            color: Color::WHITE,
        },
        Rectangle {
            center: Position { x: 500, y: 500 },
            size: Size {
                width: 500,
                height: 500,
            },
            color: Color::GREEN,
        },
        Rectangle {
            center: Position { x: 1000, y: 1000 },
            size: Size {
                width: 700,
                height: 70,
            },
            color: Color::GRAY,
        },
        Rectangle {
            center: Position { x: 1500, y: 1000 },
            size: Size {
                width: 100,
                height: 50,
            },
            color: Color::BRIGHT_PINK,
        },
    ];

    let lines = vec![
        Line {
            start: Position { x: 50, y: 20 },
            end: Position { x: 1900, y: 1900 },
            color: Color::WHITE,
            width: 10,
        },
        Line {
            start: Position { x: 1400, y: 100 },
            end: Position { x: 430, y: 430 },
            color: Color::GREEN,
            width: 5,
        },
        Line {
            start: Position { x: 100, y: 900 },
            end: Position { x: 900, y: 900 },
            color: Color::GRAY,
            width: 8,
        },
    ];

    let triangles = vec![
        Triangle {
            p1: Position { x: 1900, y: 50 },
            p2: Position { x: 1700, y: 400 },
            p3: Position { x: 1000, y: 100 },
            color: Color::WHITE,
        },
        Triangle {
            p1: Position { x: 200, y: 1100 },
            p2: Position { x: 1000, y: 1300 },
            p3: Position { x: 1200, y: 700 },
            color: Color::GREEN,
        },
        Triangle {
            p1: Position { x: 100, y: 100 },
            p2: Position { x: 300, y: 300 },
            p3: Position { x: 500, y: 700 },
            color: Color::GREEN,
        },
    ];

    canvas.render(&circles);
    canvas.render(&rectangles);
    canvas.render(&texts);
    canvas.render(&lines);
    canvas.render(&triangles);

    evaluate_test_case(&canvas, "complex_image")
}
