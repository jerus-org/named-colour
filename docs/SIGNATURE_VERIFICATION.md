# Commit Signature Verification

This repository implements **anti-impersonation signature verification** to ensure the authenticity and integrity of all commits from trusted identities.

## Overview

The signature verification system prevents malicious actors from impersonating trusted contributors by forging Git author information. This is critical for open source projects that accept contributions from untrusted third parties.

## How It Works

### The Problem

Git allows anyone to set arbitrary author names and emails:

```bash
# Attacker can impersonate a maintainer
git config user.name "Jeremiah Russell"
git config user.email "jerry@jrussell.ie"
git commit -m "Add backdoor"
# Result: Commit appears to be from maintainer!
```

Without signature verification, there's no way to verify that a commit claiming to be from a trusted identity was actually created by that person.

### The Solution

Our signature verification system enforces:

1. **Trusted identities MUST sign commits** - Any commit claiming to be from a maintainer or bot must be cryptographically signed with an approved GPG key
2. **Untrusted contributors may be unsigned** - External contributors don't need GPG setup, keeping the contribution barrier low
3. **Allowlist enforcement** - Only pre-approved keys can sign for trusted identities

### Trust Model

```
┌─────────────────────────────────────────────────────────┐
│                 Commit Verification Flow                 │
└─────────────────────────────────────────────────────────┘

Is author email in trusted list?
        │
        ├─── YES ──> MUST be signed by approved key
        │            │
        │            ├─── Signature valid? ──> ✅ PASS
        │            └─── No signature?   ──> ❌ FAIL (Impersonation!)
        │
        └─── NO ───> External contributor
                     │
                     ├─── Signed?   ──> ✅ PASS (bonus!)
                     └─── Unsigned? ──> ✅ PASS (allowed)
```

## Trusted Identities

The following identities are considered trusted and **must** sign all their commits:

| Email | Owner | Key Fingerprint(s) | Purpose |
|-------|-------|-------------------|---------|
| `jerry@jrussell.ie` | Jeremiah Russell | `E576B835ACE207E5` | Primary maintainer |
| `47631109+jerusdp@users.noreply.github.com` | Jeremiah Russell (GitHub) | `E576B835ACE207E5`, `B5690EEEBB952194` | GitHub web merges |
| `171541392+jerus-bot@users.noreply.github.com` | Jerus Bot | `EB85EDFF0BCB42F8` | CI/CD automation |

### GitHub Web-Flow Key

GitHub signs merge commits with its web-flow key `B5690EEEBB952194`. This key is trusted for merge commits performed via GitHub's web interface.

## Configuration

Trusted identities and their keys are configured in `.circleci/verify-signatures.sh`:

```bash
TRUSTED_KEYS=(
  "jerry@jrussell.ie|E576B835ACE207E5"
  "47631109+jerusdp@users.noreply.github.com|E576B835ACE207E5,B5690EEEBB952194"
  "171541392+jerus-bot@users.noreply.github.com|EB85EDFF0BCB42F8"
)
```

### Adding a New Trusted Identity

To add a new maintainer or bot:

1. Get their GPG key fingerprint:
   ```bash
   gpg --list-keys their-email@example.com
   ```

2. Add entry to `TRUSTED_KEYS` in `.circleci/verify-signatures.sh`:
   ```bash
   "their-email@example.com|FINGERPRINT1,FINGERPRINT2"
   ```

3. Commit and push the change (signed by an existing maintainer)

### Key Rotation

When a maintainer rotates their GPG key:

1. Add new fingerprint to their entry (comma-separated):
   ```bash
   "jerry@jrussell.ie|OLD_KEY,NEW_KEY"
   ```

2. After transition period (e.g., 1 month), remove old key:
   ```bash
   "jerry@jrussell.ie|NEW_KEY"
   ```

## CI/CD Integration

The signature verification runs automatically on every pull request as part of the `validation` workflow:

```yaml
jobs:
  verify_commit_signatures:
    executor: base-env
    steps:
      - checkout
      - run: Import GitHub web-flow GPG key
      - run: bash .circleci/verify-signatures.sh
```

### Workflow Requirements

The `trigger_success_pipeline` job requires `verify_commit_signatures` to pass, ensuring no PR can be merged without passing signature verification.

## For Contributors

### External Contributors (No GPG Required)

If you're contributing to this project and are not a maintainer:

- ✅ You do NOT need to set up GPG signing
- ✅ Your commits can be unsigned
- ✅ Standard PR review process applies

Just follow the normal contribution workflow!

### Trusted Maintainers (GPG Required)

If you are a maintainer or have commit access:

- ⚠️ You MUST sign all your commits
- ⚠️ Your signature must use an approved key
- ⚠️ Unsigned commits will fail CI

#### Setting Up GPG Signing

