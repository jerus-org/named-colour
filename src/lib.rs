#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
// #![cfg_attr(docsrs, warn(rustdoc::missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::invalid_codeblock_attributes))]
#![allow(dead_code)]

//! Provides Hex Codes for colours:
//! - enums that return a hex code string for named colours
//! - rgb colour struct to configure a colour an rgb colour and display as decimal or hex
//!
//! ## Examples
//!
//! ### Use Basic colour
//!
//!```toml
//! [dependencies]
//! named-colour = "0.1.6"
//!
//!```
//!
//!```
//! use named_colour::Basic;
//! println!("The colour Hex Code is: {} for the RGB colour Aqua: {}",
//!     Basic::Aqua,
//!     Basic::Aqua.as_rgb()
//! );
//!```
#[cfg_attr(
    feature = "extended",
    doc = r##"

 ### Use Extended colour

 Enable the feature in the toml file:

```toml
[dependencies]
named-colour = { version = "0.1.6", features = ["extended"]}
```

```
    use named_colour::ext::Indigo;
    println!("The colour Hex Code is: {} for the RGB colour Dark Orchid: {}",
        Indigo::DarkOrchid,
        Indigo::DarkOrchid.as_rgb()
 );
```
"##
)]
///
/// ### Create a custom colour
///
///```
/// use named_colour::ColourRgb;
/// let my_colour =ColourRgb::new(12,24,48);
/// println!("The Hex Code is: {} for my_colour: {}",
///     my_colour.as_hex(),
///     my_colour.to_string()
/// );
///```
///
/// ## Features
/// - Basic contains just 16 colours with 18 names (default)
/// - Extended contains a fuller set of colours divided in 11 collections
///
/// To use the extended colour set only configure toml with no-default features
///
///```toml
///[dependencies]
///named-colour = { version = "0.1.6", default_features = false, features = ["extended"]}
///```
///

#[cfg(feature = "basic")]
mod basic;
#[cfg(feature = "extended")]
#[allow(missing_docs)]
pub mod ext;
mod rgb;

pub use crate::rgb::ColourRgb;
pub use basic::Basic;

pub(crate) fn to_rgb(hex: &str) -> String {
    let mut no_error = true;
    let starts_with_hash = &hex[0..1] == "#";
    let red = u8::from_str_radix(&hex[1..3], 16).unwrap_or_else(|_| {
        no_error = false;
        0
    });
    let green = u8::from_str_radix(&hex[3..5], 16).unwrap_or_else(|_| {
        no_error = false;
        0
    });
    let blue = u8::from_str_radix(&hex[5..7], 16).unwrap_or_else(|_| {
        no_error = false;
        0
    });
    if starts_with_hash && no_error {
        format!("({},{},{})", red, green, blue,)
    } else {
        "Invalid Hex Code".to_string()
    }
}

#[allow(missing_docs)]
pub enum Prefix {
    None,
    Hash,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_rgb() {
        assert_eq!("(192,192,192)", to_rgb(&Basic::Silver.to_string()))
    }

    #[test]
    fn returns_invalid_string_message_on_bad_input() {
        assert_eq!("Invalid Hex Code", to_rgb("#QWERTY"))
    }
}
