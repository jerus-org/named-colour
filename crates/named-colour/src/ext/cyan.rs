//! Extended named colours providing shades collected in enums for the main colour
//!

use std::fmt;

use rgb::Rgb;
use strum::EnumCount;
use tinyrand::{RandRange, StdRand};

use super::ExtendedColour;

/// Shades of cyan
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
#[allow(missing_docs)]
pub enum Cyan {
    MediumAquaMarine,
    MediumSeaGreen,
    LightSeaGreen,
    DarkSlateGray,
    Teal,
    DarkCyan,
    Aqua,
    Cyan,
    LightCyan,
    DarkTurquoise,
    Turquoise,
    MediumTurquoise,
    PaleTurquoise,
    AquaMarine,
    Honeydew,
}

impl fmt::Display for Cyan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MediumAquaMarine => write!(f, "#66CDAA"),
            Self::MediumSeaGreen => write!(f, "#3CB371"),
            Self::LightSeaGreen => write!(f, "#20B2AA"),
            Self::DarkSlateGray => write!(f, "#2F4F4F"),
            Self::Teal => write!(f, "#008080"),
            Self::DarkCyan => write!(f, "#008B8B"),
            Self::Aqua => write!(f, "#00FFFF"),
            Self::Cyan => write!(f, "#00FFFF"),
            Self::LightCyan => write!(f, "#E0FFFF"),
            Self::DarkTurquoise => write!(f, "#00CED1"),
            Self::Turquoise => write!(f, "#40E0D0"),
            Self::MediumTurquoise => write!(f, "#48D1CC"),
            Self::PaleTurquoise => write!(f, "#AFEEEE"),
            Self::AquaMarine => write!(f, "#7FFFD4"),
            Self::Honeydew => write!(f, "#F0FFF0"),
        }
    }
}

impl_colour_methods!(Cyan);

impl Cyan {
    /// Parse a colour from string
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Cyan;
    /// # use named_colour::Prefix;
    ///    let colour = Cyan::Teal;
    ///
    ///     assert_eq!(Some(Cyan::Teal), Cyan::parse("teal"));
    ///
    ///```
    pub fn parse(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "#66cdaa" | "66cdaa" | "mediumaquamarine" => Some(Self::MediumAquaMarine),
            "#3cb371" | "3cb371" | "mediumseagreen" => Some(Self::MediumSeaGreen),
            "#20b2aa" | "20b2aa" | "lightseagreen" => Some(Self::LightSeaGreen),
            "#2f4f4f" | "2f4f4f" | "darkslategray" => Some(Self::DarkSlateGray),
            "#008080" | "008080" | "teal" => Some(Self::Teal),
            "#008b8b" | "008b8b" | "darkcyan" => Some(Self::DarkCyan),
            "#00ffff" | "00ffff" | "aqua" => Some(Self::Aqua),
            "cyan" => Some(Self::Cyan),
            "#e0ffff" | "e0ffff" | "lightcyan" => Some(Self::LightCyan),
            "#00ced1" | "00ced1" | "darkturquoise" => Some(Self::DarkTurquoise),
            "#40e0d0" | "40e0d0" | "turquoise" => Some(Self::Turquoise),
            "#48d1cc" | "48d1cc" | "mediumturquoise" => Some(Self::MediumTurquoise),
            "#afeeee" | "afeeee" | "paleturquoise" => Some(Self::PaleTurquoise),
            "#7fffd4" | "7fffd4" | "aquamarine" => Some(Self::AquaMarine),
            "#f0fff0" | "f0fff0" | "honeydew" => Some(Self::Honeydew),
            _ => None,
        }
    }

    /// Generate a random colour
    ///     
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Cyan;
    /// # fn main() {
    ///    let colour = Cyan::random();
    ///
    /// # }
    /// ```
    pub fn random() -> Self {
        let mut rand = StdRand::default();

        match rand.next_range(0..Self::COUNT) {
            0 => Self::MediumAquaMarine,
            1 => Self::MediumSeaGreen,
            2 => Self::LightSeaGreen,
            3 => Self::DarkSlateGray,
            4 => Self::Teal,
            5 => Self::DarkCyan,
            6 => Self::Aqua,
            7 => Self::Cyan,
            8 => Self::LightCyan,
            9 => Self::DarkTurquoise,
            10 => Self::Turquoise,
            11 => Self::MediumTurquoise,
            12 => Self::PaleTurquoise,
            13 => Self::AquaMarine,
            14 => Self::Honeydew,
            _ => Self::Cyan,
        }
    }
}

