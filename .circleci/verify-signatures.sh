#!/usr/bin/env bash
#
# Anti-Impersonation Signature Verification
# 
# This script enforces signature verification to prevent identity impersonation
# in open source projects. It ensures that:
# 1. Commits claiming trusted identities MUST be signed by approved keys
# 2. Commits from untrusted contributors may be unsigned (low barrier to entry)
# 3. No one can impersonate a trusted identity by setting author email
#
# Usage: ./verify-signatures.sh [base_ref] [head_ref]
#   base_ref: Base branch/commit (defaults to origin/main)
#   head_ref: Head commit to check (defaults to HEAD)
#

set -euo pipefail

# ANSI color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration: Trusted identities and their allowed key fingerprints
# Format: EMAIL|FINGERPRINT1[,FINGERPRINT2,...]
# Add more entries as team members join or keys are rotated
TRUSTED_KEYS=(
  "jerry@jrussell.ie|E576B835ACE207E5"
  "47631109+jerusdp@users.noreply.github.com|E576B835ACE207E5,B5690EEEBB952194"
  "171541392+jerus-bot@users.noreply.github.com|EB85EDFF0BCB42F8"
)

# GitHub web-flow key (for merge commits signed by GitHub)
GITHUB_WEBFLOW_KEY="B5690EEEBB952194"

# Public keys for trusted identities (for signature verification)
# Export these with: gpg --armor --export KEY_ID
TRUSTED_PUBLIC_KEYS=(
  # Jeremiah Russell (garden) - E576B835ACE207E5
  "-----BEGIN PGP PUBLIC KEY BLOCK-----

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
6wZttwua+g65
=dDZe
-----END PGP PUBLIC KEY BLOCK-----"
  # Jerus Bot - EB85EDFF0BCB42F8
  "-----BEGIN PGP PUBLIC KEY BLOCK-----

mDMEZmG2fhYJKwYBBAHaRw8BAQdAMInH7AaMw9MFbijh0Xp+SE3av2ufrv8f5aMT
13nog7u0OEplcnVzIEJvdCA8MTcxNTQxMzkyK2plcnVzLWJvdEB1c2Vycy5ub3Jl
cGx5LmdpdGh1Yi5jb20+iJAEExYIADgWIQRb1c6eyIJfIIfUQ3bZD2BNbjz+EwUC
ZmG2fgIbAQULCQgHAgYVCgkICwIEFgIDAQIeAQIXgAAKCRDZD2BNbjz+E8dpAQCK
4zDqX5RSOFaCI1ylS/u1gSbgVyPurUpcZrHplR/jRgEAm2/8yoJ7JlXsO24P0jJL
WS8tjVlWR/3CQoVQTaxQ1g24MwRmYbbaFgkrBgEEAdpHDwEBB0Cl7xEihfkrrWh7
fpN9ZXAI5PTp5d9HvCFh087PaiVftojvBBgWCAAgFiEEW9XOnsiCXyCH1EN22Q9g
TW48/hMFAmZhttoCGwIAgQkQ2Q9gTW48/hN2IAQZFggAHRYhBFoOWgeNimYarMfL
WuuF7f8Ly0L4BQJmYbbaAAoJEOuF7f8Ly0L47A0A/3w7/HCl9xkoFpTaf/C4ojWt
IWbCX66vm9DvPYtheffUAP4hDc5gc1askYYvzEXPGAZ8LIU4mW/i+A+bCXmdHKaH
DIjsAQDL2npRl9d/2owWxPPEuH0QQa96aAe1zQXWzpcdVpSJXgD9FAenk1ukKlx4
yAOIGym8X5zArsVvnNSJ/mltcM/yCQE=
=FXGz
-----END PGP PUBLIC KEY BLOCK-----"
)

echo -e "${BLUE}=== Commit Signature Verification ===${NC}"
echo "Checking for identity impersonation and signature compliance..."
echo ""

# Import trusted public keys for verification
echo "Importing trusted public keys..."
for key in "${TRUSTED_PUBLIC_KEYS[@]}"; do
  echo "$key" | gpg --import 2>/dev/null || true
done
echo ""

# Build lookup map: email -> comma-separated fingerprints
declare -A TRUSTMAP
for row in "${TRUSTED_KEYS[@]}"; do
  email="${row%%|*}"
  fps="${row#*|}"
  TRUSTMAP["$email"]="$fps"
  echo -e "${BLUE}ℹ${NC}  Trusted identity: ${email} (keys: ${fps})"
done
echo ""

# Determine commit range to check
BASE_REF="${1:-${CIRCLE_BRANCH_BASE:-origin/main}}"
HEAD_REF="${2:-${CIRCLE_SHA1:-HEAD}}"

