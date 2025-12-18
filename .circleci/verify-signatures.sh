#!/usr/bin/env bash
#
# Anti-Impersonation Signature Verification (Dynamic)
# 
# This script enforces signature verification to prevent identity impersonation
# by dynamically fetching trusted collaborators from GitHub and their GPG keys.
#
# Features:
# - No hardcoded list of trusted identities
# - Fetches collaborators with write access from GitHub API
# - Imports their public GPG keys from GitHub
# - Prevents impersonation attacks
#
# Usage: ./verify-signatures.sh [base_ref] [head_ref]
#

set -euo pipefail

# ANSI color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Security constants
CURL_HTTPS_ONLY='=https'
CURL_TLS_VERSION='1.2'

echo -e "${BLUE}=== Commit Signature Verification (Dynamic) ===${NC}"
echo "Fetching trusted identities from GitHub..."
echo ""

# Determine repository from environment or git remote
REPO_OWNER="${CIRCLE_PROJECT_USERNAME:-}"
REPO_NAME="${CIRCLE_PROJECT_REPONAME:-}"

if [[ -z "$REPO_OWNER" ]] || [[ -z "$REPO_NAME" ]]; then
  REMOTE_URL=$(git remote get-url origin 2>/dev/null || echo "")
  if [[ "$REMOTE_URL" =~ github\.com[:/]([^/]+)/([^/\.]+) ]]; then
    REPO_OWNER="${BASH_REMATCH[1]}"
    REPO_NAME="${BASH_REMATCH[2]}"
  fi
fi

if [[ -z "$REPO_OWNER" ]] || [[ -z "$REPO_NAME" ]]; then
  echo -e "${RED}✗${NC} ERROR: Could not determine repository owner/name" >&2
  exit 2
fi

echo "Repository: ${REPO_OWNER}/${REPO_NAME}"
echo ""

# Build trust map dynamically
declare -A TRUSTMAP

# Fetch collaborators with write/admin access
echo "Fetching collaborators with write access..."
COLLAB_API="https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/collaborators"

if [[ -n "${GITHUB_TOKEN:-}" ]]; then
  COLLABORATORS=$(curl -sS --proto "${CURL_HTTPS_ONLY}" --tlsv"${CURL_TLS_VERSION}" -H "Authorization: token ${GITHUB_TOKEN}" "$COLLAB_API" 2>/dev/null | \
    jq -r '.[] | select(.permissions.push == true or .permissions.admin == true) | .login' 2>/dev/null || echo "")
else
  COLLABORATORS=$(curl -sS --proto "${CURL_HTTPS_ONLY}" --tlsv"${CURL_TLS_VERSION}" "$COLLAB_API" 2>/dev/null | \
    jq -r '.[] | select(.permissions.push == true or .permissions.admin == true) | .login' 2>/dev/null || echo "")
fi

if [[ -z "$COLLABORATORS" ]]; then
  echo -e "${YELLOW}⚠${NC}  Warning: Could not fetch collaborators, will only verify against known keys in GPG keyring"
else
  echo -e "${BLUE}ℹ${NC}  Found $(echo "$COLLABORATORS" | wc -l) collaborator(s) with write access"
fi

