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
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }

    /// Convert the colour code to a hex string
    pub fn to_hex_string(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::ColourRgb;

    #[test]
    fn print_valid_hex_string() {
        assert_eq!("#CCCCCC", &ColourRgb::new(204, 204, 204).as_hex());
        let colour = ColourRgb::new(12, 24, 48);
        assert_eq!("#0C1830", colour.as_hex());
        let colour = ColourRgb::new(12, 4, 8);
        assert_eq!("#0C0408", colour.as_hex());
    }

    #[test]
    fn test_to_hex_string() {
        let colour = ColourRgb::new(12, 24, 48);
        assert_eq!("0c1830", colour.to_hex_string());
        let colour = ColourRgb::new(12, 4, 8);
        assert_eq!("0c0408", colour.to_hex_string());
    }
}
