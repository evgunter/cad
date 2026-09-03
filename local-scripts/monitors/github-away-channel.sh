#!/usr/bin/env bash
# Away-channel poller for the orchestrator (arm via the Monitor tool
# with `bash local-scripts/monitors/github-away-channel.sh`; persistent).
#
# Emits one line per event:
#   NEW ISSUE/PR #N: <title> [user]
#   COMMENT <url> [user]: <first 400 chars>
#   REACTION 👍 on watched comment <id> (<label>)
#
# COMMENT covers BOTH top-level issue/PR comments and INLINE review
# comments — two endpoints, both polled (see poll_comments).
#
# Sign-off watchlist: $CAD_SIGNOFF_WATCHLIST or the default below —
# one entry per line, "<comment-id>\t<label>". When posting a comment
# that requests a 👍 sign-off, append its id+label; the poller removes
# entries once the 👍 arrives. NOTE: reactions on top-level issue/PR
# comments and INLINE review comments live under different endpoints
# (issues/comments vs pulls/comments) — this script checks both; do
# not "simplify" that away (learned 2026-07-24).
# Expect your own comments to echo back (same account) — ignore those.
#
# COMMENT FILTERING (Ev's ask, 2026-08-11: per-comment spam is
# excessive; new-issue/PR events stay repo-wide). Because all
# orchestrators share one GitHub account, authorship cannot route —
# the ROUTING SIGNALS are the repo's own conventions instead, so the
# subscription is AUTOMATIC (no per-thread bookkeeping to forget):
#   CAD_CHANNEL_BRANCH_PREFIXES  comma-separated head-branch prefixes
#       (e.g. "lib/,mngr/cad-lib-plus"). A COMMENT on a PR passes if
#       the PR's head branch matches — your program's PRs are yours
#       by branch convention, including subagent lanes' PRs.
#   CAD_CHANNEL_SELF_TAG  your role tag as posted in comments
#       (e.g. "(LIB orchestrator)"). A COMMENT passes if its thread
#       already contains the tag (you joined the conversation — covers
#       cross-program threads and issues you filed or answered), or if
#       the comment BODY mentions the tag or one of your addresses.
# Addresses are the CANONICAL summons keywords (Ev, 2026-08-11):
#       "@ orchestrators" reaches everyone (every session includes
#       it); "@ <role>" (e.g. "@ lib", "@ m8", "@ asm") reaches one.
#       "@ <role>" is DERIVED from the tag automatically; extend with
#       CAD_CHANNEL_ADDRESSES (comma-separated) only for extras.
# BOTH CAD_CHANNEL_SELF_TAG and CAD_CHANNEL_BRANCH_PREFIXES are
# REQUIRED — arming without them is an ERROR (fail-loud; an armed
# monitor dies immediately and the arm-time notification names the
# fix), so a session cannot silently fall back to firehose mode.
# The per-program prefix registry lives in
# memories/orchestration-model.md (lib/, asm/, kernel/, mngr/<yours>).
# Thread-membership lookups are cached per issue number for CACHE_TTL
# polls; a fresh self-comment updates the cache within one TTL, so
# worst case a reply lands one cycle late, never lost.
set -u
REPO="evgunter/cad"
WATCHLIST="${CAD_SIGNOFF_WATCHLIST:-$HOME/.local/share/cad-work/signoff-watchlist.txt}"
BRANCH_PREFIXES="${CAD_CHANNEL_BRANCH_PREFIXES:-}"
SELF_TAG="${CAD_CHANNEL_SELF_TAG:-}"
if [ -z "$BRANCH_PREFIXES" ] || [ -z "$SELF_TAG" ]; then
  echo "away-channel: ERROR — CAD_CHANNEL_SELF_TAG and CAD_CHANNEL_BRANCH_PREFIXES are required at arm time (see memories/orchestration-model.md for your program's values)" >&2
  exit 78  # EX_CONFIG
fi
# "@ <role>" derived from the tag: "(LIB orchestrator)" -> "@ lib".
role=$(printf '%s' "$SELF_TAG" | tr -d '()' | awk '{print tolower($1)}')
ADDRESSES="@ orchestrators,@ ${role}${CAD_CHANNEL_ADDRESSES:+,$CAD_CHANNEL_ADDRESSES}"
CACHE_TTL="${CAD_CHANNEL_CACHE_TTL:-10}"   # polls between membership re-checks
touch "$WATCHLIST"

declare -A pr_branch_cache      # number -> head branch ("" = not a PR)
declare -A member_cache         # number -> 1/0 (thread contains SELF_TAG)
declare -A member_cache_age     # number -> poll counter at last check
poll_n=0

