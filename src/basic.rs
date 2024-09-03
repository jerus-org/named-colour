use std::{fmt, str::FromStr};

use rgb::Rgb;

use crate::Prefix;

/// 16 basic colours with 18 names!
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Basic {
    Black,
    White,
    Red,
    Lime,
    Blue,
    Yellow,
    /// Alternate name for Aqua
    Cyan,
    /// Alternate name for Cyan
    Aqua,
    /// Alternate name for Fuchsia
    Magenta,
    /// Alternate name for Magenta
    Fuchsia,
    Silver,
    Gray,
    Maroon,
    Olive,
    Green,
    Purple,
    Teal,
    Navy,
}

impl fmt::Display for Basic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Basic::Black => write!(f, "#000000"),
            Basic::White => write!(f, "#FFFFFF"),
            Basic::Red => write!(f, "#FF0000"),
            Basic::Lime => write!(f, "#00FF00"),
            Basic::Blue => write!(f, "#0000FF"),
            Basic::Yellow => write!(f, "#FFFF00"),
            Basic::Cyan => write!(f, "#00FFFF"),
            Basic::Aqua => write!(f, "#00FFFF"),
            Basic::Magenta => write!(f, "#FF00FF"),
            Basic::Fuchsia => write!(f, "#FF00FF"),
            Basic::Silver => write!(f, "#C0C0C0"),
            Basic::Gray => write!(f, "#808080"),
            Basic::Maroon => write!(f, "#800000"),
            Basic::Olive => write!(f, "#808000"),
            Basic::Green => write!(f, "#008000"),
            Basic::Purple => write!(f, "#800080"),
            Basic::Teal => write!(f, "#008080"),
            Basic::Navy => write!(f, "#000080"),
        }
    }
}

impl Basic {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::Basic;
    /// # fn example() {
    /// assert_eq!("(0,255,255)", Basic::Aqua.as_rgb())
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
            Basic::Aqua => crate::to_rgb("#00FFFF"),
            Basic::Black => crate::to_rgb("#000000"),
            Basic::Blue => crate::to_rgb("#0000FF"),
            Basic::Cyan => crate::to_rgb("#00FFFF"),
            Basic::Fuchsia => crate::to_rgb("#FF00FF"),
            Basic::Gray => crate::to_rgb("#808080"),
            Basic::Green => crate::to_rgb("#008000"),
            Basic::Magenta => crate::to_rgb("#FF00FF"),
            Basic::Lime => crate::to_rgb("#00FF00"),
            Basic::Maroon => crate::to_rgb("#800000"),
            Basic::Navy => crate::to_rgb("#000080"),
            Basic::Olive => crate::to_rgb("#808000"),
            Basic::Purple => crate::to_rgb("#800080"),
            Basic::Red => crate::to_rgb("#FF0000"),
            Basic::Silver => crate::to_rgb("#C0C0C0"),
            Basic::Teal => crate::to_rgb("#008080"),
            Basic::White => crate::to_rgb("#FFFFFF"),
            Basic::Yellow => crate::to_rgb("#FFFF00"),
        }
    }

    /// Display the colour name as an RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::Basic;
    /// # fn main() {
    ///    let rgb_colour = Basic::Black.to_rgb();
    ///    let string = rgb_colour.to_string();
    ///    assert_eq!("rgb(0,0,0)", string);
    ///
    ///  # }
    ///```

    pub fn to_rgb(&self) -> Rgb<u8> {
        let colour = match self {
            Basic::Black => "#000000",
            Basic::White => "#FFFFFF",
            Basic::Red => "#FF0000",
            Basic::Lime => "#00FF00",
            Basic::Blue => "#0000FF",
            Basic::Yellow => "#FFFF00",
            Basic::Cyan => "#00FFFF",
            Basic::Aqua => "#00FFFF",
            Basic::Magenta => "#FF00FF",
            Basic::Fuchsia => "#FF00FF",
            Basic::Silver => "#C0C0C0",
            Basic::Gray => "#808080",
            Basic::Maroon => "#800000",
            Basic::Olive => "#808000",
            Basic::Green => "#008000",
            Basic::Purple => "#800080",
            Basic::Teal => "#008080",
            Basic::Navy => "#000080",
        };

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
    /// # use named_colour::Basic;
    /// # use named_colour::Prefix;
    ///     let colour = Basic::Black;
    ///
    ///     assert_eq!("#000000", colour.to_hex_triplet(Prefix::Hash));
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

    /// Parse a colour from a string
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::Basic;
    /// # fn example() {
    ///    let colour = Basic::parse("#000000");
    ///    assert_eq!(Some(Basic::Black), colour);
    ///
    /// # }
    ///```  
    ///
    pub fn parse(name: &str) -> Option<Basic> {
        match name.to_lowercase().as_str() {
            "#000000" | "black" => Some(Basic::Black),
            "#ffffff" | "white" => Some(Basic::White),
            "#ff0000" | "red" => Some(Basic::Red),
            "#00ff00" | "lime" => Some(Basic::Lime),
            "#0000ff" | "blue" => Some(Basic::Blue),
            "#ffff00" | "yellow" => Some(Basic::Yellow),
            "cyan" => Some(Basic::Cyan),
            "#00ffff" | "aqua" => Some(Basic::Aqua),
            "#ff00ff" | "magenta" => Some(Basic::Magenta),
            "fuchsia" => Some(Basic::Fuchsia),
            "#c0c0c0" | "silver" => Some(Basic::Silver),
            "#808080" | "gray" => Some(Basic::Gray),
            "#800000" | "maroon" => Some(Basic::Maroon),
            "#808000" | "olive" => Some(Basic::Olive),
            "#008000" | "green" => Some(Basic::Green),
            "#800080" | "purple" => Some(Basic::Purple),
            "#008080" | "teal" => Some(Basic::Teal),
            "#000080" | "navy" => Some(Basic::Navy),
            _ => None,
        }
    }
}

