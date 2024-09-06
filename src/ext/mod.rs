//! Extended named colours providing shades collected in enums for the main colour
//!  

mod black;
mod blue;
mod brown;
mod cyan;
mod green;
mod purple;
mod red;
mod white;
mod yellow;

use std::str::FromStr;

use tinyrand::{RandRange, StdRand};

pub use black::Black;
pub use blue::Blue;
pub use brown::Brown;
pub use cyan::Cyan;
pub use green::Green;
pub use purple::Purple;
pub use red::Red;
pub use white::White;
pub use yellow::Yellow;

/// An extended colour
///
/// This trait is implemented for all the extended colours
///
pub trait ExtendedColour {
    /// Returns the name of the colour
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use named_colour::ExtendedColour;
    /// # use std::str::FromStr;
    /// # use named_colour::Black;
    ///
    ///     let colour = "#d3d3d3";
    ///     let name = Black::name_colour(colour).unwrap();
    ///     assert_eq!(Black::LightGray, name);
    ///
    /// ```
    fn name_colour(colour: &str) -> Option<Self>
    where
        Self: Sized,
        Self: FromStr,
    {
        if let Ok(c) = Self::from_str(colour) {
            Some(c)
        } else {
            None
        }
    }
}

/// Returns the name of the colour
///
///
/// # Examples
///
/// ```rust
/// use named_colour::ExtendedColour;
/// use std::str::FromStr;
/// use named_colour::Black;
///
///     let colour = "#d3d3d3";
///     assert_eq!(Black::LightGray, named_colour::name_colour(colour).unwrap());
///
/// ```
pub fn name_colour<T: ExtendedColour + FromStr>(colour: &str) -> Option<T> {
    T::name_colour(colour)
}

