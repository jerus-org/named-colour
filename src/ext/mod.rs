//! Extended named colours providing shades collected in enums for the main colour
//!

use std::fmt;

mod blue;
mod cyan;
mod green;
mod indigo;
mod purple;
mod red;
mod yellow;

pub use blue::Blue;
pub use cyan::Cyan;
pub use green::Green;
pub use indigo::Indigo;
pub use purple::Purple;
pub use red::Red;
pub use yellow::Yellow;

/// Shades of white
#[derive(Debug)]
pub enum White {
    #[allow(missing_docs)]
    AntiqueWhite,
    #[allow(missing_docs)]
    Beige,
    #[allow(missing_docs)]
    Bisque,
    #[allow(missing_docs)]
    BlanchedAlmond,
    #[allow(missing_docs)]
    Wheat,
    #[allow(missing_docs)]
    CornSilk,
    #[allow(missing_docs)]
    LemonChiffon,
    #[allow(missing_docs)]
    LightGoldenRodYellow,
    #[allow(missing_docs)]
    LightYellow,
    #[allow(missing_docs)]
    White,
    #[allow(missing_docs)]
    Moccasin,
    #[allow(missing_docs)]
    NavajoWhite,
    #[allow(missing_docs)]
    PeachPuff,
    #[allow(missing_docs)]
    MistyRose,
    #[allow(missing_docs)]
    LavenderBlush,
    #[allow(missing_docs)]
    Linen,
    #[allow(missing_docs)]
    OldLace,
    #[allow(missing_docs)]
    PapayaWhip,
    #[allow(missing_docs)]
    SeaShell,
    #[allow(missing_docs)]
    MintCream,
    #[allow(missing_docs)]
    FloralWhite,
    #[allow(missing_docs)]
    GhostWhite,
    #[allow(missing_docs)]
    Ivory,
    #[allow(missing_docs)]
    Snow,
    #[allow(missing_docs)]
    WhiteSmoke,
}

impl fmt::Display for White {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            White::AntiqueWhite => write!(f, "#FAEBD7"),
            White::Beige => write!(f, "#F5F5DC"),
            White::Bisque => write!(f, "#FFE4C4"),
            White::BlanchedAlmond => write!(f, "#FFEBCD"),
            White::Wheat => write!(f, "#F5DEB3"),
            White::CornSilk => write!(f, "#FFF8DC"),
            White::LemonChiffon => write!(f, "#FFFACD"),
            White::LightGoldenRodYellow => write!(f, "#FAFAD2"),
            White::LightYellow => write!(f, "#FFFFE0"),
            White::White => write!(f, "#FFFFFF"),
            White::Moccasin => write!(f, "#FFE4B5"),
            White::NavajoWhite => write!(f, "#FFDEAD"),
            White::PeachPuff => write!(f, "#FFDAB9"),
            White::MistyRose => write!(f, "#FFE4E1"),
            White::LavenderBlush => write!(f, "#FFF0F5"),
            White::Linen => write!(f, "#FAF0E6"),
            White::OldLace => write!(f, "#FDF5E6"),
            White::PapayaWhip => write!(f, "#FFEFD5"),
            White::SeaShell => write!(f, "#FFF5EE"),
            White::MintCream => write!(f, "#F5FFFA"),
            White::FloralWhite => write!(f, "#FFFAF0"),
            White::GhostWhite => write!(f, "#F8F8FF"),
            White::Ivory => write!(f, "#FFFFF0"),
            White::Snow => write!(f, "#FFFAFA"),
            White::WhiteSmoke => write!(f, "#F5F5F5"),
        }
    }
}

impl White {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::White;
    /// # fn example() {
    /// assert_eq!("(250,240,230)", White::Linen.as_rgb())
    ///
    /// # }
    ///```

