use crate::{color::Color, Drawable, Position};

pub const GLYPH_MAX_WIDTH: usize = 5;
pub const GLYPH_MAX_HEIGHT: usize = 10;

pub struct Text {
    pub text: String,
    pub position: Position<i32>,
    pub color: Color,
    pub size: u8,
}

impl Text {
    fn index_from_position(p: Position<usize>) -> usize {
        p.y * GLYPH_MAX_WIDTH + p.x
    }
}

impl Drawable for Text {
    fn draw(&self, buffer: &mut [Color], canvas_size: &crate::Size) {
        // This variable accumulates the width of all the characters that were draw so far, this is
        // necessary since the characters have variable width and cannot be calculated easily.
        let mut width_acc = 0;

        for character in self.text.chars() {
            let glyph = GLYPHS[character as usize];
            let glyph_width = GLYPH_WIDTHS[character as usize];

            for gx in 0..glyph_width {
                for gy in 0..GLYPH_MAX_HEIGHT {
                    let glyph_position = Text::index_from_position(Position {
                        x: gx as usize,
                        y: gy,
                    });

                    // The glyph is composed by an array of 0s and 1s signalling if this particular
                    // pixel should or should not be rendered.
                    if glyph[glyph_position] > 0 {
                        // Each glyph pixel have a sub-loop to account for the Text size. Meaning
                        // that a value of '2' would map each single Glyph pixel to a 2x2 pixel
                        // square in the end canvas.
                        for x in 0..self.size {
                            for y in 0..self.size {
                                {
                                    // The x/y positions rationale is the following:
                                    // 1. Account for the text position
                                    // 2. Account for all the previously drawn characters, it need
                                    //    to be multiplied by the text size
                                    // 3. Account for all the previously drawn pixels from the
                                    //    current character it need to be multiplied by the text
                                    //    size
                                    // 4. Account for all the sub-pixels in the current character
                                    //    pixel
                                    buffer[canvas_size.position_to_index(Position {
                                        x: self.position.x
                                            + self.size as i32 * (width_acc + gx as i32)
                                            + x as i32,
                                        y: self.position.y
                                            + self.size as i32 * gy as i32
                                            + y as i32,
                                    })] += self.color
                                }
                            }
                        }
                    }
                }
            }

            width_acc += glyph_width as i32 + 1;
        }
    }
}

// TODO: Finish the GLYPHS table

