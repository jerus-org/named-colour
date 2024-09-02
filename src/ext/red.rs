//! Extended named colours providing shades collected in enums for the main colour
//!

use std::fmt;

use rgb::Rgb;

use crate::Prefix;

/// Shades of red
#[allow(missing_docs)]
#[derive(Debug)]
pub enum Red {
    Maroon,
    #[allow(clippy::enum_variant_names)]
    DarkRed,
    Brown,
    Firebrick,
    Crimson,
    #[allow(clippy::enum_variant_names)]
    Red,
    Tomato,
    Coral,
    #[allow(clippy::enum_variant_names)]
    IndianRed,
    LightCoral,
    DarkSalmon,
    Salmon,
    LightSalmon,
    #[allow(clippy::enum_variant_names)]
    OrangeRed,
    DarkOrange,
    Orange,
}

impl fmt::Display for Red {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Red::Maroon => write!(f, "#800000"),
            Red::DarkRed => write!(f, "#8B0000"),
            Red::Brown => write!(f, "#A52A2A"),
            Red::Firebrick => write!(f, "#B22222"),
            Red::Crimson => write!(f, "#DC143C"),
            Red::Red => write!(f, "#FF0000"),
            Red::Tomato => write!(f, "#FF6347"),
            Red::Coral => write!(f, "#FF7F50"),
            Red::IndianRed => write!(f, "#CD5C5C"),
            Red::LightCoral => write!(f, "#F08080"),
            Red::DarkSalmon => write!(f, "#E9967A"),
            Red::Salmon => write!(f, "#FA8072"),
            Red::LightSalmon => write!(f, "#FFA07A"),
            Red::OrangeRed => write!(f, "#FF4500"),
            Red::DarkOrange => write!(f, "#FF8C00"),
            Red::Orange => write!(f, "#FFA500"),
        }
    }
}

impl Red {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Red;
    /// # fn example() {
    /// assert_eq!("(0,255,255)", Red::Firebrick.as_rgb())
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
            Red::Maroon => crate::to_rgb("#800000"),
            Red::DarkRed => crate::to_rgb("#8B0000"),
            Red::Brown => crate::to_rgb("#A52A2A"),
            Red::Firebrick => crate::to_rgb("#B22222"),
            Red::Crimson => crate::to_rgb("#DC143C"),
            Red::Red => crate::to_rgb("#FF0000"),
            Red::Tomato => crate::to_rgb("#FF6347"),
            Red::Coral => crate::to_rgb("#FF7F50"),
            Red::IndianRed => crate::to_rgb("#CD5C5C"),
            Red::LightCoral => crate::to_rgb("#F08080"),
            Red::DarkSalmon => crate::to_rgb("#E9967A"),
            Red::Salmon => crate::to_rgb("#FA8072"),
            Red::LightSalmon => crate::to_rgb("#FFA07A"),
            Red::OrangeRed => crate::to_rgb("#FF4500"),
            Red::DarkOrange => crate::to_rgb("#FF8C00"),
            Red::Orange => crate::to_rgb("#FFA500"),
        }
    }

    /// Display the colour name as an RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Red;
    /// # fn main() {
    ///    let rgb_colour = Red::Maroon.to_rgb();
    ///    let string = rgb_colour.to_string();
    ///    assert_eq!("rgb(128,0,0)", string);
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
    /// # use named_colour::ext::Red;
    /// # use named_colour::Prefix;
    ///     let colour = Red::Maroon;
    ///
    ///     assert_eq!("#800000", colour.to_hex_triplet(Prefix::Hash));
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
    #[case(Red::Maroon, "rgb(128,0,0)")]
    #[case(Red::DarkRed, "rgb(139,0,0)")]
    #[case(Red::Brown, "rgb(165,42,42)")]
    #[case(Red::Firebrick, "rgb(178,34,34)")]
    #[case(Red::Crimson, "rgb(220,20,60)")]
    #[case(Red::Red, "rgb(255,0,0)")]
    #[case(Red::Tomato, "rgb(255,99,71)")]
    #[case(Red::Coral, "rgb(255,127,80)")]
    #[case(Red::IndianRed, "rgb(205,92,92)")]
    #[case(Red::LightCoral, "rgb(240,128,128)")]
    #[case(Red::DarkSalmon, "rgb(233,150,122)")]
    #[case(Red::Salmon, "rgb(250,128,114)")]
    #[case(Red::LightSalmon, "rgb(255,160,122)")]
    #[case(Red::OrangeRed, "rgb(255,69,0)")]
    #[case(Red::DarkOrange, "rgb(255,140,0)")]
    #[case(Red::Orange, "rgb(255,165,0)")]
    fn test_rgb_string(#[case] colour: Red, #[case] expected: String) {
        let rgb_colour = colour.to_rgb();
        let string = rgb_colour.to_string();

        assert_eq!(expected, string);
    }

    #[rstest]
    #[case(Red::Maroon, "800000")]
    #[case(Red::DarkRed, "8B0000")]
    #[case(Red::Brown, "A52A2A")]
    #[case(Red::Firebrick, "B22222")]
    #[case(Red::Crimson, "DC143C")]
    #[case(Red::Red, "FF0000")]
    #[case(Red::Tomato, "FF6347")]
    #[case(Red::Coral, "FF7F50")]
    #[case(Red::IndianRed, "CD5C5C")]
    #[case(Red::LightCoral, "F08080")]
    #[case(Red::DarkSalmon, "E9967A")]
    #[case(Red::Salmon, "FA8072")]
    #[case(Red::LightSalmon, "FFA07A")]
    #[case(Red::OrangeRed, "FF4500")]
    #[case(Red::DarkOrange, "FF8C00")]
    #[case(Red::Orange, "FFA500")]
    fn test_hex_triplet_string(
        #[case] colour: Red,
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
