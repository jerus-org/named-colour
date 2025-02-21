# Rust library named-colour

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![CircleCI][circle-badge]][circle-url]
[![Rust][version-badge]][version-url]
[![Docs][docs-badge]][docs-url]
[![BuyMeaCoffee][bmac-badge]][bmac-url]
[![GitHubSponsors][ghub-badge]][ghub-url]

[crates-badge]: https://img.shields.io/crates/v/named-colour.svg
[crates-url]: https://crates.io/crates/named-colour
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/jerusdp/named-colour/blob/main/LICENSE-MIT
[circle-badge]: https://dl.circleci.com/status-badge/img/gh/jerus-org/named-colour/tree/main.svg?style=svg
[circle-url]: https://dl.circleci.com/status-badge/redirect/gh/jerus-org/named-colour/tree/main
[version-badge]: https://img.shields.io/badge/rust-1.70+-orange.svg
[version-url]: https://www.rust-lang.org
[docs-badge]:  https://docs.rs/named-colour/badge.svg
[docs-url]:  https://docs.rs/named-colour
[bmac-badge]: https://badgen.net/badge/icon/buymeacoffee?color=yellow&icon=buymeacoffee&label
[bmac-url]: https://buymeacoffee.com/jerusdp
[ghub-badge]: https://img.shields.io/badge/sponsor-30363D?logo=GitHub-Sponsors&logoColor=#white
[ghub-url]: https://github.com/sponsors/jerusdp

The rust library named-colour provides a convenient set of named colours and the Hex Code for each colour.

## Installation

To use named-colour in your project you can add the following to your `Cargo.toml`:

```toml
[dependencies]
named-colour = "0.3.16"
```

## Usage

Provides Hex Codes for colours:

- enums that return a hex code string for named colours
- rgb colour struct to configure a colour an rgb colour and display as decimal or hex

### Examples

#### Use Basic colour

```rust
use named_colour::Basic;
println!("The colour Hex Code is: {} for the RGB colour Aqua: {}",
    Basic::Aqua,
    Basic::Aqua.as_rgb()
);
```

#### Use Extended colour

Enable the feature in the toml file:

```toml
[dependencies]
named-colour = { version = "0.3.16", features = ["extended"]}
```

```rust
use named_colour::ext::Indigo;
println!("The colour Hex Code is: {} for the RGB colour Dark Orchid: {}",
    Indigo::DarkOrchid,
    Indigo::DarkOrchid.as_rgb()
);
```

#### Create a custom colour

```rust
use named_colour::ColourRgb;
let my_colour =ColourRgb::new(12,24,48);
println!("The Hex Code is: {} for my_colour: {}",
    my_colour.as_hex(),
    my_colour.to_string()
);
```

## Features

- Basic contains just 16 colours with 18 names (default)
- Extended contains a fuller set of colours divided in 11 collections

To use the extended colour set only configure toml with no-default features

```toml
[dependencies]
named-colour = { version = "0.3.16", default_features = false, features = ["extended"]}
```

## License

Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
