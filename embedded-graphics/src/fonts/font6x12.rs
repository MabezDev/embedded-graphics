//! 6x12 pixel font. Image data taken from the [Uzebox Wiki page](http://uzebox.org/wiki/Font_Bitmaps)

use super::super::drawable::*;
use super::super::transform::*;
use super::Font;
use coord::{Coord, ToUnsigned};
use pixelcolor::PixelColor;

const FONT_IMAGE: &[u8] = include_bytes!("../../data/font6x12_1bpp.raw");
const CHAR_HEIGHT: u32 = 12;
const CHAR_WIDTH: u32 = 6;
const FIRST_CHARCODE: u32 = 32; // A space
const FONT_IMAGE_WIDTH: u32 = 96;
const CHARS_PER_ROW: u32 = FONT_IMAGE_WIDTH / CHAR_WIDTH;

/// Container struct to hold a positioned piece of text
#[derive(Debug, Clone, Copy)]
pub struct Font6x12<'a, C: PixelColor> {
    /// Top left corner of the text
    pub pos: Coord,

    /// Text to draw
    text: &'a str,

    /// Fill Color of font
    color: C,
}

impl<'a, C> Font<'a, C> for Font6x12<'a, C>
where
    C: PixelColor,
{
    fn render_str(text: &'a str, color: C) -> Font6x12<'a, C> {
        Self {
            pos: Coord::new(0, 0),
            text,
            color,
        }
    }

    fn measure_str(text: &'a str) -> (usize, usize){
        (text.len() * 6, 12)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Font6x12Iterator<'a, C> {
    char_walk_x: u32,
    char_walk_y: u32,
    current_char: Option<char>,
    idx: usize,
    pos: Coord,
    text: &'a str,
    color: C,
}

impl<'a, C> IntoIterator for &'a Font6x12<'a, C>
where
    C: PixelColor,
{
    type IntoIter = Font6x12Iterator<'a, C>;
    type Item = Pixel<C>;

    fn into_iter(self) -> Self::IntoIter {
        Font6x12Iterator {
            current_char: self.text.chars().next(),
            idx: 0,
            text: self.text,
            char_walk_x: 0,
            char_walk_y: 0,
            pos: self.pos,
            color: self.color,
        }
    }
}

impl<'a, C> Iterator for Font6x12Iterator<'a, C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos[0] + (self.text.len() as i32 * CHAR_WIDTH as i32) < 0
            || self.pos[1] + (CHAR_HEIGHT as i32) < 0
        {
            return None;
        }

        if let Some(current_char) = self.current_char {
            let pixel = loop {
                // Char _code_ offset from first char, most often a space
                // E.g. first char = ' ' (32), target char = '!' (33), offset = 33 - 32 = 1
                let char_offset = current_char as u32 - FIRST_CHARCODE;
                let row = char_offset / CHARS_PER_ROW;

                // Top left corner of character, in pixels
                let char_x = (char_offset - (row * CHARS_PER_ROW)) * CHAR_WIDTH;
                let char_y = row * CHAR_HEIGHT;

                // Bit index
                // = X pixel offset for char
                // + Character row offset (row 0 = 0, row 1 = (192 * 8) = 1536)
                // + X offset for the pixel block that comprises this char
                // + Y offset for pixel block
                let bitmap_bit_index = char_x
                    + (FONT_IMAGE_WIDTH * char_y)
                    + self.char_walk_x
                    + (self.char_walk_y * FONT_IMAGE_WIDTH);

                let bitmap_byte = bitmap_bit_index / 8;
                let bitmap_bit = 7 - (bitmap_bit_index % 8);

                let color = if (FONT_IMAGE[bitmap_byte as usize] >> bitmap_bit) & 1 == 1 {
                    self.color
                } else {
                    0.into() // black
                };

                self.char_walk_x += 1;

                if self.char_walk_x >= CHAR_WIDTH {
                    self.char_walk_x = 0;
                    self.char_walk_y += 1;

                    // Done with this char, move on to the next one
                    if self.char_walk_y >= CHAR_HEIGHT {
                        self.char_walk_y = 0;
                        self.idx += 1;
                        self.current_char = self.text.chars().skip(self.idx).next();
                    }
                }

                let x =
                    self.pos[0] + (CHAR_WIDTH * self.idx as u32) as i32 + self.char_walk_x as i32;
                let y = self.pos[1] + self.char_walk_y as i32;

                if x >= 0 && y >= 0 {
                    break Some(Pixel(Coord::new(x, y).to_unsigned(), color));
                }
            };

            pixel
        } else {
            None
        }
    }
}

impl<'a, C> Drawable for Font6x12<'a, C>
where
    C: PixelColor,
{
}

impl<'a, C> Transform for Font6x12<'a, C>
where
    C: PixelColor,
{
    /// Translate the font origin from its current position to a new position by (x, y) pixels,
    /// returning a new `Font6x8`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::fonts::{ Font, Font6x8 };
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    ///
    /// let text = Font6x8::render_str("Hello world", 1u8);
    /// let moved = text.translate(Coord::new(25, 30));
    ///
    /// assert_eq!(text.pos, Coord::new(0, 0));
    /// assert_eq!(moved.pos, Coord::new(25, 30));
    /// ```
    fn translate(&self, by: Coord) -> Self {
        Self {
            pos: self.pos + by,
            ..self.clone()
        }
    }

    /// Translate the font origin from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::fonts::{ Font, Font6x12 };
    /// # use embedded_graphics::transform::Transform;
    /// # use embedded_graphics::coord::Coord;
    ///
    /// // 8px x 1px test image
    /// let mut text = Font6x12::render_str("Hello world", 1u8);
    /// text.translate_mut(Coord::new(25, 30));
    ///
    /// assert_eq!(text.pos, Coord::new(25, 30));
    /// ```
    fn translate_mut(&mut self, by: Coord) -> &mut Self {
        self.pos += by;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_screen_text_does_not_infinite_loop() {
        let text = Font6x12::render_str("Hello World!", 1u8).translate(Coord::new(5, -20));
        let mut it = text.into_iter();

        assert_eq!(it.next(), None);
    }
}
