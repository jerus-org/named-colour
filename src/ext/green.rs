//! Extended named colours providing shades collected in enums for the main colour
//!

use std::fmt;

use rgb::Rgb;

use crate::Prefix;

/// Shades of green
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Green {
    YellowGreen,
    DarkOliveGreen,
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
            Green::YellowGreen => write!(f, "#9ACD32"),
            Green::DarkOliveGreen => write!(f, "#556B2F"),
            Green::OliveDrab => write!(f, "#6B8E23"),
            Green::LawnGreen => write!(f, "#7CFC00"),
            Green::ChartReuse => write!(f, "#7FFF00"),
            Green::GreenYellow => write!(f, "#ADFF2F"),
            Green::DarkGreen => write!(f, "#006400"),
            Green::Green => write!(f, "#008000"),
            Green::ForestGreen => write!(f, "#228B22"),
            Green::Lime => write!(f, "#00FF00"),
            Green::LimeGreen => write!(f, "#32CD32"),
            Green::LightGreen => write!(f, "#90EE90"),
            Green::PaleGreen => write!(f, "#98FB98"),
            Green::DarkSeaGreen => write!(f, "#8FBC8F"),
            Green::MediumSpringGreen => write!(f, "#00FA9A"),
            Green::SpringGreen => write!(f, "#00FF7F"),
            Green::SeaGreen => write!(f, "#2E8B57"),
        }
    }
}

impl Green {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Green;
    /// # fn example() {
    /// assert_eq!("(107,142,35)", Green::OliveDrab.as_rgb())
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
            Green::YellowGreen => crate::to_rgb("#9ACD32"),
            Green::DarkOliveGreen => crate::to_rgb("#556B2F"),
            Green::OliveDrab => crate::to_rgb("#6B8E23"),
            Green::LawnGreen => crate::to_rgb("#7CFC00"),
            Green::ChartReuse => crate::to_rgb("#7FFF00"),
            Green::GreenYellow => crate::to_rgb("#ADFF2F"),
            Green::DarkGreen => crate::to_rgb("#006400"),
            Green::Green => crate::to_rgb("#008000"),
            Green::ForestGreen => crate::to_rgb("#228B22"),
            Green::Lime => crate::to_rgb("#00FF00"),
            Green::LimeGreen => crate::to_rgb("#32CD32"),
            Green::LightGreen => crate::to_rgb("#90EE90"),
            Green::PaleGreen => crate::to_rgb("#98FB98"),
            Green::DarkSeaGreen => crate::to_rgb("#8FBC8F"),
            Green::MediumSpringGreen => crate::to_rgb("#00FA9A"),
            Green::SpringGreen => crate::to_rgb("#00FF7F"),
            Green::SeaGreen => crate::to_rgb("#2E8B57"),
        }
    }

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
}

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
}
