use crate::{color::Color, Drawable, Position};

pub const GLYPH_WIDTH: usize = 5;
pub const GLYPH_HEIGHT: usize = 6;

pub struct Text {
    pub text: String,
    pub position: Position<i32>,
    pub color: Color,
    // TODO : Implement font size
}

impl Text {
    fn index_from_position(p: Position<usize>) -> usize {
        p.y * GLYPH_WIDTH + p.x
    }
}

impl Drawable for Text {
    fn draw(&self, buffer: &mut [Color], canvas_size: &crate::Size) {
        for (index, character) in self.text.chars().enumerate() {
            let glyph = GLYPHS[character as usize];

            for x in 0..GLYPH_WIDTH {
                for y in 0..GLYPH_HEIGHT {
                    if glyph[Text::index_from_position(Position { x, y })] == 1 {
                        let buffer_x = self.position.x + (index * GLYPH_WIDTH) as i32 + x as i32;
                        let buffer_y = self.position.y + y as i32;

                        buffer[canvas_size.position_to_index(Position {
                            x: buffer_x,
                            y: buffer_y,
                        })] += self.color
                    }
                }
            }
        }
    }
}

// TODO: Finish the GLYPHS table

// ASCII TABLE
#[rustfmt::skip]
pub const GLYPHS: [[u8; GLYPH_WIDTH * GLYPH_HEIGHT]; 128] = [ // We could pack this further with 1 bit per pixel
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Null character
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Start of Heading
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Start of Text
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // End of Text
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // End of Transmission
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Enquiry
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Acknowledge
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Bell, Alert
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Backspace
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Horizontal Tab
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Line Feed
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Vertical Tabulation
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Form Feed
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Carriage Return
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Shift Out
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Shift In
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Data Link Escape
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Device Control One (XON)
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Device Control Two
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Device Control Three (XOFF)
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Device Control Four
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Negative Acknowledge
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Synchronous Idle
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // End of Transmission Block
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Cancel
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // End of medium
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Substitute
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Escape
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // File Separator
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Group Separator
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Record Separator
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Unit Separator
    // ============================================
    // ----------- Printable Characters -----------
    // ============================================
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Space
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Exclamation mark
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Double quotes (or speech marks)
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Number sign
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Dollar
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Per cent sign
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Ampersand
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Single quote
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Open parenthesis (or open bracket)
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Close parenthesis (or close bracket)
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Asterisk
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Plus
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Comma
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Hyphen-minus
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Period, dot or full stop
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Slash or divide
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Zero
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // One
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Two
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Three
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Four
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Five
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Six
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Seven
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Eight
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Nine
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Colon
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Semicolon
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Less than (or open angled bracket)
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Equals
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Greater than (or close angled bracket)
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Question mark
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // At sign
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase A
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase B
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase C
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase D
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase E
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase F
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase G
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase H
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase I
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase J
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase K
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase L
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase M
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase N
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase O
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase P
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase Q
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase R
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase S
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase T
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase U
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase V
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase W
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase X
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase Y
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Uppercase Z
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Opening bracket
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Backslash
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Closing bracket
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Caret - circumflex
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Underscore
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Grave accent
    [
        0, 0, 0, 0, 0,
        0, 1, 1, 0, 0,
        0, 0, 0, 1, 0,
        0, 1, 1, 1, 0,
        1, 0, 0, 1, 0,
        0, 1, 1, 1, 0,
    ], // Lowercase a
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase b
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase c
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase d
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase e
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase f
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase g
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase h
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase i
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase j
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase k
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase l
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase m
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase n
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase o
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase p
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase q
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase r
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase s
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase t
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase u
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase v
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase w
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase x
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase y
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Lowercase z
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Opening brace
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Vertical bar
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Closing brace
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Equivalency sign - tilde
    [0; GLYPH_WIDTH * GLYPH_HEIGHT], // Delete
];