impl FromStr for Basic {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Basic::parse(s) {
            Some(colour) => Ok(colour),
            None => Err(format!("Invalid Colour: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    #[allow(deprecated)]
    fn display_as_rgb() {
        assert_eq!("(0,255,255)", Basic::Aqua.as_rgb())
    }

    #[rstest]
    #[case(Basic::Aqua, "rgb(0,255,255)")]
    #[case(Basic::Black, "rgb(0,0,0)")]
    #[case(Basic::Blue, "rgb(0,0,255)")]
    #[case(Basic::Cyan, "rgb(0,255,255)")]
    #[case(Basic::Fuchsia, "rgb(255,0,255)")]
    #[case(Basic::Gray, "rgb(128,128,128)")]
    #[case(Basic::Green, "rgb(0,128,0)")]
    #[case(Basic::Magenta, "rgb(255,0,255)")]
    #[case(Basic::Lime, "rgb(0,255,0)")]
    #[case(Basic::Maroon, "rgb(128,0,0)")]
    #[case(Basic::Navy, "rgb(0,0,128)")]
    #[case(Basic::Olive, "rgb(128,128,0)")]
    #[case(Basic::Purple, "rgb(128,0,128)")]
    #[case(Basic::Red, "rgb(255,0,0)")]
    #[case(Basic::Silver, "rgb(192,192,192)")]
    #[case(Basic::Teal, "rgb(0,128,128)")]
    #[case(Basic::White, "rgb(255,255,255)")]
    #[case(Basic::Yellow, "rgb(255,255,0)")]
    fn test_rgb_string(#[case] colour: Basic, #[case] expected: String) {
        let rgb_colour = colour.to_rgb();
        let string = rgb_colour.to_string();

        assert_eq!(expected, string);
    }

    #[rstest]
    #[case(Basic::Aqua, "00FFFF")]
    #[case(Basic::Black, "000000")]
    #[case(Basic::Blue, "0000FF")]
    #[case(Basic::Cyan, "00FFFF")]
    #[case(Basic::Fuchsia, "FF00FF")]
    #[case(Basic::Gray, "808080")]
    #[case(Basic::Green, "008000")]
    #[case(Basic::Magenta, "FF00FF")]
    #[case(Basic::Lime, "00FF00")]
    #[case(Basic::Maroon, "800000")]
    #[case(Basic::Navy, "000080")]
    #[case(Basic::Olive, "808000")]
    #[case(Basic::Purple, "800080")]
    #[case(Basic::Red, "FF0000")]
    #[case(Basic::Silver, "C0C0C0")]
    #[case(Basic::Teal, "008080")]
    #[case(Basic::White, "FFFFFF")]
    #[case(Basic::Yellow, "FFFF00")]
    fn test_hex_triplet_string(
        #[case] colour: Basic,
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
    #[case("aqua", Basic::Aqua)]
    #[case("#000000", Basic::Black)]
    #[case("black", Basic::Black)]
    #[case("#FFFFFF", Basic::White)]
    #[case("white", Basic::White)]
    #[case("#FF0000", Basic::Red)]
    #[case("red", Basic::Red)]
    #[case("#00FF00", Basic::Lime)]
    #[case("lime", Basic::Lime)]
    #[case("#0000FF", Basic::Blue)]
    #[case("blue", Basic::Blue)]
    #[case("#FFFF00", Basic::Yellow)]
    #[case("yellow", Basic::Yellow)]
    #[case("cyan", Basic::Cyan)]
    #[case("#00FFFF", Basic::Aqua)]
    #[case("aqua", Basic::Aqua)]
    #[case("#FF00FF", Basic::Magenta)]
    #[case("magenta", Basic::Magenta)]
    #[case("fuchsia", Basic::Fuchsia)]
    #[case("#C0C0C0", Basic::Silver)]
    #[case("silver", Basic::Silver)]
    #[case("#808080", Basic::Gray)]
    #[case("gray", Basic::Gray)]
    #[case("#800000", Basic::Maroon)]
    #[case("maroon", Basic::Maroon)]
    #[case("#808000", Basic::Olive)]
    #[case("olive", Basic::Olive)]
    #[case("#008000", Basic::Green)]
    #[case("green", Basic::Green)]
    #[case("#800080", Basic::Purple)]
    #[case("purple", Basic::Purple)]
    #[case("#008080", Basic::Teal)]
    #[case("teal", Basic::Teal)]
    #[case("#000080", Basic::Navy)]
    #[case("navy", Basic::Navy)]
    fn test_parse(#[case] input: &str, #[case] expected: Basic) {
        assert_eq!(expected, Basic::from_str(input).unwrap())
    }
}
