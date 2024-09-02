//! Extended named colours providing shades collected in enums for the main colour
//!

use std::fmt;

use rgb::Rgb;

use crate::Prefix;

/// Shades of brown
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Brown {
    SaddleBrown,
    Sienna,
    Chocolate,
    Peru,
    SandyBrown,
    BurlyWood,
    Tan,
    RosyBrown,
}

impl fmt::Display for Brown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Brown::SaddleBrown => write!(f, "#8B4513"),
            Brown::Sienna => write!(f, "#A0522D"),
            Brown::Chocolate => write!(f, "#D2691E"),
            Brown::Peru => write!(f, "#CD853F"),
            Brown::SandyBrown => write!(f, "#F4A460"),
            Brown::BurlyWood => write!(f, "#DEB887"),
            Brown::Tan => write!(f, "#D2B48C"),
            Brown::RosyBrown => write!(f, "#BC8F8F"),
        }
    }
}

impl Brown {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Brown;
    /// # fn example() {
    /// assert_eq!("(244,164,96)", Brown::SandyBrown.as_rgb())
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
            Brown::SaddleBrown => crate::to_rgb("#8B4513"),
            Brown::Sienna => crate::to_rgb("#A0522D"),
            Brown::Chocolate => crate::to_rgb("#D2691E"),
            Brown::Peru => crate::to_rgb("#CD853F"),
            Brown::SandyBrown => crate::to_rgb("#F4A460"),
            Brown::BurlyWood => crate::to_rgb("#DEB887"),
            Brown::Tan => crate::to_rgb("#D2B48C"),
            Brown::RosyBrown => crate::to_rgb("#BC8F8F"),
        }
    }

    /// Display the colour name as an RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Purple;
    /// # fn main() {
    ///    let colour = Purple::Violet;
    ///    let rgb_colour = colour.to_rgb();
    ///
    ///    let string = rgb_colour.to_string();
    ///    assert_eq!("rgb(238,130,238)", string);
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
    /// # use named_colour::ext::Purple;
    /// # use named_colour::Prefix;
    ///    let colour = Purple::Plum;
    ///
    ///     assert_eq!("#DDA0DD", colour.to_hex_triplet(Prefix::Hash));
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
    #[case(Brown::SaddleBrown, "rgb(139,69,19)")]
    #[case(Brown::Sienna, "rgb(160,82,45)")]
    #[case(Brown::Chocolate, "rgb(210,105,30)")]
    #[case(Brown::Peru, "rgb(205,133,63)")]
    #[case(Brown::SandyBrown, "rgb(244,164,96)")]
    #[case(Brown::BurlyWood, "rgb(222,184,135)")]
    #[case(Brown::Tan, "rgb(210,180,140)")]
    #[case(Brown::RosyBrown, "rgb(188,143,143)")]
    fn test_rgb_string(#[case] colour: Brown, #[case] expected: String) {
        let rgb_colour = colour.to_rgb();
        let string = rgb_colour.to_string();

        assert_eq!(expected, string);
    }

    #[rstest]
    #[case(Brown::SaddleBrown, "8B4513")]
    #[case(Brown::Sienna, "A0522D")]
    #[case(Brown::Chocolate, "D2691E")]
    #[case(Brown::Peru, "CD853F")]
    #[case(Brown::SandyBrown, "F4A460")]
    #[case(Brown::BurlyWood, "DEB887")]
    #[case(Brown::Tan, "D2B48C")]
    #[case(Brown::RosyBrown, "BC8F8F")]
    fn test_hex_triplet_string(
        #[case] colour: Brown,
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
