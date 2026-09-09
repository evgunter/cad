#!/usr/bin/env bash
# Install Ubuntu-archive packages on a hosted runner, asking apt about
# nothing else.
#
#   scripts/apt-install.sh [apt-get install flags] <package>...
#   scripts/apt-install.sh --selftest
#
# THE FAILURE THIS EXISTS FOR. `apt-get update` reports one exit status for
# every source list it was pointed at, and the runner image ships lists this
# repo never asked for. When one of those publishes an index that disagrees
# with its own Release file, `update` exits non-zero having successfully
# fetched every index the job's packages actually come from — apt says so
# itself ("They have been ignored, or old ones used instead") — and the step
# dies anyway. Every package installed through this script comes from Ubuntu's
# own archive, so the fix is to narrow what `update` is allowed to fail on
# down to what the job depends on.
#
# THE SCRUB IS SCOPED TO ONE TRANSACTION, not to the job. Foreign lists are
# moved aside before `update` and moved back on exit, whatever the exit — so a
# later step that legitimately needs a third-party list finds it exactly where
# the image left it, and this script cannot silently disarm one. What it can
# still do is refuse a package that only a foreign list carries; that is a loud
# failure, and the error names the lists it set aside so a reader sees the
# cause rather than an "Unable to locate package" with no explanation.
#
# THE RETRY IS THE SECOND, INDEPENDENT LEVER and stays: a hung fetch never
# returns, so `timeout` around each call is what makes a retry possible at all,
# and one host's bad minute must not be a whole-gate failure (the same argument
# as .github/actions/install-nextest). Narrowing the sources does not make the
# network reliable; it makes a failure mean something.

set -uo pipefail

# The image's third-party lists live here; Ubuntu's own may be in this
# directory (deb822 `ubuntu.sources`) or in /etc/apt/sources.list, which is
# never touched.
SOURCES_DIR="${CAD_APT_SOURCES_DIR:-/etc/apt/sources.list.d}"

# A source is OURS when its URIs are served by Ubuntu's own archive hosts. The
# test is anchored at the HOST for a reason: a PPA is
# `https://ppa.launchpadcontent.net/<owner>/<ppa>/ubuntu/`, so a pattern that
# looked for "ubuntu" anywhere in the URI would keep every PPA on the image.
OWN_SOURCE_RE_DEFAULT='^[a-z][a-z0-9+.-]*://([^/@]*@)?([a-z0-9-]+\.)*ubuntu\.com(:[0-9]+)?/'
OWN_SOURCE_RE="${CAD_APT_OWN_SOURCE_RE:-$OWN_SOURCE_RE_DEFAULT}"

# Extra `-o` options, used by --selftest to point apt at a scratch tree. Empty
# in every workflow invocation.
read -r -a APT_OPTS <<<"${CAD_APT_OPTS:-}"

HELD_DIR=""
HELD_NAMES=()

log() { printf 'apt-install: %s\n' "$*"; }

# `sudo` in CI, nothing when the caller already owns the directory (--selftest
# runs against a scratch tree as an ordinary user).
sudo_if_needed() {
  if [ -w "$SOURCES_DIR" ]; then
    "$@"
  else
    sudo "$@"
  fi
}

# Every URI a source file points apt at, one per line.
source_uris() {
  grep -oE '[a-z][a-z0-9+.-]*://[^ 	]+' "$1" 2>/dev/null
}

# True when every URI in the file is Ubuntu's own. A file with no URI at all
# (all comments) fetches nothing and is kept: dropping it would be noise.
is_own_source() {
  local uri
  while IFS= read -r uri; do
    [[ "$uri" =~ $OWN_SOURCE_RE ]] || return 1
  done < <(source_uris "$1")
  return 0
}

restore_sources() {
  local name
  [ -n "$HELD_DIR" ] || return 0
  for name in "${HELD_NAMES[@]}"; do
    [ -e "$HELD_DIR/$name" ] || continue
    sudo_if_needed mv "$HELD_DIR/$name" "$SOURCES_DIR/$name" \
      || log "WARNING: could not restore $SOURCES_DIR/$name"
  done
  rmdir "$HELD_DIR" 2>/dev/null || true
  HELD_DIR=""
}

