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

/// Shades of yellow
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Yellow {
    Gold,
    DarkGoldenRod,
    GoldenRod,
    PaleGoldenRod,
    DarkKhaki,
    Khaki,
    Olive,
    Yellow,
    YellowGreen,
}

impl fmt::Display for Yellow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Yellow::Gold => write!(f, "#FFD700"),
            Yellow::DarkGoldenRod => write!(f, "#B8860B"),
            Yellow::GoldenRod => write!(f, "#DAA520"),
            Yellow::PaleGoldenRod => write!(f, "#EEE8AA"),
            Yellow::DarkKhaki => write!(f, "#BDB76B"),
            Yellow::Khaki => write!(f, "#F0E68C"),
            Yellow::Olive => write!(f, "#808000"),
            Yellow::Yellow => write!(f, "#FFFF00"),
            Yellow::YellowGreen => write!(f, "#9ACD32"),
        }
    }
}

impl Yellow {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Yellow;
    /// # fn example() {
    /// assert_eq!("(178,34,34)", Yellow::Khaki.as_rgb())
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
            Yellow::Gold => crate::to_rgb("#FFD700"),
            Yellow::DarkGoldenRod => crate::to_rgb("#B8860B"),
            Yellow::GoldenRod => crate::to_rgb("#DAA520"),
            Yellow::PaleGoldenRod => crate::to_rgb("#EEE8AA"),
            Yellow::DarkKhaki => crate::to_rgb("#BDB76B"),
            Yellow::Khaki => crate::to_rgb("#F0E68C"),
            Yellow::Olive => crate::to_rgb("#808000"),
            Yellow::Yellow => crate::to_rgb("#FFFF00"),
            Yellow::YellowGreen => crate::to_rgb("#9ACD32"),
        }
    }

    /// Display the colour name as an RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Yellow;
    /// # fn main() {
    ///    let rgb_colour = Yellow::Khaki.to_rgb();
    ///
    ///    let string = rgb_colour.to_string();
    ///    assert_eq!("rgb(240,230,140)", string);
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
    /// # use named_colour::ext::Yellow;
    /// # use named_colour::Prefix;
    ///     let colour = Yellow::Khaki;
    ///
    ///     assert_eq!("#F0E68C", colour.to_hex_triplet(Prefix::Hash));
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
    #[case(Yellow::Gold, "rgb(255,215,0)")]
    #[case(Yellow::DarkGoldenRod, "rgb(184,134,11)")]
    #[case(Yellow::GoldenRod, "rgb(218,165,32)")]
    #[case(Yellow::PaleGoldenRod, "rgb(238,232,170)")]
    #[case(Yellow::DarkKhaki, "rgb(189,183,107)")]
    #[case(Yellow::Khaki, "rgb(240,230,140)")]
    #[case(Yellow::Olive, "rgb(128,128,0)")]
    #[case(Yellow::Yellow, "rgb(255,255,0)")]
    #[case(Yellow::YellowGreen, "rgb(154,205,50)")]
    fn test_rgb_string(#[case] colour: Yellow, #[case] expected: String) {
        let rgb_colour = colour.to_rgb();
        let string = rgb_colour.to_string();

        assert_eq!(expected, string);
    }

    #[rstest]
    #[case(Yellow::Gold, "FFD700")]
    #[case(Yellow::DarkGoldenRod, "B8860B")]
    #[case(Yellow::GoldenRod, "DAA520")]
    #[case(Yellow::PaleGoldenRod, "EEE8AA")]
    #[case(Yellow::DarkKhaki, "BDB76B")]
    #[case(Yellow::Khaki, "F0E68C")]
    #[case(Yellow::Olive, "808000")]
    #[case(Yellow::Yellow, "FFFF00")]
    #[case(Yellow::YellowGreen, "9ACD32")]
    fn test_hex_triplet_string(
        #[case] colour: Yellow,
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
