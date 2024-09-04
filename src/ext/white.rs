//! Extended named colours providing shades collected in enums for the main colour
//!

use std::{fmt, str::FromStr};

use rgb::Rgb;

use crate::Prefix;

/// Shades of white
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum White {
    AntiqueWhite,
    Beige,
    Bisque,
    BlanchedAlmond,
    Wheat,
    CornSilk,
    White,
    NavajoWhite,
    MistyRose,
    LavenderBlush,
    Linen,
    OldLace,
    SeaShell,
    MintCream,
    FloralWhite,
    GhostWhite,
    Ivory,
    Snow,
    WhiteSmoke,
    AliceBlue,
}

impl fmt::Display for White {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AntiqueWhite => write!(f, "#FAEBD7"),
            Self::Beige => write!(f, "#F5F5DC"),
            Self::Bisque => write!(f, "#FFE4C4"),
            Self::BlanchedAlmond => write!(f, "#FFEBCD"),
            Self::Wheat => write!(f, "#F5DEB3"),
            Self::CornSilk => write!(f, "#FFF8DC"),
            Self::White => write!(f, "#FFFFFF"),
            Self::NavajoWhite => write!(f, "#FFDEAD"),
            Self::MistyRose => write!(f, "#FFE4E1"),
            Self::LavenderBlush => write!(f, "#FFF0F5"),
            Self::Linen => write!(f, "#FAF0E6"),
            Self::OldLace => write!(f, "#FDF5E6"),
            Self::SeaShell => write!(f, "#FFF5EE"),
            Self::MintCream => write!(f, "#F5FFFA"),
            Self::FloralWhite => write!(f, "#FFFAF0"),
            Self::GhostWhite => write!(f, "#F8F8FF"),
            Self::Ivory => write!(f, "#FFFFF0"),
            Self::Snow => write!(f, "#FFFAFA"),
            Self::WhiteSmoke => write!(f, "#F5F5F5"),
            Self::AliceBlue => write!(f, "#F0F8FF"),
        }
    }
}

impl White {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::White;
    /// # fn example() {
    /// assert_eq!("(250,240,230)", White::Linen.as_rgb())
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
            White::AntiqueWhite => crate::to_rgb("#FAEBD7"),
            White::Beige => crate::to_rgb("#F5F5DC"),
            White::Bisque => crate::to_rgb("#FFE4C4"),
            White::BlanchedAlmond => crate::to_rgb("#FFEBCD"),
            White::Wheat => crate::to_rgb("#F5DEB3"),
            White::CornSilk => crate::to_rgb("#FFF8DC"),
            White::White => crate::to_rgb("#FFFFFF"),
            White::NavajoWhite => crate::to_rgb("#FFDEAD"),
            White::MistyRose => crate::to_rgb("#FFE4E1"),
            White::LavenderBlush => crate::to_rgb("#FFF0F5"),
            White::Linen => crate::to_rgb("#FAF0E6"),
            White::OldLace => crate::to_rgb("#FDF5E6"),
            White::SeaShell => crate::to_rgb("#FFF5EE"),
            White::MintCream => crate::to_rgb("#F5FFFA"),
            White::FloralWhite => crate::to_rgb("#FFFAF0"),
            White::GhostWhite => crate::to_rgb("#F8F8FF"),
            White::Ivory => crate::to_rgb("#FFFFF0"),
            White::Snow => crate::to_rgb("#FFFAFA"),
            White::WhiteSmoke => crate::to_rgb("#F5F5F5"),
            White::AliceBlue => crate::to_rgb("#F0F8FF"),
        }
    }

    /// Display the colour name as an RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::White;
    /// # fn main() {
    ///    let colour = White::OldLace;
    ///    let rgb_colour = colour.to_rgb();
    ///
    ///    let string = rgb_colour.to_string();
    ///    assert_eq!("rgb(253,245,230)", string);
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
    /// # use named_colour::ext::White;
    /// # use named_colour::Prefix;
    ///    let colour = White::LavenderBlush;
    ///
    ///     assert_eq!("#FFF0F5", colour.to_hex_triplet(Prefix::Hash));
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
    /// # use named_colour::ext::White;
    /// # fn main() {
    ///    let colour = White::parse("AntiqueWhite");
    ///    assert_eq!(Some(White::AntiqueWhite), colour);
    ///
    ///  # }
    ///```
    pub fn parse(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "#faebd7" | "faebd7" | "antiquewhite" => Some(Self::AntiqueWhite),
            "#f5f5dc" | "f5f5dc" | "beige" => Some(Self::Beige),
            "#ffe4c4" | "ffe4c4" | "bisque" => Some(Self::Bisque),
            "#ffebcd" | "ffebcd" | "blanchedalmond" => Some(Self::BlanchedAlmond),
            "#f5deb3" | "f5deb3" | "wheat" => Some(Self::Wheat),
            "#fff8dc" | "fff8dc" | "cornsilk" => Some(Self::CornSilk),
            "#ffffff" | "ffffff" | "white" => Some(Self::White),
            "#ffdead" | "ffdead" | "navajowhite" => Some(Self::NavajoWhite),
            "#ffe4e1" | "ffe4e1" | "mistyrose" => Some(Self::MistyRose),
            "#fff0f5" | "fff0f5" | "lavenderblush" => Some(Self::LavenderBlush),
            "#faf0e6" | "faf0e6" | "linen" => Some(Self::Linen),
            "#fdf5e6" | "fdf5e6" | "oldlace" => Some(Self::OldLace),
            "#fff5ee" | "fff5ee" | "seashell" => Some(Self::SeaShell),
            "#f5fffa" | "f5fffa" | "mintcream" => Some(Self::MintCream),
            "#fffaf0" | "fffaf0" | "floralwhite" => Some(Self::FloralWhite),
            "#f8f8ff" | "f8f8ff" | "ghostwhite" => Some(Self::GhostWhite),
            "#fffff0" | "fffff0" | "ivory" => Some(Self::Ivory),
            "#fffafa" | "fffafa" | "snow" => Some(Self::Snow),
            "#f5f5f5" | "f5f5f5" | "whitesmoke" => Some(Self::WhiteSmoke),
            "#f0f8ff" | "f0f8ff" | "aliceblue" => Some(Self::AliceBlue),
            _ => None,
        }
    }
}

