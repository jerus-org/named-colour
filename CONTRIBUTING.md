# Contributing to named-colour

Thank you for your interest in contributing to named-colour! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to security@jerus.ie.

## How to Contribute

### Reporting Bugs

- Use GitHub Issues to report bugs
- Check if the issue already exists before creating a new one
- Include a clear description and reproduction steps
- Specify the version of named-colour you're using
- Include relevant code samples or error messages

### Suggesting Enhancements

- Use GitHub Issues to suggest enhancements
- Clearly describe the proposed feature and its use case
- Explain why this enhancement would be useful

### Pull Requests

1. **Fork the repository** and create your branch from `main`
2. **Make your changes** following our coding standards (see below)
3. **Add tests** for any new functionality
4. **Run the test suite** to ensure all tests pass
5. **Run formatting and linting** (see below)
6. **Sign off your commits** using the DCO (see below)
7. **Submit a pull request** with a clear description of changes

## Development Setup

### Prerequisites

- Rust 1.75 or later
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test --all-features
```

### Documentation

Generate and view documentation:

```bash
cargo doc --open --all-features
```

Generate documentation with private items (for development):

```bash
RUSTDOCFLAGS='--cfg docsrs -Dwarnings' cargo +nightly doc --lib --no-deps --all-features --document-private-items
```

## Coding Standards

### Style Guide

This project uses standard Rust formatting:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting

All code must pass Clippy without warnings:

```bash
# Run Clippy
cargo clippy --all-targets --all-features -- -D warnings
```

### Testing

- Write tests for all new functionality
- Ensure all existing tests pass
- Aim for high code coverage
- Use `rstest` for parameterized tests where appropriate

### Documentation

- Document all public APIs
- Include examples in documentation where helpful
- Use `///` for doc comments on public items
- Use `//!` for module-level documentation

## Developer Certificate of Origin (DCO)

By contributing to this project, you certify that:

1. The contribution was created in whole or in part by you and you have the right to submit it under the open source license indicated in the file; or
2. The contribution is based upon previous work that, to the best of your knowledge, is covered under an appropriate open source license and you have the right under that license to submit that work with modifications; or
3. The contribution was provided directly to you by some other person who certified (1), (2) or (3) and you have not modified it.

You understand and agree that this project and the contribution are public and that a record of the contribution is maintained indefinitely.

### Sign-off Process

To certify the above, add a `Signed-off-by` line to your commit messages:

```
This is my commit message

Signed-off-by: Your Name <your.email@example.com>
```

You can add this automatically using:

```bash
git commit -s -m "Your commit message"
```

Or configure Git to always sign off:

```bash
git config --global format.signOff true
```

**All commits must include a sign-off.** Pull requests with unsigned commits will not be merged.

## Commit Message Guidelines

- Use clear, descriptive commit messages
- Start with a verb in present tense (e.g., "Add", "Fix", "Update")
- Keep the first line under 50 characters
- Provide additional context in the body if needed
- Reference relevant issues (e.g., "Fixes #123")

Example:
```
Add support for custom color parsing

Implement FromStr trait for ColourRgb to allow parsing
colors from hex strings. This addresses user requests
for more flexible color initialization.

Fixes #42
Signed-off-by: Your Name <your.email@example.com>
```

## Release Process

Releases are managed by project maintainers:

1. Version is bumped in Cargo.toml
2. CHANGELOG.md is updated
3. Git tag is created
4. Release is published to crates.io
5. GitHub release is created with release notes

## Getting Help

- Open an issue for questions
- Check existing issues and documentation first
- Be patient and respectful

## License

By contributing to named-colour, you agree that your contributions will be licensed under the same terms as the project:

- MIT License OR Apache License 2.0

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.

## Recognition

Contributors are recognized in:
- Git commit history
- GitHub contributors page
- Release notes (for significant contributions)

Thank you for contributing to named-colour!
