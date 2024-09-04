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
//! named-colour = "0.2.0"
//!
//!```
//!
//!```
//! use named_colour::Basic;
//! println!("The colour Hex Code is: {} for the RGB colour Aqua: {}",
//!     Basic::Aqua,
//!     Basic::Aqua.to_rgb()
//! );
//!
//! assert_eq!("rgb(0,255,255)", Basic::Aqua.to_rgb().to_string());
//!```
#[cfg_attr(
    feature = "extended",
    doc = r##"

 ### Use Extended colour

 Enable the feature in the toml file:

```toml
[dependencies]
named-colour = { version = "0.2.0", features = ["extended"]}
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
///named-colour = { version = "0.2.0", default_features = false, features = ["extended"]}
///```
///

#[cfg(feature = "basic")]
mod basic;
#[cfg(feature = "extended")]
#[allow(missing_docs)]
pub mod ext;
mod to_hex;

pub use crate::to_hex::ToHex;
pub use basic::Basic;
pub use rgb::RGB8;

#[allow(missing_docs)]
pub enum Prefix {
    None,
    Hash,
}
