#!/usr/bin/env bash
# Away-channel poller for the orchestrator (arm via the Monitor tool
# with `bash scripts/monitors/github-away-channel.sh`; persistent).
#
# Emits one line per event:
#   NEW ISSUE/PR #N: <title> [user]
#   COMMENT <url> [user]: <first 400 chars>
#   REACTION 👍 on watched comment <id> (<label>)
#
# Sign-off watchlist: $CAD_SIGNOFF_WATCHLIST or the default below —
# one entry per line, "<comment-id>\t<label>". When posting a comment
# that requests a 👍 sign-off, append its id+label; the poller removes
# entries once the 👍 arrives. NOTE: reactions on top-level issue/PR
# comments and INLINE review comments live under different endpoints
# (issues/comments vs pulls/comments) — this script checks both; do
# not "simplify" that away (learned 2026-07-24).
# Expect your own comments to echo back (same account) — ignore those.
set -u
REPO="evgunter/cad"
WATCHLIST="${CAD_SIGNOFF_WATCHLIST:-$HOME/.local/share/cad-work/signoff-watchlist.txt}"
touch "$WATCHLIST"
last=$(date -u +%Y-%m-%dT%H:%M:%SZ)
seen=$(gh api "repos/$REPO/issues?state=all&per_page=30" --jq '.[].number' 2>/dev/null | sort -n | tail -1); seen=${seen:-0}
while true; do
  now=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  top=$(gh api "repos/$REPO/issues?state=all&per_page=30" --jq '.[].number' 2>/dev/null | sort -n | tail -1)
  if [ -n "$top" ] && [ "$top" -gt "$seen" ]; then
    gh api "repos/$REPO/issues?state=all&per_page=30" --jq ".[] | select(.number > $seen) | \"NEW ISSUE/PR #\(.number): \(.title) [\(.user.login)]\"" 2>/dev/null
    seen=$top
  fi
  gh api "repos/$REPO/issues/comments?sort=created&direction=desc&since=$last&per_page=20" --jq '.[] | "COMMENT \(.html_url) [\(.user.login)]: \(.body[0:400] | gsub("\n"; " · "))"' 2>/dev/null
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