set_aside_foreign_sources() {
  local path name kept=()
  [ -d "$SOURCES_DIR" ] || { log "no $SOURCES_DIR; nothing to narrow"; return 0; }
  HELD_DIR="$(mktemp -d)"
  trap restore_sources EXIT
  for path in "$SOURCES_DIR"/*; do
    [ -f "$path" ] || continue
    name="$(basename "$path")"
    if is_own_source "$path"; then
      kept+=("$name")
      continue
    fi
    if sudo_if_needed mv "$path" "$HELD_DIR/$name"; then
      HELD_NAMES+=("$name")
    else
      log "WARNING: could not set aside $path; apt will be asked about it"
    fi
  done
  log "keeping ${#kept[@]} Ubuntu source list(s): ${kept[*]:-(none)}"
  if [ "${#HELD_NAMES[@]}" -gt 0 ]; then
    log "set aside ${#HELD_NAMES[@]} non-Ubuntu source list(s) for this install," \
        "restored on exit: ${HELD_NAMES[*]}"
  fi
}

foreign_note() {
  [ "${#HELD_NAMES[@]}" -gt 0 ] || return 0
  log "the non-Ubuntu lists set aside for this install were: ${HELD_NAMES[*]}." \
      "If a package above lives on one of those, this script is the wrong door" \
      "— it installs from Ubuntu's archive only."
}

# `timeout` INSIDE the privilege change, not outside it: `timeout sudo apt-get`
# signals sudo, which need not forward it to the child, so the hang it is there
# to break can outlive the timeout.
apt_update() {
  sudo_if_needed timeout -k 10 120 apt-get update -qq "${APT_OPTS[@]}"
}

apt_install() {
  sudo_if_needed timeout -k 10 300 apt-get install -y -qq "${APT_OPTS[@]}" "$@"
}

main() {
  [ "$#" -gt 0 ] || { echo "usage: $0 <apt-get install args>" >&2; exit 2; }
  set_aside_foreign_sources
  local attempt
  for attempt in 1 2 3; do
    if apt_update; then
      if [ -n "${CAD_APT_SELFTEST_UPDATE_ONLY:-}" ]; then
        return 0
      fi
      if apt_install "$@"; then
        return 0
      fi
    fi
    echo "::warning::apt-install: attempt ${attempt} failed or hung; retrying"
    sleep $((attempt * 5))
  done
  foreign_note
  echo "::error title=apt-install failed::apt failed or hung on three attempts" \
       "against Ubuntu's archive alone. The image's third-party lists were not" \
       "in play, so this is Ubuntu's mirror or the network — re-run the lane."
  return 1
}

# --------------------------------------------------------------- --selftest
#
# THE MIRROR THAT BROKE THIS HAS SINCE RECOVERED, so the shape of the failure
# is CONSTRUCTED here rather than waited for: a change that makes `apt-get
# update` pass on a healthy mirror proves nothing about the case this script
# is about. The fixture is two `file://` repositories and a scratch
# `sources.list.d`, and apt is pointed at them with `-o Dir::*` so the selftest
# needs no root and touches no system file:
#
#   * `own.sources`   — deb822, a repository whose Release file agrees with
#                       its index. Stands in for Ubuntu's archive; the scratch
#                       run's OWN_SOURCE_RE matches it.
#   * `third-party.list` — a repository whose Release file states a SHA256 for
#                       `Packages.gz` that the file does not have. This is the
#                       exact condition the runner image's google-chrome list
#                       served: apt reports `Hash Sum mismatch`, ignores the
#                       index, and exits non-zero.
#
# What it proves: (1) unnarrowed `apt-get update` over that pair FAILS, with
# the mismatch in its output — the red run, reconstructed; (2) the same
# `update` through this script SUCCEEDS; (3) the package on the retained
# repository is still visible to apt afterwards, so narrowing did not cost the
# job its packages; (4) every set-aside file is back in place when the script
# exits. The INSTALL half is not exercised here — it needs root and a real
# archive — and does not need to be: four workflow steps install through this
# script on every run, and a scrub that broke them reds those steps.

selftest_repo() { # <dir> <package> <corrupt:0|1>
  local dir="$1" pkg="$2" corrupt="$3" d="$1/dists/stable/main/binary-amd64"
  mkdir -p "$d"
  cat >"$d/Packages" <<EOF
Package: $pkg
Version: 1.0
Architecture: amd64
Maintainer: fixture <fixture@invalid>
Filename: pool/$pkg.deb
Size: 10
SHA256: $(printf '%064d' 0)
Description: apt-install selftest fixture

EOF
  gzip -9nkf "$d/Packages"
  local hp sp hz sz
  hp="$(sha256sum "$d/Packages" | cut -d' ' -f1)"; sp="$(stat -c%s "$d/Packages")"
  hz="$(sha256sum "$d/Packages.gz" | cut -d' ' -f1)"; sz="$(stat -c%s "$d/Packages.gz")"
  # The whole point of the corrupt arm: a Release file that disagrees with the
  # index it signs for.
  [ "$corrupt" = 1 ] && hz="$(printf '%064d' 1)"
  cat >"$dir/dists/stable/Release" <<EOF
Origin: fixture
Suite: stable
Codename: stable
Architectures: amd64
Components: main
Date: $(date -Ru)
SHA256:
 $hp $sp main/binary-amd64/Packages
 $hz $sz main/binary-amd64/Packages.gz
EOF
}

selftest_main() {
  local t fails=0
  SELFTEST_TMP="$(mktemp -d)"
  t="$SELFTEST_TMP"
  trap 'rm -rf "$SELFTEST_TMP"' EXIT
  selftest_repo "$t/own" ownpkg 0
  selftest_repo "$t/third-party" chromelike 1
  mkdir -p "$t/sources.list.d" "$t/state/lists/partial" "$t/cache/archives/partial"
  cat >"$t/sources.list.d/own.sources" <<EOF
Types: deb
URIs: file://$t/own
Suites: stable
Components: main
Trusted: yes
EOF
  echo "deb [trusted=yes] file://$t/third-party stable main" \
    >"$t/sources.list.d/third-party.list"
  echo "# a comment, no URI at all" >"$t/sources.list.d/comment-only.list"

  local opts="-o Dir::Etc::sourcelist=/dev/null -o Dir::Etc::sourceparts=$t/sources.list.d"
  opts="$opts -o Dir::State::Lists=$t/state/lists -o Dir::Cache=$t/cache"
  opts="$opts -o Dir::Etc::trusted=/dev/null -o Dir::Etc::trustedparts=/dev/null"
  opts="$opts -o APT::Get::List-Cleanup=0"
  # Matches the fixture's own repository and nothing else, standing in for the
  # `*.ubuntu.com` host test the workflow invocations use.
  local own_re="^file://$t/own(/|\$)"

  check() { # <name> <expected 0|1> <actual>
    if [ "$2" = "$3" ]; then
      printf 'ok   %s\n' "$1"
    else
      printf 'FAIL %s (wanted exit %s, got %s)\n' "$1" "$2" "$3"; fails=$((fails + 1))
    fi
  }

  # (1) THE RED RUN, RECONSTRUCTED.
  local out rc
  # shellcheck disable=SC2086
  out="$(apt-get update -q $opts 2>&1)"; rc=$?
  check "unnarrowed apt-get update fails on the mismatched index" 1 "$((rc == 0 ? 0 : 1))"
  case "$out" in
    *"Hash Sum mismatch"*) printf 'ok   the failure is a hash-sum mismatch\n' ;;
    *) printf 'FAIL fixture did not produce a hash-sum mismatch:\n%s\n' "$out"; fails=$((fails + 1)) ;;
  esac

  # (2) THE SAME UPDATE, THROUGH THIS SCRIPT. The lists apt just cached are
  # discarded first, so assertions (2) and (3) rest on the narrowed run and
  # cannot pass on what the failed run left behind.
  rm -rf "$t/state/lists"
  mkdir -p "$t/state/lists/partial"
  CAD_APT_SOURCES_DIR="$t/sources.list.d" CAD_APT_OWN_SOURCE_RE="$own_re" \
    CAD_APT_OPTS="$opts" CAD_APT_SELFTEST_UPDATE_ONLY=1 \
    "$0" ownpkg >"$t/narrowed.log" 2>&1
  rc=$?
  check "narrowed apt-get update succeeds on the same pair" 0 "$rc"
  [ "$rc" = 0 ] || cat "$t/narrowed.log"

  # (3) THE PACKAGES ARE STILL THERE.
  # shellcheck disable=SC2086
  out="$(apt-cache policy ownpkg $opts 2>&1)"
  case "$out" in
    *1.0*) printf 'ok   the retained repository still offers its package\n' ;;
    *) printf 'FAIL retained package invisible after narrowing:\n%s\n' "$out"; fails=$((fails + 1)) ;;
  esac

  # (4) NOTHING WAS LEFT DISARMED.
  local missing=()
  for f in own.sources third-party.list comment-only.list; do
    [ -f "$t/sources.list.d/$f" ] || missing+=("$f")
  done
  check "every source list is restored on exit (${missing[*]:-none missing})" 0 "${#missing[@]}"

  # (5) THE CLASSIFIER, against the shapes a runner image actually ships.
  local uri want got
  # The production pattern itself, not a copy of it: these cases are the
  # reason the test is anchored at the host.
  OWN_SOURCE_RE="$OWN_SOURCE_RE_DEFAULT"
  while read -r want uri; do
    printf 'deb %s stable main\n' "$uri" >"$t/case.list"
    if is_own_source "$t/case.list"; then got=own; else got=foreign; fi
    check "classify $uri as $want" 0 "$([ "$got" = "$want" ] && echo 0 || echo 1)"
  done <<'CASES'
own http://archive.ubuntu.com/ubuntu/
own http://azure.archive.ubuntu.com/ubuntu/
own http://security.ubuntu.com/ubuntu/
foreign https://dl.google.com/linux/chrome-stable/deb/
foreign https://ppa.launchpadcontent.net/deadsnakes/ppa/ubuntu/
foreign https://download.docker.com/linux/ubuntu
foreign https://packages.microsoft.com/repos/ubuntu.com/
CASES

  if [ "$fails" -gt 0 ]; then
    printf '%s selftest assertion(s) failed\n' "$fails" >&2
    return 1
  fi
  printf 'apt-install --selftest: all assertions passed\n'
  return 0
}

if [ "${1:-}" = "--selftest" ]; then
  selftest_main
  exit $?
fi

main "$@"
