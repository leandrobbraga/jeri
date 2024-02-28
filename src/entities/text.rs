use crate::{color::Color, Drawable, Position};

pub const GLYPH_MAX_HEIGHT: usize = 10;

pub struct Text {
    pub text: String,
    pub position: Position<i32>,
    pub color: Color,
    pub size: u8,
    text_width: i32,
}

impl Text {
    pub fn new(text: String, position: Position<i32>, color: Color, size: u8) -> Text {
        let text_width = text
            .chars()
            .map(|character| GLYPH_WIDTHS[character as usize] as i32 + 1)
            .sum();

        Text {
            text,
            position,
            color,
            size,
            text_width,
        }
    }
}

impl Drawable for Text {
    #[inline(always)]
    fn color_at(&self, position: Position<i32>) -> Option<Color> {
        // Fast path
        if !((position.x >= self.position.x)
            & (position.x < self.position.x + (self.size as i32 * self.text_width))
            & (position.y >= self.position.y)
            & (position.y < self.position.y + (self.size as i32 * GLYPH_MAX_HEIGHT as i32)))
        {
            return None;
        }

        let mut x = self.position.x;

        for character in self.text.chars() {
            let glyph_width = GLYPH_WIDTHS[character as usize];

            if (position.x >= x) & (position.x < (x + (glyph_width * self.size) as i32)) {
                let glyph = GLYPHS[character as usize];

                let glyph_x = (position.x - x) / self.size as i32;
                let glyph_y = (position.y - self.position.y) / self.size as i32;

                // The glyph is composed of an array of bytes, each byte representing a 'line' of 8
                // pixels.
                let glyph_line = glyph[glyph_y as usize];

                // Verify if the bit representing the xth row is set, it need to be done from left
                // to right since the glyph is indexed from top-left to bottom-right.
                if ((glyph_line >> (glyph_width - glyph_x as u8 - 1)) & 1) == 1 {
                    return Some(self.color);
                } else {
                    return None;
                }
            } else {
                // We didn't find the right character yet, so we continue iterating
                // The '1' accounts for the gap between characters
                x += (glyph_width as i32 + 1) * self.size as i32;
            }
        }

        None
    }
}

// TODO: Finish the GLYPHS table