/// Returns a random colour
///
/// # Examples
///
/// ```rust
/// # use named_colour::ExtendedColour;
///     
///     let colour = named_colour::random_named_colour();
///
/// ```
///
pub fn random_named_colour() -> Box<dyn ExtendedColour> {
    let mut rand = StdRand::default();

    let family = rand.next_range(0..8_u32);
    match family {
        0 => Box::new(Black::random()),
        1 => Box::new(Blue::random()),
        2 => Box::new(Brown::random()),
        3 => Box::new(Cyan::random()),
        4 => Box::new(Green::random()),
        5 => Box::new(Purple::random()),
        6 => Box::new(Red::random()),
        7 => Box::new(White::random()),
        8 => Box::new(Yellow::random()),
        _ => Box::new(Black::random()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AnonColour {
    NotNamed,
}

impl FromStr for AnonColour {
    type Err = ();
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(AnonColour::NotNamed)
    }
}

impl ExtendedColour for AnonColour {}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("#123456", Some(AnonColour::NotNamed))]
    #[case("#708090", Some(Black::SlateGray))]
    #[case("708090", Some(Black::SlateGray))]
    #[case("slategray", Some(Black::SlateGray))]
    #[case("slategrey", Some(Black::SlateGrey))]
    #[case("#778899", Some(Black::LightSlateGray))]
    #[case("778899", Some(Black::LightSlateGray))]
    #[case("lightslategray", Some(Black::LightSlateGray))]
    #[case("lightslategrey", Some(Black::LightSlateGrey))]
    #[case("#000000", Some(Black::Black))]
    #[case("000000", Some(Black::Black))]
    #[case("black", Some(Black::Black))]
    #[case("#696969", Some(Black::DimGray))]
    #[case("696969", Some(Black::DimGray))]
    #[case("dimgray", Some(Black::DimGray))]
    #[case("dimgrey", Some(Black::DimGrey))]
    #[case("#808080", Some(Black::Gray))]
    #[case("808080", Some(Black::Gray))]
    #[case("gray", Some(Black::Gray))]
    #[case("grey", Some(Black::Grey))]
    #[case("#A9A9A9", Some(Black::DarkGray))]
    #[case("A9A9A9", Some(Black::DarkGray))]
    #[case("darkgray", Some(Black::DarkGray))]
    #[case("darkgrey", Some(Black::DarkGrey))]
    #[case("#C0C0C0", Some(Black::Silver))]
    #[case("C0C0C0", Some(Black::Silver))]
    #[case("silver", Some(Black::Silver))]
    #[case("#D3D3D3", Some(Black::LightGray))]
    #[case("D3D3D3", Some(Black::LightGray))]
    #[case("lightgray", Some(Black::LightGray))]
    #[case("lightgrey", Some(Black::LightGrey))]
    #[case("#DCDCDC", Some(Black::Gainsboro))]
    #[case("DCDCDC", Some(Black::Gainsboro))]
    #[case("gainsboro", Some(Black::Gainsboro))]
    #[case("#b0e0e6", Some(Blue::PowderBlue))]
    #[case("b0e0e6", Some(Blue::PowderBlue))]
    #[case("powderblue", Some(Blue::PowderBlue))]
    #[case("#5f9ea0", Some(Blue::CadetBlue))]
    #[case("5f9ea0", Some(Blue::CadetBlue))]
    #[case("cadetblue", Some(Blue::CadetBlue))]
    #[case("#4682b4", Some(Blue::SteelBlue))]
    #[case("4682b4", Some(Blue::SteelBlue))]
    #[case("steelblue", Some(Blue::SteelBlue))]
    #[case("#6495ed", Some(Blue::CornflowerBlue))]
    #[case("6495ed", Some(Blue::CornflowerBlue))]
    #[case("cornflowerblue", Some(Blue::CornflowerBlue))]
    #[case("#00bfff", Some(Blue::DeepSkyBlue))]
    #[case("00bfff", Some(Blue::DeepSkyBlue))]
    #[case("deepskyblue", Some(Blue::DeepSkyBlue))]
    #[case("#1e90ff", Some(Blue::DodgerBlue))]
    #[case("1e90ff", Some(Blue::DodgerBlue))]
    #[case("dodgerblue", Some(Blue::DodgerBlue))]
    #[case("#add8e6", Some(Blue::LightBlue))]
    #[case("add8e6", Some(Blue::LightBlue))]
    #[case("lightblue", Some(Blue::LightBlue))]
    #[case("#87ceeb", Some(Blue::SkyBlue))]
    #[case("87ceeb", Some(Blue::SkyBlue))]
    #[case("skyblue", Some(Blue::SkyBlue))]
    #[case("#b0c4de", Some(Blue::LightSteelBlue))]
    #[case("b0c4de", Some(Blue::LightSteelBlue))]
    #[case("lightsteelblue", Some(Blue::LightSteelBlue))]
    #[case("#87cefa", Some(Blue::LightSkyBlue))]
    #[case("87cefa", Some(Blue::LightSkyBlue))]
    #[case("lightskyblue", Some(Blue::LightSkyBlue))]
    #[case("#191970", Some(Blue::MidnightBlue))]
    #[case("191970", Some(Blue::MidnightBlue))]
    #[case("midnightblue", Some(Blue::MidnightBlue))]
    #[case("#000080", Some(Blue::Navy))]
    #[case("000080", Some(Blue::Navy))]
    #[case("navy", Some(Blue::Navy))]
    #[case("#00008b", Some(Blue::DarkBlue))]
    #[case("00008b", Some(Blue::DarkBlue))]
    #[case("darkblue", Some(Blue::DarkBlue))]
    #[case("#0000cd", Some(Blue::MediumBlue))]
    #[case("0000cd", Some(Blue::MediumBlue))]
    #[case("mediumblue", Some(Blue::MediumBlue))]
    #[case("#0000ff", Some(Blue::Blue))]
    #[case("0000ff", Some(Blue::Blue))]
    #[case("blue", Some(Blue::Blue))]
    #[case("#4169e1", Some(Blue::RoyalBlue))]
    #[case("4169e1", Some(Blue::RoyalBlue))]
    #[case("royalblue", Some(Blue::RoyalBlue))]
    #[case("#f0ffff", Some(Blue::Azure))]
    #[case("f0ffff", Some(Blue::Azure))]
    #[case("azure", Some(Blue::Azure))]
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
    #[case("#66cdaa", Some(Cyan::MediumAquaMarine))]
    #[case("66cdaa", Some(Cyan::MediumAquaMarine))]
    #[case("mediumaquamarine", Some(Cyan::MediumAquaMarine))]
    #[case("#3cb371", Some(Cyan::MediumSeaGreen))]
    #[case("3cb371", Some(Cyan::MediumSeaGreen))]
    #[case("mediumseagreen", Some(Cyan::MediumSeaGreen))]
    #[case("#20b2aa", Some(Cyan::LightSeaGreen))]
    #[case("20b2aa", Some(Cyan::LightSeaGreen))]
    #[case("lightseagreen", Some(Cyan::LightSeaGreen))]
    #[case("#2f4f4f", Some(Cyan::DarkSlateGray))]
    #[case("2f4f4f", Some(Cyan::DarkSlateGray))]
    #[case("darkslategray", Some(Cyan::DarkSlateGray))]
    #[case("#008080", Some(Cyan::Teal))]
    #[case("008080", Some(Cyan::Teal))]
    #[case("teal", Some(Cyan::Teal))]
    #[case("#008b8b", Some(Cyan::DarkCyan))]
    #[case("008b8b", Some(Cyan::DarkCyan))]
    #[case("darkcyan", Some(Cyan::DarkCyan))]
    #[case("#00ffff", Some(Cyan::Aqua))]
    #[case("00ffff", Some(Cyan::Aqua))]
    #[case("aqua", Some(Cyan::Aqua))]
    #[case("cyan", Some(Cyan::Cyan))]
    #[case("#e0ffff", Some(Cyan::LightCyan))]
    #[case("e0ffff", Some(Cyan::LightCyan))]
    #[case("lightcyan", Some(Cyan::LightCyan))]
    #[case("#00ced1", Some(Cyan::DarkTurquoise))]
    #[case("00ced1", Some(Cyan::DarkTurquoise))]
    #[case("darkturquoise", Some(Cyan::DarkTurquoise))]
    #[case("#40e0d0", Some(Cyan::Turquoise))]
    #[case("40e0d0", Some(Cyan::Turquoise))]
    #[case("turquoise", Some(Cyan::Turquoise))]
    #[case("#48d1cc", Some(Cyan::MediumTurquoise))]
    #[case("48d1cc", Some(Cyan::MediumTurquoise))]
    #[case("mediumturquoise", Some(Cyan::MediumTurquoise))]
    #[case("#afeeee", Some(Cyan::PaleTurquoise))]
    #[case("afeeee", Some(Cyan::PaleTurquoise))]
    #[case("paleturquoise", Some(Cyan::PaleTurquoise))]
    #[case("#7fffd4", Some(Cyan::AquaMarine))]
    #[case("7fffd4", Some(Cyan::AquaMarine))]
    #[case("aquamarine", Some(Cyan::AquaMarine))]
    #[case("#f0fff0", Some(Cyan::Honeydew))]
    #[case("f0fff0", Some(Cyan::Honeydew))]
    #[case("honeydew", Some(Cyan::Honeydew))]
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
    #[case("#800000", Some(Red::Maroon))]
    #[case("800000", Some(Red::Maroon))]
    #[case("maroon", Some(Red::Maroon))]
    #[case("#8b0000", Some(Red::DarkRed))]
    #[case("8b0000", Some(Red::DarkRed))]
    #[case("darkred", Some(Red::DarkRed))]
    #[case("#a52a2a", Some(Red::Brown))]
    #[case("a52a2a", Some(Red::Brown))]
    #[case("brown", Some(Red::Brown))]
    #[case("#b22222", Some(Red::Firebrick))]
    #[case("b22222", Some(Red::Firebrick))]
    #[case("firebrick", Some(Red::Firebrick))]
    #[case("#dc143c", Some(Red::Crimson))]
    #[case("dc143c", Some(Red::Crimson))]
    #[case("crimson", Some(Red::Crimson))]
    #[case("#ff0000", Some(Red::Red))]
    #[case("ff0000", Some(Red::Red))]
    #[case("red", Some(Red::Red))]
    #[case("#ff6347", Some(Red::Tomato))]
    #[case("ff6347", Some(Red::Tomato))]
    #[case("tomato", Some(Red::Tomato))]
    #[case("#ff7f50", Some(Red::Coral))]
    #[case("ff7f50", Some(Red::Coral))]
    #[case("coral", Some(Red::Coral))]
    #[case("#cd5c5c", Some(Red::IndianRed))]
    #[case("cd5c5c", Some(Red::IndianRed))]
    #[case("indianred", Some(Red::IndianRed))]
    #[case("#f08080", Some(Red::LightCoral))]
    #[case("f08080", Some(Red::LightCoral))]
    #[case("lightcoral", Some(Red::LightCoral))]
    #[case("#e9967a", Some(Red::DarkSalmon))]
    #[case("e9967a", Some(Red::DarkSalmon))]
    #[case("darksalmon", Some(Red::DarkSalmon))]
    #[case("#fa8072", Some(Red::Salmon))]
    #[case("fa8072", Some(Red::Salmon))]
    #[case("salmon", Some(Red::Salmon))]
    #[case("#ffa07a", Some(Red::LightSalmon))]
    #[case("ffa07a", Some(Red::LightSalmon))]
    #[case("lightsalmon", Some(Red::LightSalmon))]
    #[case("#ff4500", Some(Red::OrangeRed))]
    #[case("ff4500", Some(Red::OrangeRed))]
    #[case("orangered", Some(Red::OrangeRed))]
    #[case("#ff8c00", Some(Red::DarkOrange))]
    #[case("ff8c00", Some(Red::DarkOrange))]
    #[case("darkorange", Some(Red::DarkOrange))]
    #[case("#ffa500", Some(Red::Orange))]
    #[case("ffa500", Some(Red::Orange))]
    #[case("orange", Some(Red::Orange))]
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
    #[case("#faebd7", Some(White::AntiqueWhite))]
    #[case("faebd7", Some(White::AntiqueWhite))]
    #[case("AntiqueWhite", Some(White::AntiqueWhite))]
    #[case("#f5f5dc", Some(White::Beige))]
    #[case("f5f5dc", Some(White::Beige))]
    #[case("Beige", Some(White::Beige))]
    #[case("#ffe4c4", Some(White::Bisque))]
    #[case("ffe4c4", Some(White::Bisque))]
    #[case("Bisque", Some(White::Bisque))]
    #[case("#ffebcd", Some(White::BlanchedAlmond))]
    #[case("ffebcd", Some(White::BlanchedAlmond))]
    #[case("BlanchedAlmond", Some(White::BlanchedAlmond))]
    #[case("#f5deb3", Some(White::Wheat))]
    #[case("f5deb3", Some(White::Wheat))]
    #[case("wheat", Some(White::Wheat))]
    #[case("#fff8dc", Some(White::CornSilk))]
    #[case("fff8dc", Some(White::CornSilk))]
    #[case("CornSilk", Some(White::CornSilk))]
    #[case("#ffffff", Some(White::White))]
    #[case("ffffff", Some(White::White))]
    #[case("White", Some(White::White))]
    #[case("#ffdead", Some(White::NavajoWhite))]
    #[case("ffdead", Some(White::NavajoWhite))]
    #[case("NavajoWhite", Some(White::NavajoWhite))]
    #[case("#ffe4e1", Some(White::MistyRose))]
    #[case("ffe4e1", Some(White::MistyRose))]
    #[case("MistyRose", Some(White::MistyRose))]
    #[case("#fff0f5", Some(White::LavenderBlush))]
    #[case("fff0f5", Some(White::LavenderBlush))]
    #[case("LavenderBlush", Some(White::LavenderBlush))]
    #[case("#faf0e6", Some(White::Linen))]
    #[case("faf0e6", Some(White::Linen))]
    #[case("Linen", Some(White::Linen))]
    #[case("#fdf5e6", Some(White::OldLace))]
    #[case("fdf5e6", Some(White::OldLace))]
    #[case("OldLace", Some(White::OldLace))]
    #[case("#fff5ee", Some(White::SeaShell))]
    #[case("fff5ee", Some(White::SeaShell))]
    #[case("SeaShell", Some(White::SeaShell))]
    #[case("#f5fffa", Some(White::MintCream))]
    #[case("f5fffa", Some(White::MintCream))]
    #[case("MintCream", Some(White::MintCream))]
    #[case("#fffaf0", Some(White::FloralWhite))]
    #[case("fffaf0", Some(White::FloralWhite))]
    #[case("FloralWhite", Some(White::FloralWhite))]
    #[case("#f8f8ff", Some(White::GhostWhite))]
    #[case("f8f8ff", Some(White::GhostWhite))]
    #[case("GhostWhite", Some(White::GhostWhite))]
    #[case("#fffff0", Some(White::Ivory))]
    #[case("fffff0", Some(White::Ivory))]
    #[case("Ivory", Some(White::Ivory))]
    #[case("#fffafa", Some(White::Snow))]
    #[case("fffafa", Some(White::Snow))]
    #[case("Snow", Some(White::Snow))]
    #[case("#f5f5f5", Some(White::WhiteSmoke))]
    #[case("f5f5f5", Some(White::WhiteSmoke))]
    #[case("WhiteSmoke", Some(White::WhiteSmoke))]
    #[case("#f0f8ff", Some(White::AliceBlue))]
    #[case("f0f8ff", Some(White::AliceBlue))]
    #[case("AliceBlue", Some(White::AliceBlue))]
    #[case("#ffd700", Some(Yellow::Gold))]
    #[case("ffd700", Some(Yellow::Gold))]
    #[case("Gold", Some(Yellow::Gold))]
    #[case("#b8860b", Some(Yellow::DarkGoldenrod))]
    #[case("b8860b", Some(Yellow::DarkGoldenrod))]
    #[case("DarkGoldenRod", Some(Yellow::DarkGoldenrod))]
    #[case("#dab500", Some(Yellow::Goldenrod))]
    #[case("dab500", Some(Yellow::Goldenrod))]
    #[case("GoldenRod", Some(Yellow::Goldenrod))]
    #[case("#eee8aa", Some(Yellow::PaleGoldenrod))]
    #[case("eee8aa", Some(Yellow::PaleGoldenrod))]
    #[case("PaleGoldenRod", Some(Yellow::PaleGoldenrod))]
    #[case("#bdb76b", Some(Yellow::DarkKhaki))]
    #[case("bdb76b", Some(Yellow::DarkKhaki))]
    #[case("DarkKhaki", Some(Yellow::DarkKhaki))]
    #[case("#f0e68c", Some(Yellow::Khaki))]
    #[case("f0e68c", Some(Yellow::Khaki))]
    #[case("Khaki", Some(Yellow::Khaki))]
    #[case("#ffff00", Some(Yellow::Yellow))]
    #[case("ffff00", Some(Yellow::Yellow))]
    #[case("Yellow", Some(Yellow::Yellow))]
    #[case("#9acd32", Some(Yellow::YellowGreen))]
    #[case("9acd32", Some(Yellow::YellowGreen))]
    #[case("YellowGreen", Some(Yellow::YellowGreen))]
    #[case("#ffdab9", Some(Yellow::PeachPuff))]
    #[case("ffdab9", Some(Yellow::PeachPuff))]
    #[case("PeachPuff", Some(Yellow::PeachPuff))]
    #[case("#ffe4b5", Some(Yellow::Moccasin))]
    #[case("ffe4b5", Some(Yellow::Moccasin))]
    #[case("Moccasin", Some(Yellow::Moccasin))]
    #[case("#ffefd5", Some(Yellow::PapayaWhip))]
    #[case("ffefd5", Some(Yellow::PapayaWhip))]
    #[case("PapayaWhip", Some(Yellow::PapayaWhip))]
    #[case("#fffacd", Some(Yellow::LemonChiffon))]
    #[case("fffacd", Some(Yellow::LemonChiffon))]
    #[case("LemonChiffon", Some(Yellow::LemonChiffon))]
    #[case("#fafad2", Some(Yellow::LightGoldenrodYellow))]
    #[case("fafad2", Some(Yellow::LightGoldenrodYellow))]
    #[case("LightGoldenrodYellow", Some(Yellow::LightGoldenrodYellow))]
    #[case("#ffffe0", Some(Yellow::LightYellow))]
    #[case("ffffe0", Some(Yellow::LightYellow))]
    #[case("LightYellow", Some(Yellow::LightYellow))]
    fn test_name_colour<T: ExtendedColour + std::cmp::PartialEq + std::fmt::Debug + FromStr>(
        #[case] input: &str,
        #[case] expected: Option<T>,
    ) {
        assert_eq!(expected, T::name_colour(input))
    }
}