    pub fn as_rgb(&self) -> String {
        match self {
            White::AntiqueWhite => crate::to_rgb("#FAEBD7"),
            White::Beige => crate::to_rgb("#F5F5DC"),
            White::Bisque => crate::to_rgb("#FFE4C4"),
            White::BlanchedAlmond => crate::to_rgb("#FFEBCD"),
            White::Wheat => crate::to_rgb("#F5DEB3"),
            White::CornSilk => crate::to_rgb("#FFF8DC"),
            White::LemonChiffon => crate::to_rgb("#FFFACD"),
            White::LightGoldenRodYellow => crate::to_rgb("#FAFAD2"),
            White::LightYellow => crate::to_rgb("#FFFFE0"),
            White::White => crate::to_rgb("#FFFFFF"),
            White::Moccasin => crate::to_rgb("#FFE4B5"),
            White::NavajoWhite => crate::to_rgb("#FFDEAD"),
            White::PeachPuff => crate::to_rgb("#FFDAB9"),
            White::MistyRose => crate::to_rgb("#FFE4E1"),
            White::LavenderBlush => crate::to_rgb("#FFF0F5"),
            White::Linen => crate::to_rgb("#FAF0E6"),
            White::OldLace => crate::to_rgb("#FDF5E6"),
            White::PapayaWhip => crate::to_rgb("#FFEFD5"),
            White::SeaShell => crate::to_rgb("#FFF5EE"),
            White::MintCream => crate::to_rgb("#F5FFFA"),
            White::FloralWhite => crate::to_rgb("#FFFAF0"),
            White::GhostWhite => crate::to_rgb("#F8F8FF"),
            White::Ivory => crate::to_rgb("#FFFFF0"),
            White::Snow => crate::to_rgb("#FFFAFA"),
            White::WhiteSmoke => crate::to_rgb("#F5F5F5"),
        }
    }
}

/// Shades of brown
#[derive(Debug)]
pub enum Brown {
    #[allow(missing_docs)]
    SaddleBrown,
    #[allow(missing_docs)]
    Sienna,
    #[allow(missing_docs)]
    Chocolate,
    #[allow(missing_docs)]
    Peru,
    #[allow(missing_docs)]
    SandyBrown,
    #[allow(missing_docs)]
    BurlyWood,
    #[allow(missing_docs)]
    Tan,
    #[allow(missing_docs)]
    RosyBrown,
}

impl fmt::Display for Brown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Brown::SaddleBrown => write!(f, "#8B4513"),
            Brown::Sienna => write!(f, "#A0522D"),
            Brown::Chocolate => write!(f, "#D2691E"),
            Brown::Peru => write!(f, "#CD853F"),
            Brown::SandyBrown => write!(f, "#F4A460"),
            Brown::BurlyWood => write!(f, "#DEB887"),
            Brown::Tan => write!(f, "#D2B48C"),
            Brown::RosyBrown => write!(f, "#BC8F8F"),
        }
    }
}

impl Brown {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Brown;
    /// # fn example() {
    /// assert_eq!("(244,164,96)", Brown::SandyBrown.as_rgb())
    ///
    /// # }
    ///```

    pub fn as_rgb(&self) -> String {
        match self {
            Brown::SaddleBrown => crate::to_rgb("#8B4513"),
            Brown::Sienna => crate::to_rgb("#A0522D"),
            Brown::Chocolate => crate::to_rgb("#D2691E"),
            Brown::Peru => crate::to_rgb("#CD853F"),
            Brown::SandyBrown => crate::to_rgb("#F4A460"),
            Brown::BurlyWood => crate::to_rgb("#DEB887"),
            Brown::Tan => crate::to_rgb("#D2B48C"),
            Brown::RosyBrown => crate::to_rgb("#BC8F8F"),
        }
    }
}

/// Shades of lavender
#[derive(Debug)]
pub enum Lavender {
    #[allow(missing_docs)]
    SlateGray,
    #[allow(missing_docs)]
    LightSlateGray,
    #[allow(missing_docs)]
    LightSteelBlue,
    #[allow(missing_docs)]
    Lavender,
    #[allow(missing_docs)]
    AliceBlue,
}

impl fmt::Display for Lavender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lavender::SlateGray => write!(f, "#708090"),
            Lavender::LightSlateGray => write!(f, "#778899"),
            Lavender::LightSteelBlue => write!(f, "#B0C4DE"),
            Lavender::Lavender => write!(f, "#E6E6FA"),
            Lavender::AliceBlue => write!(f, "#F0F8FF"),
        }
    }
}

impl Lavender {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Lavender;
    /// # fn example() {
    /// assert_eq!("(230,230,250)", Lavender::Lavender.as_rgb())
    ///
    /// # }
    ///```

    pub fn as_rgb(&self) -> String {
        match self {
            Lavender::SlateGray => crate::to_rgb("#708090"),
            Lavender::LightSlateGray => crate::to_rgb("#778899"),
            Lavender::LightSteelBlue => crate::to_rgb("#B0C4DE"),
            Lavender::Lavender => crate::to_rgb("#E6E6FA"),
            Lavender::AliceBlue => crate::to_rgb("#F0F8FF"),
        }
    }
}

