use std::fmt;

/// 16 basic colours with 18 names!
#[derive(Debug)]
pub enum Basic {
    #[allow(missing_docs)]
    Black,
    #[allow(missing_docs)]
    White,
    #[allow(missing_docs)]
    Red,
    #[allow(missing_docs)]
    Lime,
    #[allow(missing_docs)]
    Blue,
    #[allow(missing_docs)]
    Yellow,
    /// Alternate name for Aqua
    Cyan,
    /// Alternate name for Cyan
    Aqua,
    /// Alternate name for Fuchsia
    Magenta,
    /// Alternate name for Magenta
    Fuchsia,
    #[allow(missing_docs)]
    Silver,
    #[allow(missing_docs)]
    Gray,
    #[allow(missing_docs)]
    Maroon,
    #[allow(missing_docs)]
    Olive,
    #[allow(missing_docs)]
    Green,
    #[allow(missing_docs)]
    Purple,
    #[allow(missing_docs)]
    Teal,
    #[allow(missing_docs)]
    Navy,
}

impl fmt::Display for Basic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Basic::Black => write!(f, "#000000"),
            Basic::White => write!(f, "#FFFFFF"),
            Basic::Red => write!(f, "#FF0000"),
            Basic::Lime => write!(f, "#00FF00"),
            Basic::Blue => write!(f, "#0000FF"),
            Basic::Yellow => write!(f, "#FFFF00"),
            Basic::Cyan => write!(f, "#00FFFF"),
            Basic::Aqua => write!(f, "#00FFFF"),
            Basic::Magenta => write!(f, "#FF00FF"),
            Basic::Fuchsia => write!(f, "#FF00FF"),
            Basic::Silver => write!(f, "#C0C0C0"),
            Basic::Gray => write!(f, "#808080"),
            Basic::Maroon => write!(f, "#800000"),
            Basic::Olive => write!(f, "#808000"),
            Basic::Green => write!(f, "#008000"),
            Basic::Purple => write!(f, "#800080"),
            Basic::Teal => write!(f, "#008080"),
            Basic::Navy => write!(f, "#000080"),
        }
    }
}

impl Basic {
    /// Display the hex code string as a decimal RGB Tuple
    ///
    /// ## Example
    ///
    ///```
    /// # use named_colour::Basic;
    /// # fn example() {
    /// assert_eq!("(0,255,255)", Basic::Aqua.as_rgb())
    ///
    /// # }
    ///```

    pub fn as_rgb(&self) -> String {
        match self {
            Basic::Black => crate::to_rgb("#000000"),
            Basic::White => crate::to_rgb("#FFFFFF"),
            Basic::Red => crate::to_rgb("#FF0000"),
            Basic::Lime => crate::to_rgb("#00FF00"),
            Basic::Blue => crate::to_rgb("#0000FF"),
            Basic::Yellow => crate::to_rgb("#FFFF00"),
            Basic::Cyan => crate::to_rgb("#00FFFF"),
            Basic::Aqua => crate::to_rgb("#00FFFF"),
            Basic::Magenta => crate::to_rgb("#FF00FF"),
            Basic::Fuchsia => crate::to_rgb("#FF00FF"),
            Basic::Silver => crate::to_rgb("#C0C0C0"),
            Basic::Gray => crate::to_rgb("#808080"),
            Basic::Maroon => crate::to_rgb("#800000"),
            Basic::Olive => crate::to_rgb("#808000"),
            Basic::Green => crate::to_rgb("#008000"),
            Basic::Purple => crate::to_rgb("#800080"),
            Basic::Teal => crate::to_rgb("#008080"),
            Basic::Navy => crate::to_rgb("#000080"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_as_rgb() {
        assert_eq!("(0,255,255)", Basic::Aqua.as_rgb())
    }
}
