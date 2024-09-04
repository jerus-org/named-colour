//! Extended named colours providing shades collected in enums for the main colour
//!

use std::{fmt, str::FromStr};

use rgb::Rgb;

use crate::Prefix;

/// Shades of red
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl fmt::Display for Red {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Red::Maroon => write!(f, "#800000"),
            Red::DarkRed => write!(f, "#8B0000"),
            Red::Brown => write!(f, "#A52A2A"),
            Red::Firebrick => write!(f, "#B22222"),
            Red::Crimson => write!(f, "#DC143C"),
            Red::Red => write!(f, "#FF0000"),
            Red::Tomato => write!(f, "#FF6347"),
            Red::Coral => write!(f, "#FF7F50"),
            Red::IndianRed => write!(f, "#CD5C5C"),
            Red::LightCoral => write!(f, "#F08080"),
            Red::DarkSalmon => write!(f, "#E9967A"),
            Red::Salmon => write!(f, "#FA8072"),
            Red::LightSalmon => write!(f, "#FFA07A"),
            Red::OrangeRed => write!(f, "#FF4500"),
            Red::DarkOrange => write!(f, "#FF8C00"),
            Red::Orange => write!(f, "#FFA500"),
        }
    }
}

impl Red {
    /// Display the colour name as an RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Red;
    /// # fn main() {
    ///    let rgb_colour = Red::Maroon.to_rgb();
    ///    let string = rgb_colour.to_string();
    ///    assert_eq!("rgb(128,0,0)", string);
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
    /// # use named_colour::ext::Red;
    /// # use named_colour::Prefix;
    ///     let colour = Red::Maroon;
    ///
    ///     assert_eq!("#800000", colour.to_hex_triplet(Prefix::Hash));
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
    /// # use named_colour::ext::Red;
    /// # use std::str::FromStr;
    /// # fn main() {
    ///    let colour = Red::from_str("Maroon");
    ///    assert_eq!(Red::Maroon, colour.unwrap());
    ///
    ///  # }
    ///```      
    ///
    pub fn parse(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "#800000" | "800000" | "maroon" => Some(Self::Maroon),
            "#8b0000" | "8b0000" | "darkred" => Some(Self::DarkRed),
            "#a52a2a" | "a52a2a" | "brown" => Some(Self::Brown),
            "#b22222" | "b22222" | "firebrick" => Some(Self::Firebrick),
            "#dc143c" | "dc143c" | "crimson" => Some(Self::Crimson),
            "#ff0000" | "ff0000" | "red" => Some(Self::Red),
            "#ff6347" | "ff6347" | "tomato" => Some(Self::Tomato),
            "#ff7f50" | "ff7f50" | "coral" => Some(Self::Coral),
            "#cd5c5c" | "cd5c5c" | "indianred" => Some(Self::IndianRed),
            "#f08080" | "f08080" | "lightcoral" => Some(Self::LightCoral),
            "#e9967a" | "e9967a" | "darksalmon" => Some(Self::DarkSalmon),
            "#fa8072" | "fa8072" | "salmon" => Some(Self::Salmon),
            "#ffa07a" | "ffa07a" | "lightsalmon" => Some(Self::LightSalmon),
            "#ff4500" | "ff4500" | "orangered" => Some(Self::OrangeRed),
            "#ff8c00" | "ff8c00" | "darkorange" => Some(Self::DarkOrange),
            "#ffa500" | "ffa500" | "orange" => Some(Self::Orange),
            _ => None,
        }
    }
}

impl FromStr for Red {
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
    #[case(Red::Maroon, "rgb(128,0,0)")]
    #[case(Red::DarkRed, "rgb(139,0,0)")]
    #[case(Red::Brown, "rgb(165,42,42)")]
    #[case(Red::Firebrick, "rgb(178,34,34)")]
    #[case(Red::Crimson, "rgb(220,20,60)")]
    #[case(Red::Red, "rgb(255,0,0)")]
    #[case(Red::Tomato, "rgb(255,99,71)")]
    #[case(Red::Coral, "rgb(255,127,80)")]
    #[case(Red::IndianRed, "rgb(205,92,92)")]
    #[case(Red::LightCoral, "rgb(240,128,128)")]
    #[case(Red::DarkSalmon, "rgb(233,150,122)")]
    #[case(Red::Salmon, "rgb(250,128,114)")]
    #[case(Red::LightSalmon, "rgb(255,160,122)")]
    #[case(Red::OrangeRed, "rgb(255,69,0)")]
    #[case(Red::DarkOrange, "rgb(255,140,0)")]
    #[case(Red::Orange, "rgb(255,165,0)")]
    fn test_rgb_string(#[case] colour: Red, #[case] expected: String) {
        let rgb_colour = colour.to_rgb();
        let string = rgb_colour.to_string();

        assert_eq!(expected, string);
    }

