//! Extended named colours providing shades collected in enums for the main colour
//!

use std::{fmt, str::FromStr};

use rgb::Rgb;

use crate::Prefix;

use super::ExtendedColour;

/// Shades of purple
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Purple {
    Indigo,
    Purple,
    DarkMagenta,
    DarkViolet,
    DarkSlateBlue,
    BlueViolet,
    DarkOrchid,
    Fuchsia,
    Magenta,
    SlateBlue,
    MediumSlateBlue,
    MediumOrchid,
    MediumPurple,
    Orchid,
    Violet,
    Plum,
    Thistle,
    Lavender,
    Pink,
    MediumVioletRed,
    PaleVioletRed,
    DeepPink,
    HotPink,
    LightPink,
}

impl fmt::Display for Purple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Purple::Indigo => write!(f, "#4B0082"),
            Purple::Purple => write!(f, "#800080"),
            Purple::DarkMagenta => write!(f, "#8B008B"),
            Purple::DarkViolet => write!(f, "#9400D3"),
            Purple::DarkSlateBlue => write!(f, "#483D8B"),
            Purple::BlueViolet => write!(f, "#8A2BE2"),
            Purple::DarkOrchid => write!(f, "#9932CC"),
            Purple::Fuchsia => write!(f, "#FF00FF"),
            Purple::Magenta => write!(f, "#FF00FF"),
            Purple::SlateBlue => write!(f, "#6A5ACD"),
            Purple::MediumSlateBlue => write!(f, "#7B68EE"),
            Purple::MediumOrchid => write!(f, "#BA55D3"),
            Purple::MediumPurple => write!(f, "#9370DB"),
            Purple::Orchid => write!(f, "#DA70D6"),
            Purple::Violet => write!(f, "#EE82EE"),
            Purple::Plum => write!(f, "#DDA0DD"),
            Purple::Thistle => write!(f, "#D8BFD8"),
            Purple::Lavender => write!(f, "#E6E6FA"),
            Purple::Pink => write!(f, "#FFC0CB"),
            Purple::MediumVioletRed => write!(f, "#C71585"),
            Purple::PaleVioletRed => write!(f, "#DB7093"),
            Purple::DeepPink => write!(f, "#FF1493"),
            Purple::HotPink => write!(f, "#FF69B4"),
            Purple::LightPink => write!(f, "#FFB6C1"),
        }
    }
}

impl Purple {
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

    /// Parse a colour from string
    ///
    /// ## Example
    ///
    /// ```
    /// # use named_colour::ext::Purple;
    /// # fn main() {
    ///    let colour = Purple::Purple;
    ///    assert_eq!(colour, Purple::parse("purple").unwrap());
    ///    assert_eq!(colour, Purple::parse("#800080").unwrap());
    ///    assert_eq!(colour, Purple::parse("#800080").unwrap());
    ///
    /// # }
    /// ```
    pub fn parse(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "#4b0082" | "4b0082" | "indigo" => Some(Self::Indigo),
            "#800080" | "800080" | "purple" => Some(Self::Purple),
            "#8b008b" | "8b008b" | "darkmagenta" => Some(Self::DarkMagenta),
            "#9400d3" | "9400d3" | "darkviolet" => Some(Self::DarkViolet),
            "#483d8b" | "483d8b" | "darkslateblue" => Some(Self::DarkSlateBlue),
            "#8a2be2" | "8a2be2" | "blueviolet" => Some(Self::BlueViolet),
            "#9932cc" | "9932cc" | "darkorchid" => Some(Self::DarkOrchid),
            "#a020f0" | "a020f0" | "fuchsia" => Some(Self::Fuchsia),
            "#ff00ff" | "ff00ff" | "magenta" => Some(Self::Magenta),
            "#6a5acd" | "6a5acd" | "slateblue" => Some(Self::SlateBlue),
            "#7b68ee" | "7b68ee" | "mediumslateblue" => Some(Self::MediumSlateBlue),
            "#ba55d3" | "ba55d3" | "mediumorchid" => Some(Self::MediumOrchid),
            "#9370db" | "9370db" | "mediumpurple" => Some(Self::MediumPurple),
            "#da70d6" | "da70d6" | "orchid" => Some(Self::Orchid),
            "#ee82ee" | "ee82ee" | "violet" => Some(Self::Violet),
            "#dda0dd" | "dda0dd" | "plum" => Some(Self::Plum),
            "#d8bfd8" | "d8bfd8" | "thistle" => Some(Self::Thistle),
            "#e6e6fa" | "e6e6fa" | "lavender" => Some(Self::Lavender),
            "#ffc0cb" | "ffc0cb" | "pink" => Some(Self::Pink),
            "#c71585" | "c71585" | "mediumvioletred" => Some(Self::MediumVioletRed),
            "#db7093" | "db7093" | "palevioletred" => Some(Self::PaleVioletRed),
            "#ff1493" | "ff1493" | "deeppink" => Some(Self::DeepPink),
            "#ff69b4" | "ff69b4" | "hotpink" => Some(Self::HotPink),
            "#ffb6c1" | "ffb6c1" | "lightpink" => Some(Self::LightPink),
            _ => None,
        }
    }
}

