//! Extended named colours providing shades collected in enums for the main colour
//!

use std::{fmt, str::FromStr};

use rgb::Rgb;
use strum::EnumCount;
use tinyrand::{RandRange, StdRand};

use crate::Prefix;

use super::ExtendedColour;

/// Shades of green
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
#[allow(missing_docs)]
pub enum Green {
    YellowGreen,
    DarkOliveGreen,
    Olive,
    OliveDrab,
    LawnGreen,
    ChartReuse,
    GreenYellow,
    DarkGreen,
    Green,
    ForestGreen,
    Lime,
    LimeGreen,
    LightGreen,
    PaleGreen,
    DarkSeaGreen,
    MediumSpringGreen,
    SpringGreen,
    SeaGreen,
}

impl fmt::Display for Green {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::YellowGreen => write!(f, "#9ACD32"),
            Self::DarkOliveGreen => write!(f, "#556B2F"),
            Self::Olive => write!(f, "#808000"),
            Self::OliveDrab => write!(f, "#6B8E23"),
            Self::LawnGreen => write!(f, "#7CFC00"),
            Self::ChartReuse => write!(f, "#7FFF00"),
            Self::GreenYellow => write!(f, "#ADFF2F"),
            Self::DarkGreen => write!(f, "#006400"),
            Self::Green => write!(f, "#008000"),
            Self::ForestGreen => write!(f, "#228B22"),
            Self::Lime => write!(f, "#00FF00"),
            Self::LimeGreen => write!(f, "#32CD32"),
            Self::LightGreen => write!(f, "#90EE90"),
            Self::PaleGreen => write!(f, "#98FB98"),
            Self::DarkSeaGreen => write!(f, "#8FBC8F"),
            Self::MediumSpringGreen => write!(f, "#00FA9A"),
            Self::SpringGreen => write!(f, "#00FF7F"),
            Self::SeaGreen => write!(f, "#2E8B57"),
        }
    }
}

impl Green {
    /// Display the colour name as an RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Green;
    /// # fn main() {
    ///    let colour = Green::LawnGreen;
    ///    let rgb_colour = colour.to_rgb();
    ///
    ///    let string = rgb_colour.to_string();
    ///    assert_eq!("rgb(124,252,0)", string);
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
    /// # use named_colour::ext::Green;
    /// # use named_colour::Prefix;
    ///    let colour = Green::LawnGreen;
    ///
    ///     assert_eq!("#7CFC00", colour.to_hex_triplet(Prefix::Hash));
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
    /// # use named_colour::ext::Green;
    /// # fn main() {
    ///    let colour = Green::parse("#7CFC00");
    ///    assert_eq!(Some(Green::LawnGreen), colour);
    ///
    ///  # }
    ///```
    pub fn parse(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "#9acd32" | "9acd32" | "yellowgreen" => Some(Self::YellowGreen),
            "#556b2f" | "556b2f" | "darkolivegreen" => Some(Self::DarkOliveGreen),
            "#808000" | "808000" | "olive" => Some(Self::Olive),
            "#6b8e23" | "6b8e23" | "olivedrab" => Some(Self::OliveDrab),
            "#7cfc00" | "7cfc00" | "lawngreen" => Some(Self::LawnGreen),
            "#7fff00" | "7fff00" | "chartreuse" => Some(Self::ChartReuse),
            "#adff2f" | "adff2f" | "greenyellow" => Some(Self::GreenYellow),
            "#006400" | "006400" | "darkgreen" => Some(Self::DarkGreen),
            "#008000" | "008000" | "green" => Some(Self::Green),
            "#228b22" | "228b22" | "forestgreen" => Some(Self::ForestGreen),
            "#32cd32" | "32cd32" | "limegreen" => Some(Self::LimeGreen),
            "#90ee90" | "90ee90" | "lightgreen" => Some(Self::LightGreen),
            "#98fb98" | "98fb98" | "palegreen" => Some(Self::PaleGreen),
            "#8fbc8f" | "8fbc8f" | "darkseagreen" => Some(Self::DarkSeaGreen),
            "#00fa9a" | "00fa9a" | "mediumspringgreen" => Some(Self::MediumSpringGreen),
            "#00ff7f" | "00ff7f" | "springgreen" => Some(Self::SpringGreen),
            "#2e8b57" | "2e8b57" | "seagreen" => Some(Self::SeaGreen),
            "#00ff00" | "00ff00" | "lime" => Some(Self::Lime),
            _ => None,
        }
    }

    /// Generate a random colour
    ///     
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Green;
    /// # fn main() {
    ///    let colour = Green::random();
    ///
    /// # }
    /// ```
    pub fn random() -> Self {
        let mut rand = StdRand::default();

        match rand.next_range(0..Self::COUNT) {
            0 => Self::YellowGreen,
            1 => Self::DarkOliveGreen,
            2 => Self::Olive,
            3 => Self::OliveDrab,
            4 => Self::LawnGreen,
            5 => Self::ChartReuse,
            6 => Self::GreenYellow,
            7 => Self::DarkGreen,
            8 => Self::Green,
            9 => Self::ForestGreen,
            10 => Self::Lime,
            11 => Self::LimeGreen,
            12 => Self::LightGreen,
            13 => Self::PaleGreen,
            14 => Self::DarkSeaGreen,
            15 => Self::MediumSpringGreen,
            16 => Self::SpringGreen,
            17 => Self::SeaGreen,
            _ => Self::Green,
        }
    }
}