    #[rstest]
    #[case(Red::Maroon, "800000")]
    #[case(Red::DarkRed, "8B0000")]
    #[case(Red::Brown, "A52A2A")]
    #[case(Red::Firebrick, "B22222")]
    #[case(Red::Crimson, "DC143C")]
    #[case(Red::Red, "FF0000")]
    #[case(Red::Tomato, "FF6347")]
    #[case(Red::Coral, "FF7F50")]
    #[case(Red::IndianRed, "CD5C5C")]
    #[case(Red::LightCoral, "F08080")]
    #[case(Red::DarkSalmon, "E9967A")]
    #[case(Red::Salmon, "FA8072")]
    #[case(Red::LightSalmon, "FFA07A")]
    #[case(Red::OrangeRed, "FF4500")]
    #[case(Red::DarkOrange, "FF8C00")]
    #[case(Red::Orange, "FFA500")]
    fn test_hex_triplet_string(
        #[case] colour: Red,
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
    #[case("#800000", Red::Maroon)]
    #[case("800000", Red::Maroon)]
    #[case("maroon", Red::Maroon)]
    #[case("#8b0000", Red::DarkRed)]
    #[case("8b0000", Red::DarkRed)]
    #[case("darkred", Red::DarkRed)]
    #[case("#a52a2a", Red::Brown)]
    #[case("a52a2a", Red::Brown)]
    #[case("brown", Red::Brown)]
    #[case("#b22222", Red::Firebrick)]
    #[case("b22222", Red::Firebrick)]
    #[case("firebrick", Red::Firebrick)]
    #[case("#dc143c", Red::Crimson)]
    #[case("dc143c", Red::Crimson)]
    #[case("crimson", Red::Crimson)]
    #[case("#ff0000", Red::Red)]
    #[case("ff0000", Red::Red)]
    #[case("red", Red::Red)]
    #[case("#ff6347", Red::Tomato)]
    #[case("ff6347", Red::Tomato)]
    #[case("tomato", Red::Tomato)]
    #[case("#ff7f50", Red::Coral)]
    #[case("ff7f50", Red::Coral)]
    #[case("coral", Red::Coral)]
    #[case("#cd5c5c", Red::IndianRed)]
    #[case("cd5c5c", Red::IndianRed)]
    #[case("indianred", Red::IndianRed)]
    #[case("#f08080", Red::LightCoral)]
    #[case("f08080", Red::LightCoral)]
    #[case("lightcoral", Red::LightCoral)]
    #[case("#e9967a", Red::DarkSalmon)]
    #[case("e9967a", Red::DarkSalmon)]
    #[case("darksalmon", Red::DarkSalmon)]
    #[case("#fa8072", Red::Salmon)]
    #[case("fa8072", Red::Salmon)]
    #[case("salmon", Red::Salmon)]
    #[case("#ffa07a", Red::LightSalmon)]
    #[case("ffa07a", Red::LightSalmon)]
    #[case("lightsalmon", Red::LightSalmon)]
    #[case("#ff4500", Red::OrangeRed)]
    #[case("ff4500", Red::OrangeRed)]
    #[case("orangered", Red::OrangeRed)]
    #[case("#ff8c00", Red::DarkOrange)]
    #[case("ff8c00", Red::DarkOrange)]
    #[case("darkorange", Red::DarkOrange)]
    #[case("#ffa500", Red::Orange)]
    #[case("ffa500", Red::Orange)]
    #[case("orange", Red::Orange)]
    fn test_from_str(#[case] input: &str, #[case] expected: Red) {
        assert_eq!(expected, Red::from_str(input).unwrap())
    }
}
