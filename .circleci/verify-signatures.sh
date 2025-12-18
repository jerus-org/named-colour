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

echo -e "${BLUE}=== Commit Signature Verification ===${NC}"
echo "Checking for identity impersonation and signature compliance..."
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