// ASCII TABLE
#[rustfmt::skip]
pub const GLYPHS: [[u8; GLYPH_MAX_HEIGHT]; 128] = [ // We could pack this further with 1 bit per pixel
    [0; GLYPH_MAX_HEIGHT], // Null character
    [0; GLYPH_MAX_HEIGHT], // Start of Heading
    [0; GLYPH_MAX_HEIGHT], // Start of Text
    [0; GLYPH_MAX_HEIGHT], // End of Text
    [0; GLYPH_MAX_HEIGHT], // End of Transmission
    [0; GLYPH_MAX_HEIGHT], // Enquiry
    [0; GLYPH_MAX_HEIGHT], // Acknowledge
    [0; GLYPH_MAX_HEIGHT], // Bell, Alert
    [0; GLYPH_MAX_HEIGHT], // Backspace
    [0; GLYPH_MAX_HEIGHT], // Horizontal Tab
    [0; GLYPH_MAX_HEIGHT], // Line Feed
    [0; GLYPH_MAX_HEIGHT], // Vertical Tabulation
    [0; GLYPH_MAX_HEIGHT], // Form Feed
    [0; GLYPH_MAX_HEIGHT], // Carriage Return
    [0; GLYPH_MAX_HEIGHT], // Shift Out
    [0; GLYPH_MAX_HEIGHT], // Shift In
    [0; GLYPH_MAX_HEIGHT], // Data Link Escape
    [0; GLYPH_MAX_HEIGHT], // Device Control One (XON)
    [0; GLYPH_MAX_HEIGHT], // Device Control Two
    [0; GLYPH_MAX_HEIGHT], // Device Control Three (XOFF)
    [0; GLYPH_MAX_HEIGHT], // Device Control Four
    [0; GLYPH_MAX_HEIGHT], // Negative Acknowledge
    [0; GLYPH_MAX_HEIGHT], // Synchronous Idle
    [0; GLYPH_MAX_HEIGHT], // End of Transmission Block
    [0; GLYPH_MAX_HEIGHT], // Cancel
    [0; GLYPH_MAX_HEIGHT], // End of medium
    [0; GLYPH_MAX_HEIGHT], // Substitute
    [0; GLYPH_MAX_HEIGHT], // Escape
    [0; GLYPH_MAX_HEIGHT], // File Separator
    [0; GLYPH_MAX_HEIGHT], // Group Separator
    [0; GLYPH_MAX_HEIGHT], // Record Separator
    [0; GLYPH_MAX_HEIGHT], // Unit Separator
    // ============================================
    // ----------- Printable Characters -----------
    // ============================================
    [0; GLYPH_MAX_HEIGHT], // Space
    [
        0b00000000,
        0b00000001,
        0b00000001,
        0b00000001,
        0b00000001,
        0b00000000,
        0b00000001,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Exclamation mark
    [
        0b00000000,
        0b00000101,
        0b00000101,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Double quotes (or speech marks)
    [0; GLYPH_MAX_HEIGHT], // Number sign
    [0; GLYPH_MAX_HEIGHT], // Dollar
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00001001,
        0b00000010,
        0b00000100,
        0b00001001,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Per cent sign
    [0; GLYPH_MAX_HEIGHT], // Ampersand
    [
        0b00000000,
        0b00000001,
        0b00000001,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Single quote
    [
        0b00000000,
        0b00000001,
        0b00000010,
        0b00000010,
        0b00000010,
        0b00000010,
        0b00000001,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Open parenthesis (or open bracket)
    [
        0b00000000,
        0b00000010,
        0b00000001,
        0b00000001,
        0b00000001,
        0b00000001,
        0b00000010,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Open parenthesis (or open bracket)
    [
        0b00000000,
        0b00010101,
        0b00001110,
        0b00011111,
        0b00001110,
        0b00010101,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Asterisk
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000010,
        0b00000111,
        0b00000010,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Plus
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000001,
        0b00000010,
        0b00000000,
        0b00000000,
    ], // Comma
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000111,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Hyphen-minus
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000001,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Period, dot or full stop 
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000001,
        0b00000010,
        0b00000100,
        0b00001000,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Slash or divide
    [
        0b00000000,
        0b00000010,
        0b00000101,
        0b00000101,
        0b00000101,
        0b00000101,
        0b00000010,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Zero
    [
        0b00000000,
        0b00000010,
        0b00000110,
        0b00000010,
        0b00000010,
        0b00000010,
        0b00000111,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // One
    [
        0b00000000,
        0b00000010,
        0b00000101,
        0b00000001,
        0b00000010,
        0b00000100,
        0b00000111,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Two
    [
        0b00000010,
        0b00000101,
        0b00000001,
        0b00000010,
        0b00000001,
        0b00000101,
        0b00000010,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Three
    [
        0b00000000,
        0b00000010,
        0b00000100,
        0b00000101,
        0b00000111,
        0b00000001,
        0b00000001,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Four
    [
        0b00000000,
        0b00000111,
        0b00000100,
        0b00000111,
        0b00000001,
        0b00000001,
        0b00000110,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Five 5
    [
        0b00000000,
        0b00000010,
        0b00000100,
        0b00000110,
        0b00000101,
        0b00000101,
        0b00000010,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Six
    [
        0b00000000,
        0b00000111,
        0b00000001,
        0b00000010,
        0b00000010,
        0b00000100,
        0b00000100,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Seven
    [
        0b00000000,
        0b00000010,
        0b00000101,
        0b00000010,
        0b00000101,
        0b00000101,
        0b00000010,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Eight
    [
        0b00000000,
        0b00000010,
        0b00000101,
        0b00000011,
        0b00000001,
        0b00000001,
        0b00000010,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Nine
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000001,
        0b00000000,
        0b00000001,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Colon
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000001,
        0b00000000,
        0b00000001,
        0b00000010,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Colon
    [
        0b00000000,
        0b00000000,
        0b00000001,
        0b00000010,
        0b00000100,
        0b00000010,
        0b00000001,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Less than (or open angled bracket)
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000111,
        0b00000000,
        0b00000111,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Equals
    [
        0b00000000,
        0b00000000,
        0b00000100,
        0b00000010,
        0b00000001,
        0b00000010,
        0b00000100,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Greater than (or close angled bracket)
    [
        0b00000000,
        0b00001110,
        0b00010001,
        0b00000001,
        0b00000110,
        0b00000000,
        0b00000100,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Question mark
    [0; GLYPH_MAX_HEIGHT], // At sign
    [0; GLYPH_MAX_HEIGHT], // Uppercase A
    [0; GLYPH_MAX_HEIGHT], // Uppercase B
    [0; GLYPH_MAX_HEIGHT], // Uppercase C
    [0; GLYPH_MAX_HEIGHT], // Uppercase D
    [0; GLYPH_MAX_HEIGHT], // Uppercase E
    [0; GLYPH_MAX_HEIGHT], // Uppercase F
    [0; GLYPH_MAX_HEIGHT], // Uppercase G
    [0; GLYPH_MAX_HEIGHT], // Uppercase H
    [0; GLYPH_MAX_HEIGHT], // Uppercase I
    [0; GLYPH_MAX_HEIGHT], // Uppercase J
    [0; GLYPH_MAX_HEIGHT], // Uppercase K
    [0; GLYPH_MAX_HEIGHT], // Uppercase L
    [0; GLYPH_MAX_HEIGHT], // Uppercase M
    [0; GLYPH_MAX_HEIGHT], // Uppercase N
    [0; GLYPH_MAX_HEIGHT], // Uppercase O
    [0; GLYPH_MAX_HEIGHT], // Uppercase P
    [0; GLYPH_MAX_HEIGHT], // Uppercase Q
    [0; GLYPH_MAX_HEIGHT], // Uppercase R
    [0; GLYPH_MAX_HEIGHT], // Uppercase S
    [0; GLYPH_MAX_HEIGHT], // Uppercase T
    [0; GLYPH_MAX_HEIGHT], // Uppercase U
    [0; GLYPH_MAX_HEIGHT], // Uppercase V
    [0; GLYPH_MAX_HEIGHT], // Uppercase W
    [0; GLYPH_MAX_HEIGHT], // Uppercase X
    [0; GLYPH_MAX_HEIGHT], // Uppercase Y
    [0; GLYPH_MAX_HEIGHT], // Uppercase Z
    [
        0b00000000,
        0b00000011,
        0b00000010,
        0b00000010,
        0b00000010,
        0b00000010,
        0b00000011,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Opening bracket
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00001000,
        0b00000100,
        0b00000010,
        0b00000001,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Backslash
    [
        0b00000000,
        0b00000011,
        0b00000001,
        0b00000001,
        0b00000001,
        0b00000001,
        0b00000011,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Closing bracket
    [0; GLYPH_MAX_HEIGHT], // Caret - circumflex
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000000,
        0b00001111,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Underscore
    [0; GLYPH_MAX_HEIGHT], // Grave accent
    [
        0b00000000,
        0b00000000,
        0b00000110,
        0b00000001,
        0b00000111,
        0b00001001,
        0b00000111,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase a
    [
        0b00000000,
        0b00001000,
        0b00001000,
        0b00001000,
        0b00001110,
        0b00001001,
        0b00001110,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase b
    [
        0b00000000,
        0b00000000,
        0b00000110,
        0b00001001,
        0b00001000,
        0b00001001,
        0b00000110,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase c
    [
        0b00000000,
        0b00000001,
        0b00000001,
        0b00000001,
        0b00000111,
        0b00001001,
        0b00000111,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase d
    [
        0b00000000,
        0b00000000,
        0b00000110,
        0b00001001,
        0b00001111,
        0b00001000,
        0b00000110,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase e
    [
        0b00000000,
        0b00000011,
        0b00000100,
        0b00000100,
        0b00001110,
        0b00000100,
        0b00000100,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase f
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000111,
        0b00001001,
        0b00001001,
        0b00000111,
        0b00000001,
        0b00001001,
        0b00000110,
    ], // Lowercase g
    [
        0b00000000,
        0b00001000,
        0b00001000,
        0b00001000,
        0b00001110,
        0b00001001,
        0b00001001,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase h
    [
        0b00000000,
        0b00000000,
        0b00000001,
        0b00000000,
        0b00000001,
        0b00000001,
        0b00000001,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase i
    [
        0b00000000,
        0b00000000,
        0b00000001,
        0b00000000,
        0b00000011,
        0b00000001,
        0b00000001,
        0b00000001,
        0b00000001,
        0b00000110,
    ], // Lowercase j
    [
        0b00000000,
        0b00000000,
        0b00000100,
        0b00000100,
        0b00000101,
        0b00000110,
        0b00000101,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase k
    [
        0b00000000,
        0b00000110,
        0b00000010,
        0b00000010,
        0b00000010,
        0b00000010,
        0b00000011,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase l
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00001010,
        0b00010101,
        0b00010101,
        0b00010101,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase m
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00001010,
        0b00000101,
        0b00000101,
        0b00000101,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase n
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000110,
        0b00001001,
        0b00001001,
        0b00000110,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase o
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000110,
        0b00001001,
        0b00001001,
        0b00001110,
        0b00001000,
        0b00001000,
        0b00001000,
    ], // Lowercase p
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00000111,
        0b00001001,
        0b00001001,
        0b00000111,
        0b00000001,
        0b00000001,
        0b00000001,
    ], // Lowercase q
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00001011,
        0b00001101,
        0b00001000,
        0b00001000,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase r
    [
        0b00000000,
        0b00000000,
        0b00000111,
        0b00001000,
        0b00000110,
        0b00000001,
        0b00001110,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase s
    [
        0b00000000,
        0b00000010,
        0b00000010,
        0b00000111,
        0b00000010,
        0b00000010,
        0b00000011,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase t
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00001001,
        0b00001001,
        0b00001001,
        0b00000111,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase u
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00010001,
        0b00010001,
        0b00001010,
        0b00000100,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase v
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00010101,
        0b00010101,
        0b00010101,
        0b00001010,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase w
    [
        0b00000000,
        0b00000000,
        0b00010001,
        0b00001010,
        0b00000100,
        0b00001010,
        0b00010001,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase x
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00001001,
        0b00001001,
        0b00001001,
        0b00000111,
        0b00000001,
        0b00000001,
        0b00000111,
    ], // Lowercase y
    [
        0b00000000,
        0b00000000,
        0b00000000,
        0b00001111,
        0b00000010,
        0b00000100,
        0b00001111,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Lowercase z
    [
        0b00000000,
        0b00000001,
        0b00000010,
        0b00000010,
        0b00000110,
        0b00000010,
        0b00000010,
        0b00000001,
        0b00000000,
        0b00000000,
    ], // Opening brace
    [
        0b00000000,
        0b00000001,
        0b00000001,
        0b00000001,
        0b00000001,
        0b00000001,
        0b00000001,
        0b00000000,
        0b00000000,
        0b00000000,
    ], // Vertical bar
    [
        0b00000000,
        0b00000100,
        0b00000010,
        0b00000010,
        0b00000011,
        0b00000010,
        0b00000010,
        0b00000100,
        0b00000000,
        0b00000000,
    ], // Closing brace
    [0; GLYPH_MAX_HEIGHT], // Equivalency sign - tilde
    [0; GLYPH_MAX_HEIGHT], // Delete
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
