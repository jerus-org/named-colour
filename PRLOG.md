# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- deps: update dependency toolkit to v2.16.0(pr [#122])

## [0.3.24] - 2025-10-21

### Added

- add release-hook.sh script for automated changelog generation(pr [#120])

### Changed

- chore-rename CHANGELOG.md to PRLOG.md(pr [#118])
- chore-update release.toml to reference PRLOG.md instead of CHANGELOG.md(pr [#119])

### Fixed

- deps: update dependency toolkit to v2.13.4(pr [#121])

## [0.3.23] - 2025-08-21

### Fixed

- deps: update rust crate rstest to 0.26.1(pr [#117])
- deps: update rust crate strum to 0.27.2(pr [#116])

## [0.3.22] - 2025-07-21

### Changed

- 🔧 chore(config)-update renovate configuration(pr [#113])

### Fixed

- deps: update dependency toolkit to v2.12.1(pr [#115])
- deps: update rust crate rgb to 0.8.52(pr [#114])

## [0.3.21] - 2025-05-21

### Changed

- 🔧 chore(config)-update renovate schedule(pr [#111])
- 🔧 chore(config)-update renovate schedule(pr [#112])

## [0.3.20] - 2025-04-27

### Changed

- 👷 ci(circleci)-update config for release pipeline support(pr [#110])

### Security

- Dependencies: update dependency toolkit to v2.8.1(pr [#108])

## [0.3.19] - 2025-03-21

### Security

- Dependencies: update dependency toolkit to v2.5.1(pr [#107])

## [0.3.18] - 2025-03-07

### Security

- Dependencies: update rust crate rstest to 0.25.0(pr [#106])
- Dependencies: update dependency toolkit to v2.1.0(pr [#105])

## [0.3.17] - 2025-02-28

### Security

- Dependencies: update dependency toolkit to v2.0.10(pr [#104])

## [0.3.16] - 2025-02-21

### Security

- Dependencies: update rust crate strum to 0.27.1(pr [#103])

## [0.3.15] - 2025-02-14

### Security

- Dependencies: update rust crate strum to 0.27.0(pr [#102])

## [0.3.14] - 2025-01-31

### Changed

- chore(config)-migrate renovate config(pr [#101])

## [0.3.13] - 2025-01-20

### Changed

- 👷 ci(circleci): integrate versioning in workflow(pr [#100])

## [0.3.12] - 2025-01-11

### Changed

- ci(circleci)-add save_next_version job and update make_release dependencies(pr [#99])

### Security

- Dependencies: update dependency toolkit to v2(pr [#98])

## [0.3.11] - 2025-01-03

### Security

- Dependencies: update rust crate rstest to 0.24.0(pr [#97])

## [0.3.10] - 2024-12-27

### Security

- Dependencies: update dependency toolkit to v1.23.0(pr [#96])

## [0.3.9] - 2024-12-20

### Security

- Dependencies: update dependency toolkit to v1.20.2(pr [#95])

## [0.3.8] - 2024-12-13

### Changed

- chore-update schedule for dependency updates to Thursday(pr [#93])
- chore(circleci)-update circleci-toolkit orb to version 1.20.1(pr [#94])

## [0.3.7] - 2024-12-07

### Security

- Dependencies: bump hashbrown from 0.15.0 to 0.15.2 in the cargo group across 1 directory(pr [#92])

## [0.3.6] - 2024-11-15

### Security

- Dependencies: update dependency toolkit to v1.16.1(pr [#89])
- Dependencies: update dependency toolkit to v1.18.0(pr [#90])
- Dependencies: update dependency toolkit to v1.19.0(pr [#91])

## [0.3.5] - 2024-11-01

### Changed

- chore(circleci)-update circleci-toolkit orb to version 1.15.0(pr [#88])

### Security

- Dependencies: update dependency toolkit to v1.14.0(pr [#87])

## [0.3.4] - 2024-10-04

### Security

- Dependencies: update rust crate rstest to 0.23.0(pr [#86])

## [0.3.3] - 2024-09-28

### Added

- update renovate.json to enable circleci-toolkit with sourceUrl(pr [#85])

## [0.3.2] - 2024-09-14

### Added

- add rangeStrategy configuration to renovate.json(pr [#84])

## [0.3.1] - 2024-09-06

### Added

- add new dependencies and implement random color generation(pr [#73])
- add random color generation and use EnumCount for Blue enum(pr [#76])
- add random color generation for Yellow enum(pr [#75])
- add random color generation for Brown and fix example in Blue(pr [#77])
- add random color generation and EnumCount derivation(pr [#78])
- add random color generation and derive EnumCount for Green enum(pr [#79])
- add random color generation and derive EnumCount trait(pr [#80])
- add random color generation using tinyrand(pr [#81])
- add random color generation for White enum(pr [#82])

### Changed

- refactor-remove unnecessary allow attribute and add documentation for Prefix enum(pr [#74])
- refactor(ext)-return Box<dyn ExtendedColour> from random_named_colour function(pr [#83])

## [0.3.0] - 2024-09-05

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
[#73]: https://github.com/jerus-org/named-colour/pull/73
[#74]: https://github.com/jerus-org/named-colour/pull/74
[#76]: https://github.com/jerus-org/named-colour/pull/76
[#75]: https://github.com/jerus-org/named-colour/pull/75
[#77]: https://github.com/jerus-org/named-colour/pull/77
[#78]: https://github.com/jerus-org/named-colour/pull/78
[#79]: https://github.com/jerus-org/named-colour/pull/79
[#80]: https://github.com/jerus-org/named-colour/pull/80
[#81]: https://github.com/jerus-org/named-colour/pull/81
[#82]: https://github.com/jerus-org/named-colour/pull/82
[#83]: https://github.com/jerus-org/named-colour/pull/83
[#84]: https://github.com/jerus-org/named-colour/pull/84
[#85]: https://github.com/jerus-org/named-colour/pull/85
[#86]: https://github.com/jerus-org/named-colour/pull/86
[#87]: https://github.com/jerus-org/named-colour/pull/87
[#88]: https://github.com/jerus-org/named-colour/pull/88
[#89]: https://github.com/jerus-org/named-colour/pull/89
[#90]: https://github.com/jerus-org/named-colour/pull/90
[#91]: https://github.com/jerus-org/named-colour/pull/91
[#92]: https://github.com/jerus-org/named-colour/pull/92
[#93]: https://github.com/jerus-org/named-colour/pull/93
[#94]: https://github.com/jerus-org/named-colour/pull/94
[#95]: https://github.com/jerus-org/named-colour/pull/95
[#96]: https://github.com/jerus-org/named-colour/pull/96
[#97]: https://github.com/jerus-org/named-colour/pull/97
[#98]: https://github.com/jerus-org/named-colour/pull/98
[#99]: https://github.com/jerus-org/named-colour/pull/99
[#100]: https://github.com/jerus-org/named-colour/pull/100
[#101]: https://github.com/jerus-org/named-colour/pull/101
[#102]: https://github.com/jerus-org/named-colour/pull/102
[#103]: https://github.com/jerus-org/named-colour/pull/103
[#104]: https://github.com/jerus-org/named-colour/pull/104
[#106]: https://github.com/jerus-org/named-colour/pull/106
[#105]: https://github.com/jerus-org/named-colour/pull/105
[#107]: https://github.com/jerus-org/named-colour/pull/107
[#108]: https://github.com/jerus-org/named-colour/pull/108
[#110]: https://github.com/jerus-org/named-colour/pull/110
[#111]: https://github.com/jerus-org/named-colour/pull/111
[#112]: https://github.com/jerus-org/named-colour/pull/112
[#113]: https://github.com/jerus-org/named-colour/pull/113
[#115]: https://github.com/jerus-org/named-colour/pull/115
[#114]: https://github.com/jerus-org/named-colour/pull/114
[#117]: https://github.com/jerus-org/named-colour/pull/117
[#116]: https://github.com/jerus-org/named-colour/pull/116
[#118]: https://github.com/jerus-org/named-colour/pull/118
[#119]: https://github.com/jerus-org/named-colour/pull/119
[#120]: https://github.com/jerus-org/named-colour/pull/120
[#121]: https://github.com/jerus-org/named-colour/pull/121
[#122]: https://github.com/jerus-org/named-colour/pull/122
[Unreleased]: https://github.com/jerus-org/named-colour/compare/v0.3.24...HEAD
[0.3.24]: https://github.com/jerus-org/named-colour/compare/v0.3.23...v0.3.24
[0.3.23]: https://github.com/jerus-org/named-colour/compare/v0.3.22...v0.3.23
[0.3.22]: https://github.com/jerus-org/named-colour/compare/v0.3.21...v0.3.22
[0.3.21]: https://github.com/jerus-org/named-colour/compare/v0.3.20...v0.3.21
[0.3.20]: https://github.com/jerus-org/named-colour/compare/v0.3.19...v0.3.20
[0.3.19]: https://github.com/jerus-org/named-colour/compare/v0.3.18...v0.3.19
[0.3.18]: https://github.com/jerus-org/named-colour/compare/v0.3.17...v0.3.18
[0.3.17]: https://github.com/jerus-org/named-colour/compare/v0.3.16...v0.3.17
[0.3.16]: https://github.com/jerus-org/named-colour/compare/v0.3.15...v0.3.16
[0.3.15]: https://github.com/jerus-org/named-colour/compare/v0.3.14...v0.3.15
[0.3.14]: https://github.com/jerus-org/named-colour/compare/v0.3.13...v0.3.14
[0.3.13]: https://github.com/jerus-org/named-colour/compare/v0.3.12...v0.3.13
[0.3.12]: https://github.com/jerus-org/named-colour/compare/v0.3.11...v0.3.12
[0.3.11]: https://github.com/jerus-org/named-colour/compare/v0.3.10...v0.3.11
[0.3.10]: https://github.com/jerus-org/named-colour/compare/v0.3.9...v0.3.10
[0.3.9]: https://github.com/jerus-org/named-colour/compare/v0.3.8...v0.3.9
[0.3.8]: https://github.com/jerus-org/named-colour/compare/v0.3.7...v0.3.8
[0.3.7]: https://github.com/jerus-org/named-colour/compare/v0.3.6...v0.3.7
[0.3.6]: https://github.com/jerus-org/named-colour/compare/v0.3.5...v0.3.6
[0.3.5]: https://github.com/jerus-org/named-colour/compare/v0.3.4...v0.3.5
[0.3.4]: https://github.com/jerus-org/named-colour/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/jerus-org/named-colour/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/jerus-org/named-colour/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/jerus-org/named-colour/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/jerus-org/named-colour/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/jerus-org/named-colour/compare/v0.1.6...v0.2.0
[0.1.6]: https://github.com/jerus-org/named-colour/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/jerus-org/named-colour/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/jerus-org/named-colour/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/jerus-org/named-colour/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/jerus-org/named-colour/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/jerus-org/named-colour/releases/tag/v0.1.1
