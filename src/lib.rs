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
//! named-colour = "0.3.11"
//!
//!```
//!
#[cfg_attr(
    not(feature = "extended"),
    doc = r##"
```
    use named_colour::Basic;
    println!("The colour Hex Code is: {} for the RGB colour Aqua: {}",
        Basic::Aqua,
        Basic::Aqua.to_rgb()
    );
    assert_eq!("rgb(0,255,255)", Basic::Aqua.to_rgb().to_string());
```
"##
)]
#[cfg_attr(
    feature = "extended",
    doc = r##"

 ### Use Extended colour

 Enable the feature in the toml file:

```toml
[dependencies]
named-colour = { version = "0.3.11", features = ["extended"]}
```

```
    use named_colour::ext::Purple;
    println!("The colour Hex Code is: {} for the RGB colour Dark Orchid: {}",
        Purple::DarkOrchid,
        Purple::DarkOrchid.to_rgb()
 );
```
"##
)]
///
/// ### Create a custom colour
///
///```
/// use named_colour::ToHex;
/// use rgb::Rgb;
/// fn main() {
///     assert_eq!("#CCCCCC", &Rgb::new(204, 204, 204).as_hex());
///     let colour = Rgb::new(12, 24, 48);
///     assert_eq!("#0C1830", colour.as_hex());
///     let colour = Rgb::new(12, 4, 8);
///     assert_eq!("#0C0408", colour.as_hex());
/// }
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
///named-colour = { version = "0.3.11", default_features = false, features = ["extended"]}
///```
///
///
///
mod to_hex;

#[cfg(not(feature = "extended"))]
mod basic;
#[cfg(feature = "extended")]
pub mod ext;

pub use crate::to_hex::ToHex;
pub use rgb::RGB8;

#[cfg(not(feature = "extended"))]
pub use basic::Basic;

#[cfg(feature = "extended")]
pub use ext::name_colour;
#[cfg(feature = "extended")]
pub use ext::random_named_colour;
#[cfg(feature = "extended")]
pub use ext::Black;
#[cfg(feature = "extended")]
pub use ext::Blue;
#[cfg(feature = "extended")]
pub use ext::Brown;
#[cfg(feature = "extended")]
pub use ext::Cyan;
#[cfg(feature = "extended")]
pub use ext::ExtendedColour;
#[cfg(feature = "extended")]
pub use ext::Green;
#[cfg(feature = "extended")]
pub use ext::Purple;
#[cfg(feature = "extended")]
pub use ext::Red;
#[cfg(feature = "extended")]
pub use ext::White;
#[cfg(feature = "extended")]
pub use ext::Yellow;

/// Prefixes
///
/// Prefixes allowed to 6 character hex code to specify colour
pub enum Prefix {
    /// No prefix yields 6 character string
    None,
    /// Prefix # yields 7 character string
    Hash,
}
