# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- add parsing functionality for Basic colours(pr [#51])
- support parsing color codes without hash prefix in Basic module(pr [#52])
- add parsing for black shades and implement FromStr trait(pr [#53])
- add parse method and FromStr implementation for Black and Blue enums(pr [#54])
- add parsing from string for Brown enum(pr [#55])
- add parsing from string for Cyan enum and implement FromStr trait(pr [#56])
- BREAKING: add Olive shade and implement FromStr for Green enum(pr [#57])
- add parsing from string for Purple enum and implement FromStr trait(pr [#58])
- add parsing and string conversion for Red enum(pr [#59])
- add parsing and string conversion for White enum(pr [#60])
- add new shades and implement parsing from string(pr [#61])
- add NamedColour trait and implement for Black(pr [#63])
- implement ExtendedColour trait for Brown and add tests for name_colour function(pr [#65])
- implement ExtendedColour trait for Cyan(pr [#66])
- implement ExtendedColour trait and add comprehensive tests for Green(pr [#67])
- implement ExtendedColour trait and add comprehensive tests for Red(pr [#68])
- implement ExtendedColour trait and add comprehensive tests for name_colour function(pr [#69])
- add ExtendedColour trait implementation and additional tests for White(pr [#70])
- implement ExtendedColour trait and add comprehensive tests for Yellow shades(pr [#71])
- BREAKING: update default feature to extended and adjust documentation accordingly(pr [#72])

### Changed

- refactor-rename Rgb to RGB8 in lib.rs(pr [#50])
- refactor-remove deprecated as_rgb methods and associated tests(pr [#62])
- refactor-rename NamedColour to ExtendedColour and update trait implementations(pr [#64])

## [0.2.0] - 2024-09-03

### Added

- add support for RGB and Hex triplet conversion(pr [#34])
- add `to_rgb` and `to_hex_triplet` for yellow(pr [#38])
- add green color shades and refactor module structure(pr [#39])
- add Cyan color shades and refactor Green color shades(pr [#40])
- add extended blue shades with RGB and hex triplet support(pr [#41])
- add Indigo enum with shades and related methods(pr [#42])
- implement `to_rgb` and `to_hex_triplet` for purple (pr [#43])
- implement `to_rgb` and `to_hex_triplet` for white enum(pr [#44])
- add Brown color shades and refactor existing implementation(pr [#45])
- add lavender module with extended named colours(pr [#46])
- add extended shades of black with RGB and hex triplet conversions(pr [#47])
- replace ColourRgb with ToHex trait and Rgb struct(pr [#48])
- BREAKING: add Rgb module to public API(pr [#49])

### Changed

- chore(circleci)-update toolkit orb to version 1.5.0 and add update_changelog parameter(pr [#32])
- chore-update .gitignore and add Cargo.lock(pr [#33])
- chore-add comment for future removal of deprecated `as_rgb` functions(pr [#35])
- refactor-remove commented-out code and extra newline in basic.rs(pr [#36])
- refactor(ext)-reorganize ext module and move Red enum to separate file(pr [#37])

## [0.1.6] - 2024-08-28

### Added

- add to_hex_string method for ColourRgb(pr [#29])
- derive Debug, PartialEq, Eq, Clone, and Copy for ColourRgb struct(pr [#31])

### Changed

- ci-update toolkit orb to version 1.4.3(pr [#28])
- ci-add security job to CircleCI config(pr [#30])

## [0.1.5] - 2024-08-03

### Changed

- ci-add bot-check context to toolkit/make_release workflow(pr [#27])

## [0.1.4] - 2024-07-29

### Changed

- ci-update toolkit version from 0.22.0 to 0.24.1(pr [#24])
- chore-extend configuration with package rules, regex managers, and dependency dashboard(pr [#25])

### Fixed

- correct crate name and config and replacement strings in lib.rs, README.md and release.toml(pr [#26])

## [0.1.3] - 2024-07-15

### Changed

- chore-release replacements and updates(pr [#23])

## [0.1.2] - 2024-07-14

### Changed

- docs-add documentation for DarkRed enum variant in Red enum(pr [#20])
- docs-move documentation update entry to correct version section(pr [#21])
- chore-bump version from 0.1.0 to 0.1.1(pr [#22])

## [0.1.1] - 2024-07-14

### Changed

- chore-move to jerus-org and adopt circleci ci(pr [#16])
- chore-clean up and complete ci migration(pr [#17])
- ci-upgrade jerus-org/circleci-toolkit from v0.20.0 to v0.22.0(pr [#18])
- style-change license syntax from 'or' to 'OR'(pr [#19])

[#16]: https://github.com/jerus-org/named-colour/pull/16
[#17]: https://github.com/jerus-org/named-colour/pull/17
[#18]: https://github.com/jerus-org/named-colour/pull/18
[#19]: https://github.com/jerus-org/named-colour/pull/19
[#20]: https://github.com/jerus-org/named-colour/pull/20
[#21]: https://github.com/jerus-org/named-colour/pull/21
[#22]: https://github.com/jerus-org/named-colour/pull/22
[#23]: https://github.com/jerus-org/named-colour/pull/23
[#24]: https://github.com/jerus-org/named-colour/pull/24
[#25]: https://github.com/jerus-org/named-colour/pull/25
[#26]: https://github.com/jerus-org/named-colour/pull/26
[#27]: https://github.com/jerus-org/named-colour/pull/27
[#28]: https://github.com/jerus-org/named-colour/pull/28
[#29]: https://github.com/jerus-org/named-colour/pull/29
[#30]: https://github.com/jerus-org/named-colour/pull/30
[#31]: https://github.com/jerus-org/named-colour/pull/31
[#32]: https://github.com/jerus-org/named-colour/pull/32
[#33]: https://github.com/jerus-org/named-colour/pull/33
[#34]: https://github.com/jerus-org/named-colour/pull/34
[#35]: https://github.com/jerus-org/named-colour/pull/35
[#36]: https://github.com/jerus-org/named-colour/pull/36
[#37]: https://github.com/jerus-org/named-colour/pull/37
[#38]: https://github.com/jerus-org/named-colour/pull/38
[#39]: https://github.com/jerus-org/named-colour/pull/39
[#40]: https://github.com/jerus-org/named-colour/pull/40
[#41]: https://github.com/jerus-org/named-colour/pull/41
[#42]: https://github.com/jerus-org/named-colour/pull/42
[#43]: https://github.com/jerus-org/named-colour/pull/43
[#44]: https://github.com/jerus-org/named-colour/pull/44
[#45]: https://github.com/jerus-org/named-colour/pull/45
[#46]: https://github.com/jerus-org/named-colour/pull/46
[#47]: https://github.com/jerus-org/named-colour/pull/47
[#48]: https://github.com/jerus-org/named-colour/pull/48
[#49]: https://github.com/jerus-org/named-colour/pull/49
[#50]: https://github.com/jerus-org/named-colour/pull/50
[#51]: https://github.com/jerus-org/named-colour/pull/51
[#52]: https://github.com/jerus-org/named-colour/pull/52
[#53]: https://github.com/jerus-org/named-colour/pull/53
[#54]: https://github.com/jerus-org/named-colour/pull/54
[#55]: https://github.com/jerus-org/named-colour/pull/55
[#56]: https://github.com/jerus-org/named-colour/pull/56
[#57]: https://github.com/jerus-org/named-colour/pull/57
[#58]: https://github.com/jerus-org/named-colour/pull/58
[#59]: https://github.com/jerus-org/named-colour/pull/59
[#60]: https://github.com/jerus-org/named-colour/pull/60
[#61]: https://github.com/jerus-org/named-colour/pull/61
[#62]: https://github.com/jerus-org/named-colour/pull/62
[#63]: https://github.com/jerus-org/named-colour/pull/63
[#64]: https://github.com/jerus-org/named-colour/pull/64
[#65]: https://github.com/jerus-org/named-colour/pull/65
[#66]: https://github.com/jerus-org/named-colour/pull/66
[#67]: https://github.com/jerus-org/named-colour/pull/67
[#68]: https://github.com/jerus-org/named-colour/pull/68
[#69]: https://github.com/jerus-org/named-colour/pull/69
[#70]: https://github.com/jerus-org/named-colour/pull/70
[#71]: https://github.com/jerus-org/named-colour/pull/71
[#72]: https://github.com/jerus-org/named-colour/pull/72
[Unreleased]: https://github.com/jerus-org/named-colour/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/jerus-org/named-colour/compare/v0.1.6...v0.2.0
[0.1.6]: https://github.com/jerus-org/named-colour/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/jerus-org/named-colour/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/jerus-org/named-colour/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/jerus-org/named-colour/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/jerus-org/named-colour/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/jerus-org/named-colour/releases/tag/v0.1.1