impl FromStr for White {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse(s) {
            Some(colour) => Ok(colour),
            None => Err(format!("Invalid Colour: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(White::AntiqueWhite, "rgb(250,235,215)")]
    #[case(White::Beige, "rgb(245,245,220)")]
    #[case(White::Bisque, "rgb(255,228,196)")]
    #[case(White::BlanchedAlmond, "rgb(255,235,205)")]
    #[case(White::Wheat, "rgb(245,222,179)")]
    #[case(White::CornSilk, "rgb(255,248,220)")]
    #[case(White::White, "rgb(255,255,255)")]
    #[case(White::NavajoWhite, "rgb(255,222,173)")]
    #[case(White::MistyRose, "rgb(255,228,225)")]
    #[case(White::LavenderBlush, "rgb(255,240,245)")]
    #[case(White::Linen, "rgb(250,240,230)")]
    #[case(White::OldLace, "rgb(253,245,230)")]
    #[case(White::SeaShell, "rgb(255,245,238)")]
    #[case(White::MintCream, "rgb(245,255,250)")]
    #[case(White::FloralWhite, "rgb(255,250,240)")]
    #[case(White::GhostWhite, "rgb(248,248,255)")]
    #[case(White::Ivory, "rgb(255,255,240)")]
    #[case(White::Snow, "rgb(255,250,250)")]
    #[case(White::WhiteSmoke, "rgb(245,245,245)")]
    fn test_rgb_string(#[case] colour: White, #[case] expected: String) {
        let rgb_colour = colour.to_rgb();
        let string = rgb_colour.to_string();

        assert_eq!(expected, string);
    }

    #[rstest]
    #[case(White::AntiqueWhite, "FAEBD7")]
    #[case(White::Beige, "F5F5DC")]
    #[case(White::Bisque, "FFE4C4")]
    #[case(White::BlanchedAlmond, "FFEBCD")]
    #[case(White::Wheat, "F5DEB3")]
    #[case(White::CornSilk, "FFF8DC")]
    #[case(White::White, "FFFFFF")]
    #[case(White::NavajoWhite, "FFDEAD")]
    #[case(White::MistyRose, "FFE4E1")]
    #[case(White::LavenderBlush, "FFF0F5")]
    #[case(White::Linen, "FAF0E6")]
    #[case(White::OldLace, "FDF5E6")]
    #[case(White::SeaShell, "FFF5EE")]
    #[case(White::MintCream, "F5FFFA")]
    #[case(White::FloralWhite, "FFFAF0")]
    #[case(White::GhostWhite, "F8F8FF")]
    #[case(White::Ivory, "FFFFF0")]
    #[case(White::Snow, "FFFAFA")]
    #[case(White::WhiteSmoke, "F5F5F5")]
    fn test_hex_triplet_string(
        #[case] colour: White,
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
    #[case("#faebd7", White::AntiqueWhite)]
    #[case("faebd7", White::AntiqueWhite)]
    #[case("AntiqueWhite", White::AntiqueWhite)]
    #[case("#f5f5dc", White::Beige)]
    #[case("f5f5dc", White::Beige)]
    #[case("Beige", White::Beige)]
    #[case("#ffe4c4", White::Bisque)]
    #[case("ffe4c4", White::Bisque)]
    #[case("Bisque", White::Bisque)]
    #[case("#ffebcd", White::BlanchedAlmond)]
    #[case("ffebcd", White::BlanchedAlmond)]
    #[case("BlanchedAlmond", White::BlanchedAlmond)]
    #[case("#f5deb3", White::Wheat)]
    #[case("f5deb3", White::Wheat)]
    #[case("wheat", White::Wheat)]
    #[case("#fff8dc", White::CornSilk)]
    #[case("fff8dc", White::CornSilk)]
    #[case("CornSilk", White::CornSilk)]
    #[case("#ffffff", White::White)]
    #[case("ffffff", White::White)]
    #[case("White", White::White)]
    #[case("#ffdead", White::NavajoWhite)]
    #[case("ffdead", White::NavajoWhite)]
    #[case("NavajoWhite", White::NavajoWhite)]
    #[case("#ffe4e1", White::MistyRose)]
    #[case("ffe4e1", White::MistyRose)]
    #[case("MistyRose", White::MistyRose)]
    #[case("#fff0f5", White::LavenderBlush)]
    #[case("fff0f5", White::LavenderBlush)]
    #[case("LavenderBlush", White::LavenderBlush)]
    #[case("#faf0e6", White::Linen)]
    #[case("faf0e6", White::Linen)]
    #[case("Linen", White::Linen)]
    #[case("#fdf5e6", White::OldLace)]
    #[case("fdf5e6", White::OldLace)]
    #[case("OldLace", White::OldLace)]
    #[case("#fff5ee", White::SeaShell)]
    #[case("fff5ee", White::SeaShell)]
    #[case("SeaShell", White::SeaShell)]
    #[case("#f5fffa", White::MintCream)]
    #[case("f5fffa", White::MintCream)]
    #[case("MintCream", White::MintCream)]
    #[case("#fffaf0", White::FloralWhite)]
    #[case("fffaf0", White::FloralWhite)]
    #[case("FloralWhite", White::FloralWhite)]
    #[case("#f8f8ff", White::GhostWhite)]
    #[case("f8f8ff", White::GhostWhite)]
    #[case("GhostWhite", White::GhostWhite)]
    #[case("#fffff0", White::Ivory)]
    #[case("fffff0", White::Ivory)]
    #[case("Ivory", White::Ivory)]
    #[case("#fffafa", White::Snow)]
    #[case("fffafa", White::Snow)]
    #[case("Snow", White::Snow)]
    #[case("#f5f5f5", White::WhiteSmoke)]
    #[case("f5f5f5", White::WhiteSmoke)]
    #[case("WhiteSmoke", White::WhiteSmoke)]
    #[case("#f0f8ff", White::AliceBlue)]
    #[case("f0f8ff", White::AliceBlue)]
    #[case("AliceBlue", White::AliceBlue)]
    fn test_from_str(#[case] input: &str, #[case] expected: White) {
        assert_eq!(expected, White::from_str(input).unwrap())
    }
}
