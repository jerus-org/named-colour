//! Extended named colours providing shades collected in enums for the main colour
//!

use std::{fmt, str::FromStr};

use rgb::Rgb;
use strum::EnumCount;
use tinyrand::{RandRange, StdRand};

use crate::Prefix;

use super::ExtendedColour;

/// Shades of blue
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
#[allow(missing_docs)]
pub enum Blue {
    PowderBlue,
    CadetBlue,
    SteelBlue,
    CornflowerBlue,
    DeepSkyBlue,
    DodgerBlue,
    LightBlue,
    SkyBlue,
    LightSkyBlue,
    MidnightBlue,
    Navy,
    DarkBlue,
    MediumBlue,
    Blue,
    RoyalBlue,
    Azure,
    LightSteelBlue,
}

impl fmt::Display for Blue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PowderBlue => write!(f, "#B0E0E6"),
            Self::CadetBlue => write!(f, "#5F9EA0"),
            Self::SteelBlue => write!(f, "#4682B4"),
            Self::CornflowerBlue => write!(f, "#6495ED"),
            Self::DeepSkyBlue => write!(f, "#00BFFF"),
            Self::DodgerBlue => write!(f, "#1E90FF"),
            Self::LightBlue => write!(f, "#ADD8E6"),
            Self::SkyBlue => write!(f, "#87CEEB"),
            Self::LightSkyBlue => write!(f, "#87CEFA"),
            Self::LightSteelBlue => write!(f, "#B0C4DE"),
            Self::MidnightBlue => write!(f, "#191970"),
            Self::Navy => write!(f, "#000080"),
            Self::DarkBlue => write!(f, "#00008B"),
            Self::MediumBlue => write!(f, "#0000CD"),
            Self::Blue => write!(f, "#0000FF"),
            Self::RoyalBlue => write!(f, "#4169E1"),
            Self::Azure => write!(f, "#F0FFFF"),
        }
    }
}

impl Blue {
    /// Display the colour name as an RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Blue;
    /// # fn main() {
    ///    let colour = Blue::Azure;
    ///    let rgb_colour = colour.to_rgb();
    ///
    ///    let string = rgb_colour.to_string();
    ///    assert_eq!("rgb(240,255,255)", string);
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
    /// # use named_colour::ext::Blue;
    /// # use named_colour::Prefix;
    ///    let colour = Blue::Azure;
    ///
    ///     assert_eq!("#F0FFFF", colour.to_hex_triplet(Prefix::Hash));
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
    /// # use named_colour::ext::Blue;
    /// # use named_colour::Prefix;
    ///    let colour = Blue::Azure;
    ///
    ///     assert_eq!(Blue::Azure, Blue::parse("#F0FFFF").unwrap());
    ///
    ///```
    pub fn parse(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "#b0e0e6" | "b0e0e6" | "powderblue" => Some(Self::PowderBlue),
            "#5f9ea0" | "5f9ea0" | "cadetblue" => Some(Self::CadetBlue),
            "#4682b4" | "4682b4" | "steelblue" => Some(Self::SteelBlue),
            "#6495ed" | "6495ed" | "cornflowerblue" => Some(Self::CornflowerBlue),
            "#00bfff" | "00bfff" | "deepskyblue" => Some(Self::DeepSkyBlue),
            "#1e90ff" | "1e90ff" | "dodgerblue" => Some(Self::DodgerBlue),
            "#add8e6" | "add8e6" | "lightblue" => Some(Self::LightBlue),
            "#87ceeb" | "87ceeb" | "skyblue" => Some(Self::SkyBlue),
            "#b0c4de" | "b0c4de" | "lightsteelblue" => Some(Self::LightSteelBlue),
            "#87cefa" | "87cefa" | "lightskyblue" => Some(Self::LightSkyBlue),
            "#191970" | "191970" | "midnightblue" => Some(Self::MidnightBlue),
            "#000080" | "000080" | "navy" => Some(Self::Navy),
            "#00008b" | "00008b" | "darkblue" => Some(Self::DarkBlue),
            "#0000cd" | "0000cd" | "mediumblue" => Some(Self::MediumBlue),
            "#0000ff" | "0000ff" | "blue" => Some(Self::Blue),
            "#4169e1" | "4169e1" | "royalblue" => Some(Self::RoyalBlue),
            "#f0ffff" | "f0ffff" | "azure" => Some(Self::Azure),
            _ => None,
        }
    }

    /// Generate a random colour
    ///     
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Blue;
    /// # fn main() {
    ///    let colour = Blue::random();
    ///
    /// # }
    /// ```
    pub fn random() -> Self {
        let mut rand = StdRand::default();

        match rand.next_range(0..Self::COUNT) {
            0 => Self::PowderBlue,
            1 => Self::CadetBlue,
            2 => Self::SteelBlue,
            3 => Self::CornflowerBlue,
            4 => Self::DeepSkyBlue,
            5 => Self::DodgerBlue,
            6 => Self::LightBlue,
            7 => Self::SkyBlue,
            8 => Self::LightSkyBlue,
            9 => Self::LightSteelBlue,
            10 => Self::MidnightBlue,
            11 => Self::Navy,
            12 => Self::DarkBlue,
            13 => Self::MediumBlue,
            14 => Self::Blue,
            15 => Self::RoyalBlue,
            16 => Self::Azure,
            _ => Self::Blue,
        }
    }
}

