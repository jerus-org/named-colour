//! Extended named colours providing shades collected in enums for the main colour
//!

use std::{fmt, str::FromStr};

use rgb::Rgb;

use crate::Prefix;

use super::ExtendedColour;

/// Shades of yellow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Yellow {
    Gold,
    DarkGoldenrod,
    Goldenrod,
    PaleGoldenrod,
    DarkKhaki,
    Khaki,
    Yellow,
    YellowGreen,
    PeachPuff,
    Moccasin,
    PapayaWhip,
    LightGoldenrodYellow,
    LemonChiffon,
    LightYellow,
}

impl fmt::Display for Yellow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gold => write!(f, "#FFD700"),
            Self::DarkGoldenrod => write!(f, "#B8860B"),
            Self::Goldenrod => write!(f, "#DAA520"),
            Self::PaleGoldenrod => write!(f, "#EEE8AA"),
            Self::PeachPuff => write!(f, "#FFDAB9"),
            Self::Moccasin => write!(f, "#FFE4B5"),
            Self::PapayaWhip => write!(f, "#FFEFD5"),
            Self::DarkKhaki => write!(f, "#BDB76B"),
            Self::LemonChiffon => write!(f, "#FFFACD"),
            Self::LightGoldenrodYellow => write!(f, "#FAFAD2"),
            Self::Khaki => write!(f, "#F0E68C"),
            Self::Yellow => write!(f, "#FFFF00"),
            Self::YellowGreen => write!(f, "#9ACD32"),
            Self::LightYellow => write!(f, "#FFFFE0"),
        }
    }
}

impl Yellow {
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

    pub fn parse(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "#ffd700" | "ffd700" | "gold" => Some(Self::Gold),
            "#b8860b" | "b8860b" | "darkgoldenrod" => Some(Self::DarkGoldenrod),
            "#dab500" | "dab500" | "goldenrod" => Some(Self::Goldenrod),
            "#eee8aa" | "eee8aa" | "palegoldenrod" => Some(Self::PaleGoldenrod),
            "#bdb76b" | "bdb76b" | "darkkhaki" => Some(Self::DarkKhaki),
            "#f0e68c" | "f0e68c" | "khaki" => Some(Self::Khaki),
            "#ffff00" | "ffff00" | "yellow" => Some(Self::Yellow),
            "#9acd32" | "9acd32" | "yellowgreen" => Some(Self::YellowGreen),
            "#ffdab9" | "ffdab9" | "peachpuff" => Some(Self::PeachPuff),
            "#ffe4b5" | "ffe4b5" | "moccasin" => Some(Self::Moccasin),
            "#ffefd5" | "ffefd5" | "papayawhip" => Some(Self::PapayaWhip),
            "#fffacd" | "fffacd" | "lemonchiffon" => Some(Self::LemonChiffon),
            "#fafad2" | "fafad2" | "lightgoldenrodyellow" => Some(Self::LightGoldenrodYellow),
            "#ffffe0" | "ffffe0" | "lightyellow" => Some(Self::LightYellow),
            _ => None,
        }
    }
}

impl FromStr for Yellow {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse(s) {
            Some(colour) => Ok(colour),
            None => Err(format!("Invalid Colour: {}", s)),
        }
    }
}