impl FromStr for Purple {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse(s) {
            Some(colour) => Ok(colour),
            None => Err(format!("Invalid Colour: {}", s)),
        }
    }
}

impl ExtendedColour for Purple {}

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

    #[rstest]
    #[case("#4b0082", Purple::Indigo)]
    #[case("4b0082", Purple::Indigo)]
    #[case("indigo", Purple::Indigo)]
    #[case("#800080", Purple::Purple)]
    #[case("800080", Purple::Purple)]
    #[case("purple", Purple::Purple)]
    #[case("#8b008b", Purple::DarkMagenta)]
    #[case("8b008b", Purple::DarkMagenta)]
    #[case("darkmagenta", Purple::DarkMagenta)]
    #[case("#9400d3", Purple::DarkViolet)]
    #[case("9400d3", Purple::DarkViolet)]
    #[case("darkviolet", Purple::DarkViolet)]
    #[case("#483d8b", Purple::DarkSlateBlue)]
    #[case("483d8b", Purple::DarkSlateBlue)]
    #[case("darkslateblue", Purple::DarkSlateBlue)]
    #[case("#8a2be2", Purple::BlueViolet)]
    #[case("8a2be2", Purple::BlueViolet)]
    #[case("blueviolet", Purple::BlueViolet)]
    #[case("#9932cc", Purple::DarkOrchid)]
    #[case("9932cc", Purple::DarkOrchid)]
    #[case("darkorchid", Purple::DarkOrchid)]
    #[case("#a020f0", Purple::Fuchsia)]
    #[case("a020f0", Purple::Fuchsia)]
    #[case("fuchsia", Purple::Fuchsia)]
    #[case("#ff00ff", Purple::Magenta)]
    #[case("ff00ff", Purple::Magenta)]
    #[case("magenta", Purple::Magenta)]
    #[case("#6a5acd", Purple::SlateBlue)]
    #[case("6a5acd", Purple::SlateBlue)]
    #[case("slateblue", Purple::SlateBlue)]
    #[case("#7b68ee", Purple::MediumSlateBlue)]
    #[case("7b68ee", Purple::MediumSlateBlue)]
    #[case("mediumslateblue", Purple::MediumSlateBlue)]
    #[case("#ba55d3", Purple::MediumOrchid)]
    #[case("ba55d3", Purple::MediumOrchid)]
    #[case("mediumorchid", Purple::MediumOrchid)]
    #[case("#9370db", Purple::MediumPurple)]
    #[case("9370db", Purple::MediumPurple)]
    #[case("mediumpurple", Purple::MediumPurple)]
    #[case("#da70d6", Purple::Orchid)]
    #[case("da70d6", Purple::Orchid)]
    #[case("orchid", Purple::Orchid)]
    #[case("#ee82ee", Purple::Violet)]
    #[case("ee82ee", Purple::Violet)]
    #[case("violet", Purple::Violet)]
    #[case("#dda0dd", Purple::Plum)]
    #[case("dda0dd", Purple::Plum)]
    #[case("plum", Purple::Plum)]
    #[case("#d8bfd8", Purple::Thistle)]
    #[case("d8bfd8", Purple::Thistle)]
    #[case("thistle", Purple::Thistle)]
    #[case("#e6e6fa", Purple::Lavender)]
    #[case("e6e6fa", Purple::Lavender)]
    #[case("lavender", Purple::Lavender)]
    #[case("#ffc0cb", Purple::Pink)]
    #[case("ffc0cb", Purple::Pink)]
    #[case("pink", Purple::Pink)]
    #[case("#c71585", Purple::MediumVioletRed)]
    #[case("c71585", Purple::MediumVioletRed)]
    #[case("mediumvioletred", Purple::MediumVioletRed)]
    #[case("#db7093", Purple::PaleVioletRed)]
    #[case("db7093", Purple::PaleVioletRed)]
    #[case("palevioletred", Purple::PaleVioletRed)]
    #[case("deeppink", Purple::DeepPink)]
    #[case("ff1493", Purple::DeepPink)]
    #[case("deeppink", Purple::DeepPink)]
    #[case("#ff69b4", Purple::HotPink)]
    #[case("ff69b4", Purple::HotPink)]
    #[case("hotpink", Purple::HotPink)]
    #[case("#ffb6c1", Purple::LightPink)]
    #[case("ffb6c1", Purple::LightPink)]
    #[case("lightpink", Purple::LightPink)]
    fn test_from_str(#[case] input: &str, #[case] expected: Purple) {
        assert_eq!(expected, Purple::from_str(input).unwrap())
    }

    #[rstest]
    #[case("#4b0082", Some(Purple::Indigo))]
    #[case("4b0082", Some(Purple::Indigo))]
    #[case("indigo", Some(Purple::Indigo))]
    #[case("#800080", Some(Purple::Purple))]
    #[case("800080", Some(Purple::Purple))]
    #[case("purple", Some(Purple::Purple))]
    #[case("#8b008b", Some(Purple::DarkMagenta))]
    #[case("8b008b", Some(Purple::DarkMagenta))]
    #[case("darkmagenta", Some(Purple::DarkMagenta))]
    #[case("#9400d3", Some(Purple::DarkViolet))]
    #[case("9400d3", Some(Purple::DarkViolet))]
    #[case("darkviolet", Some(Purple::DarkViolet))]
    #[case("#483d8b", Some(Purple::DarkSlateBlue))]
    #[case("483d8b", Some(Purple::DarkSlateBlue))]
    #[case("darkslateblue", Some(Purple::DarkSlateBlue))]
    #[case("#8a2be2", Some(Purple::BlueViolet))]
    #[case("8a2be2", Some(Purple::BlueViolet))]
    #[case("blueviolet", Some(Purple::BlueViolet))]
    #[case("#9932cc", Some(Purple::DarkOrchid))]
    #[case("9932cc", Some(Purple::DarkOrchid))]
    #[case("darkorchid", Some(Purple::DarkOrchid))]
    #[case("#a020f0", Some(Purple::Fuchsia))]
    #[case("a020f0", Some(Purple::Fuchsia))]
    #[case("fuchsia", Some(Purple::Fuchsia))]
    #[case("#ff00ff", Some(Purple::Magenta))]
    #[case("ff00ff", Some(Purple::Magenta))]
    #[case("magenta", Some(Purple::Magenta))]
    #[case("#6a5acd", Some(Purple::SlateBlue))]
    #[case("6a5acd", Some(Purple::SlateBlue))]
    #[case("slateblue", Some(Purple::SlateBlue))]
    #[case("#7b68ee", Some(Purple::MediumSlateBlue))]
    #[case("7b68ee", Some(Purple::MediumSlateBlue))]
    #[case("mediumslateblue", Some(Purple::MediumSlateBlue))]
    #[case("#ba55d3", Some(Purple::MediumOrchid))]
    #[case("ba55d3", Some(Purple::MediumOrchid))]
    #[case("mediumorchid", Some(Purple::MediumOrchid))]
    #[case("#9370db", Some(Purple::MediumPurple))]
    #[case("9370db", Some(Purple::MediumPurple))]
    #[case("mediumpurple", Some(Purple::MediumPurple))]
    #[case("#da70d6", Some(Purple::Orchid))]
    #[case("da70d6", Some(Purple::Orchid))]
    #[case("orchid", Some(Purple::Orchid))]
    #[case("#ee82ee", Some(Purple::Violet))]
    #[case("ee82ee", Some(Purple::Violet))]
    #[case("violet", Some(Purple::Violet))]
    #[case("#dda0dd", Some(Purple::Plum))]
    #[case("dda0dd", Some(Purple::Plum))]
    #[case("plum", Some(Purple::Plum))]
    #[case("#d8bfd8", Some(Purple::Thistle))]
    #[case("d8bfd8", Some(Purple::Thistle))]
    #[case("thistle", Some(Purple::Thistle))]
    #[case("#e6e6fa", Some(Purple::Lavender))]
    #[case("e6e6fa", Some(Purple::Lavender))]
    #[case("lavender", Some(Purple::Lavender))]
    #[case("#ffc0cb", Some(Purple::Pink))]
    #[case("ffc0cb", Some(Purple::Pink))]
    #[case("pink", Some(Purple::Pink))]
    #[case("#c71585", Some(Purple::MediumVioletRed))]
    #[case("c71585", Some(Purple::MediumVioletRed))]
    #[case("mediumvioletred", Some(Purple::MediumVioletRed))]
    #[case("#db7093", Some(Purple::PaleVioletRed))]
    #[case("db7093", Some(Purple::PaleVioletRed))]
    #[case("palevioletred", Some(Purple::PaleVioletRed))]
    #[case("deeppink", Some(Purple::DeepPink))]
    #[case("ff1493", Some(Purple::DeepPink))]
    #[case("deeppink", Some(Purple::DeepPink))]
    #[case("#ff69b4", Some(Purple::HotPink))]
    #[case("ff69b4", Some(Purple::HotPink))]
    #[case("hotpink", Some(Purple::HotPink))]
    #[case("#ffb6c1", Some(Purple::LightPink))]
    #[case("ffb6c1", Some(Purple::LightPink))]
    #[case("lightpink", Some(Purple::LightPink))]
    #[case("012345", None)]
    fn test_name_colour(#[case] input: &str, #[case] expected: Option<Purple>) {
        assert_eq!(expected, Purple::name_colour(input))
    }
}
