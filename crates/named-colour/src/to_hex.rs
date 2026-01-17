use rgb::Rgb;

/// Implement the `ToHex` trait
///
/// Provides interfaces for functions to display hex versions of RGB colours
///
/// An implementation is provided for `Rgb<u8>`
///
pub trait ToHex {
    /// Return the colour code as an uppercase hex string with a # prefix
    fn as_hex(&self) -> String;
    /// Return the colour code as a lowercase hex string with no prefix
    fn to_hex_string(&self) -> String;
}

impl ToHex for Rgb<u8> {
    fn as_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    fn to_hex_string(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::ToHex;
    use rgb::Rgb;

    #[test]
    fn print_valid_hex_string_for_rgb_u8() {
        assert_eq!("#CCCCCC", &Rgb::new(204, 204, 204).as_hex());
        let colour = Rgb::new(12, 24, 48);
        assert_eq!("#0C1830", colour.as_hex());
        let colour = Rgb::new(12, 4, 8);
        assert_eq!("#0C0408", colour.as_hex());
    }

    #[test]
    fn test_to_hex_string_for_rgb_u8() {
        let colour = Rgb::new(12, 24, 48);
        assert_eq!("0c1830", colour.to_hex_string());
        let colour = Rgb::new(12, 4, 8);
        assert_eq!("0c0408", colour.to_hex_string());
    }
}
