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

pub use black::Black;
pub use blue::Blue;
pub use brown::Brown;
pub use cyan::Cyan;
pub use green::Green;
pub use purple::Purple;
pub use red::Red;
pub use white::White;
pub use yellow::Yellow;

pub trait NamedColour {
    fn name_colour(colour: &str) -> Option<Self>
    where
        Self: Sized;
}

pub fn name_colour<T: NamedColour>(colour: &str) -> Option<T> {
    T::name_colour(colour)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
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
    fn test_name_colour<T: NamedColour + std::cmp::PartialEq + std::fmt::Debug>(
        #[case] input: &str,
        #[case] expected: Option<T>,
    ) {
        assert_eq!(expected, T::name_colour(input))
    }
}
