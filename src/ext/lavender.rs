//! Extended named colours providing shades collected in enums for the main colour
//!

use std::fmt;

use rgb::Rgb;

use crate::Prefix;

/// Shades of lavender
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Lavender {
    SlateGray,
    LightSlateGray,
    LightSteelBlue,
    Lavender,
    AliceBlue,
}

impl fmt::Display for Lavender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lavender::SlateGray => write!(f, "#708090"),
            Lavender::LightSlateGray => write!(f, "#778899"),
            Lavender::LightSteelBlue => write!(f, "#B0C4DE"),
            Lavender::Lavender => write!(f, "#E6E6FA"),
            Lavender::AliceBlue => write!(f, "#F0F8FF"),
        }
    }
}

impl Lavender {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Lavender;
    /// # fn example() {
    /// assert_eq!("(230,230,250)", Lavender::Lavender.as_rgb())
    ///
    /// # }
    ///```
    #[deprecated(
        since = "0.2.0",
        note = r#"Use `to_rgb` for Rgb struct an then `to_string()` to display as decimal Rgb triplet instead
        Will be removed in 0.3.0"#
    )]
    pub fn as_rgb(&self) -> String {
        match self {
            Lavender::SlateGray => crate::to_rgb("#708090"),
            Lavender::LightSlateGray => crate::to_rgb("#778899"),
            Lavender::LightSteelBlue => crate::to_rgb("#B0C4DE"),
            Lavender::Lavender => crate::to_rgb("#E6E6FA"),
            Lavender::AliceBlue => crate::to_rgb("#F0F8FF"),
        }
    }

    /// Display the colour name as an RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Lavender;
    /// # fn main() {
    ///    let colour = Lavender::SlateGray;
    ///    let rgb_colour = colour.to_rgb();
    ///
    ///    let string = rgb_colour.to_string();
    ///    assert_eq!("rgb(112,128,144)", string);
    ///
    ///  # }
    ///```

    pub fn to_rgb(&self) -> Rgb<u8> {
        let colour = self.to_string();

        let r: u8 = u8::from_str_radix(&colour[1..3], 16).unwrap();
        let g: u8 = u8::from_str_radix(&colour[3..5], 16).unwrap();
        let b: u8 = u8::from_str_radix(&colour[5..7], 16).unwrap();

        Rgb::new(r, g, b)
    }

    /// Display the colour name as an RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Lavender;
    /// # use named_colour::Prefix;
    ///    let colour = Lavender::SlateGray;
    ///
    ///     assert_eq!("#708090", colour.to_hex_triplet(Prefix::Hash));
    ///
    ///```

    pub fn to_hex_triplet(&self, prefix: Prefix) -> String {
        let rgb = self.to_rgb();

        let prefix = match prefix {
            Prefix::Hash => "#",
            Prefix::None => "",
        };

        format!("{}{:02X}{:02X}{:02X}", prefix, rgb.r, rgb.g, rgb.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Lavender::SlateGray, "rgb(112,128,144)")]
    #[case(Lavender::LightSlateGray, "rgb(119,136,153)")]
    #[case(Lavender::LightSteelBlue, "rgb(176,196,222)")]
    #[case(Lavender::Lavender, "rgb(230,230,250)")]
    #[case(Lavender::AliceBlue, "rgb(240,248,255)")]
    fn test_rgb_string(#[case] colour: Lavender, #[case] expected: String) {
        let rgb_colour = colour.to_rgb();
        let string = rgb_colour.to_string();

        assert_eq!(expected, string);
    }

    #[rstest]
    #[case(Lavender::SlateGray, "708090")]
    #[case(Lavender::LightSlateGray, "778899")]
    #[case(Lavender::LightSteelBlue, "B0C4DE")]
    #[case(Lavender::Lavender, "E6E6FA")]
    #[case(Lavender::AliceBlue, "F0F8FF")]
    fn test_hex_triplet_string(
        #[case] colour: Lavender,
        #[values(Prefix::None, Prefix::Hash)] prefix: Prefix,
        #[case] expected: String,
    ) {
        let prefix_string = match prefix {
            Prefix::None => "".to_string(),
            Prefix::Hash => "#".to_string(),
        };

        let expected = format!("{}{}", prefix_string, expected);

        let hex_colour = colour.to_hex_triplet(prefix);

        assert_eq!(expected, hex_colour);
    }
}
