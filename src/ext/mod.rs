//! Extended named colours providing shades collected in enums for the main colour
//!

use std::fmt;

mod blue;
mod brown;
mod cyan;
mod green;
mod indigo;
mod purple;
mod red;
mod white;
mod yellow;

pub use blue::Blue;
pub use brown::Brown;
pub use cyan::Cyan;
pub use green::Green;
pub use indigo::Indigo;
pub use purple::Purple;
pub use red::Red;
pub use white::White;
pub use yellow::Yellow;

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
    #[allow(deprecated)]
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
    #[allow(deprecated)]
    fn display_white_as_rgb() {
        assert_eq!("(250,240,230)", White::Linen.as_rgb())
    }

    #[test]
    #[allow(deprecated)]
    fn display_yellow_as_rgb() {
        assert_eq!("(240,230,140)", Yellow::Khaki.as_rgb())
    }
}
