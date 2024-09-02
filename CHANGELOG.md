# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- add support for RGB and Hex triplet conversion(pr [#34])

### Changed

- chore(circleci)-update toolkit orb to version 1.5.0 and add update_changelog parameter(pr [#32])
- chore-update .gitignore and add Cargo.lock(pr [#33])
- chore-add comment for future removal of deprecated `as_rgb` functions(pr [#35])

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
[Unreleased]: https://github.com/jerus-org/named-colour/compare/v0.1.6...HEAD
[0.1.6]: https://github.com/jerus-org/named-colour/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/jerus-org/named-colour/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/jerus-org/named-colour/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/jerus-org/named-colour/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/jerus-org/named-colour/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/jerus-org/named-colour/releases/tag/v0.1.1
