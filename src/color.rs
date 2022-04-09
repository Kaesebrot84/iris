use std::fmt;

/// Represents a color, holding red, green, blue and alpha values as `u8` each.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ R: {}, G: {}, B: {}, A: {} }}", self.r, self.g, self.b, self.a)
    }
}

/// Indexing implementation for the `Color` struct using `ColorChannel` as index.
///
/// # Examples
///
/// ```
/// let color = Color { r: 1, g: 2, b: 3, a: 4 };
/// assert_eq!(1, color[ColorChannel::R]);
/// assert_eq!(2, color[ColorChannel::G]);
/// assert_eq!(3, color[ColorChannel::B]);
/// assert_eq!(4, color[ColorChannel::A]);
/// ```
///
impl ::std::ops::Index<ColorChannel> for Color {
    type Output = u8;
    fn index(&self, index: ColorChannel) -> &Self::Output {
        match index {
            ColorChannel::R => &self.r,
            ColorChannel::G => &self.g,
            ColorChannel::B => &self.b,
            ColorChannel::A => &self.a,
        }
    }
}

/// Represents possible color channels in a RGBA color.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ColorChannel {
    R,
    G,
    B,
    A,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_index_ut() {
        let color = Color { r: 1, g: 2, b: 3, a: 4 };
        assert_eq!(1, color[ColorChannel::R]);
        assert_eq!(2, color[ColorChannel::G]);
        assert_eq!(3, color[ColorChannel::B]);
        assert_eq!(4, color[ColorChannel::A]);
    }
}