# Does this COMMENT event pass the filter? $1=issue/PR number, $2=body.
comment_passes() {
  local num="$1" body="$2"
  # 1. Body summons: my tag or any of my @-addresses (case-insensitive).
  if [ -n "$SELF_TAG" ] && printf '%s' "$body" | grep -qiF "$SELF_TAG"; then return 0; fi
  if [ -n "$ADDRESSES" ]; then
    local IFS=','; local a
    for a in $ADDRESSES; do
      [ -n "$a" ] && printf '%s' "$body" | grep -qiF "$a" && return 0
    done
  fi
  # 2. PR head-branch prefix match (cached forever — branches don't move).
  if [ -n "$BRANCH_PREFIXES" ]; then
    if [ -z "${pr_branch_cache[$num]+x}" ]; then
      pr_branch_cache[$num]=$(gh api "repos/$REPO/pulls/$num" --jq '.head.ref' 2>/dev/null || echo "")
    fi
    local br="${pr_branch_cache[$num]}"
    if [ -n "$br" ]; then
      local IFS=','; local p
      for p in $BRANCH_PREFIXES; do
        [ -n "$p" ] && case "$br" in "$p"*) return 0;; esac
      done
    fi
  fi
  # 3. Thread membership: the thread carries my tag — in the issue/PR
  # TITLE or BODY (filing an issue signed with your tag subscribes
  # you) or any COMMENT ("<TAG> subscribing." works). TTL-cached.
  if [ -n "$SELF_TAG" ]; then
    local age="${member_cache_age[$num]:--999}"
    if [ -z "${member_cache[$num]+x}" ] || [ $((poll_n - age)) -ge "$CACHE_TTL" ]; then
      if { gh api "repos/$REPO/issues/$num" --jq '"\(.title)\n\(.body // "")"' 2>/dev/null;
           gh api "repos/$REPO/issues/$num/comments?per_page=100" --paginate \
             --jq '.[].body' 2>/dev/null; } | grep -qiF "$SELF_TAG"; then
        member_cache[$num]=1
      else
        member_cache[$num]=0
      fi
      member_cache_age[$num]=$poll_n
    fi
    [ "${member_cache[$num]}" = "1" ] && return 0
  fi
  return 1
}

# Emit the passing comments of ONE comment feed. $1 = API path.
# Called for BOTH feeds each poll: top-level issue/PR comments
# (issues/comments) and INLINE review comments (pulls/comments).
# The two live under different endpoints — the same split the
# reaction lookup below has always had to respect. Polling only
# issues/comments silently dropped every inline comment, so a
# review left on a PR your BRANCH PREFIX owns passed every test in
# comment_passes() and still never arrived: it was never fetched
# (found 2026-08-12). Do not "simplify" this back to one feed.
poll_comments() {
  local path="$1" c_url c_user c_body c_head c_num
  while IFS=$'\t' read -r c_url c_user c_body; do
    [ -n "$c_url" ] || continue
    # SELF-SUPPRESSION (Ev, 2026-08-12): a comment that LEADS
    # with this session's own role tag is our own echo (shared
    # account — authorship cannot distinguish orchestrators; the
    # leading tag can: nobody else signs as us). Mid-body tag
    # mentions (Ev summoning us) still pass. Convention: every
    # comment we post LEADS with the tag.
    c_head=$(printf '%s' "$c_body" | sed 's/^[[:space:]*_>#-]*//' | head -c 64)
    case "$c_head" in "$SELF_TAG"*) continue;; esac
    # issue/PR number from .../issues/N#... or .../pull/N#...
    # (inline comments are .../pull/N#discussion_rNNN — same shape).
    c_num=$(printf '%s' "$c_url" | sed -n 's#.*/\(issues\|pull\)/\([0-9]*\).*#\2#p')
    if comment_passes "${c_num:-0}" "$c_body"; then
      printf 'COMMENT %s [%s]: %s\n' "$c_url" "$c_user" "$(printf '%s' "$c_body" | head -c 400 | tr '\n' ' ')"
    fi
  done < <(gh api "$path" --jq '.[] | [.html_url, .user.login, .body] | @tsv' 2>/dev/null)
}
last=$(date -u +%Y-%m-%dT%H:%M:%SZ)
seen=$(gh api "repos/$REPO/issues?state=all&per_page=30" --jq '.[].number' 2>/dev/null | sort -n | tail -1); seen=${seen:-0}
while true; do
  now=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  top=$(gh api "repos/$REPO/issues?state=all&per_page=30" --jq '.[].number' 2>/dev/null | sort -n | tail -1)
  if [ -n "$top" ] && [ "$top" -gt "$seen" ]; then
    gh api "repos/$REPO/issues?state=all&per_page=30" --jq ".[] | select(.number > $seen) | \"NEW ISSUE/PR #\(.number): \(.title) [\(.user.login)]\"" 2>/dev/null
    seen=$top
  fi
  poll_n=$((poll_n + 1))
  poll_comments "repos/$REPO/issues/comments?sort=created&direction=desc&since=$last&per_page=20"
  poll_comments "repos/$REPO/pulls/comments?sort=created&direction=desc&since=$last&per_page=20"
  if [ -s "$WATCHLIST" ]; then
    # whitespace-separated (id label) — tab OR space; a space-written
    # entry once silently broke reaction detection (2026-07-29)
    while read -r cid label; do
      [ -n "$cid" ] || continue
      r=$(gh api "repos/$REPO/issues/comments/$cid/reactions" --jq '[.[] | select(.content=="+1")] | length' 2>/dev/null)
      if [ -z "${r:-}" ]; then r=$(gh api "repos/$REPO/pulls/comments/$cid/reactions" --jq '[.[] | select(.content=="+1")] | length' 2>/dev/null); fi
      if [ "${r:-0}" -gt 0 ] 2>/dev/null; then
        echo "REACTION 👍 on watched comment $cid ($label)"
        sed -i "/^$cid[[:space:]]/d; /^$cid$/d" "$WATCHLIST"
      fi
    done < "$WATCHLIST"
  fi
  last=$now; sleep 60
done
