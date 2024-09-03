//! Extended named colours providing shades collected in enums for the main colour
//!

use std::fmt;

use rgb::Rgb;

use crate::Prefix;

/// Shades of blue
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Blue {
    PowderBlue,
    CadetBlue,
    SteelBlue,
    CornFlowerBlue,
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
}

impl fmt::Display for Blue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Blue::PowderBlue => write!(f, "#B0E0E6"),
            Blue::CadetBlue => write!(f, "#5F9EA0"),
            Blue::SteelBlue => write!(f, "#4682B4"),
            Blue::CornFlowerBlue => write!(f, "#6495ED"),
            Blue::DeepSkyBlue => write!(f, "#00BFFF"),
            Blue::DodgerBlue => write!(f, "#1E90FF"),
            Blue::LightBlue => write!(f, "#ADD8E6"),
            Blue::SkyBlue => write!(f, "#87CEEB"),
            Blue::LightSkyBlue => write!(f, "#87CEFA"),
            Blue::MidnightBlue => write!(f, "#191970"),
            Blue::Navy => write!(f, "#000080"),
            Blue::DarkBlue => write!(f, "#00008B"),
            Blue::MediumBlue => write!(f, "#0000CD"),
            Blue::Blue => write!(f, "#0000FF"),
            Blue::RoyalBlue => write!(f, "#4169E1"),
            Blue::Azure => write!(f, "#F0FFFF"),
        }
    }
}

impl Blue {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Blue;
    /// # fn example() {
    /// assert_eq!("(0,0,255)", Blue::SteelBlue.as_rgb())
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
            Blue::PowderBlue => crate::to_rgb("#B0E0E6"),
            Blue::CadetBlue => crate::to_rgb("#5F9EA0"),
            Blue::SteelBlue => crate::to_rgb("#4682B4"),
            Blue::CornFlowerBlue => crate::to_rgb("#6495ED"),
            Blue::DeepSkyBlue => crate::to_rgb("#00BFFF"),
            Blue::DodgerBlue => crate::to_rgb("#1E90FF"),
            Blue::LightBlue => crate::to_rgb("#ADD8E6"),
            Blue::SkyBlue => crate::to_rgb("#87CEEB"),
            Blue::LightSkyBlue => crate::to_rgb("#87CEFA"),
            Blue::MidnightBlue => crate::to_rgb("#191970"),
            Blue::Navy => crate::to_rgb("#000080"),
            Blue::DarkBlue => crate::to_rgb("#00008B"),
            Blue::MediumBlue => crate::to_rgb("#0000CD"),
            Blue::Blue => crate::to_rgb("#0000FF"),
            Blue::RoyalBlue => crate::to_rgb("#4169E1"),
            Blue::Azure => crate::to_rgb("#F0FFFF"),
        }
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Blue::PowderBlue, "rgb(176,224,230)")]
    #[case(Blue::CadetBlue, "rgb(95,158,160)")]
    #[case(Blue::SteelBlue, "rgb(70,130,180)")]
    #[case(Blue::CornFlowerBlue, "rgb(100,149,237)")]
    #[case(Blue::DeepSkyBlue, "rgb(0,191,255)")]
    #[case(Blue::DodgerBlue, "rgb(30,144,255)")]
    #[case(Blue::LightBlue, "rgb(173,216,230)")]
    #[case(Blue::SkyBlue, "rgb(135,206,235)")]
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
    #[case(Blue::CornFlowerBlue, "6495ED")]
    #[case(Blue::DeepSkyBlue, "00BFFF")]
    #[case(Blue::DodgerBlue, "1E90FF")]
    #[case(Blue::LightBlue, "ADD8E6")]
    #[case(Blue::SkyBlue, "87CEEB")]
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
}