//! Extended named colours providing shades collected in enums for the main colour
//!

use std::fmt;

use rgb::Rgb;
use strum::EnumCount;
use tinyrand::{RandRange, StdRand};

use super::ExtendedColour;

/// Shades of brown
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
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
            Self::SaddleBrown => write!(f, "#8B4513"),
            Self::Sienna => write!(f, "#A0522D"),
            Self::Chocolate => write!(f, "#D2691E"),
            Self::Peru => write!(f, "#CD853F"),
            Self::SandyBrown => write!(f, "#F4A460"),
            Self::BurlyWood => write!(f, "#DEB887"),
            Self::Tan => write!(f, "#D2B48C"),
            Self::RosyBrown => write!(f, "#BC8F8F"),
        }
    }
}

impl_colour_methods!(Brown);

impl Brown {
    /// Parse a colour from string
    ///
    /// ## Example
    ///
    /// ```
    /// # use named_colour::ext::Brown;
    ///     let colour = Brown::SandyBrown;
    ///     assert_eq!(Some(Brown::SandyBrown), Brown::parse("sandybrown"));    
    /// ```    
    ///
    pub fn parse(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "#8b4513" | "8b4513" | "saddlebrown" => Some(Self::SaddleBrown),
            "#a0522d" | "a0522d" | "sienna" => Some(Self::Sienna),
            "#d2691e" | "d2691e" | "chocolate" => Some(Self::Chocolate),
            "#cd853f" | "cd853f" | "peru" => Some(Self::Peru),
            "#f4a460" | "f4a460" | "sandybrown" => Some(Self::SandyBrown),
            "#deb887" | "deb887" | "burlywood" => Some(Self::BurlyWood),
            "#d2b48c" | "d2b48c" | "tan" => Some(Self::Tan),
            "#bc8f8f" | "bc8f8f" | "rosybrown" => Some(Self::RosyBrown),
            _ => None,
        }
    }

    /// Generate a random colour
    ///     
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Brown;
    /// # fn main() {
    ///    let colour = Brown::random();
    ///
    /// # }
    /// ```
    pub fn random() -> Self {
        let mut rand = StdRand::default();

        match rand.next_range(0..Self::COUNT) {
            0 => Self::SaddleBrown,
            1 => Self::Sienna,
            2 => Self::Chocolate,
            3 => Self::Peru,
            4 => Self::SandyBrown,
            5 => Self::BurlyWood,
            6 => Self::Tan,
            7 => Self::RosyBrown,
            _ => Self::Peru,
        }
    }
}

impl ExtendedColour for Brown {}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::Prefix;

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

        let expected = format!("{prefix_string}{expected}");

        let hex_colour = colour.to_hex_triplet(prefix);

        assert_eq!(expected, hex_colour);
    }

    #[rstest]
    #[case("#8b4513", Brown::SaddleBrown)]
    #[case("#a0522d", Brown::Sienna)]
    #[case("#d2691e", Brown::Chocolate)]
    #[case("#cd853f", Brown::Peru)]
    #[case("#f4a460", Brown::SandyBrown)]
    #[case("#deb887", Brown::BurlyWood)]
    #[case("#d2b48c", Brown::Tan)]
    #[case("#bc8f8f", Brown::RosyBrown)]
    #[case("8b4513", Brown::SaddleBrown)]
    #[case("a0522d", Brown::Sienna)]
    #[case("d2691e", Brown::Chocolate)]
    #[case("cd853f", Brown::Peru)]
    #[case("f4a460", Brown::SandyBrown)]
    #[case("deb887", Brown::BurlyWood)]
    #[case("d2b48c", Brown::Tan)]
    #[case("bc8f8f", Brown::RosyBrown)]
    #[case("saddlebrown", Brown::SaddleBrown)]
    #[case("sienna", Brown::Sienna)]
    #[case("chocolate", Brown::Chocolate)]
    #[case("peru", Brown::Peru)]
    #[case("sandybrown", Brown::SandyBrown)]
    #[case("burlywood", Brown::BurlyWood)]
    #[case("tan", Brown::Tan)]
    #[case("rosybrown", Brown::RosyBrown)]
    fn test_from_str(#[case] input: &str, #[case] expected: Brown) {
        assert_eq!(expected, Brown::from_str(input).unwrap())
    }

    #[rstest]
    #[case("#8b4513", Some(Brown::SaddleBrown))]
    #[case("#a0522d", Some(Brown::Sienna))]
    #[case("#d2691e", Some(Brown::Chocolate))]
    #[case("#cd853f", Some(Brown::Peru))]
    #[case("#f4a460", Some(Brown::SandyBrown))]
    #[case("#deb887", Some(Brown::BurlyWood))]
    #[case("#d2b48c", Some(Brown::Tan))]
    #[case("#bc8f8f", Some(Brown::RosyBrown))]
    #[case("8b4513", Some(Brown::SaddleBrown))]
    #[case("a0522d", Some(Brown::Sienna))]
    #[case("d2691e", Some(Brown::Chocolate))]
    #[case("cd853f", Some(Brown::Peru))]
    #[case("f4a460", Some(Brown::SandyBrown))]
    #[case("deb887", Some(Brown::BurlyWood))]
    #[case("d2b48c", Some(Brown::Tan))]
    #[case("bc8f8f", Some(Brown::RosyBrown))]
    #[case("saddlebrown", Some(Brown::SaddleBrown))]
    #[case("sienna", Some(Brown::Sienna))]
    #[case("chocolate", Some(Brown::Chocolate))]
    #[case("peru", Some(Brown::Peru))]
    #[case("sandybrown", Some(Brown::SandyBrown))]
    #[case("burlywood", Some(Brown::BurlyWood))]
    #[case("tan", Some(Brown::Tan))]
    #[case("rosybrown", Some(Brown::RosyBrown))]
    #[case("012345", None)]
    fn test_name_colour(#[case] input: &str, #[case] expected: Option<Brown>) {
        assert_eq!(expected, Brown::name_colour(input))
    }
}