# For each collaborator, fetch and import their GPG keys
while IFS= read -r username; do
  [[ -z "$username" ]] && continue
  
  echo -e "${BLUE}ℹ${NC}  Processing: ${username}"
  
  KEYS_API="https://api.github.com/users/${username}/gpg_keys"
  if [[ -n "${GITHUB_TOKEN:-}" ]]; then
    GPG_KEYS=$(curl -sS --proto "${CURL_HTTPS_ONLY}" --tlsv"${CURL_TLS_VERSION}" -H "Authorization: token ${GITHUB_TOKEN}" "$KEYS_API" 2>/dev/null)
  else
    GPG_KEYS=$(curl -sS --proto "${CURL_HTTPS_ONLY}" --tlsv"${CURL_TLS_VERSION}" "$KEYS_API" 2>/dev/null)
  fi
  
  if [[ -z "$GPG_KEYS" ]] || [[ "$GPG_KEYS" == "[]" ]]; then
    echo -e "${YELLOW}  ⚠${NC}  No GPG keys found for ${username}"
    continue
  fi
  
  # Import each key and map emails to key IDs
  echo "$GPG_KEYS" | jq -c '.[]' 2>/dev/null | while read -r key_obj; do
    RAW_KEY=$(echo "$key_obj" | jq -r '.raw_key // empty')
    KEY_ID=$(echo "$key_obj" | jq -r '.key_id // empty')
    
    if [[ -n "$RAW_KEY" ]] && [[ "$RAW_KEY" != "null" ]]; then
      # Import the public key
      echo "$RAW_KEY" | gpg --import 2>/dev/null && \
        echo -e "${GREEN}  ✓${NC}  Imported key ${KEY_ID:0:16}... for ${username}" || \
        echo -e "${YELLOW}  ⚠${NC}  Failed to import key for ${username}"
      
      # Get emails from the key
      EMAILS=$(echo "$key_obj" | jq -r '.emails[]?.email // empty' 2>/dev/null)
      while IFS= read -r email; do
        [[ -z "$email" ]] && continue
        
        # Add to trust map
        if [[ -n "${TRUSTMAP[$email]:-}" ]]; then
          TRUSTMAP["$email"]="${TRUSTMAP[$email]},${KEY_ID}"
        else
          TRUSTMAP["$email"]="$KEY_ID"
        fi
        echo -e "${BLUE}  ℹ${NC}  Trusting: ${email} → ${KEY_ID:0:16}..."
      done <<< "$EMAILS"
      
      # Also trust GitHub noreply emails (these are used for web edits and bots)
      # Format: username@users.noreply.github.com or ID+username@users.noreply.github.com
      GITHUB_EMAIL="${username}@users.noreply.github.com"
      if [[ -n "${TRUSTMAP[$GITHUB_EMAIL]:-}" ]]; then
        TRUSTMAP["$GITHUB_EMAIL"]="${TRUSTMAP[$GITHUB_EMAIL]},${KEY_ID}"
      else
        TRUSTMAP["$GITHUB_EMAIL"]="$KEY_ID"
      fi
      
      # Also trust numeric ID format (for bots)
      GITHUB_USER_ID=$(curl -sS --proto "${CURL_HTTPS_ONLY}" --tlsv"${CURL_TLS_VERSION}" "https://api.github.com/users/${username}" 2>/dev/null | jq -r '.id // empty')
      if [[ -n "$GITHUB_USER_ID" ]]; then
        GITHUB_ID_EMAIL="${GITHUB_USER_ID}+${username}@users.noreply.github.com"
        if [[ -n "${TRUSTMAP[$GITHUB_ID_EMAIL]:-}" ]]; then
          TRUSTMAP["$GITHUB_ID_EMAIL"]="${TRUSTMAP[$GITHUB_ID_EMAIL]},${KEY_ID}"
        else
          TRUSTMAP["$GITHUB_ID_EMAIL"]="$KEY_ID"
        fi
      fi
    fi
  done
done <<< "$COLLABORATORS"

echo ""

# Import GitHub's web-flow key for merge commits
echo "Importing GitHub web-flow key..."
# Security: This curl is safe because:
# 1. URL is hardcoded HTTPS (no user input)
# 2. --proto '=https' enforces HTTPS-only (no HTTP fallback)
# 3. --tlsv1.2 enforces TLS 1.2 minimum
# 4. -L follows redirects ONLY within HTTPS due to --proto restriction
# 5. GitHub.com is a trusted source for their own web-flow signing key
# nosemgrep: detected-curl-with-l-option
curl -sL --proto '=https' --tlsv1.2 https://github.com/web-flow.gpg | gpg --import 2>/dev/null && \
  echo -e "${GREEN}✓${NC} GitHub web-flow key imported" || \
  echo -e "${YELLOW}⚠${NC} Could not import GitHub web-flow key"

echo ""

# Show summary of trusted identities
echo -e "${BLUE}=== Trusted Identities ===${NC}"
# Check if TRUSTMAP has any entries (compatible with bash strict mode)
TRUSTMAP_COUNT=0
for key in "${!TRUSTMAP[@]}"; do
  TRUSTMAP_COUNT=$((TRUSTMAP_COUNT + 1))
  break
done

if [[ $TRUSTMAP_COUNT -eq 0 ]]; then
  echo -e "${YELLOW}⚠${NC}  No trusted identities configured"
  echo "  This means ALL commits will be allowed (unsigned commits OK)"
  echo "  Consider configuring GITHUB_TOKEN to fetch collaborator keys"
else
  for email in "${!TRUSTMAP[@]}"; do
    echo -e "${BLUE}ℹ${NC}  ${email} (keys: ${TRUSTMAP[$email]})"
  done
fi
echo ""

# Determine commit range
BASE_REF="${1:-${CIRCLE_BRANCH_BASE:-origin/main}}"
HEAD_REF="${2:-${CIRCLE_SHA1:-HEAD}}"

