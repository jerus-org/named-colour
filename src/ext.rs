//! Extended named colours providing shades collected in enums for the main colour
//!

use std::fmt;

/// Shades of red
#[derive(Debug)]
pub enum Red {
    #[allow(missing_docs)]
    Maroon,
    #[allow(missing_docs)]
    DarkRed,
    #[allow(missing_docs)]
    Brown,
    #[allow(missing_docs)]
    Firebrick,
    #[allow(missing_docs)]
    Crimson,
    #[allow(missing_docs)]
    Red,
    #[allow(missing_docs)]
    Tomato,
    #[allow(missing_docs)]
    Coral,
    #[allow(missing_docs)]
    IndianRed,
    #[allow(missing_docs)]
    LightCoral,
    #[allow(missing_docs)]
    DarkSalmon,
    #[allow(missing_docs)]
    Salmon,
    #[allow(missing_docs)]
    LightSalmon,
    #[allow(missing_docs)]
    OrangeRed,
    #[allow(missing_docs)]
    DarkOrange,
    #[allow(missing_docs)]
    Orange,
}

impl fmt::Display for Red {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Red::Maroon => write!(f, "#800000"),
            Red::DarkRed => write!(f, "#8B0000"),
            Red::Brown => write!(f, "#A52A2A"),
            Red::Firebrick => write!(f, "#B22222"),
            Red::Crimson => write!(f, "#DC143C"),
            Red::Red => write!(f, "#FF0000"),
            Red::Tomato => write!(f, "#FF6347"),
            Red::Coral => write!(f, "#FF7F50"),
            Red::IndianRed => write!(f, "#CD5C5C"),
            Red::LightCoral => write!(f, "#F08080"),
            Red::DarkSalmon => write!(f, "#E9967A"),
            Red::Salmon => write!(f, "#FA8072"),
            Red::LightSalmon => write!(f, "#FFA07A"),
            Red::OrangeRed => write!(f, "#FF4500"),
            Red::DarkOrange => write!(f, "#FF8C00"),
            Red::Orange => write!(f, "#FFA500"),
        }
    }
}

impl Red {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Red;
    /// # fn example() {
    /// assert_eq!("(0,255,255)", Red::Firebrick.as_rgb())
    ///
    /// # }
    ///```

    pub fn as_rgb(&self) -> String {
        match self {
            Red::Maroon => crate::to_rgb("#800000"),
            Red::DarkRed => crate::to_rgb("#8B0000"),
            Red::Brown => crate::to_rgb("#A52A2A"),
            Red::Firebrick => crate::to_rgb("#B22222"),
            Red::Crimson => crate::to_rgb("#DC143C"),
            Red::Red => crate::to_rgb("#FF0000"),
            Red::Tomato => crate::to_rgb("#FF6347"),
            Red::Coral => crate::to_rgb("#FF7F50"),
            Red::IndianRed => crate::to_rgb("#CD5C5C"),
            Red::LightCoral => crate::to_rgb("#F08080"),
            Red::DarkSalmon => crate::to_rgb("#E9967A"),
            Red::Salmon => crate::to_rgb("#FA8072"),
            Red::LightSalmon => crate::to_rgb("#FFA07A"),
            Red::OrangeRed => crate::to_rgb("#FF4500"),
            Red::DarkOrange => crate::to_rgb("#FF8C00"),
            Red::Orange => crate::to_rgb("#FFA500"),
        }
    }
}

/// Shades of yellow
#[derive(Debug)]
pub enum Yellow {
    #[allow(missing_docs)]
    Gold,
    #[allow(missing_docs)]
    DarkGoldenRod,
    #[allow(missing_docs)]
    GoldenRod,
    #[allow(missing_docs)]
    PaleGoldenRod,
    #[allow(missing_docs)]
    DarkKhaki,
    #[allow(missing_docs)]
    Khaki,
    #[allow(missing_docs)]
    Olive,
    #[allow(missing_docs)]
    Yellow,
    #[allow(missing_docs)]
    YellowGreen,
}

impl fmt::Display for Yellow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Yellow::Gold => write!(f, "#FFD700"),
            Yellow::DarkGoldenRod => write!(f, "#B8860B"),
            Yellow::GoldenRod => write!(f, "#DAA520"),
            Yellow::PaleGoldenRod => write!(f, "#EEE8AA"),
            Yellow::DarkKhaki => write!(f, "#BDB76B"),
            Yellow::Khaki => write!(f, "#F0E68C"),
            Yellow::Olive => write!(f, "#808000"),
            Yellow::Yellow => write!(f, "#FFFF00"),
            Yellow::YellowGreen => write!(f, "#9ACD32"),
        }
    }
}

impl Yellow {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::ext::Yellow;
    /// # fn example() {
    /// assert_eq!("(178,34,34)", Yellow::Khaki.as_rgb())
    ///
    /// # }
    ///```

    pub fn as_rgb(&self) -> String {
        match self {
            Yellow::Gold => crate::to_rgb("#FFD700"),
            Yellow::DarkGoldenRod => crate::to_rgb("#B8860B"),
            Yellow::GoldenRod => crate::to_rgb("#DAA520"),
            Yellow::PaleGoldenRod => crate::to_rgb("#EEE8AA"),
            Yellow::DarkKhaki => crate::to_rgb("#BDB76B"),
            Yellow::Khaki => crate::to_rgb("#F0E68C"),
            Yellow::Olive => crate::to_rgb("#808000"),
            Yellow::Yellow => crate::to_rgb("#FFFF00"),
            Yellow::YellowGreen => crate::to_rgb("#9ACD32"),
        }
    }
}

