//! Extended named colours providing shades collected in enums for the main colour
//!

use std::fmt;

use rgb::Rgb;

use crate::Prefix;

/// Shades of black
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Black {
    SlateGray,
    LightSlateGray,
    Black,
    DimGray,
    DimGrey,
    Gray,
    Grey,
    DarkGray,
    DarkGrey,
    Silver,
    LightGray,
    LightGrey,
    Gainsboro,
}

impl fmt::Display for Black {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Black::SlateGray => write!(f, "#708090"),
            Black::LightSlateGray => write!(f, "#778899"),
            Black::Black => write!(f, "#000000"),
            Black::DimGray => write!(f, "#696969"),
            Black::DimGrey => write!(f, "#696969"),
            Black::Gray => write!(f, "#808080"),
            Black::Grey => write!(f, "#808080"),
            Black::DarkGray => write!(f, "#A9A9A9"),
            Black::DarkGrey => write!(f, "#A9A9A9"),
            Black::Silver => write!(f, "#C0C0C0"),
            Black::LightGray => write!(f, "#D3D3D3"),
            Black::LightGrey => write!(f, "#D3D3D3"),
            Black::Gainsboro => write!(f, "#DCDCDC"),
        }
    }
}

impl Black {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Black;
    /// # fn example() {
    ///
    /// assert_eq!("(112,128,144)", Black::SlateGray.as_rgb())
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
            Black::SlateGray => crate::to_rgb("#708090"),
            Black::LightSlateGray => crate::to_rgb("#778899"),
            Black::Black => crate::to_rgb("#000000"),
            Black::DimGray => crate::to_rgb("#696969"),
            Black::DimGrey => crate::to_rgb("#696969"),
            Black::Gray => crate::to_rgb("#808080"),
            Black::Grey => crate::to_rgb("#808080"),
            Black::DarkGray => crate::to_rgb("#A9A9A9"),
            Black::DarkGrey => crate::to_rgb("#A9A9A9"),
            Black::Silver => crate::to_rgb("#C0C0C0"),
            Black::LightGray => crate::to_rgb("#D3D3D3"),
            Black::LightGrey => crate::to_rgb("#D3D3D3"),
            Black::Gainsboro => crate::to_rgb("#DCDCDC"),
        }
    }

    /// Display the colour name as an RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Black;
    /// # fn main() {
    ///    let colour = Black::Gainsboro;
    ///    let rgb_colour = colour.to_rgb();
    ///
    ///    let string = rgb_colour.to_string();
    ///    assert_eq!("rgb(220,220,220)", string);
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
    /// # use named_colour::ext::Black;
    /// # use named_colour::Prefix;
    ///    let colour = Black::Gainsboro;
    ///
    ///     assert_eq!("#DCDCDC", colour.to_hex_triplet(Prefix::Hash));
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
    #[case(Black::SlateGray, "rgb(112,128,144)")]
    #[case(Black::LightSlateGray, "rgb(119,136,153)")]
    #[case(Black::Black, "rgb(0,0,0)")]
    #[case(Black::DimGray, "rgb(105,105,105)")]
    #[case(Black::DimGrey, "rgb(105,105,105)")]
    #[case(Black::Gray, "rgb(128,128,128)")]
    #[case(Black::Grey, "rgb(128,128,128)")]
    #[case(Black::DarkGray, "rgb(169,169,169)")]
    #[case(Black::DarkGrey, "rgb(169,169,169)")]
    #[case(Black::Silver, "rgb(192,192,192)")]
    #[case(Black::LightGray, "rgb(211,211,211)")]
    #[case(Black::LightGrey, "rgb(211,211,211)")]
    #[case(Black::Gainsboro, "rgb(220,220,220)")]
    fn test_rgb_string(#[case] colour: Black, #[case] expected: String) {
        let rgb_colour = colour.to_rgb();
        let string = rgb_colour.to_string();

        assert_eq!(expected, string);
    }

    #[rstest]
    #[case(Black::SlateGray, "708090")]
    #[case(Black::LightSlateGray, "778899")]
    #[case(Black::Black, "000000")]
    #[case(Black::DimGray, "696969")]
    #[case(Black::DimGrey, "696969")]
    #[case(Black::Gray, "808080")]
    #[case(Black::Grey, "808080")]
    #[case(Black::DarkGray, "A9A9A9")]
    #[case(Black::DarkGrey, "A9A9A9")]
    #[case(Black::Silver, "C0C0C0")]
    #[case(Black::LightGray, "D3D3D3")]
    #[case(Black::LightGrey, "D3D3D3")]
    #[case(Black::Gainsboro, "DCDCDC")]
    fn test_hex_triplet_string(
        #[case] colour: Black,
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