impl FromStr for Green {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse(s) {
            Some(colour) => Ok(colour),
            None => Err(format!("Invalid Colour: {}", s)),
        }
    }
}

impl ExtendedColour for Green {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Green::YellowGreen, "rgb(154,205,50)")]
    #[case(Green::DarkOliveGreen, "rgb(85,107,47)")]
    #[case(Green::OliveDrab, "rgb(107,142,35)")]
    #[case(Green::LawnGreen, "rgb(124,252,0)")]
    #[case(Green::ChartReuse, "rgb(127,255,0)")]
    #[case(Green::GreenYellow, "rgb(173,255,47)")]
    #[case(Green::DarkGreen, "rgb(0,100,0)")]
    #[case(Green::Green, "rgb(0,128,0)")]
    #[case(Green::ForestGreen, "rgb(34,139,34)")]
    #[case(Green::Lime, "rgb(0,255,0)")]
    #[case(Green::LimeGreen, "rgb(50,205,50)")]
    #[case(Green::LightGreen, "rgb(144,238,144)")]
    #[case(Green::PaleGreen, "rgb(152,251,152)")]
    #[case(Green::DarkSeaGreen, "rgb(143,188,143)")]
    #[case(Green::MediumSpringGreen, "rgb(0,250,154)")]
    #[case(Green::SpringGreen, "rgb(0,255,127)")]
    #[case(Green::SeaGreen, "rgb(46,139,87)")]
    fn test_rgb_string(#[case] colour: Green, #[case] expected: String) {
        let rgb_colour = colour.to_rgb();
        let string = rgb_colour.to_string();

        assert_eq!(expected, string);
    }

    #[rstest]
    #[case(Green::YellowGreen, "9ACD32")]
    #[case(Green::DarkOliveGreen, "556B2F")]
    #[case(Green::OliveDrab, "6B8E23")]
    #[case(Green::LawnGreen, "7CFC00")]
    #[case(Green::ChartReuse, "7FFF00")]
    #[case(Green::GreenYellow, "ADFF2F")]
    #[case(Green::DarkGreen, "006400")]
    #[case(Green::Green, "008000")]
    #[case(Green::ForestGreen, "228B22")]
    #[case(Green::Lime, "00FF00")]
    #[case(Green::LimeGreen, "32CD32")]
    #[case(Green::LightGreen, "90EE90")]
    #[case(Green::PaleGreen, "98FB98")]
    #[case(Green::DarkSeaGreen, "8FBC8F")]
    #[case(Green::MediumSpringGreen, "00FA9A")]
    #[case(Green::SpringGreen, "00FF7F")]
    #[case(Green::SeaGreen, "2E8B57")]
    fn test_hex_triplet_string(
        #[case] colour: Green,
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
    #[case("#9acd32", Green::YellowGreen)]
    #[case("9acd32", Green::YellowGreen)]
    #[case("yellowgreen", Green::YellowGreen)]
    #[case("#556b2f", Green::DarkOliveGreen)]
    #[case("556b2f", Green::DarkOliveGreen)]
    #[case("darkolivegreen", Green::DarkOliveGreen)]
    #[case("#808000", Green::Olive)]
    #[case("808000", Green::Olive)]
    #[case("olive", Green::Olive)]
    #[case("#6b8e23", Green::OliveDrab)]
    #[case("6b8e23", Green::OliveDrab)]
    #[case("olivedrab", Green::OliveDrab)]
    #[case("#7cfc00", Green::LawnGreen)]
    #[case("7cfc00", Green::LawnGreen)]
    #[case("lawngreen", Green::LawnGreen)]
    #[case("#7fff00", Green::ChartReuse)]
    #[case("7fff00", Green::ChartReuse)]
    #[case("chartreuse", Green::ChartReuse)]
    #[case("#adff2f", Green::GreenYellow)]
    #[case("adff2f", Green::GreenYellow)]
    #[case("greenyellow", Green::GreenYellow)]
    #[case("#008000", Green::Green)]
    #[case("008000", Green::Green)]
    #[case("green", Green::Green)]
    #[case("#228b22", Green::ForestGreen)]
    #[case("228b22", Green::ForestGreen)]
    #[case("forestgreen", Green::ForestGreen)]
    #[case("#00ff7f", Green::SpringGreen)]
    #[case("00ff7f", Green::SpringGreen)]
    #[case("springgreen", Green::SpringGreen)]
    #[case("#98fb98", Green::PaleGreen)]
    #[case("98fb98", Green::PaleGreen)]
    #[case("palegreen", Green::PaleGreen)]
    #[case("#8fbc8f", Green::DarkSeaGreen)]
    #[case("8fbc8f", Green::DarkSeaGreen)]
    #[case("darkseagreen", Green::DarkSeaGreen)]
    #[case("#00fa9a", Green::MediumSpringGreen)]
    #[case("00fa9a", Green::MediumSpringGreen)]
    #[case("mediumspringgreen", Green::MediumSpringGreen)]
    #[case("#2e8b57", Green::SeaGreen)]
    #[case("2e8b57", Green::SeaGreen)]
    #[case("seagreen", Green::SeaGreen)]
    fn test_from_str(#[case] input: &str, #[case] expected: Green) {
        assert_eq!(expected, Green::from_str(input).unwrap())
    }

    #[rstest]
    #[case("#9acd32", Some(Green::YellowGreen))]
    #[case("9acd32", Some(Green::YellowGreen))]
    #[case("yellowgreen", Some(Green::YellowGreen))]
    #[case("#556b2f", Some(Green::DarkOliveGreen))]
    #[case("556b2f", Some(Green::DarkOliveGreen))]
    #[case("darkolivegreen", Some(Green::DarkOliveGreen))]
    #[case("#808000", Some(Green::Olive))]
    #[case("808000", Some(Green::Olive))]
    #[case("olive", Some(Green::Olive))]
    #[case("#6b8e23", Some(Green::OliveDrab))]
    #[case("6b8e23", Some(Green::OliveDrab))]
    #[case("olivedrab", Some(Green::OliveDrab))]
    #[case("#7cfc00", Some(Green::LawnGreen))]
    #[case("7cfc00", Some(Green::LawnGreen))]
    #[case("lawngreen", Some(Green::LawnGreen))]
    #[case("#7fff00", Some(Green::ChartReuse))]
    #[case("7fff00", Some(Green::ChartReuse))]
    #[case("chartreuse", Some(Green::ChartReuse))]
    #[case("#adff2f", Some(Green::GreenYellow))]
    #[case("adff2f", Some(Green::GreenYellow))]
    #[case("greenyellow", Some(Green::GreenYellow))]
    #[case("#008000", Some(Green::Green))]
    #[case("008000", Some(Green::Green))]
    #[case("green", Some(Green::Green))]
    #[case("#228b22", Some(Green::ForestGreen))]
    #[case("228b22", Some(Green::ForestGreen))]
    #[case("forestgreen", Some(Green::ForestGreen))]
    #[case("#00ff7f", Some(Green::SpringGreen))]
    #[case("00ff7f", Some(Green::SpringGreen))]
    #[case("springgreen", Some(Green::SpringGreen))]
    #[case("#98fb98", Some(Green::PaleGreen))]
    #[case("98fb98", Some(Green::PaleGreen))]
    #[case("palegreen", Some(Green::PaleGreen))]
    #[case("#8fbc8f", Some(Green::DarkSeaGreen))]
    #[case("8fbc8f", Some(Green::DarkSeaGreen))]
    #[case("darkseagreen", Some(Green::DarkSeaGreen))]
    #[case("#00fa9a", Some(Green::MediumSpringGreen))]
    #[case("00fa9a", Some(Green::MediumSpringGreen))]
    #[case("mediumspringgreen", Some(Green::MediumSpringGreen))]
    #[case("#2e8b57", Some(Green::SeaGreen))]
    #[case("2e8b57", Some(Green::SeaGreen))]
    #[case("seagreen", Some(Green::SeaGreen))]
    #[case("012345", None)]
    fn test_name_colour(#[case] input: &str, #[case] expected: Option<Green>) {
        assert_eq!(expected, Green::name_colour(input))
    }
}