/// Shades of green
#[derive(Debug)]
pub enum Green {
    #[allow(missing_docs)]
    YellowGreen,
    #[allow(missing_docs)]
    DarkOliveGreen,
    #[allow(missing_docs)]
    OliveDrab,
    #[allow(missing_docs)]
    LawnGreen,
    #[allow(missing_docs)]
    ChartReuse,
    #[allow(missing_docs)]
    GreenYellow,
    #[allow(missing_docs)]
    DarkGreen,
    #[allow(missing_docs)]
    Green,
    #[allow(missing_docs)]
    ForestGreen,
    #[allow(missing_docs)]
    Lime,
    #[allow(missing_docs)]
    LimeGreen,
    #[allow(missing_docs)]
    LightGreen,
    #[allow(missing_docs)]
    PaleGreen,
    #[allow(missing_docs)]
    DarkSeaGreen,
    #[allow(missing_docs)]
    MediumSpringGreen,
    #[allow(missing_docs)]
    SpringGreen,
    #[allow(missing_docs)]
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
}

/// Shades of cyan
#[derive(Debug)]
pub enum Cyan {
    #[allow(missing_docs)]
    MediumAquaMarine,
    #[allow(missing_docs)]
    MediumSeaGreen,
    #[allow(missing_docs)]
    LightSeaGreen,
    #[allow(missing_docs)]
    DarkSlateGray,
    #[allow(missing_docs)]
    Teal,
    #[allow(missing_docs)]
    DarkCyan,
    #[allow(missing_docs)]
    Aqua,
    #[allow(missing_docs)]
    Cyan,
    #[allow(missing_docs)]
    LightCyan,
    #[allow(missing_docs)]
    DarkTurquoise,
    #[allow(missing_docs)]
    Turquoise,
    #[allow(missing_docs)]
    MediumTurquoise,
    #[allow(missing_docs)]
    PaleTurquoise,
    #[allow(missing_docs)]
    AquaMarine,
    #[allow(missing_docs)]
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
}

/// Shades of blue
#[derive(Debug)]
pub enum Blue {
    #[allow(missing_docs)]
    PowderBlue,
    #[allow(missing_docs)]
    CadetBlue,
    #[allow(missing_docs)]
    SteelBlue,
    #[allow(missing_docs)]
    CornFlowerBlue,
    #[allow(missing_docs)]
    DeepSkyBlue,
    #[allow(missing_docs)]
    DodgerBlue,
    #[allow(missing_docs)]
    LightBlue,
    #[allow(missing_docs)]
    SkyBlue,
    #[allow(missing_docs)]
    LightSkyBlue,
    #[allow(missing_docs)]
    MidnightBlue,
    #[allow(missing_docs)]
    Navy,
    #[allow(missing_docs)]
    DarkBlue,
    #[allow(missing_docs)]
    MediumBlue,
    #[allow(missing_docs)]
    Blue,
    #[allow(missing_docs)]
    RoyalBlue,
    #[allow(missing_docs)]
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
}

/// Shades of indigo
#[derive(Debug)]
pub enum Indigo {
    #[allow(missing_docs)]
    BlueViolet,
    #[allow(missing_docs)]
    Indigo,
    #[allow(missing_docs)]
    DarkSlateBlue,
    #[allow(missing_docs)]
    SlateBlue,
    #[allow(missing_docs)]
    MediumSlateBlue,
    #[allow(missing_docs)]
    MediumPurple,
    #[allow(missing_docs)]
    DarkMagenta,
    #[allow(missing_docs)]
    DarkViolet,
    #[allow(missing_docs)]
    DarkOrchid,
    #[allow(missing_docs)]
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
}

/// Shades of purple
#[derive(Debug)]
pub enum Purple {
    #[allow(missing_docs)]
    Purple,
    #[allow(missing_docs)]
    Thistle,
    #[allow(missing_docs)]
    Plum,
    #[allow(missing_docs)]
    Violet,
    #[allow(missing_docs)]
    Magenta,
    #[allow(missing_docs)]
    Fuchsia,
    #[allow(missing_docs)]
    Orchid,
    #[allow(missing_docs)]
    MediumVioletRed,
    #[allow(missing_docs)]
    PaleVioletRed,
    #[allow(missing_docs)]
    DeepPink,
    #[allow(missing_docs)]
    HotPink,
    #[allow(missing_docs)]
    LightPink,
    #[allow(missing_docs)]
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
}

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
    fn display_blue_as_rgb() {
        assert_eq!("(70,130,180)", Blue::SteelBlue.as_rgb())
    }

    #[test]
    fn display_brown_as_rgb() {
        assert_eq!("(244,164,96)", Brown::SandyBrown.as_rgb())
    }
    #[test]
    fn display_cyan_as_rgb() {
        assert_eq!("(0,128,128)", Cyan::Teal.as_rgb())
    }

    #[test]
    fn display_green_as_rgb() {
        assert_eq!("(107,142,35)", Green::OliveDrab.as_rgb())
    }

    #[test]
    fn display_indigo_as_rgb() {
        assert_eq!("(75,0,130)", Indigo::Indigo.as_rgb())
    }

    #[test]
    fn display_lavender_as_rgb() {
        assert_eq!("(230,230,250)", Lavender::Lavender.as_rgb())
    }

    #[test]
    fn display_purple_as_rgb() {
        assert_eq!("(128,0,128)", Purple::Purple.as_rgb())
    }

    #[test]
    fn display_red_as_rgb() {
        assert_eq!("(178,34,34)", Red::Firebrick.as_rgb())
    }

    #[test]
    fn display_white_as_rgb() {
        assert_eq!("(250,240,230)", White::Linen.as_rgb())
    }

    #[test]
    fn display_yellow_as_rgb() {
        assert_eq!("(240,230,140)", Yellow::Khaki.as_rgb())
    }
}
