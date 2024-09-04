//! Extended named colours providing shades collected in enums for the main colour
//!

use std::{fmt, str::FromStr};

use rgb::Rgb;

use crate::Prefix;

use super::ExtendedColour;

/// Shades of black
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Black {
    SlateGray,
    SlateGrey,
    LightSlateGray,
    LightSlateGrey,
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
            Black::SlateGrey => write!(f, "#708090"),
            Black::LightSlateGray => write!(f, "#778899"),
            Black::LightSlateGrey => write!(f, "#778899"),
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

    /// Parse a colour from string
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Black;
    /// # fn main() {
    ///    let colour = Black::parse("gainsboro");
    ///
    ///    assert_eq!(Some(Black::Gainsboro), colour);
    /// # }
    /// ```
    ///
    pub fn parse(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "#708090" | "708090" | "slategray" => Some(Self::SlateGray),
            "slategrey" => Some(Self::SlateGrey),
            "#778899" | "778899" | "lightslategray" => Some(Self::LightSlateGray),
            "lightslategrey" => Some(Self::LightSlateGrey),
            "#000000" | "000000" | "black" => Some(Self::Black),
            "#696969" | "696969" | "dimgray" => Some(Self::DimGray),
            "dimgrey" => Some(Self::DimGrey),
            "#808080" | "808080" | "gray" => Some(Self::Gray),
            "grey" => Some(Self::Grey),
            "#a9a9a9" | "a9a9a9" | "darkgray" => Some(Self::DarkGray),
            "darkgrey" => Some(Self::DarkGrey),
            "#c0c0c0" | "c0c0c0" | "silver" => Some(Self::Silver),
            "#d3d3d3" | "d3d3d3" | "lightgray" => Some(Self::LightGray),
            "lightgrey" => Some(Self::LightGrey),
            "#dcdcdc" | "dcdcdc" | "gainsboro" => Some(Self::Gainsboro),
            _ => None,
        }
    }
}

impl FromStr for Black {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse(s) {
            Some(colour) => Ok(colour),
            None => Err(format!("Invalid Colour: {}", s)),
        }
    }
}

impl ExtendedColour for Black {
    fn name_colour(colour: &str) -> Option<Self>
    where
        Self: Sized,
    {
        if let Ok(c) = Self::from_str(colour) {
            Some(c)
        } else {
            None
        }
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

    #[rstest]
    #[case("#708090", Black::SlateGray)]
    #[case("708090", Black::SlateGray)]
    #[case("slategray", Black::SlateGray)]
    #[case("slategrey", Black::SlateGrey)]
    #[case("#778899", Black::LightSlateGray)]
    #[case("778899", Black::LightSlateGray)]
    #[case("lightslategray", Black::LightSlateGray)]
    #[case("lightslategrey", Black::LightSlateGrey)]
    #[case("#000000", Black::Black)]
    #[case("000000", Black::Black)]
    #[case("black", Black::Black)]
    #[case("#696969", Black::DimGray)]
    #[case("696969", Black::DimGray)]
    #[case("dimgray", Black::DimGray)]
    #[case("dimgrey", Black::DimGrey)]
    #[case("#808080", Black::Gray)]
    #[case("808080", Black::Gray)]
    #[case("gray", Black::Gray)]
    #[case("grey", Black::Grey)]
    #[case("#A9A9A9", Black::DarkGray)]
    #[case("A9A9A9", Black::DarkGray)]
    #[case("darkgray", Black::DarkGray)]
    #[case("darkgrey", Black::DarkGrey)]
    #[case("#C0C0C0", Black::Silver)]
    #[case("C0C0C0", Black::Silver)]
    #[case("silver", Black::Silver)]
    #[case("#D3D3D3", Black::LightGray)]
    #[case("D3D3D3", Black::LightGray)]
    #[case("lightgray", Black::LightGray)]
    #[case("lightgrey", Black::LightGrey)]
    #[case("#DCDCDC", Black::Gainsboro)]
    #[case("DCDCDC", Black::Gainsboro)]
    #[case("gainsboro", Black::Gainsboro)]
    fn test_parse(#[case] input: &str, #[case] expected: Black) {
        assert_eq!(expected, Black::from_str(input).unwrap())
    }

    #[rstest]
    #[case("#708090", Some(Black::SlateGray))]
    #[case("708090", Some(Black::SlateGray))]
    #[case("slategray", Some(Black::SlateGray))]
    #[case("slategrey", Some(Black::SlateGrey))]
    #[case("#778899", Some(Black::LightSlateGray))]
    #[case("778899", Some(Black::LightSlateGray))]
    #[case("lightslategray", Some(Black::LightSlateGray))]
    #[case("lightslategrey", Some(Black::LightSlateGrey))]
    #[case("#000000", Some(Black::Black))]
    #[case("000000", Some(Black::Black))]
    #[case("black", Some(Black::Black))]
    #[case("#696969", Some(Black::DimGray))]
    #[case("696969", Some(Black::DimGray))]
    #[case("dimgray", Some(Black::DimGray))]
    #[case("dimgrey", Some(Black::DimGrey))]
    #[case("#808080", Some(Black::Gray))]
    #[case("808080", Some(Black::Gray))]
    #[case("gray", Some(Black::Gray))]
    #[case("grey", Some(Black::Grey))]
    #[case("#A9A9A9", Some(Black::DarkGray))]
    #[case("A9A9A9", Some(Black::DarkGray))]
    #[case("darkgray", Some(Black::DarkGray))]
    #[case("darkgrey", Some(Black::DarkGrey))]
    #[case("#C0C0C0", Some(Black::Silver))]
    #[case("C0C0C0", Some(Black::Silver))]
    #[case("silver", Some(Black::Silver))]
    #[case("#D3D3D3", Some(Black::LightGray))]
    #[case("D3D3D3", Some(Black::LightGray))]
    #[case("lightgray", Some(Black::LightGray))]
    #[case("lightgrey", Some(Black::LightGrey))]
    #[case("#DCDCDC", Some(Black::Gainsboro))]
    #[case("DCDCDC", Some(Black::Gainsboro))]
    #[case("gainsboro", Some(Black::Gainsboro))]
    #[case("012345", None)]
    fn test_name_colour(#[case] input: &str, #[case] expected: Option<Black>) {
        assert_eq!(expected, Black::name_colour(input))
    }
}
