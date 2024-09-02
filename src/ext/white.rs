//! Extended named colours providing shades collected in enums for the main colour
//!

use std::fmt;

use rgb::Rgb;

use crate::Prefix;

/// Shades of white
#[derive(Debug)]
#[allow(missing_docs)]
pub enum White {
    AntiqueWhite,
    Beige,
    Bisque,
    BlanchedAlmond,
    Wheat,
    CornSilk,
    LemonChiffon,
    LightGoldenRodYellow,
    LightYellow,
    White,
    Moccasin,
    NavajoWhite,
    PeachPuff,
    MistyRose,
    LavenderBlush,
    Linen,
    OldLace,
    PapayaWhip,
    SeaShell,
    MintCream,
    FloralWhite,
    GhostWhite,
    Ivory,
    Snow,
    WhiteSmoke,
}

impl fmt::Display for White {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            White::AntiqueWhite => write!(f, "#FAEBD7"),
            White::Beige => write!(f, "#F5F5DC"),
            White::Bisque => write!(f, "#FFE4C4"),
            White::BlanchedAlmond => write!(f, "#FFEBCD"),
            White::Wheat => write!(f, "#F5DEB3"),
            White::CornSilk => write!(f, "#FFF8DC"),
            White::LemonChiffon => write!(f, "#FFFACD"),
            White::LightGoldenRodYellow => write!(f, "#FAFAD2"),
            White::LightYellow => write!(f, "#FFFFE0"),
            White::White => write!(f, "#FFFFFF"),
            White::Moccasin => write!(f, "#FFE4B5"),
            White::NavajoWhite => write!(f, "#FFDEAD"),
            White::PeachPuff => write!(f, "#FFDAB9"),
            White::MistyRose => write!(f, "#FFE4E1"),
            White::LavenderBlush => write!(f, "#FFF0F5"),
            White::Linen => write!(f, "#FAF0E6"),
            White::OldLace => write!(f, "#FDF5E6"),
            White::PapayaWhip => write!(f, "#FFEFD5"),
            White::SeaShell => write!(f, "#FFF5EE"),
            White::MintCream => write!(f, "#F5FFFA"),
            White::FloralWhite => write!(f, "#FFFAF0"),
            White::GhostWhite => write!(f, "#F8F8FF"),
            White::Ivory => write!(f, "#FFFFF0"),
            White::Snow => write!(f, "#FFFAFA"),
            White::WhiteSmoke => write!(f, "#F5F5F5"),
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
            White::LemonChiffon => crate::to_rgb("#FFFACD"),
            White::LightGoldenRodYellow => crate::to_rgb("#FAFAD2"),
            White::LightYellow => crate::to_rgb("#FFFFE0"),
            White::White => crate::to_rgb("#FFFFFF"),
            White::Moccasin => crate::to_rgb("#FFE4B5"),
            White::NavajoWhite => crate::to_rgb("#FFDEAD"),
            White::PeachPuff => crate::to_rgb("#FFDAB9"),
            White::MistyRose => crate::to_rgb("#FFE4E1"),
            White::LavenderBlush => crate::to_rgb("#FFF0F5"),
            White::Linen => crate::to_rgb("#FAF0E6"),
            White::OldLace => crate::to_rgb("#FDF5E6"),
            White::PapayaWhip => crate::to_rgb("#FFEFD5"),
            White::SeaShell => crate::to_rgb("#FFF5EE"),
            White::MintCream => crate::to_rgb("#F5FFFA"),
            White::FloralWhite => crate::to_rgb("#FFFAF0"),
            White::GhostWhite => crate::to_rgb("#F8F8FF"),
            White::Ivory => crate::to_rgb("#FFFFF0"),
            White::Snow => crate::to_rgb("#FFFAFA"),
            White::WhiteSmoke => crate::to_rgb("#F5F5F5"),
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
    #[case(White::AntiqueWhite, "rgb(250,235,215)")]
    #[case(White::Beige, "rgb(245,245,220)")]
    #[case(White::Bisque, "rgb(255,228,196)")]
    #[case(White::BlanchedAlmond, "rgb(255,235,205)")]
    #[case(White::Wheat, "rgb(245,222,179)")]
    #[case(White::CornSilk, "rgb(255,248,220)")]
    #[case(White::LemonChiffon, "rgb(255,250,205)")]
    #[case(White::LightGoldenRodYellow, "rgb(250,250,210)")]
    #[case(White::LightYellow, "rgb(255,255,224)")]
    #[case(White::White, "rgb(255,255,255)")]
    #[case(White::Moccasin, "rgb(255,228,181)")]
    #[case(White::NavajoWhite, "rgb(255,222,173)")]
    #[case(White::PeachPuff, "rgb(255,218,185)")]
    #[case(White::MistyRose, "rgb(255,228,225)")]
    #[case(White::LavenderBlush, "rgb(255,240,245)")]
    #[case(White::Linen, "rgb(250,240,230)")]
    #[case(White::OldLace, "rgb(253,245,230)")]
    #[case(White::PapayaWhip, "rgb(255,239,213)")]
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
    #[case(White::LemonChiffon, "FFFACD")]
    #[case(White::LightGoldenRodYellow, "FAFAD2")]
    #[case(White::LightYellow, "FFFFE0")]
    #[case(White::White, "FFFFFF")]
    #[case(White::Moccasin, "FFE4B5")]
    #[case(White::NavajoWhite, "FFDEAD")]
    #[case(White::PeachPuff, "FFDAB9")]
    #[case(White::MistyRose, "FFE4E1")]
    #[case(White::LavenderBlush, "FFF0F5")]
    #[case(White::Linen, "FAF0E6")]
    #[case(White::OldLace, "FDF5E6")]
    #[case(White::PapayaWhip, "FFEFD5")]
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
}