// ASCII TABLE
// TODO: Compress the glyphs in a single 'u64' value, where each bit is a single pixel in the font.
//       It might be necessary to have an external tool to encode the data.
#[rustfmt::skip]
pub const GLYPHS: [[u8; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT]; 128] = [ // We could pack this further with 1 bit per pixel
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Null character
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Start of Heading
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Start of Text
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // End of Text
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // End of Transmission
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Enquiry
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Acknowledge
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Bell, Alert
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Backspace
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Horizontal Tab
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Line Feed
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Vertical Tabulation
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Form Feed
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Carriage Return
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Shift Out
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Shift In
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Data Link Escape
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Device Control One (XON)
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Device Control Two
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Device Control Three (XOFF)
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Device Control Four
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Negative Acknowledge
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Synchronous Idle
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // End of Transmission Block
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Cancel
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // End of medium
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Substitute
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Escape
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // File Separator
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Group Separator
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Record Separator
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Unit Separator
    // ============================================
    // ----------- Printable Characters -----------
    // ============================================
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Space
    [
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Exclamation mark
    [
        0, 0, 0, 0, 0,
        1, 0, 1, 0, 0,
        1, 0, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Double quotes (or speech marks)
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Number sign
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Dollar
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 0, 1, 0,
        0, 0, 1, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 0, 1, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Per cent sign
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Ampersand
    [
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Single quote
    [
        0, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Open parenthesis (or open bracket)
    [
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Open parenthesis (or open bracket)
    [
        0, 0, 0, 0, 0,
        1, 0, 1, 0, 1,
        0, 1, 1, 1, 0,
        1, 1, 1, 1, 1,
        0, 1, 1, 1, 0,
        1, 0, 1, 0, 1,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Asterisk
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 1, 1, 0, 0,
        0, 1, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Plus
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Comma
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 1, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Hyphen-minus
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Period, dot or full stop 
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 1, 0,
        0, 0, 1, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Slash or divide
    [
        0, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 1, 0, 0,
        1, 0, 1, 0, 0,
        1, 0, 1, 0, 0,
        1, 0, 1, 0, 0,
        0, 1, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Zero
    [
        0, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 1, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // One
    [
        0, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 1, 0, 0,
        0, 0, 1, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 1, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Two
    [
        0, 1, 0, 0, 0,
        1, 0, 1, 0, 0,
        0, 0, 1, 0, 0,
        0, 1, 0, 0, 0,
        0, 0, 1, 0, 0,
        1, 0, 1, 0, 0,
        0, 1, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Three
    [
        0, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 1, 0, 0,
        1, 1, 1, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Four
    [
        0, 0, 0, 0, 0,
        1, 1, 1, 0, 0,
        1, 0, 0, 0, 0,
        1, 1, 1, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 1, 0, 0,
        1, 1, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Five 5
    [
        0, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 1, 0, 0, 0,
        1, 0, 1, 0, 0,
        1, 0, 1, 0, 0,
        0, 1, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Six
    [
        0, 0, 0, 0, 0,
        1, 1, 1, 0, 0,
        0, 0, 1, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Seven
    [
        0, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 1, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 1, 0, 0,
        1, 0, 1, 0, 0,
        0, 1, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Eight
    [
        0, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 1, 0, 0,
        0, 1, 1, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 1, 0, 0,
        0, 1, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Nine
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Colon
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Colon
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 1, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Less than (or open angled bracket)
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 1, 1, 0, 0,
        0, 0, 0, 0, 0,
        1, 1, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Equals
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 0, 1, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Greater than (or close angled bracket)
    [
        0, 0, 0, 0, 0,
        0, 1, 1, 1, 0, 
        1, 0, 0, 0, 1, 
        0, 0, 0, 0, 1, 
        0, 0, 1, 1, 0, 
        0, 0, 0, 0, 0, 
        0, 0, 1, 0, 0, 
        0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 
    ], // Question mark
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // At sign
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase A
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase B
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase C
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase D
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase E
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase F
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase G
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase H
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase I
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase J
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase K
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase L
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase M
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase N
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase O
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase P
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase Q
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase R
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase S
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase T
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase U
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase V
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase W
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase X
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase Y
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Uppercase Z
    [
        0, 0, 0, 0, 0,
        1, 1, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 1, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Opening bracket
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 0, 1, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Backslash
    [
        0, 0, 0, 0, 0,
        1, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 1, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Closing bracket
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Caret - circumflex
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 1, 1, 1, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Underscore
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Grave accent
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 1, 1, 0, 0,
        0, 0, 0, 1, 0,
        0, 1, 1, 1, 0,
        1, 0, 0, 1, 0,
        0, 1, 1, 1, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase a
    [
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 1, 1, 0, 0,
        1, 0, 0, 1, 0,
        1, 1, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase b
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 1, 1, 0, 0,
        1, 0, 0, 1, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 1, 0,
        0, 1, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase c
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 1, 0,
        0, 0, 0, 1, 0,
        0, 0, 0, 1, 0,
        0, 1, 1, 1, 0,
        1, 0, 0, 1, 0,
        0, 1, 1, 1, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase d
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 1, 1, 0, 0,
        1, 0, 0, 1, 0,
        1, 1, 1, 1, 0,
        1, 0, 0, 0, 0,
        0, 1, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase e
    [
        0, 0, 0, 0, 0,
        0, 0, 1, 1, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 1, 1, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase f
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 1, 1, 1, 0,
        1, 0, 0, 1, 0,
        1, 0, 0, 1, 0,
        0, 1, 1, 1, 0,
        0, 0, 0, 1, 0,
        1, 0, 0, 1, 0,
        0, 1, 1, 0, 0,
    ], // Lowercase g
    [
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 1, 1, 0, 0,
        1, 0, 0, 1, 0,
        1, 0, 0, 1, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase h
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase i
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 1, 1, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 1, 0, 0,
        1, 1, 0, 0, 0,
    ], // Lowercase j
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 1, 0, 0,
        1, 1, 0, 0, 0,
        1, 0, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase k
    [
        0, 0, 0, 0, 0,
        1, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase l
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 1, 0, 1, 0,
        1, 0, 1, 0, 1,
        1, 0, 1, 0, 1,
        1, 0, 1, 0, 1,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase m
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 1, 0, 0,
        0, 1, 0, 1, 0,
        0, 1, 0, 1, 0,
        0, 1, 0, 1, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase n
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 1, 1, 0, 0,
        1, 0, 0, 1, 0,
        1, 0, 0, 1, 0,
        0, 1, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase o
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 1, 1, 0, 0,
        1, 0, 0, 1, 0,
        1, 0, 0, 1, 0,
        1, 1, 1, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
    ], // Lowercase p
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 1, 1, 1, 0,
        1, 0, 0, 1, 0,
        1, 0, 0, 1, 0,
        0, 1, 1, 1, 0,
        0, 0, 0, 1, 0,
        0, 0, 0, 1, 0,
        0, 0, 0, 1, 0,
    ], // Lowercase q
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 1, 1, 0,
        1, 1, 0, 1, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase r
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 1, 1, 1, 0,
        1, 0, 0, 0, 0,
        0, 1, 1, 0, 0,
        0, 0, 0, 1, 0,
        1, 1, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase s
    [
        0, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 1, 1, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase t
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 0, 1, 0,
        1, 0, 0, 1, 0,
        1, 0, 0, 1, 0,
        0, 1, 1, 1, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase u
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 1,
        1, 0, 0, 0, 1,
        0, 1, 0, 1, 0,
        0, 0, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase v
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 1, 0, 1,
        1, 0, 1, 0, 1,
        1, 0, 1, 0, 1,
        0, 1, 0, 1, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase w
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 1,
        0, 1, 0, 1, 0,
        0, 0, 1, 0, 0,
        0, 1, 0, 1, 0,
        1, 0, 0, 0, 1,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase x
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 0, 0, 1, 0,
        1, 0, 0, 1, 0,
        1, 0, 0, 1, 0,
        0, 1, 1, 1, 0,
        0, 0, 0, 1, 0,
        0, 0, 0, 1, 0,
        0, 1, 1, 1, 0,
    ], // Lowercase y
    [
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        1, 1, 1, 1, 0,
        0, 0, 1, 0, 0,
        0, 1, 0, 0, 0,
        1, 1, 1, 1, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Lowercase z
    [
        0, 0, 0, 0, 0,
        0, 0, 1, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Opening brace
    [
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Vertical bar
    [
        0, 0, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 1, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
        1, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ], // Closing brace
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Equivalency sign - tilde
    [0; GLYPH_MAX_WIDTH * GLYPH_MAX_HEIGHT], // Delete
];

#[rustfmt::skip]
pub const GLYPH_WIDTHS: [u8;  128] = [ // We could pack this further with 1 bit per pixel
    0, // Null character
    0, // Start of Heading
    0, // Start of Text
    0, // End of Text
    0, // End of Transmission
    0, // Enquiry
    0, // Acknowledge
    0, // Bell, Alert
    0, // Backspace
    0, // Horizontal Tab
    0, // Line Feed
    0, // Vertical Tabulation
    0, // Form Feed
    0, // Carriage Return
    0, // Shift Out
    0, // Shift In
    0, // Data Link Escape
    0, // Device Control One (XON)
    0, // Device Control Two
    0, // Device Control Three (XOFF)
    0, // Device Control Four
    0, // Negative Acknowledge
    0, // Synchronous Idle
    0, // End of Transmission Block
    0, // Cancel
    0, // End of medium
    0, // Substitute
    0, // Escape
    0, // File Separator
    0, // Group Separator
    0, // Record Separator
    0, // Unit Separator
    // ============================================
    // ----------- Printable Characters -----------
    // ============================================
    4, // Space
    1, // Exclamation mark
    3, // Double quotes (or speech marks)
    0, // Number sign
    0, // Dollar
    4, // Per cent sign
    0, // Ampersand
    1, // Single quote
    2, // Open parenthesis (or open bracket)
    2, // Open parenthesis (or open bracket)
    5, // Asterisk
    3, // Plus
    2, // Comma
    3, // Hyphen-minus
    1, // Period, dot or full stop 
    4, // Slash or divide
    3, // Zero
    3, // One
    3, // Two
    3, // Three
    3, // Four
    3, // Five
    3, // Six
    3, // Seven
    3, // Eight
    3, // Nine
    1, // Colon
    2, // Semicolon
    3, // Less than (or open angled bracket)
    3, // Equals
    3, // Greater than (or close angled bracket)
    5, // Question mark
    0, // At sign
    0, // Uppercase A
    0, // Uppercase B
    0, // Uppercase C
    0, // Uppercase D
    0, // Uppercase E
    0, // Uppercase F
    0, // Uppercase G
    0, // Uppercase H
    0, // Uppercase I
    0, // Uppercase J
    0, // Uppercase K
    0, // Uppercase L
    0, // Uppercase M
    0, // Uppercase N
    0, // Uppercase O
    0, // Uppercase P
    0, // Uppercase Q
    0, // Uppercase R
    0, // Uppercase S
    0, // Uppercase T
    0, // Uppercase U
    0, // Uppercase V
    0, // Uppercase W
    0, // Uppercase X
    0, // Uppercase Y
    0, // Uppercase Z
    2, // Opening bracket
    4, // Backslash
    2, // Closing bracket
    0, // Caret - circumflex
    4, // Underscore
    0, // Grave accent
    4, // Lowercase a
    4, // Lowercase b
    4, // Lowercase c
    4, // Lowercase d
    4, // Lowercase e
    4, // Lowercase f
    4, // Lowercase g
    4, // Lowercase h
    1, // Lowercase i
    3, // Lowercase j
    3, // Lowercase k
    3, // Lowercase l
    5, // Lowercase m
    4, // Lowercase n
    4, // Lowercase o
    4, // Lowercase p
    4, // Lowercase q
    4, // Lowercase r
    4, // Lowercase s
    3, // Lowercase t
    4, // Lowercase u
    5, // Lowercase v
    5, // Lowercase w
    5, // Lowercase x
    4, // Lowercase y
    4, // Lowercase z
    3, // Opening brace
    1, // Vertical bar
    3, // Closing brace
    0, // Equivalency sign - tilde
    0, // Delete
];