impl ExtendedColour for Yellow {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Yellow::Gold, "rgb(255,215,0)")]
    #[case(Yellow::DarkGoldenrod, "rgb(184,134,11)")]
    #[case(Yellow::Goldenrod, "rgb(218,165,32)")]
    #[case(Yellow::PaleGoldenrod, "rgb(238,232,170)")]
    #[case(Yellow::DarkKhaki, "rgb(189,183,107)")]
    #[case(Yellow::Khaki, "rgb(240,230,140)")]
    #[case(Yellow::Yellow, "rgb(255,255,0)")]
    #[case(Yellow::YellowGreen, "rgb(154,205,50)")]
    #[case(Yellow::PeachPuff, "rgb(255,218,185)")]
    #[case(Yellow::Moccasin, "rgb(255,228,181)")]
    #[case(Yellow::PapayaWhip, "rgb(255,239,213)")]
    #[case(Yellow::LemonChiffon, "rgb(255,250,205)")]
    #[case(Yellow::LightGoldenrodYellow, "rgb(250,250,210)")]
    #[case(Yellow::LightYellow, "rgb(255,255,224)")]
    fn test_rgb_string(#[case] colour: Yellow, #[case] expected: String) {
        let rgb_colour = colour.to_rgb();
        let string = rgb_colour.to_string();

        assert_eq!(expected, string);
    }

    #[rstest]
    #[case(Yellow::Gold, "FFD700")]
    #[case(Yellow::DarkGoldenrod, "B8860B")]
    #[case(Yellow::Goldenrod, "DAA520")]
    #[case(Yellow::PaleGoldenrod, "EEE8AA")]
    #[case(Yellow::DarkKhaki, "BDB76B")]
    #[case(Yellow::Khaki, "F0E68C")]
    #[case(Yellow::Yellow, "FFFF00")]
    #[case(Yellow::YellowGreen, "9ACD32")]
    #[case(Yellow::PeachPuff, "FFDAB9")]
    #[case(Yellow::Moccasin, "FFE4B5")]
    #[case(Yellow::PapayaWhip, "FFEFD5")]
    #[case(Yellow::LemonChiffon, "FFFACD")]
    #[case(Yellow::LightGoldenrodYellow, "FAFAD2")]
    #[case(Yellow::LightYellow, "FFFFE0")]
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

    #[rstest]
    #[case("#ffd700", Yellow::Gold)]
    #[case("ffd700", Yellow::Gold)]
    #[case("Gold", Yellow::Gold)]
    #[case("#b8860b", Yellow::DarkGoldenrod)]
    #[case("b8860b", Yellow::DarkGoldenrod)]
    #[case("DarkGoldenRod", Yellow::DarkGoldenrod)]
    #[case("#dab500", Yellow::Goldenrod)]
    #[case("dab500", Yellow::Goldenrod)]
    #[case("GoldenRod", Yellow::Goldenrod)]
    #[case("#eee8aa", Yellow::PaleGoldenrod)]
    #[case("eee8aa", Yellow::PaleGoldenrod)]
    #[case("PaleGoldenRod", Yellow::PaleGoldenrod)]
    #[case("#bdb76b", Yellow::DarkKhaki)]
    #[case("bdb76b", Yellow::DarkKhaki)]
    #[case("DarkKhaki", Yellow::DarkKhaki)]
    #[case("#f0e68c", Yellow::Khaki)]
    #[case("f0e68c", Yellow::Khaki)]
    #[case("Khaki", Yellow::Khaki)]
    #[case("#ffff00", Yellow::Yellow)]
    #[case("ffff00", Yellow::Yellow)]
    #[case("Yellow", Yellow::Yellow)]
    #[case("#9acd32", Yellow::YellowGreen)]
    #[case("9acd32", Yellow::YellowGreen)]
    #[case("YellowGreen", Yellow::YellowGreen)]
    #[case("#ffdab9", Yellow::PeachPuff)]
    #[case("ffdab9", Yellow::PeachPuff)]
    #[case("PeachPuff", Yellow::PeachPuff)]
    #[case("#ffe4b5", Yellow::Moccasin)]
    #[case("ffe4b5", Yellow::Moccasin)]
    #[case("Moccasin", Yellow::Moccasin)]
    #[case("#ffefd5", Yellow::PapayaWhip)]
    #[case("ffefd5", Yellow::PapayaWhip)]
    #[case("PapayaWhip", Yellow::PapayaWhip)]
    #[case("#fffacd", Yellow::LemonChiffon)]
    #[case("fffacd", Yellow::LemonChiffon)]
    #[case("LemonChiffon", Yellow::LemonChiffon)]
    #[case("#fafad2", Yellow::LightGoldenrodYellow)]
    #[case("fafad2", Yellow::LightGoldenrodYellow)]
    #[case("LightGoldenrodYellow", Yellow::LightGoldenrodYellow)]
    #[case("#ffffe0", Yellow::LightYellow)]
    #[case("ffffe0", Yellow::LightYellow)]
    #[case("LightYellow", Yellow::LightYellow)]
    fn test_from_str(#[case] input: &str, #[case] expected: Yellow) {
        assert_eq!(expected, Yellow::from_str(input).unwrap())
    }

    #[rstest]
    #[case("#ffd700", Some(Yellow::Gold))]
    #[case("ffd700", Some(Yellow::Gold))]
    #[case("Gold", Some(Yellow::Gold))]
    #[case("#b8860b", Some(Yellow::DarkGoldenrod))]
    #[case("b8860b", Some(Yellow::DarkGoldenrod))]
    #[case("DarkGoldenRod", Some(Yellow::DarkGoldenrod))]
    #[case("#dab500", Some(Yellow::Goldenrod))]
    #[case("dab500", Some(Yellow::Goldenrod))]
    #[case("GoldenRod", Some(Yellow::Goldenrod))]
    #[case("#eee8aa", Some(Yellow::PaleGoldenrod))]
    #[case("eee8aa", Some(Yellow::PaleGoldenrod))]
    #[case("PaleGoldenRod", Some(Yellow::PaleGoldenrod))]
    #[case("#bdb76b", Some(Yellow::DarkKhaki))]
    #[case("bdb76b", Some(Yellow::DarkKhaki))]
    #[case("DarkKhaki", Some(Yellow::DarkKhaki))]
    #[case("#f0e68c", Some(Yellow::Khaki))]
    #[case("f0e68c", Some(Yellow::Khaki))]
    #[case("Khaki", Some(Yellow::Khaki))]
    #[case("#ffff00", Some(Yellow::Yellow))]
    #[case("ffff00", Some(Yellow::Yellow))]
    #[case("Yellow", Some(Yellow::Yellow))]
    #[case("#9acd32", Some(Yellow::YellowGreen))]
    #[case("9acd32", Some(Yellow::YellowGreen))]
    #[case("YellowGreen", Some(Yellow::YellowGreen))]
    #[case("#ffdab9", Some(Yellow::PeachPuff))]
    #[case("ffdab9", Some(Yellow::PeachPuff))]
    #[case("PeachPuff", Some(Yellow::PeachPuff))]
    #[case("#ffe4b5", Some(Yellow::Moccasin))]
    #[case("ffe4b5", Some(Yellow::Moccasin))]
    #[case("Moccasin", Some(Yellow::Moccasin))]
    #[case("#ffefd5", Some(Yellow::PapayaWhip))]
    #[case("ffefd5", Some(Yellow::PapayaWhip))]
    #[case("PapayaWhip", Some(Yellow::PapayaWhip))]
    #[case("#fffacd", Some(Yellow::LemonChiffon))]
    #[case("fffacd", Some(Yellow::LemonChiffon))]
    #[case("LemonChiffon", Some(Yellow::LemonChiffon))]
    #[case("#fafad2", Some(Yellow::LightGoldenrodYellow))]
    #[case("fafad2", Some(Yellow::LightGoldenrodYellow))]
    #[case("LightGoldenrodYellow", Some(Yellow::LightGoldenrodYellow))]
    #[case("#ffffe0", Some(Yellow::LightYellow))]
    #[case("ffffe0", Some(Yellow::LightYellow))]
    #[case("LightYellow", Some(Yellow::LightYellow))]
    #[case("012345", None)]
    fn test_name_colour(#[case] input: &str, #[case] expected: Option<Yellow>) {
        assert_eq!(expected, Yellow::name_colour(input))
    }
}