impl ExtendedColour for Cyan {}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::Prefix;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Cyan::MediumAquaMarine, "rgb(102,205,170)")]
    #[case(Cyan::MediumSeaGreen, "rgb(60,179,113)")]
    #[case(Cyan::LightSeaGreen, "rgb(32,178,170)")]
    #[case(Cyan::DarkSlateGray, "rgb(47,79,79)")]
    #[case(Cyan::Teal, "rgb(0,128,128)")]
    #[case(Cyan::DarkCyan, "rgb(0,139,139)")]
    #[case(Cyan::Aqua, "rgb(0,255,255)")]
    #[case(Cyan::Cyan, "rgb(0,255,255)")]
    #[case(Cyan::LightCyan, "rgb(224,255,255)")]
    #[case(Cyan::DarkTurquoise, "rgb(0,206,209)")]
    #[case(Cyan::Turquoise, "rgb(64,224,208)")]
    #[case(Cyan::MediumTurquoise, "rgb(72,209,204)")]
    #[case(Cyan::PaleTurquoise, "rgb(175,238,238)")]
    #[case(Cyan::AquaMarine, "rgb(127,255,212)")]
    #[case(Cyan::Honeydew, "rgb(240,255,240)")]
    fn test_rgb_string(#[case] colour: Cyan, #[case] expected: String) {
        let rgb_colour = colour.to_rgb();
        let string = rgb_colour.to_string();

        assert_eq!(expected, string);
    }

    #[rstest]
    #[case(Cyan::MediumAquaMarine, "66CDAA")]
    #[case(Cyan::MediumSeaGreen, "3CB371")]
    #[case(Cyan::LightSeaGreen, "20B2AA")]
    #[case(Cyan::DarkSlateGray, "2F4F4F")]
    #[case(Cyan::Teal, "008080")]
    #[case(Cyan::DarkCyan, "008B8B")]
    #[case(Cyan::Aqua, "00FFFF")]
    #[case(Cyan::Cyan, "00FFFF")]
    #[case(Cyan::LightCyan, "E0FFFF")]
    #[case(Cyan::DarkTurquoise, "00CED1")]
    #[case(Cyan::Turquoise, "40E0D0")]
    #[case(Cyan::MediumTurquoise, "48D1CC")]
    #[case(Cyan::PaleTurquoise, "AFEEEE")]
    #[case(Cyan::AquaMarine, "7FFFD4")]
    #[case(Cyan::Honeydew, "F0FFF0")]
    fn test_hex_triplet_string(
        #[case] colour: Cyan,
        #[values(Prefix::None, Prefix::Hash)] prefix: Prefix,
        #[case] expected: String,
    ) {
        let prefix_string = match prefix {
            Prefix::None => "".to_string(),
            Prefix::Hash => "#".to_string(),
        };

        let expected = format!("{prefix_string}{expected}");

        let hex_colour = colour.to_hex_triplet(prefix);

        assert_eq!(expected, hex_colour);
    }

    #[rstest]
    #[case("#66cdaa", Cyan::MediumAquaMarine)]
    #[case("66cdaa", Cyan::MediumAquaMarine)]
    #[case("mediumaquamarine", Cyan::MediumAquaMarine)]
    #[case("#3cb371", Cyan::MediumSeaGreen)]
    #[case("3cb371", Cyan::MediumSeaGreen)]
    #[case("mediumseagreen", Cyan::MediumSeaGreen)]
    #[case("#20b2aa", Cyan::LightSeaGreen)]
    #[case("20b2aa", Cyan::LightSeaGreen)]
    #[case("lightseagreen", Cyan::LightSeaGreen)]
    #[case("#2f4f4f", Cyan::DarkSlateGray)]
    #[case("2f4f4f", Cyan::DarkSlateGray)]
    #[case("darkslategray", Cyan::DarkSlateGray)]
    #[case("#008080", Cyan::Teal)]
    #[case("008080", Cyan::Teal)]
    #[case("teal", Cyan::Teal)]
    #[case("#008b8b", Cyan::DarkCyan)]
    #[case("008b8b", Cyan::DarkCyan)]
    #[case("darkcyan", Cyan::DarkCyan)]
    #[case("#00ffff", Cyan::Aqua)]
    #[case("00ffff", Cyan::Aqua)]
    #[case("aqua", Cyan::Aqua)]
    #[case("cyan", Cyan::Cyan)]
    #[case("#e0ffff", Cyan::LightCyan)]
    #[case("e0ffff", Cyan::LightCyan)]
    #[case("lightcyan", Cyan::LightCyan)]
    #[case("#00ced1", Cyan::DarkTurquoise)]
    #[case("00ced1", Cyan::DarkTurquoise)]
    #[case("darkturquoise", Cyan::DarkTurquoise)]
    #[case("#40e0d0", Cyan::Turquoise)]
    #[case("40e0d0", Cyan::Turquoise)]
    #[case("turquoise", Cyan::Turquoise)]
    #[case("#48d1cc", Cyan::MediumTurquoise)]
    #[case("48d1cc", Cyan::MediumTurquoise)]
    #[case("mediumturquoise", Cyan::MediumTurquoise)]
    #[case("#afeeee", Cyan::PaleTurquoise)]
    #[case("afeeee", Cyan::PaleTurquoise)]
    #[case("paleturquoise", Cyan::PaleTurquoise)]
    #[case("#7fffd4", Cyan::AquaMarine)]
    #[case("7fffd4", Cyan::AquaMarine)]
    #[case("aquamarine", Cyan::AquaMarine)]
    #[case("#f0fff0", Cyan::Honeydew)]
    #[case("f0fff0", Cyan::Honeydew)]
    #[case("honeydew", Cyan::Honeydew)]
    fn test_from_str(#[case] input: &str, #[case] expected: Cyan) {
        assert_eq!(expected, Cyan::from_str(input).unwrap())
    }

    #[rstest]
    #[case("#66cdaa", Some(Cyan::MediumAquaMarine))]
    #[case("66cdaa", Some(Cyan::MediumAquaMarine))]
    #[case("mediumaquamarine", Some(Cyan::MediumAquaMarine))]
    #[case("#3cb371", Some(Cyan::MediumSeaGreen))]
    #[case("3cb371", Some(Cyan::MediumSeaGreen))]
    #[case("mediumseagreen", Some(Cyan::MediumSeaGreen))]
    #[case("#20b2aa", Some(Cyan::LightSeaGreen))]
    #[case("20b2aa", Some(Cyan::LightSeaGreen))]
    #[case("lightseagreen", Some(Cyan::LightSeaGreen))]
    #[case("#2f4f4f", Some(Cyan::DarkSlateGray))]
    #[case("2f4f4f", Some(Cyan::DarkSlateGray))]
    #[case("darkslategray", Some(Cyan::DarkSlateGray))]
    #[case("#008080", Some(Cyan::Teal))]
    #[case("008080", Some(Cyan::Teal))]
    #[case("teal", Some(Cyan::Teal))]
    #[case("#008b8b", Some(Cyan::DarkCyan))]
    #[case("008b8b", Some(Cyan::DarkCyan))]
    #[case("darkcyan", Some(Cyan::DarkCyan))]
    #[case("#00ffff", Some(Cyan::Aqua))]
    #[case("00ffff", Some(Cyan::Aqua))]
    #[case("aqua", Some(Cyan::Aqua))]
    #[case("cyan", Some(Cyan::Cyan))]
    #[case("#e0ffff", Some(Cyan::LightCyan))]
    #[case("e0ffff", Some(Cyan::LightCyan))]
    #[case("lightcyan", Some(Cyan::LightCyan))]
    #[case("#00ced1", Some(Cyan::DarkTurquoise))]
    #[case("00ced1", Some(Cyan::DarkTurquoise))]
    #[case("darkturquoise", Some(Cyan::DarkTurquoise))]
    #[case("#40e0d0", Some(Cyan::Turquoise))]
    #[case("40e0d0", Some(Cyan::Turquoise))]
    #[case("turquoise", Some(Cyan::Turquoise))]
    #[case("#48d1cc", Some(Cyan::MediumTurquoise))]
    #[case("48d1cc", Some(Cyan::MediumTurquoise))]
    #[case("mediumturquoise", Some(Cyan::MediumTurquoise))]
    #[case("#afeeee", Some(Cyan::PaleTurquoise))]
    #[case("afeeee", Some(Cyan::PaleTurquoise))]
    #[case("paleturquoise", Some(Cyan::PaleTurquoise))]
    #[case("#7fffd4", Some(Cyan::AquaMarine))]
    #[case("7fffd4", Some(Cyan::AquaMarine))]
    #[case("aquamarine", Some(Cyan::AquaMarine))]
    #[case("#f0fff0", Some(Cyan::Honeydew))]
    #[case("f0fff0", Some(Cyan::Honeydew))]
    #[case("honeydew", Some(Cyan::Honeydew))]
    #[case("012345", None)]
    fn test_name_colour(#[case] input: &str, #[case] expected: Option<Cyan>) {
        assert_eq!(expected, Cyan::name_colour(input))
    }
}
