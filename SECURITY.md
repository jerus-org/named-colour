# Security Policy

## Supported Versions

We release security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.3.x   | :white_check_mark: |
| < 0.3   | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report security vulnerabilities by email to:

**security@jerus.ie**

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the following information in your report:

- Type of vulnerability
- Full paths of source file(s) related to the manifestation of the vulnerability
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the vulnerability, including how an attacker might exploit it

This information will help us triage your report more quickly.

## Security Update Process

1. **Report received**: Security team acknowledges receipt within 48 hours
2. **Validation**: Team validates and assesses severity (typically within 7 days)
3. **Fix development**: Security fix is developed in a private branch
4. **Coordinated disclosure**: Fix is released, security advisory published
5. **Public disclosure**: After fix is available, vulnerability details are disclosed

## Security Advisories

Security advisories will be published via:

- GitHub Security Advisories
- Release notes on GitHub
- Announcement in README.md

## Security Best Practices for Users

When using named-colour in your projects:

- Always use the latest version from crates.io
- Review the CHANGELOG.md for security-related updates
- Subscribe to GitHub notifications for security advisories
- Use `cargo audit` to check for known vulnerabilities in dependencies

## Scope

This security policy applies to:

- The named-colour crate published on crates.io
- The source code in the jerus-org/named-colour GitHub repository

## Attribution

This security policy follows best practices from:
- OpenSSF Security Best Practices
- GitHub Security Lab recommendations