git fetch --no-tags --depth=200 origin +refs/heads/*:refs/remotes/origin/* >/dev/null 2>&1 || true

MERGE_BASE="$(git merge-base "$BASE_REF" "$HEAD_REF" 2>/dev/null || true)"
if [[ -z "$MERGE_BASE" ]]; then
  echo -e "${RED}✗${NC} ERROR: Could not compute merge-base for ${BASE_REF}..${HEAD_REF}" >&2
  exit 2
fi

echo -e "${BLUE}ℹ${NC}  Checking commit range: ${MERGE_BASE:0:8}..${HEAD_REF:0:8}"
echo ""

COMMIT_COUNT=$(git rev-list --count --no-merges "${MERGE_BASE}..${HEAD_REF}")
if [[ "$COMMIT_COUNT" -eq 0 ]]; then
  echo -e "${GREEN}✓${NC} No new commits to verify"
  exit 0
fi

echo -e "${BLUE}ℹ${NC}  Found ${COMMIT_COUNT} commit(s) to verify"
echo ""

status=0
checked=0
trusted_verified=0
untrusted_ok=0

# Verify each commit
while IFS= read -r sha; do
  checked=$((checked + 1))
  
  author_email="$(git show -s --format='%ae' "$sha")"
  author_name="$(git show -s --format='%an' "$sha")"
  sig_status="$(git show -s --format='%G?' "$sha")"
  key_id="$(git show -s --format='%GK' "$sha")"
  signer="$(git show -s --format='%GS' "$sha")"
  subject="$(git show -s --format='%s' "$sha")"
  sha_short="${sha:0:8}"
  
  # Check if author email is in trusted list
  if [[ -n "${TRUSTMAP[$author_email]:-}" ]]; then
    # TRUSTED IDENTITY - must be signed
    allowed="${TRUSTMAP[$author_email]}"
    
    if [[ "$sig_status" != "G" ]] && [[ "$sig_status" != "U" ]]; then
      echo -e "${RED}✗ FAIL${NC} ${sha_short} ${subject}"
      echo -e "    ${RED}Security violation${NC}: ${author_email} claims trusted identity"
      echo -e "    but signature status is '${sig_status}' (expected G or U)"
      echo -e "    ${RED}Possible impersonation attempt!${NC}"
      echo ""
      status=1
      continue
    fi
    
    # Verify key is in allowlist
    match=0
    IFS=',' read -ra fps <<< "$allowed"
    for fp in "${fps[@]}"; do
      if [[ "$key_id" == *"$fp"* ]]; then
        match=1
        break
      fi
    done
    
    if [[ $match -eq 0 ]]; then
      echo -e "${RED}✗ FAIL${NC} ${sha_short} ${subject}"
      echo -e "    ${RED}Key mismatch${NC}: ${author_email} signed with ${key_id}"
      echo -e "    but expected one of: {${allowed}}"
      echo ""
      status=1
    else
      trusted_verified=$((trusted_verified + 1))
      echo -e "${GREEN}✓ OK${NC}   ${sha_short} ${subject}"
      echo -e "    ${GREEN}Verified${NC}: ${author_name} <${author_email}>"
      echo -e "    Signed by: ${signer} (${key_id:0:16}...)"
      echo ""
    fi
  else
    # EXTERNAL CONTRIBUTOR - unsigned OK
    untrusted_ok=$((untrusted_ok + 1))
    
    if [[ "$sig_status" == "G" ]] || [[ "$sig_status" == "U" ]]; then
      echo -e "${GREEN}✓ OK${NC}   ${sha_short} ${subject}"
      echo -e "    ${YELLOW}External${NC}: ${author_name} <${author_email}>"
      echo -e "    Signed by: ${signer} (${key_id:0:16}...)"
      echo ""
    else
      echo -e "${GREEN}✓ OK${NC}   ${sha_short} ${subject}"
      echo -e "    ${YELLOW}External${NC}: ${author_name} <${author_email}>"
      echo -e "    ${YELLOW}Unsigned${NC} (allowed for external contributors)"
      echo ""
    fi
  fi
done < <(git rev-list --no-merges "${MERGE_BASE}..${HEAD_REF}")

echo -e "${BLUE}=== Verification Summary ===${NC}"
echo "Commits checked:         ${checked}"
echo -e "Trusted verified:        ${GREEN}${trusted_verified}${NC}"
echo -e "External contributors:   ${GREEN}${untrusted_ok}${NC}"

if [[ $status -eq 0 ]]; then
  echo ""
  echo -e "${GREEN}✓ All signature checks passed!${NC}"
  echo ""
  echo "No impersonation attempts detected."
else
  echo ""
  echo -e "${RED}✗ Signature verification FAILED!${NC}"
  echo ""
  echo "Action required:"
  echo "  - Review failed commits immediately"
  echo "  - Verify the committer's identity"
  echo "  - Do NOT merge if impersonation is suspected"
fi

exit $status
