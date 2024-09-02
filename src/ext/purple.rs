//! Extended named colours providing shades collected in enums for the main colour
//!

use std::fmt;

use rgb::Rgb;

use crate::Prefix;

/// Shades of purple
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Purple {
    Purple,
    Thistle,
    Plum,
    Violet,
    Magenta,
    Fuchsia,
    Orchid,
    MediumVioletRed,
    PaleVioletRed,
    DeepPink,
    HotPink,
    LightPink,
    Pink,
}

impl fmt::Display for Purple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Purple::Purple => write!(f, "#800080"),
            Purple::Thistle => write!(f, "#D8BFD8"),
            Purple::Plum => write!(f, "#DDA0DD"),
            Purple::Violet => write!(f, "#EE82EE"),
            Purple::Magenta => write!(f, "#FF00FF"),
            Purple::Fuchsia => write!(f, "#FF00FF"),
            Purple::Orchid => write!(f, "#DA70D6"),
            Purple::MediumVioletRed => write!(f, "#C71585"),
            Purple::PaleVioletRed => write!(f, "#DB7093"),
            Purple::DeepPink => write!(f, "#FF1493"),
            Purple::HotPink => write!(f, "#FF69B4"),
            Purple::LightPink => write!(f, "#FFB6C1"),
            Purple::Pink => write!(f, "#FFC0CB"),
        }
    }
}

impl Purple {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Purple;
    /// # fn example() {
    /// assert_eq!("128,0,128)", Purple::Purple.as_rgb())
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
            Purple::Purple => crate::to_rgb("#800080"),
            Purple::Thistle => crate::to_rgb("#D8BFD8"),
            Purple::Plum => crate::to_rgb("#DDA0DD"),
            Purple::Violet => crate::to_rgb("#EE82EE"),
            Purple::Magenta => crate::to_rgb("#FF00FF"),
            Purple::Fuchsia => crate::to_rgb("#FF00FF"),
            Purple::Orchid => crate::to_rgb("#DA70D6"),
            Purple::MediumVioletRed => crate::to_rgb("#C71585"),
            Purple::PaleVioletRed => crate::to_rgb("#DB7093"),
            Purple::DeepPink => crate::to_rgb("#FF1493"),
            Purple::HotPink => crate::to_rgb("#FF69B4"),
            Purple::LightPink => crate::to_rgb("#FFB6C1"),
            Purple::Pink => crate::to_rgb("#FFC0CB"),
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
    #[case(Purple::Purple, "rgb(128,0,128)")]
    #[case(Purple::Thistle, "rgb(216,191,216)")]
    #[case(Purple::Plum, "rgb(221,160,221)")]
    #[case(Purple::Violet, "rgb(238,130,238)")]
    #[case(Purple::Magenta, "rgb(255,0,255)")]
    #[case(Purple::Fuchsia, "rgb(255,0,255)")]
    #[case(Purple::Orchid, "rgb(218,112,214)")]
    #[case(Purple::MediumVioletRed, "rgb(199,21,133)")]
    #[case(Purple::PaleVioletRed, "rgb(219,112,147)")]
    #[case(Purple::DeepPink, "rgb(255,20,147)")]
    #[case(Purple::HotPink, "rgb(255,105,180)")]
    #[case(Purple::LightPink, "rgb(255,182,193)")]
    #[case(Purple::Pink, "rgb(255,192,203)")]
    fn test_rgb_string(#[case] colour: Purple, #[case] expected: String) {
        let rgb_colour = colour.to_rgb();
        let string = rgb_colour.to_string();

        assert_eq!(expected, string);
    }

    #[rstest]
    #[case(Purple::Purple, "800080")]
    #[case(Purple::Thistle, "D8BFD8")]
    #[case(Purple::Plum, "DDA0DD")]
    #[case(Purple::Violet, "EE82EE")]
    #[case(Purple::Magenta, "FF00FF")]
    #[case(Purple::Fuchsia, "FF00FF")]
    #[case(Purple::Orchid, "DA70D6")]
    #[case(Purple::MediumVioletRed, "C71585")]
    #[case(Purple::PaleVioletRed, "DB7093")]
    #[case(Purple::DeepPink, "FF1493")]
    #[case(Purple::HotPink, "FF69B4")]
    #[case(Purple::LightPink, "FFB6C1")]
    #[case(Purple::Pink, "FFC0CB")]
    fn test_hex_triplet_string(
        #[case] colour: Purple,
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
