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

pub trait ExtendedColour {
    fn name_colour(colour: &str) -> Option<Self>
    where
        Self: Sized;
}

pub fn name_colour<T: ExtendedColour>(colour: &str) -> Option<T> {
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
    fn test_name_colour<T: ExtendedColour + std::cmp::PartialEq + std::fmt::Debug>(
        #[case] input: &str,
        #[case] expected: Option<T>,
    ) {
        assert_eq!(expected, T::name_colour(input))
    }
}