# Ensure we have the base branch
echo "Fetching base branch for comparison..."
git fetch --no-tags --depth=200 origin +refs/heads/*:refs/remotes/origin/* >/dev/null 2>&1 || true

# Find merge base
MERGE_BASE="$(git merge-base "$BASE_REF" "$HEAD_REF" 2>/dev/null || true)"
if [[ -z "$MERGE_BASE" ]]; then
  echo -e "${RED}✗${NC} ERROR: Could not compute merge-base for ${BASE_REF}..${HEAD_REF}"
  echo "This might be a new branch with no common history."
  exit 2
fi

echo -e "${BLUE}ℹ${NC}  Checking commit range: ${MERGE_BASE:0:8}..${HEAD_REF:0:8}"
echo ""

# Count commits in range
COMMIT_COUNT=$(git rev-list --count --no-merges "${MERGE_BASE}..${HEAD_REF}")
if [[ "$COMMIT_COUNT" -eq 0 ]]; then
  echo -e "${GREEN}✓${NC} No new commits to verify."
  exit 0
fi

echo -e "${BLUE}ℹ${NC}  Found ${COMMIT_COUNT} commit(s) to verify"
echo ""

status=0
checked=0
trusted_verified=0
untrusted_ok=0

# Iterate all commits in PR (exclude merge commits)
while IFS= read -r sha; do
  checked=$((checked + 1))
  
  # Extract commit metadata
  author_email="$(git show -s --format='%ae' "$sha")"
  author_name="$(git show -s --format='%an' "$sha")"
  sig_status="$(git show -s --format='%G?' "$sha")"   # G=good, U=untrusted, N=none, B/E/X/R=problems
  key_id="$(git show -s --format='%GK' "$sha")"       # signing key fingerprint (if any)
  signer="$(git show -s --format='%GS' "$sha")"       # signer uid (if any)
  subject="$(git show -s --format='%s' "$sha")"
  
  # Format commit info
  sha_short="${sha:0:8}"
  
  # Check if this email claims a trusted identity
  if [[ -n "${TRUSTMAP[$author_email]:-}" ]]; then
    # TRUSTED IDENTITY - Must be signed by approved key
    allowed="${TRUSTMAP[$author_email]}"
    
    # First check: Must have valid signature
    if [[ "$sig_status" != "G" && "$sig_status" != "U" ]]; then
      echo -e "${RED}✗ FAIL${NC} ${sha_short} ${subject}"
      echo -e "    ${RED}Security violation${NC}: Author ${author_email} claims trusted identity"
      echo -e "    but signature status is '${sig_status}' (expected G or U)"
      echo -e "    This could be an impersonation attempt!"
      echo ""
      status=1
      continue
    fi
    
    # Second check: Key must be in allowlist
    match=0
    IFS=',' read -ra fps <<< "$allowed"
    for fp in "${fps[@]}"; do
      # Match both short and long key IDs
      if [[ "$key_id" == *"$fp"* ]]; then
        match=1
        break
      fi
    done
    
    if [[ $match -eq 0 ]]; then
      echo -e "${RED}✗ FAIL${NC} ${sha_short} ${subject}"
      echo -e "    ${RED}Key mismatch${NC}: Author ${author_email} signed with key ${key_id}"
      echo -e "    but this key is not in the allowlist: {${allowed}}"
      echo -e "    Either the key needs to be added, or this is an impersonation!"
      echo ""
      status=1
    else
      trusted_verified=$((trusted_verified + 1))
      echo -e "${GREEN}✓ OK${NC}   ${sha_short} ${subject}"
      echo -e "    ${GREEN}Verified${NC}: ${author_name} <${author_email}>"
      echo -e "    Signed by: ${signer} (key: ${key_id:0:16}...)"
      echo ""
    fi
    
  else
    # UNTRUSTED/EXTERNAL CONTRIBUTOR
    # Unsigned commits are ALLOWED (keeps contribution friction low)
    # But we note the signature status for awareness
    
    untrusted_ok=$((untrusted_ok + 1))
    
    if [[ "$sig_status" == "G" || "$sig_status" == "U" ]]; then
      echo -e "${GREEN}✓ OK${NC}   ${sha_short} ${subject}"
      echo -e "    ${YELLOW}External contributor${NC}: ${author_name} <${author_email}>"
      echo -e "    Has signature from: ${signer} (key: ${key_id:0:16}...)"
      echo ""
    else
      echo -e "${GREEN}✓ OK${NC}   ${sha_short} ${subject}"
      echo -e "    ${YELLOW}External contributor${NC}: ${author_name} <${author_email}>"
      echo -e "    ${YELLOW}Unsigned${NC} (allowed for external contributors)"
      echo ""
    fi
  fi
done < <(git rev-list --no-merges "${MERGE_BASE}..${HEAD_REF}")

echo -e "${BLUE}=== Verification Summary ===${NC}"
echo -e "Commits checked:         ${checked}"
echo -e "Trusted identities:      ${GREEN}${trusted_verified} verified${NC}"
echo -e "External contributors:   ${GREEN}${untrusted_ok} ok${NC}"

if [[ $status -eq 0 ]]; then
  echo ""
  echo -e "${GREEN}✓ All signature checks passed!${NC}"
  echo ""
  echo "No impersonation attempts detected."
  echo "All commits from trusted identities are properly signed."
else
  echo ""
  echo -e "${RED}✗ Signature verification FAILED!${NC}"
  echo ""
  echo "One or more commits failed verification:"
  echo "  - Trusted identity claimed without proper signature, OR"
  echo "  - Commit signed with key not in allowlist"
  echo ""
  echo "This could indicate:"
  echo "  1. Attempted identity impersonation"
  echo "  2. Trusted user using wrong GPG key"
  echo "  3. Key rotation not yet updated in config"
  echo ""
  echo "Action required:"
  echo "  - If this is a legitimate commit, add the key to TRUSTED_KEYS"
  echo "  - If this is impersonation, reject the PR immediately"
fi

exit $status
