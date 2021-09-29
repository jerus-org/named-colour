use std::fmt;

type ColourCode = u8;

/// Store RGB colour specification
///
/// ## Example
///
///```
/// use named_colour::ColourRgb;
/// let my_colour =ColourRgb::new(12,24,48);
/// println!("The Hex Code is: {} for my_colour: {}",
///     my_colour.as_hex(),
///     my_colour.to_string()
/// );
///```
///
pub struct ColourRgb {
    red: ColourCode,
    green: ColourCode,
    blue: ColourCode,
}

impl fmt::Display for ColourRgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.red, self.green, self.blue)
    }
}

impl ColourRgb {
    /// Create a new RGB colour from three colour codes.
    /// ColourCode is a synonym for u8.
    pub fn new(red: ColourCode, green: ColourCode, blue: ColourCode) -> Self {
        Self { red, green, blue }
    }

    /// Display the colour code using the hex code format
    pub fn as_hex(&self) -> String {
        format!("#{:X}{:X}{:X}", self.red, self.green, self.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::ColourRgb;

    #[test]
    fn print_valid_hex_string() {
        assert_eq!("#CCCCCC", &ColourRgb::new(204, 204, 204).as_hex())
    }
}
