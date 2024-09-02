//! Extended named colours providing shades collected in enums for the main colour
//!

use std::fmt;

use rgb::Rgb;

use crate::Prefix;

/// Shades of indigo
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Indigo {
    BlueViolet,
    Indigo,
    DarkSlateBlue,
    SlateBlue,
    MediumSlateBlue,
    MediumPurple,
    DarkMagenta,
    DarkViolet,
    DarkOrchid,
    MediumOrchid,
}

impl fmt::Display for Indigo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Indigo::BlueViolet => write!(f, "#8A2BE2"),
            Indigo::Indigo => write!(f, "#4B0082"),
            Indigo::DarkSlateBlue => write!(f, "#483D8B"),
            Indigo::SlateBlue => write!(f, "#6A5ACD"),
            Indigo::MediumSlateBlue => write!(f, "#7B68EE"),
            Indigo::MediumPurple => write!(f, "#9370DB"),
            Indigo::DarkMagenta => write!(f, "#8B008B"),
            Indigo::DarkViolet => write!(f, "#9400D3"),
            Indigo::DarkOrchid => write!(f, "#9932CC"),
            Indigo::MediumOrchid => write!(f, "#BA55D3"),
        }
    }
}

impl Indigo {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Indigo;
    /// # fn example() {
    /// assert_eq!("(75,0,130)", Indigo::Indigo.as_rgb())
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
            Indigo::BlueViolet => crate::to_rgb("#8A2BE2"),
            Indigo::Indigo => crate::to_rgb("#4B0082"),
            Indigo::DarkSlateBlue => crate::to_rgb("#483D8B"),
            Indigo::SlateBlue => crate::to_rgb("#6A5ACD"),
            Indigo::MediumSlateBlue => crate::to_rgb("#7B68EE"),
            Indigo::MediumPurple => crate::to_rgb("#9370DB"),
            Indigo::DarkMagenta => crate::to_rgb("#8B008B"),
            Indigo::DarkViolet => crate::to_rgb("#9400D3"),
            Indigo::DarkOrchid => crate::to_rgb("#9932CC"),
            Indigo::MediumOrchid => crate::to_rgb("#BA55D3"),
        }
    }

    /// Display the colour name as an RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Indigo;
    /// # fn main() {
    ///    let colour = Indigo::DarkOrchid;
    ///    let rgb_colour = colour.to_rgb();
    ///
    ///    let string = rgb_colour.to_string();
    ///    assert_eq!("rgb(153,50,204)", string);
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
    /// # use named_colour::ext::Indigo;
    /// # use named_colour::Prefix;
    ///    let colour = Indigo::DarkOrchid;
    ///
    ///     assert_eq!("#9932CC", colour.to_hex_triplet(Prefix::Hash));
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
    #[case(Indigo::BlueViolet, "rgb(138,43,226)")]
    #[case(Indigo::Indigo, "rgb(75,0,130)")]
    #[case(Indigo::DarkSlateBlue, "rgb(72,61,139)")]
    #[case(Indigo::SlateBlue, "rgb(106,90,205)")]
    #[case(Indigo::MediumSlateBlue, "rgb(123,104,238)")]
    #[case(Indigo::MediumPurple, "rgb(147,112,219)")]
    #[case(Indigo::DarkMagenta, "rgb(139,0,139)")]
    #[case(Indigo::DarkViolet, "rgb(148,0,211)")]
    #[case(Indigo::DarkOrchid, "rgb(153,50,204)")]
    #[case(Indigo::MediumOrchid, "rgb(186,85,211)")]
    fn test_rgb_string(#[case] colour: Indigo, #[case] expected: String) {
        let rgb_colour = colour.to_rgb();
        let string = rgb_colour.to_string();

        assert_eq!(expected, string);
    }

    #[rstest]
    #[case(Indigo::BlueViolet, "8A2BE2")]
    #[case(Indigo::Indigo, "4B0082")]
    #[case(Indigo::DarkSlateBlue, "483D8B")]
    #[case(Indigo::SlateBlue, "6A5ACD")]
    #[case(Indigo::MediumSlateBlue, "7B68EE")]
    #[case(Indigo::MediumPurple, "9370DB")]
    #[case(Indigo::DarkMagenta, "8B008B")]
    #[case(Indigo::DarkViolet, "9400D3")]
    #[case(Indigo::DarkOrchid, "9932CC")]
    #[case(Indigo::MediumOrchid, "BA55D3")]
    fn test_hex_triplet_string(
        #[case] colour: Indigo,
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
