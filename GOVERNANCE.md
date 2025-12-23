# Project Governance

This document describes the governance model for the named-colour project.

## Project Structure

### Maintainer

The project is maintained by:

- **Jeremiah Russell** (@jerusdp) - Project Lead and Primary Maintainer
  - Email: jrussell@jerus.ie
  - Responsibilities: Strategic direction, releases, security responses, final decision authority

### Contributors

Contributors are individuals who have contributed code, documentation, or other improvements to the project. All contributors are recognized in the Git history and GitHub contributors page.

## Decision-Making Process

### Day-to-Day Decisions

- **Minor Changes**: Bug fixes, documentation improvements, and minor enhancements are reviewed and merged by the maintainer through the standard pull request process.
- **Review Criteria**: Code quality, test coverage, adherence to coding standards, backward compatibility.

### Significant Changes

Significant changes include:

- New features or major enhancements
- Breaking API changes
- Architectural changes
- Changes to project governance

**Process for Significant Changes:**

1. **Discussion**: Propose the change via a GitHub issue
2. **Community Input**: Allow time for community feedback (typically 7-14 days)
3. **Design**: Create detailed design documentation if needed
4. **Decision**: Maintainer makes final decision considering community input
5. **Implementation**: Changes are implemented via pull request
6. **Review**: Code review by maintainer before merging

### Emergency Security Fixes

Security vulnerabilities may bypass normal discussion periods:

1. Reported privately to security@jerus.ie
2. Fixed in a private branch
3. Released with coordinated disclosure
4. Community notified after fix is available

## Contribution Process

1. **Fork**: Create a fork of the repository
2. **Branch**: Create a feature branch from `main`
3. **Develop**: Make changes following [CONTRIBUTING.md](CONTRIBUTING.md)
4. **Test**: Ensure all tests pass and add new tests
5. **Sign-off**: Include DCO sign-off in commits
6. **Pull Request**: Submit PR with clear description
7. **Review**: Maintainer reviews and provides feedback
8. **Merge**: Once approved, maintainer merges the PR

## Release Process

Releases are managed by the maintainer:

1. **Version Bump**: Update version in Cargo.toml following semantic versioning
2. **Changelog**: Update CHANGELOG.md with all changes since last release
3. **Testing**: Ensure all tests pass on CI
4. **Tag**: Create Git tag (e.g., `v0.3.26`)
5. **Publish**: Publish to crates.io
6. **GitHub Release**: Create GitHub release with release notes
7. **Announcement**: Update README.md if needed

### Versioning

The project follows [Semantic Versioning 2.0.0](https://semver.org/):

- **Major** (X.0.0): Breaking changes
- **Minor** (0.X.0): New features, backward compatible
- **Patch** (0.0.X): Bug fixes, backward compatible

## Roles and Responsibilities

### Maintainer Responsibilities

- Review and merge pull requests
- Manage releases and versioning
- Respond to security reports
- Maintain CI/CD infrastructure
- Enforce Code of Conduct
- Make final decisions on significant changes
- Keep project documentation up to date

### Contributor Responsibilities

- Follow [Code of Conduct](CODE_OF_CONDUCT.md)
- Follow [Contributing Guidelines](CONTRIBUTING.md)
- Write quality code with tests
- Be responsive to review feedback
- Sign off commits with DCO

## Communication Channels

- **GitHub Issues**: Bug reports, feature requests, discussions
- **Pull Requests**: Code contributions and reviews
- **Email**: security@jerus.ie for security issues
- **GitHub Discussions**: General questions and community discussions (if enabled)

## Conflict Resolution

1. **First Step**: Discuss issue respectfully in the relevant GitHub issue or PR
2. **Escalation**: If unresolved, contact the maintainer directly
3. **Code of Conduct**: Violations should be reported to security@jerus.ie
4. **Final Authority**: Maintainer has final decision-making authority

## Becoming a Maintainer

The project currently has a single maintainer model. As the project grows:

- Active contributors may be invited to become maintainers
- Criteria include: consistent quality contributions, community engagement, understanding of project goals
- New maintainers are added by consensus of existing maintainers

## Changes to Governance

This governance document may be changed through:

1. Proposal via GitHub issue
2. Community discussion period (minimum 14 days)
3. Approval by maintainer
4. Update via pull request

## License

All contributions are licensed under MIT OR Apache-2.0, as specified in [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).

## Acknowledgments

This governance model is inspired by:
- Rust project governance
- CNCF project governance best practices
- OpenSSF Best Practices recommendations

---

*Last updated: 2025-12-23*
