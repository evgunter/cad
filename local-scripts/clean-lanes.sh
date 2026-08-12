#!/usr/bin/env bash
# clean-lanes.sh — bulk-delete finished agent-lane dirs under the cad-work root.
# Safety model: each directory's checks and its rm -rf happen sequentially in
# the same clean_one() call — there is no global audit pass that could go stale
# between checking and deleting.
set -euo pipefail

ROOT="${CAD_WORK_ROOT:-$HOME/.local/share/cad-work}"
PROTECTED=(monitors freecad review-probes review-scratch demos demo-ideas signoff-watchlist.txt)
DRY_RUN=0
FORCE_NONGIT=0

usage() { echo "usage: clean-lanes.sh [--dry-run] [--force-nongit] DIR..." >&2; exit 2; }

# Check one git work tree. On success print its untracked-file count;
# on any hazard print the reason (with offending lines) and return 1.
repo_check() {
  local r=$1 out dirty stashes
  [[ -n $(git -C "$r" remote) ]] || { echo "repo $r: no remote configured"; return 1; }
  out=$(git -C "$r" log --branches --not --remotes --oneline 2>&1) \
    || { echo "repo $r: git log failed: $out"; return 1; }
  [[ -z $out ]] || { printf 'repo %s: unpushed commits:\n%s\n' "$r" "$out"; return 1; }
  if ! git -C "$r" symbolic-ref -q HEAD >/dev/null; then # detached HEAD isn't in --branches
    out=$(git -C "$r" log HEAD --not --remotes --oneline 2>&1) \
      || { echo "repo $r: git log (detached HEAD) failed: $out"; return 1; }
    [[ -z $out ]] || { printf 'repo %s: unpushed commits on detached HEAD:\n%s\n' "$r" "$out"; return 1; }
  fi
  out=$(git --no-optional-locks -C "$r" status --porcelain)
  dirty=$(grep -v '^??' <<<"$out" || true)
  [[ -z $dirty ]] || { printf 'repo %s: modified/staged tracked files:\n%s\n' "$r" "$dirty"; return 1; }
  stashes=$(git -C "$r" stash list)
  [[ -z $stashes ]] || { printf 'repo %s: stash entries:\n%s\n' "$r" "$stashes"; return 1; }
  grep -c '^??' <<<"$out" || true
}

# All safety checks for one lane dir, then its rm -rf, in one sequential pass.
clean_one() {
  local d=$1 rp base p out g
  rp=$(realpath -e -- "$d") || { echo "REFUSED $d: does not exist / cannot resolve" >&2; return 1; }
  [[ -d $rp ]] || { echo "REFUSED $d: not a directory ($rp)" >&2; return 1; }
  [[ $rp == "$ROOT_RP"/* ]] || { echo "REFUSED $d: $rp is not strictly inside $ROOT_RP" >&2; return 1; }
  base=${rp##*/}
  for p in "${PROTECTED[@]}"; do
    [[ $base != "$p" ]] || { echo "REFUSED $d: protected name '$base'" >&2; return 1; }
  done
  local repos untracked=0
  mapfile -t repos < <(find "$rp" -maxdepth 2 -type d -name .git)
  if [[ ${#repos[@]} -eq 0 ]]; then
    if (( FORCE_NONGIT )); then
      echo "note: $rp has no git repo; deleting due to --force-nongit"
    else
      echo "REFUSED $d: no git repo inside (pass --force-nongit to delete anyway)" >&2; return 1
    fi
  fi
  for g in "${repos[@]}"; do
    if out=$(repo_check "${g%/.git}"); then
      untracked=$((untracked + out))
    else
      printf 'REFUSED %s:\n%s\n' "$d" "$out" >&2; return 1
    fi
  done
  local size
  size=$(du -sh -- "$rp" | cut -f1)
  if (( DRY_RUN )); then
    echo "DRY-RUN: would delete $rp ($size; $untracked untracked file(s))"
  else
    rm -rf -- "$rp" # the checks above ran just now, in this same call
    echo "DELETED $rp (freed $size; $untracked untracked file(s))"
  fi
}

dirs=()
for arg in "$@"; do
  case $arg in
    --dry-run) DRY_RUN=1 ;;
    --force-nongit) FORCE_NONGIT=1 ;;
    -*) echo "unknown option: $arg" >&2; usage ;;
    *) dirs+=("$arg") ;;
  esac
done
(( ${#dirs[@]} )) || usage
ROOT_RP=$(realpath -e -- "$ROOT") || { echo "cad-work root $ROOT does not exist" >&2; exit 2; }

failures=0
for d in "${dirs[@]}"; do
  # Subshell with errexit restored, so one dir's refusal or unexpected error
  # cannot abort the loop; parent errexit is briefly off to observe the status.
  set +e
  ( set -e; clean_one "$d" )
  status=$?
  set -e
  (( status == 0 )) || failures=$((failures + 1))
done
if (( failures )); then
  echo "clean-lanes: refused $failures of ${#dirs[@]} dir(s)" >&2
  exit 1
fi