impl FromStr for Blue {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse(s) {
            Some(colour) => Ok(colour),
            None => Err(format!("Invalid Colour: {}", s)),
        }
    }
}

impl ExtendedColour for Blue {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Blue::PowderBlue, "rgb(176,224,230)")]
    #[case(Blue::CadetBlue, "rgb(95,158,160)")]
    #[case(Blue::SteelBlue, "rgb(70,130,180)")]
    #[case(Blue::CornflowerBlue, "rgb(100,149,237)")]
    #[case(Blue::DeepSkyBlue, "rgb(0,191,255)")]
    #[case(Blue::DodgerBlue, "rgb(30,144,255)")]
    #[case(Blue::LightBlue, "rgb(173,216,230)")]
    #[case(Blue::SkyBlue, "rgb(135,206,235)")]
    #[case(Blue::LightSteelBlue, "rgb(176,196,222)")]
    #[case(Blue::LightSkyBlue, "rgb(135,206,250)")]
    #[case(Blue::MidnightBlue, "rgb(25,25,112)")]
    #[case(Blue::Navy, "rgb(0,0,128)")]
    #[case(Blue::DarkBlue, "rgb(0,0,139)")]
    #[case(Blue::MediumBlue, "rgb(0,0,205)")]
    #[case(Blue::Blue, "rgb(0,0,255)")]
    #[case(Blue::RoyalBlue, "rgb(65,105,225)")]
    #[case(Blue::Azure, "rgb(240,255,255)")]

    fn test_rgb_string(#[case] colour: Blue, #[case] expected: String) {
        let rgb_colour = colour.to_rgb();
        let string = rgb_colour.to_string();

        assert_eq!(expected, string);
    }

    #[rstest]
    #[case(Blue::PowderBlue, "B0E0E6")]
    #[case(Blue::CadetBlue, "5F9EA0")]
    #[case(Blue::SteelBlue, "4682B4")]
    #[case(Blue::CornflowerBlue, "6495ED")]
    #[case(Blue::DeepSkyBlue, "00BFFF")]
    #[case(Blue::DodgerBlue, "1E90FF")]
    #[case(Blue::LightBlue, "ADD8E6")]
    #[case(Blue::SkyBlue, "87CEEB")]
    #[case(Blue::LightSteelBlue, "B0C4DE")]
    #[case(Blue::LightSkyBlue, "87CEFA")]
    #[case(Blue::MidnightBlue, "191970")]
    #[case(Blue::Navy, "000080")]
    #[case(Blue::DarkBlue, "00008B")]
    #[case(Blue::MediumBlue, "0000CD")]
    #[case(Blue::Blue, "0000FF")]
    #[case(Blue::RoyalBlue, "4169E1")]
    #[case(Blue::Azure, "F0FFFF")]

    fn test_hex_triplet_string(
        #[case] colour: Blue,
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
    #[case("#b0e0e6", Blue::PowderBlue)]
    #[case("b0e0e6", Blue::PowderBlue)]
    #[case("powderblue", Blue::PowderBlue)]
    #[case("#5f9ea0", Blue::CadetBlue)]
    #[case("5f9ea0", Blue::CadetBlue)]
    #[case("cadetblue", Blue::CadetBlue)]
    #[case("#4682b4", Blue::SteelBlue)]
    #[case("4682b4", Blue::SteelBlue)]
    #[case("steelblue", Blue::SteelBlue)]
    #[case("#6495ed", Blue::CornflowerBlue)]
    #[case("6495ed", Blue::CornflowerBlue)]
    #[case("cornflowerblue", Blue::CornflowerBlue)]
    #[case("#00bfff", Blue::DeepSkyBlue)]
    #[case("00bfff", Blue::DeepSkyBlue)]
    #[case("deepskyblue", Blue::DeepSkyBlue)]
    #[case("#1e90ff", Blue::DodgerBlue)]
    #[case("1e90ff", Blue::DodgerBlue)]
    #[case("dodgerblue", Blue::DodgerBlue)]
    #[case("#add8e6", Blue::LightBlue)]
    #[case("add8e6", Blue::LightBlue)]
    #[case("lightblue", Blue::LightBlue)]
    #[case("#87ceeb", Blue::SkyBlue)]
    #[case("87ceeb", Blue::SkyBlue)]
    #[case("skyblue", Blue::SkyBlue)]
    #[case("#b0c4de", Blue::LightSteelBlue)]
    #[case("b0c4de", Blue::LightSteelBlue)]
    #[case("lightsteelblue", Blue::LightSteelBlue)]
    #[case("#87cefa", Blue::LightSkyBlue)]
    #[case("87cefa", Blue::LightSkyBlue)]
    #[case("lightskyblue", Blue::LightSkyBlue)]
    #[case("#191970", Blue::MidnightBlue)]
    #[case("191970", Blue::MidnightBlue)]
    #[case("midnightblue", Blue::MidnightBlue)]
    #[case("#000080", Blue::Navy)]
    #[case("000080", Blue::Navy)]
    #[case("navy", Blue::Navy)]
    #[case("#00008b", Blue::DarkBlue)]
    #[case("00008b", Blue::DarkBlue)]
    #[case("darkblue", Blue::DarkBlue)]
    #[case("#0000cd", Blue::MediumBlue)]
    #[case("0000cd", Blue::MediumBlue)]
    #[case("mediumblue", Blue::MediumBlue)]
    #[case("#0000ff", Blue::Blue)]
    #[case("0000ff", Blue::Blue)]
    #[case("blue", Blue::Blue)]
    #[case("#4169e1", Blue::RoyalBlue)]
    #[case("4169e1", Blue::RoyalBlue)]
    #[case("royalblue", Blue::RoyalBlue)]
    #[case("#f0ffff", Blue::Azure)]
    #[case("f0ffff", Blue::Azure)]
    #[case("azure", Blue::Azure)]
    fn test_from_str(#[case] input: &str, #[case] expected: Blue) {
        assert_eq!(expected, Blue::from_str(input).unwrap())
    }

    #[rstest]
    #[case("#b0e0e6", Some(Blue::PowderBlue))]
    #[case("b0e0e6", Some(Blue::PowderBlue))]
    #[case("powderblue", Some(Blue::PowderBlue))]
    #[case("#5f9ea0", Some(Blue::CadetBlue))]
    #[case("5f9ea0", Some(Blue::CadetBlue))]
    #[case("cadetblue", Some(Blue::CadetBlue))]
    #[case("#4682b4", Some(Blue::SteelBlue))]
    #[case("4682b4", Some(Blue::SteelBlue))]
    #[case("steelblue", Some(Blue::SteelBlue))]
    #[case("#6495ed", Some(Blue::CornflowerBlue))]
    #[case("6495ed", Some(Blue::CornflowerBlue))]
    #[case("cornflowerblue", Some(Blue::CornflowerBlue))]
    #[case("#00bfff", Some(Blue::DeepSkyBlue))]
    #[case("00bfff", Some(Blue::DeepSkyBlue))]
    #[case("deepskyblue", Some(Blue::DeepSkyBlue))]
    #[case("#1e90ff", Some(Blue::DodgerBlue))]
    #[case("1e90ff", Some(Blue::DodgerBlue))]
    #[case("dodgerblue", Some(Blue::DodgerBlue))]
    #[case("#add8e6", Some(Blue::LightBlue))]
    #[case("add8e6", Some(Blue::LightBlue))]
    #[case("lightblue", Some(Blue::LightBlue))]
    #[case("#87ceeb", Some(Blue::SkyBlue))]
    #[case("87ceeb", Some(Blue::SkyBlue))]
    #[case("skyblue", Some(Blue::SkyBlue))]
    #[case("#b0c4de", Some(Blue::LightSteelBlue))]
    #[case("b0c4de", Some(Blue::LightSteelBlue))]
    #[case("lightsteelblue", Some(Blue::LightSteelBlue))]
    #[case("#87cefa", Some(Blue::LightSkyBlue))]
    #[case("87cefa", Some(Blue::LightSkyBlue))]
    #[case("lightskyblue", Some(Blue::LightSkyBlue))]
    #[case("#191970", Some(Blue::MidnightBlue))]
    #[case("191970", Some(Blue::MidnightBlue))]
    #[case("midnightblue", Some(Blue::MidnightBlue))]
    #[case("#000080", Some(Blue::Navy))]
    #[case("000080", Some(Blue::Navy))]
    #[case("navy", Some(Blue::Navy))]
    #[case("#00008b", Some(Blue::DarkBlue))]
    #[case("00008b", Some(Blue::DarkBlue))]
    #[case("darkblue", Some(Blue::DarkBlue))]
    #[case("#0000cd", Some(Blue::MediumBlue))]
    #[case("0000cd", Some(Blue::MediumBlue))]
    #[case("mediumblue", Some(Blue::MediumBlue))]
    #[case("#0000ff", Some(Blue::Blue))]
    #[case("0000ff", Some(Blue::Blue))]
    #[case("blue", Some(Blue::Blue))]
    #[case("#4169e1", Some(Blue::RoyalBlue))]
    #[case("4169e1", Some(Blue::RoyalBlue))]
    #[case("royalblue", Some(Blue::RoyalBlue))]
    #[case("#f0ffff", Some(Blue::Azure))]
    #[case("f0ffff", Some(Blue::Azure))]
    #[case("azure", Some(Blue::Azure))]
    #[case("012345", None)]
    fn test_name_colour(#[case] input: &str, #[case] expected: Option<Blue>) {
        assert_eq!(expected, Blue::name_colour(input))
    }
}
