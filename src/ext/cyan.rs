//! Extended named colours providing shades collected in enums for the main colour
//!

use std::fmt;

use rgb::Rgb;

use crate::Prefix;

/// Shades of cyan
#[derive(Debug)]
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
            Cyan::MediumAquaMarine => write!(f, "#66CDAA"),
            Cyan::MediumSeaGreen => write!(f, "#3CB371"),
            Cyan::LightSeaGreen => write!(f, "#20B2AA"),
            Cyan::DarkSlateGray => write!(f, "#2F4F4F"),
            Cyan::Teal => write!(f, "#008080"),
            Cyan::DarkCyan => write!(f, "#008B8B"),
            Cyan::Aqua => write!(f, "#00FFFF"),
            Cyan::Cyan => write!(f, "#00FFFF"),
            Cyan::LightCyan => write!(f, "#E0FFFF"),
            Cyan::DarkTurquoise => write!(f, "#00CED1"),
            Cyan::Turquoise => write!(f, "#40E0D0"),
            Cyan::MediumTurquoise => write!(f, "#48D1CC"),
            Cyan::PaleTurquoise => write!(f, "#AFEEEE"),
            Cyan::AquaMarine => write!(f, "#7FFFD4"),
            Cyan::Honeydew => write!(f, "#F0FFF0"),
        }
    }
}

impl Cyan {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Cyan;
    /// # fn example() {
    /// assert_eq!("(0,128,128)", Cyan::Teal.as_rgb())
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
            Cyan::MediumAquaMarine => crate::to_rgb("#66CDAA"),
            Cyan::MediumSeaGreen => crate::to_rgb("#3CB371"),
            Cyan::LightSeaGreen => crate::to_rgb("#20B2AA"),
            Cyan::DarkSlateGray => crate::to_rgb("#2F4F4F"),
            Cyan::Teal => crate::to_rgb("#008080"),
            Cyan::DarkCyan => crate::to_rgb("#008B8B"),
            Cyan::Aqua => crate::to_rgb("#00FFFF"),
            Cyan::Cyan => crate::to_rgb("#00FFFF"),
            Cyan::LightCyan => crate::to_rgb("#E0FFFF"),
            Cyan::DarkTurquoise => crate::to_rgb("#00CED1"),
            Cyan::Turquoise => crate::to_rgb("#40E0D0"),
            Cyan::MediumTurquoise => crate::to_rgb("#48D1CC"),
            Cyan::PaleTurquoise => crate::to_rgb("#AFEEEE"),
            Cyan::AquaMarine => crate::to_rgb("#7FFFD4"),
            Cyan::Honeydew => crate::to_rgb("#F0FFF0"),
        }
    }

    /// Display the colour name as an RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Cyan;
    /// # fn main() {
    ///    let colour = Cyan::Teal;
    ///    let rgb_colour = colour.to_rgb();
    ///
    ///    let string = rgb_colour.to_string();
    ///    assert_eq!("rgb(0,128,128)", string);
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
    /// # use named_colour::ext::Cyan;
    /// # use named_colour::Prefix;
    ///    let colour = Cyan::Teal;
    ///
    ///     assert_eq!("#008080", colour.to_hex_triplet(Prefix::Hash));
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

        let expected = format!("{}{}", prefix_string, expected);

        let hex_colour = colour.to_hex_triplet(prefix);

        assert_eq!(expected, hex_colour);
    }
}