1. **Generate a GPG key** (if you don't have one):
   ```bash
   gpg --full-generate-key
   # Choose RSA, 4096 bits, no expiration
   ```

2. **Configure Git to use your key**:
   ```bash
   # Get your key ID
   gpg --list-secret-keys --keyid-format=long
   
   # Configure Git
   git config --global user.signingkey YOUR_KEY_ID
   git config --global commit.gpgsign true
   ```

3. **Add your public key to GitHub**:
   ```bash
   # Export public key
   gpg --armor --export YOUR_KEY_ID
   
   # Add to GitHub: Settings > SSH and GPG keys > New GPG key
   ```

4. **Get your key added to allowlist**:
   - Submit a PR adding your key fingerprint to `TRUSTED_KEYS`
   - Another maintainer must review and merge

#### Verifying Your Commits

Before pushing:

```bash
# Check if your commit is signed
git log --show-signature -1

# You should see:
# gpg: Signature made ...
# gpg: Good signature from "Your Name <your@email>"
```

## For Maintainers

### Reviewing PRs

When reviewing a PR, check the CI output for the `verify_commit_signatures` job:

#### ✅ Passing Example
```
=== Commit Signature Verification ===
✓ OK   abc12345 feat: add new feature
    External contributor: John Doe <john@example.com>
    Unsigned (allowed for external contributors)

✓ OK   def67890 ci: update workflow
    Verified: Jeremiah Russell <jerry@jrussell.ie>
    Signed by: Jeremiah Russell (garden) (key: E576B835ACE207E5)

✓ All signature checks passed!
```

#### ❌ Failing Example (Impersonation Attempt)
```
=== Commit Signature Verification ===
✗ FAIL abc12345 Add backdoor
    Security violation: Author jerry@jrussell.ie claims trusted identity
    but signature status is 'N' (expected G or U)
    This could be an impersonation attempt!

✗ Signature verification FAILED!
```

**Action**: If verification fails, investigate immediately. This could indicate:
1. Attempted impersonation attack
2. Maintainer forgot to sign
3. Maintainer using wrong key

### Merging Strategy

**Important**: Use **merge commits**, not rebase or squash:

- ✅ Merge commits preserve original signatures
- ❌ Rebase re-signs all commits, losing original verification
- ❌ Squash combines commits, losing individual attribution

#### GitHub Settings

Configure repository settings:
```
✅ Allow merge commits
❌ Allow squash merging (loses attribution)
❌ Allow rebase merging (incompatible with vigilant mode)
```

#### When Merging

1. Ensure all CI checks pass (including signature verification)
2. Click "Create a merge commit" in GitHub UI
3. GitHub will sign the merge commit with its web-flow key
4. Result: Original commits preserve their signatures, merge commit signed by GitHub

### Handling Verification Failures

If a commit from a trusted identity fails verification:

1. **Check if it's the right person**:
   - Ask them to confirm they created the commit
   - They can run: `git log --show-signature <commit-hash>`

2. **If legitimate but wrong key**:
   - They may have a new key not in allowlist
   - Add their new key fingerprint to `TRUSTED_KEYS`

3. **If impersonation**:
   - **Do not merge the PR**
   - Report to GitHub security
   - Review recent commits from that contributor
   - Consider blocking the user

## Security Considerations

### What This Protects Against

✅ **Identity impersonation** - Attacker can't claim to be a maintainer  
✅ **Supply chain attacks** - Can track which actual person/bot contributed code  
✅ **Attribution manipulation** - Clear audit trail of who wrote what  
✅ **Unauthorized commits** - Only approved keys can commit as trusted identities  

### What This Doesn't Protect Against

❌ **Compromised maintainer account** - If attacker steals GPG key  
❌ **Social engineering** - Attacker convinces maintainer to sign malicious code  
❌ **Code review bypasses** - Still need careful review of all changes  
❌ **Dependency attacks** - External dependencies not covered  

### Defense in Depth

This is one layer of security. Also important:

- **Code review** - All PRs require review
- **CI/CD tests** - Automated security scanning
- **Branch protection** - Prevent force pushes
- **Two-factor authentication** - On all maintainer accounts
- **Key management** - Secure GPG private keys

## Troubleshooting

### "Could not compute merge-base"

The verification script couldn't find common history between your branch and main.

**Fix**: Rebase your branch on latest main:
```bash
git fetch origin
git rebase origin/main
```

### "Author claims trusted identity but signature status is 'N'"

You are a maintainer but your commit is unsigned.

**Fix**: Re-commit with signature:
```bash
git commit --amend -S --no-edit
git push --force-with-lease
```

### "Key mismatch"

Your commit is signed but with a key not in the allowlist.

**Fix**: Either:
1. Re-sign with approved key:
   ```bash
   git config user.signingkey APPROVED_KEY_ID
   git commit --amend -S --no-edit
   git push --force-with-lease
   ```

2. Or add your new key to allowlist (via separate PR)

### "Signature verification error"

GPG can't verify your signature.

**Check**:
- Is your key expired? `gpg --list-keys`
- Is your key uploaded to GitHub?
- Is your Git config correct? `git config user.signingkey`

## References

- [GitHub: Signing commits](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits)
- [GitHub: Vigilant mode](https://github.blog/changelog/2021-05-17-vigilant-mode-for-commit-signature-verification/)
- [GPG documentation](https://gnupg.org/documentation/)
- [Git commit signature verification](https://git-scm.com/book/en/v2/Git-Tools-Signing-Your-Work)

## Questions?

If you have questions about signature verification:
- Open an issue
- Contact maintainers
- Review this documentation

Remember: Signature verification protects everyone in the project by ensuring code authenticity!
