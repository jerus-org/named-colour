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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
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
}
