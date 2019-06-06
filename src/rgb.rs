// This code is copied from
// https://rust-lang-nursery.github.io/rust-cookbook/text/string_parsing.html, as
// a convenient example of FromStr, and then extended to provide fmt::Display as well.
// (with tests to make sure I didn't screw it up.)
use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }
}

impl FromStr for RGB {
    type Err = std::num::ParseIntError;

    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {

        // u8::from_str_radix(src: &str, radix: u32) converts a string
        // slice in a given base to u8
        let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
        let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
        let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;

        Ok(RGB { r, g, b })
    }
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_to() {
        let rgb = RGB::new(255, 255, 255);
        assert_eq!(RGB::from_str("#FFFFFF"), Ok(rgb));
        assert_eq!(str::parse::<RGB>("#FFFFFF"), Ok(rgb));

        let str = rgb.to_string();
        assert_eq!(RGB::from_str(&str), Ok(rgb));

        let rgb = RGB::new(1, 2, 3);
        assert_eq!(RGB::from_str("#010203"), Ok(rgb));
        assert_eq!(str::parse::<RGB>("#010203"), Ok(rgb));
        assert_eq!(rgb.to_string(), "#010203".to_string());

        // TODO: for production code, also check errors.
    }
}