/// Shades of black
#[derive(Debug)]
pub enum Black {
    #[allow(missing_docs)]
    SlateGray,
    #[allow(missing_docs)]
    LightSlateGray,
    #[allow(missing_docs)]
    Black,
    #[allow(missing_docs)]
    DimGray,
    #[allow(missing_docs)]
    DimGrey,
    #[allow(missing_docs)]
    Gray,
    #[allow(missing_docs)]
    Grey,
    #[allow(missing_docs)]
    DarkGray,
    #[allow(missing_docs)]
    DarkGrey,
    #[allow(missing_docs)]
    Silver,
    #[allow(missing_docs)]
    LightGray,
    #[allow(missing_docs)]
    LightGrey,
    #[allow(missing_docs)]
    Gainsboro,
}

impl fmt::Display for Black {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Black::SlateGray => write!(f, "#708090"),
            Black::LightSlateGray => write!(f, "#778899"),
            Black::Black => write!(f, "#000000"),
            Black::DimGray => write!(f, "#696969"),
            Black::DimGrey => write!(f, "#696969"),
            Black::Gray => write!(f, "#808080"),
            Black::Grey => write!(f, "#808080"),
            Black::DarkGray => write!(f, "#A9A9A9"),
            Black::DarkGrey => write!(f, "#A9A9A9"),
            Black::Silver => write!(f, "#C0C0C0"),
            Black::LightGray => write!(f, "#D3D3D3"),
            Black::LightGrey => write!(f, "#D3D3D3"),
            Black::Gainsboro => write!(f, "#DCDCDC"),
        }
    }
}

impl Black {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Black;
    /// # fn example() {
    ///
    /// assert_eq!("(112,128,144)", Black::SlateGray.as_rgb())
    ///
    /// # }
    ///```

    pub fn as_rgb(&self) -> String {
        match self {
            Black::SlateGray => crate::to_rgb("#708090"),
            Black::LightSlateGray => crate::to_rgb("#778899"),
            Black::Black => crate::to_rgb("#000000"),
            Black::DimGray => crate::to_rgb("#696969"),
            Black::DimGrey => crate::to_rgb("#696969"),
            Black::Gray => crate::to_rgb("#808080"),
            Black::Grey => crate::to_rgb("#808080"),
            Black::DarkGray => crate::to_rgb("#A9A9A9"),
            Black::DarkGrey => crate::to_rgb("#A9A9A9"),
            Black::Silver => crate::to_rgb("#C0C0C0"),
            Black::LightGray => crate::to_rgb("#D3D3D3"),
            Black::LightGrey => crate::to_rgb("#D3D3D3"),
            Black::Gainsboro => crate::to_rgb("#DCDCDC"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_black_as_rgb() {
        assert_eq!("(112,128,144)", Black::SlateGray.as_rgb())
    }

    #[test]
    #[allow(deprecated)]
    fn display_blue_as_rgb() {
        assert_eq!("(70,130,180)", Blue::SteelBlue.as_rgb())
    }

    #[test]
    fn display_brown_as_rgb() {
        assert_eq!("(244,164,96)", Brown::SandyBrown.as_rgb())
    }
    #[test]
    #[allow(deprecated)]
    fn display_cyan_as_rgb() {
        assert_eq!("(0,128,128)", Cyan::Teal.as_rgb())
    }

    #[test]
    #[allow(deprecated)]
    fn display_green_as_rgb() {
        assert_eq!("(107,142,35)", Green::OliveDrab.as_rgb())
    }

    #[test]
    #[allow(deprecated)]
    fn display_indigo_as_rgb() {
        assert_eq!("(75,0,130)", Indigo::Indigo.as_rgb())
    }

    #[test]
    fn display_lavender_as_rgb() {
        assert_eq!("(230,230,250)", Lavender::Lavender.as_rgb())
    }

    #[test]
    #[allow(deprecated)]
    fn display_purple_as_rgb() {
        assert_eq!("(128,0,128)", Purple::Purple.as_rgb())
    }

    #[test]
    #[allow(deprecated)]
    fn display_red_as_rgb() {
        assert_eq!("(178,34,34)", Red::Firebrick.as_rgb())
    }

    #[test]
    fn display_white_as_rgb() {
        assert_eq!("(250,240,230)", White::Linen.as_rgb())
    }

    #[test]
    #[allow(deprecated)]
    fn display_yellow_as_rgb() {
        assert_eq!("(240,230,140)", Yellow::Khaki.as_rgb())
    }
}
