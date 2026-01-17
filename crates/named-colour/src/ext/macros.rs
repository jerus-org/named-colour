//! Macros for reducing code duplication in colour modules

/// Implements common methods for colour enums
///
/// This macro generates:
/// - `to_rgb()` - converts the colour's hex representation to an RGB tuple
/// - `to_hex_triplet()` - formats the colour as a hex string with optional prefix
/// - `FromStr` trait implementation
///
/// The colour enum must implement `Display` (returning hex like "#RRGGBB") and have a `parse()` method.
macro_rules! impl_colour_methods {
    ($colour_type:ty) => {
        impl $colour_type {
            /// Display the colour as an RGB tuple
            pub fn to_rgb(&self) -> Rgb<u8> {
                let colour = self.to_string();

                let r: u8 = u8::from_str_radix(&colour[1..3], 16).unwrap();
                let g: u8 = u8::from_str_radix(&colour[3..5], 16).unwrap();
                let b: u8 = u8::from_str_radix(&colour[5..7], 16).unwrap();

                Rgb::new(r, g, b)
            }

            /// Display the colour as a hex triplet with optional prefix
            pub fn to_hex_triplet(&self, prefix: crate::Prefix) -> String {
                let rgb = self.to_rgb();

                let prefix = match prefix {
                    crate::Prefix::Hash => "#",
                    crate::Prefix::None => "",
                };

                format!("{}{:02X}{:02X}{:02X}", prefix, rgb.r, rgb.g, rgb.b)
            }
        }

        impl std::str::FromStr for $colour_type {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match Self::parse(s) {
                    Some(colour) => Ok(colour),
                    None => Err(format!("Invalid Colour: {s}")),
                }
            }
        }
    };
}
