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
# the image left it, and this script cannot silently disarm one. Two things
# that "one transaction" has to mean and would otherwise not:
#   * NOTHING IS MOVED THAT CANNOT BE MOVED BACK. `hold_path` is the only
#     producer of a destination, and it refuses unless the holding directory
#     exists and is writable — so a `mktemp -d` that failed, or a `TMPDIR`
#     that is not there, degrades to NOT NARROWING rather than to moving the
#     image's lists to `/` and reporting success.
#
#     THE INVARIANT IS NOT "guard the mktemp that bit us". It is: NO PATH
#     BUILT FROM A POSSIBLY-EMPTY VARIABLE IS EVER A `mv`, `mkdir` OR
#     redirection TARGET — an empty prefix does not fail, it silently
#     re-roots the whole operation at `/`. Written the narrow way, the guard
#     went into `set_aside_foreign_sources` and the identical shape survived
#     in `selftest_main`'s own scratch directory, in the same diff. Both are
#     guarded now; a third such variable owes the same check at its
#     assignment, and there is no degraded mode to fall back on unless the
#     caller has one.
#   * `APT::Get::List-Cleanup=0` on the update. Apt's default is to delete the
#     cached indexes of every repository not in the sources it was just run
#     over, so a narrowed update with cleanup on would leave the set-aside
#     repositories with their files back and their indexes gone — a later
#     `apt-get install <third-party pkg>` that worked on the stock image would
#     then fail. Restoring the files is not restoring the state.
#
# WHAT IT STILL REFUSES, loudly: a package only a foreign list carries. That
# failure is apt's own ("Unable to locate package"), it is NOT retried, and it
# is not dressed up as a mirror outage — the error names the lists set aside so
# the reader sees why.
#
# THE RETRY IS THE SECOND, INDEPENDENT LEVER and stays: a hung fetch never
# returns, so `timeout` around each call is what makes a retry possible at all,
# and one host's bad minute must not be a whole-gate failure (the same argument
# as .github/actions/install-nextest). Narrowing the sources does not make the
# network reliable; it makes a failure mean something.
#
# THE TWO GUARDS, and the arithmetic that has to hold between them. `timeout`
# here bounds a HUNG call so the retry can happen at all; the caller's
# `timeout-minutes` is the outer bound, so a bad run fails in minutes rather
# than tens of them. That only works if the outer bound admits at least one
# retry: one attempt is UPDATE_TIMEOUT + INSTALL_TIMEOUT = 360 s, so a step
# giving this script less than about seven minutes cannot retry a hang at all
# and the inner guard is decoration. Every caller therefore sets
# `timeout-minutes: 8`, which admits one retry of a hang and not three.
#
# ON SIGNALS, because the caller now `exec`s this script and so it is the
# process the runner signals. A cancel or a `timeout-minutes` expiry arrives
# as INT then TERM; both are trapped, the retry loop stops at the next
# boundary, and the EXIT trap restores. NOT INSTANTLY: bash defers a trap until
# the foreground child returns, so a signal during a 120 s `apt-get update`
# is handled when that call ends, not when it arrives. That is the honest
# bound — the alternative is backgrounding apt and waiting on it, which buys
# promptness and costs the exit status of the thing we are running.

set -uo pipefail

# The image's third-party lists live here; Ubuntu's own may be in this
# directory (deb822 `ubuntu.sources`) or in /etc/apt/sources.list, which is
# never touched.
SOURCES_DIR="${CAD_APT_SOURCES_DIR:-/etc/apt/sources.list.d}"

# A source is OURS when its URIs are served by Ubuntu's own archive hosts. Two
# things the pattern has to get right, both of which a looser one gets wrong:
# it is anchored at the HOST, because a PPA is
# `https://ppa.launchpadcontent.net/<owner>/<ppa>/ubuntu/` and "ubuntu"
# anywhere in the URI would keep every PPA on the image; and it requires the
# host to END there, because `archive.ubuntu.com.evil.example` otherwise reads
# as ours.
OWN_SOURCE_RE_DEFAULT='^[a-z][a-z0-9+.-]*://([^/@]*@)?([a-z0-9-]+\.)*ubuntu\.com(:[0-9]+)?/'
OWN_SOURCE_RE="${CAD_APT_OWN_SOURCE_RE:-$OWN_SOURCE_RE_DEFAULT}"

UPDATE_TIMEOUT=120
INSTALL_TIMEOUT=240

# Extra `-o` options, used by --selftest to point apt at a scratch tree. Empty
# in every workflow invocation.
read -r -a APT_OPTS <<<"${CAD_APT_OPTS:-}"

HELD_DIR=""
HELD_NAMES=()
SIGNALLED=""

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

# Every URI a source file points apt at, one per line. COMMENTS ARE STRIPPED
# FIRST: a `#`-commented `deb https://dl.google.com/…` line in an otherwise
# Ubuntu file is not a source, and reading it as one would set that file aside
# and leave `update` running over no sources at all — which exits 0 and
# installs nothing, the quietest possible wrong answer.
source_uris() {
  sed 's/#.*//' "$1" 2>/dev/null | grep -oE '[a-z][a-z0-9+.-]*://[^ 	]+'
}

# True when EVERY URI in the file is Ubuntu's own — one foreign URI makes the
# file foreign, because apt fetches all of them and any one of them can be the
# index that fails. A file with no URI at all (all comments) fetches nothing
# and is kept: dropping it would be noise.
is_own_source() {
  local uri
  while IFS= read -r uri; do
    [[ "$uri" =~ $OWN_SOURCE_RE ]] || return 1
  done < <(source_uris "$1")
  return 0
}

# THE ONLY PRODUCER OF A DESTINATION for the `mv` that sets a list aside, and
# the reason it is a function: every runtime-computed target goes through this
# check, so a future one cannot skip it. An empty or unusable `HELD_DIR` —
# `mktemp -d` refused because `TMPDIR` does not exist, or the disk is full —
# yields nothing, and the caller must then leave the file where it is. Without
# this the `mv` target degrades to `/$name`: the image's lists land in the
# root, `restore_sources` finds no holding directory and returns, and the log
# says "restored on exit" over a runner whose apt configuration is gone.
hold_path() {
  [ -n "$HELD_DIR" ] || return 1
  [ -d "$HELD_DIR" ] && [ -w "$HELD_DIR" ] || return 1
  printf '%s/%s' "$HELD_DIR" "$1"
}

restore_sources() {
  local name dest
  [ "${#HELD_NAMES[@]}" -gt 0 ] || return 0
  if [ -z "$HELD_DIR" ] || [ ! -d "$HELD_DIR" ]; then
    log "ERROR: ${#HELD_NAMES[@]} source list(s) were set aside and the holding" \
        "directory is gone. They are NOT restored: ${HELD_NAMES[*]}"
    return 1
  fi
  for name in "${HELD_NAMES[@]}"; do
    dest="$HELD_DIR/$name"
    [ -e "$dest" ] || continue
    sudo_if_needed mv "$dest" "$SOURCES_DIR/$name" \
      || log "ERROR: could not restore $SOURCES_DIR/$name"
  done
  rmdir "$HELD_DIR" 2>/dev/null || true
}

set_aside_foreign_sources() {
  local path name dest kept=()
  [ -d "$SOURCES_DIR" ] || { log "no $SOURCES_DIR; nothing to narrow"; return 0; }
  HELD_DIR="$(mktemp -d 2>/dev/null)" || HELD_DIR=""
  trap restore_sources EXIT
  for path in "$SOURCES_DIR"/*; do
    [ -f "$path" ] || continue
    name="$(basename "$path")"
    if is_own_source "$path"; then
      kept+=("$name")
      continue
    fi
    if ! dest="$(hold_path "$name")"; then
      log "WARNING: no usable holding directory (TMPDIR=${TMPDIR:-/tmp}), so" \
          "$name STAYS in place and this update is not narrowed"
      kept+=("$name")
      continue
    fi
    if sudo_if_needed mv "$path" "$dest"; then
      HELD_NAMES+=("$name")
    else
      log "WARNING: could not set aside $path; apt will be asked about it"
      kept+=("$name")
    fi
  done
  log "keeping ${#kept[@]} source list(s): ${kept[*]:-(none)}"
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

# `timeout` INSIDE the privilege change, not outside it. sudo does relay a
# TERM to its child; what it does not relay is the SIGKILL `-k` sends when the
# TERM went unanswered, which is the case `-k` exists for.
apt_update() {
  sudo_if_needed timeout -k 10 "$UPDATE_TIMEOUT" apt-get update -qq \
    -o APT::Get::List-Cleanup=0 "${APT_OPTS[@]}"
}

apt_install() {
  sudo_if_needed timeout -k 10 "$INSTALL_TIMEOUT" apt-get install -y -qq \
    "${APT_OPTS[@]}" "$@"
}

# 124 is `timeout`'s own "the command ran out of time"; 137 is a child killed
# by `-k`'s SIGKILL. Every other non-zero status is apt's, and apt's are not
# hangs.
is_timeout_status() { [ "$1" = 124 ] || [ "$1" = 137 ]; }

on_signal() {
  SIGNALLED="$1"
  log "received SIG$1; stopping after the call in flight and restoring"
}

main() {
  [ "$#" -gt 0 ] || { echo "usage: $0 <apt-get install args>" >&2; exit 2; }
  trap 'on_signal INT' INT
  trap 'on_signal TERM' TERM
  set_aside_foreign_sources
  local attempt rc
  for attempt in 1 2 3; do
    apt_update
    rc=$?
    if [ "$rc" = 0 ]; then
      if [ -n "${CAD_APT_SELFTEST_UPDATE_ONLY:-}" ]; then
        return 0
      fi
      apt_install "$@"
      rc=$?
      [ "$rc" = 0 ] && return 0
      # A PACKAGE APT CANNOT FIND IS NOT A MIRROR OUTAGE. Retrying it buys
      # three copies of the same error and 38 s, and then reports it as
      # somebody else's problem — which is the exact defect this script's own
      # item is about. Only a HUNG install is worth another attempt.
      if ! is_timeout_status "$rc"; then
        foreign_note
        log "apt-get install failed (status $rc). Its error above is the verdict."
        return "$rc"
      fi
    fi
    [ -z "$SIGNALLED" ] || break
    [ "$attempt" = 3 ] && break
    echo "::warning::apt-install: attempt ${attempt} failed or hung; retrying"
    sleep $((attempt * 5))
  done
  if [ -n "$SIGNALLED" ]; then
    log "abandoned on SIG$SIGNALLED"
    case "$SIGNALLED" in INT) return 130 ;; *) return 143 ;; esac
  fi
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
# is about.
#
# EVERY ROW BELOW IS WRITTEN AGAINST A NAMED FAILURE — the mutation it must
# red on — and the failure is stated at the row. A row that only observes the
# happy path blesses what was built instead of pinning what the script is for,
# and the property most at risk of that is "restores on ANY exit": a battery
# that only ever sees a successful run cannot see it stop being true.
#
# The fixtures are `file://` repositories and a scratch `sources.list.d`, with
# apt pointed at them by `-o Dir::*`, so nothing here needs root and nothing
# touches a system file:
#   * `own.sources`        deb822, valid. Stands in for Ubuntu's archive.
#   * `third-party.list`   one-line format, and its Release file states a
#                          SHA256 for `Packages.gz` that the file does not
#                          have — the runner image's google-chrome condition.
#   * `neighbour.sources`  deb822, valid, foreign. Two jobs: the image's real
#                          foreign lists are deb822 (`google-chrome.sources`),
#                          so a scrub restricted to `*.list` must red here;
#                          and its cached indexes are what proves the update
#                          does not evict a set-aside repository.
#   * `own-broken.sources` valid deb822 pointing at a repository with the same
#                          bad index, used ONLY by the failing-run row.

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

selftest_deb822() { # <file> <uri>
  cat >"$1" <<EOF
Types: deb
URIs: $2
Suites: stable
Components: main
Trusted: yes
EOF
}

SELFTEST_FAILS=0
selftest_check() { # <name> <want> <got>
  if [ "$2" = "$3" ]; then
    printf 'ok   %s\n' "$1"
  else
    printf 'FAIL %s (wanted %s, got %s)\n' "$1" "$2" "$3"
    SELFTEST_FAILS=$((SELFTEST_FAILS + 1))
  fi
}

selftest_main() {
  local t out rc
  # THE SAME RULE AS `hold_path`, and it is here because the first version of
  # this file obeyed it in the production half and broke it forty lines down:
  # an unguarded `mktemp -d` returns empty, `$t/own` becomes `/own`, and the
  # battery writes eighteen fixture entries to the filesystem root. Unlike the
  # production half there is no useful degraded mode — a selftest with nowhere
  # to build its repositories has nothing to assert — so this fails loudly.
  SELFTEST_TMP="$(mktemp -d 2>/dev/null)" || SELFTEST_TMP=""
  if [ -z "$SELFTEST_TMP" ] || [ ! -d "$SELFTEST_TMP" ] || [ ! -w "$SELFTEST_TMP" ]; then
    echo "apt-install --selftest: no usable scratch directory (TMPDIR=${TMPDIR:-/tmp})." \
         "Refusing to run rather than building fixtures at /." >&2
    return 2
  fi
  t="$SELFTEST_TMP"
  trap 'rm -rf "$SELFTEST_TMP"' EXIT
  selftest_repo "$t/own" ownpkg 0
  selftest_repo "$t/third-party" chromelike 1
  selftest_repo "$t/neighbour" neighbourpkg 0
  selftest_repo "$t/own-broken" ownbroken 1
  local sd="$t/sources.list.d"
  mkdir -p "$sd" "$t/state/lists/partial" "$t/cache/archives/partial" "$t/bin"
  selftest_deb822 "$sd/own.sources" "file://$t/own"
  echo "deb [trusted=yes] file://$t/third-party stable main" >"$sd/third-party.list"
  selftest_deb822 "$sd/neighbour.sources" "file://$t/neighbour"
  echo "# a comment, no URI at all" >"$sd/comment-only.list"

  # One builder, because three of the rows below need the same options over a
  # DIFFERENT sources directory and a string substitution over a path full of
  # slashes is how that goes wrong quietly.
  selftest_opts() {
    printf -- '-o Dir::Etc::sourcelist=/dev/null -o Dir::Etc::sourceparts=%s %s %s' \
      "$1" "-o Dir::State::Lists=$t/state/lists -o Dir::Cache=$t/cache" \
      "-o Dir::Etc::trusted=/dev/null -o Dir::Etc::trustedparts=/dev/null"
  }
  local opts; opts="$(selftest_opts "$sd")"
  # NOTE what is NOT here: `APT::Get::List-Cleanup=0`. Production sets it, and
  # a selftest that set it too would be the only place it was set — which is
  # how a property ends up proved of the test and not of the script.
  local own_re="^file://$t/own(/|\$)"
  local narrowed=(env CAD_APT_SOURCES_DIR="$sd" CAD_APT_OWN_SOURCE_RE="$own_re"
                  CAD_APT_OPTS="$opts")

  # ---- (1) THE RED RUN, RECONSTRUCTED. Reds if the fixture stops producing
  # the condition this whole script is about.
  # shellcheck disable=SC2086
  out="$(apt-get update -q $opts 2>&1)"; rc=$?
  selftest_check "unnarrowed apt-get update fails on the mismatched index" \
    "nonzero" "$([ "$rc" = 0 ] && echo zero || echo nonzero)"
  selftest_check "and the failure is a hash-sum mismatch" "yes" \
    "$(case "$out" in *"Hash Sum mismatch"*) echo yes ;; *) echo "no: $out" ;; esac)"

  # ---- (2) CACHE THE NEIGHBOUR'S INDEXES, for row (6). A run whose own_re
  # matches every fixture leaves all three repositories cached.
  # shellcheck disable=SC2086
  apt-get update -q $opts -o APT::Get::List-Cleanup=0 >/dev/null 2>&1
  local neighbour_before
  neighbour_before=$(find "$t/state/lists" -name '*neighbour*' | wc -l)
  selftest_check "the fixture caches the neighbour's indexes at all" "yes" \
    "$([ "$neighbour_before" -gt 0 ] && echo yes || echo no)"

  # ---- (3) THE SAME UPDATE, THROUGH THIS SCRIPT. Reds if the narrowing stops
  # working. Apt's cached lists for the BROKEN repo are discarded first so this
  # cannot pass on what the failed run left behind.
  rm -rf "$t/state/lists/"*third-party*
  "${narrowed[@]}" CAD_APT_SELFTEST_UPDATE_ONLY=1 "$0" ownpkg >"$t/narrowed.log" 2>&1
  rc=$?
  selftest_check "narrowed apt-get update succeeds on the same pair" 0 "$rc"
  [ "$rc" = 0 ] || cat "$t/narrowed.log"

  # ---- (4) THE PACKAGES ARE STILL THERE. Reds if the narrowing takes the
  # job's own archive with it.
  # shellcheck disable=SC2086
  out="$(apt-cache policy ownpkg $opts 2>&1)"
  selftest_check "the retained repository still offers its package" "yes" \
    "$(case "$out" in *1.0*) echo yes ;; *) echo "no: $out" ;; esac)"

  # ---- (5) EVERY LIST IS BACK, and the deb822 foreign one is among them.
  # Reds on: a scrub restricted to `*.list`; a restore of only the first held
  # name; a trap that never fires.
  local f missing=()
  for f in own.sources third-party.list neighbour.sources comment-only.list; do
    [ -f "$sd/$f" ] || missing+=("$f")
  done
  selftest_check "every source list is restored after a SUCCESSFUL run" "none" \
    "${missing[*]:-none}"
  selftest_check "the deb822 foreign list was set aside, not ignored" "yes" \
    "$(grep -q 'set aside .*neighbour.sources' "$t/narrowed.log" && echo yes || echo no)"
  selftest_check "the comment-only list was kept, not set aside" "yes" \
    "$(grep -q 'set aside .*comment-only' "$t/narrowed.log" && echo no || echo yes)"

  # ---- (6) THE SET-ASIDE REPOSITORY KEEPS ITS CACHED INDEXES. Reds if
  # production stops passing `APT::Get::List-Cleanup=0`: apt's default evicts
  # every repository absent from the sources it just read, so a later
  # `apt-get install <third-party pkg>` with no update of its own breaks.
  local neighbour_after
  neighbour_after=$(find "$t/state/lists" -name '*neighbour*' | wc -l)
  selftest_check "a set-aside repository's cached indexes survive the update" \
    "$neighbour_before" "$neighbour_after"

  # ---- (7) A RUN THAT FAILS STILL RESTORES, AND SAYS WHAT IT SET ASIDE.
  # This is the row the first battery did not have: every other row observes a
  # `return 0`, so "restores on ANY exit" had nothing pinning it. Reds on: the
  # EXIT trap deleted; restore moved onto the success path; `foreign_note`
  # deleted; `main` returning 0 on failure.
  local bd="$t/broken.list.d"
  mkdir -p "$bd"
  selftest_deb822 "$bd/own-broken.sources" "file://$t/own-broken"
  echo "deb [trusted=yes] file://$t/third-party stable main" >"$bd/third-party.list"
  local bopts; bopts="$(selftest_opts "$bd")"
  env CAD_APT_SOURCES_DIR="$bd" CAD_APT_OWN_SOURCE_RE="^file://$t/own-broken(/|\$)" \
    CAD_APT_OPTS="$bopts" CAD_APT_SELFTEST_UPDATE_ONLY=1 \
    "$0" ownbroken >"$t/failing.log" 2>&1
  rc=$?
  selftest_check "a narrowed update over a BROKEN own archive fails" "nonzero" \
    "$([ "$rc" = 0 ] && echo zero || echo nonzero)"
  missing=()
  for f in own-broken.sources third-party.list; do
    [ -f "$bd/$f" ] || missing+=("$f")
  done
  selftest_check "every source list is restored after a FAILING run" "none" \
    "${missing[*]:-none}"
  selftest_check "the failing run names the lists it set aside" "yes" \
    "$(grep -q 'the non-Ubuntu lists set aside' "$t/failing.log" && echo yes || echo no)"

  # ---- (8) AN UNUSABLE HOLDING DIRECTORY MOVES NOTHING. Reds on the
  # unguarded `mktemp -d` whose empty result made the `mv` target `/$name`:
  # the image's lists moved to the filesystem root, nothing restored them, and
  # the script exited 0 saying "restored on exit".
  local hd="$t/hold.list.d"
  mkdir -p "$hd"
  selftest_deb822 "$hd/own.sources" "file://$t/own"
  selftest_deb822 "$hd/neighbour.sources" "file://$t/neighbour"
  local hopts; hopts="$(selftest_opts "$hd")"
  env TMPDIR="$t/no-such-dir" CAD_APT_SOURCES_DIR="$hd" \
    CAD_APT_OWN_SOURCE_RE="$own_re" CAD_APT_OPTS="$hopts" \
    CAD_APT_SELFTEST_UPDATE_ONLY=1 "$0" ownpkg >"$t/notmp.log" 2>&1
  missing=()
  for f in own.sources neighbour.sources; do
    [ -f "$hd/$f" ] || missing+=("$f")
  done
  selftest_check "an unusable TMPDIR leaves every list where it was" "none" \
    "${missing[*]:-none}"
  selftest_check "and says so instead of claiming it narrowed" "yes" \
    "$(grep -q 'no usable holding directory' "$t/notmp.log" && echo yes || echo no)"

  # ---- (9) THE INSTALL'S ARGV, through a stub `apt-get` on PATH. Reds on:
  # `"$@"` dropped from the install; `-y` dropped; a missing package retried
  # three times and reported as a mirror outage.
  cat >"$t/bin/apt-get" <<'STUB'
#!/usr/bin/env bash
printf '%s\n' "$*" >>"$STUB_LOG"
case "$1" in
  update)  [ -n "${STUB_UPDATE_FAILS:-}" ] && [ "$(grep -c '^update' "$STUB_LOG")" -le "$STUB_UPDATE_FAILS" ] && exit 1
           exit 0 ;;
  install) exit "${STUB_INSTALL_RC:-0}" ;;
esac
exit 0
STUB
  chmod +x "$t/bin/apt-get"
  local sbd="$t/stub.list.d"
  mkdir -p "$sbd"
  selftest_deb822 "$sbd/own.sources" "file://$t/own"
  : >"$t/stub.log"
  env PATH="$t/bin:$PATH" STUB_LOG="$t/stub.log" CAD_APT_SOURCES_DIR="$sbd" \
    CAD_APT_OWN_SOURCE_RE="$own_re" "$0" --no-install-recommends alpha beta \
    >"$t/stub-run.log" 2>&1
  rc=$?
  selftest_check "a stubbed run succeeds" 0 "$rc"
  selftest_check "the install carries -y, -qq, the flag and every package" "yes" \
    "$(grep -q -- '^install -y -qq --no-install-recommends alpha beta$' "$t/stub.log" \
       && echo yes || echo "no: $(grep '^install' "$t/stub.log")")"
  selftest_check "the update passes APT::Get::List-Cleanup=0" "yes" \
    "$(grep -q -- '^update .*List-Cleanup=0' "$t/stub.log" && echo yes || echo no)"

  : >"$t/stub.log"
  env PATH="$t/bin:$PATH" STUB_LOG="$t/stub.log" STUB_UPDATE_FAILS=2 \
    CAD_APT_SOURCES_DIR="$sbd" CAD_APT_OWN_SOURCE_RE="$own_re" \
    "$0" alpha >/dev/null 2>&1
  selftest_check "a failing update is retried (three update calls, then install)" \
    "3" "$(grep -c '^update' "$t/stub.log")"

  : >"$t/stub.log"
  env PATH="$t/bin:$PATH" STUB_LOG="$t/stub.log" STUB_INSTALL_RC=100 \
    CAD_APT_SOURCES_DIR="$sbd" CAD_APT_OWN_SOURCE_RE="$own_re" \
    "$0" alpha >"$t/missing.log" 2>&1
  rc=$?
  selftest_check "a package apt cannot find fails at once, not three times" \
    "1" "$(grep -c '^install' "$t/stub.log")"
  selftest_check "and keeps apt's status rather than inventing one" 100 "$rc"
  selftest_check "and is NOT reported as a mirror outage" "yes" \
    "$(grep -q "Ubuntu's mirror or the network" "$t/missing.log" && echo no || echo yes)"

  # ---- (10) THE CLASSIFIER, against the shapes a runner image ships and the
  # two that a looser pattern gets wrong.
  local uri want got line
  OWN_SOURCE_RE="$OWN_SOURCE_RE_DEFAULT"
  while read -r want line; do
    printf 'deb %s stable main\n' "$line" >"$t/case.list"
    if is_own_source "$t/case.list"; then got=own; else got=foreign; fi
    selftest_check "classify $line" "$want" "$got"
  done <<'CASES'
own http://archive.ubuntu.com/ubuntu/
own http://azure.archive.ubuntu.com/ubuntu/
own http://security.ubuntu.com/ubuntu/
foreign https://dl.google.com/linux/chrome-stable/deb/
foreign https://ppa.launchpadcontent.net/deadsnakes/ppa/ubuntu/
foreign https://download.docker.com/linux/ubuntu
foreign https://packages.microsoft.com/repos/ubuntu.com/
foreign https://archive.ubuntu.com.evil.example/ubuntu/
CASES
  # A file with more than one URI is foreign if ANY of them is: apt fetches
  # all of them, so one bad index is enough. Reds on a classifier that keeps
  # the file when it finds one Ubuntu URI.
  printf 'deb http://archive.ubuntu.com/ubuntu/ noble main\ndeb https://dl.google.com/linux/chrome-stable/deb/ stable main\n' \
    >"$t/mixed.list"
  is_own_source "$t/mixed.list"; got=$?
  selftest_check "a mixed-URI file is foreign" "1" "$got"
  # A commented-out foreign URI is not a source. Reds on a classifier that
  # reads comments: the runner's only Ubuntu file would be set aside, `update`
  # would run over nothing, exit 0, and the install would pass on packages the
  # image already carries.
  printf '# deb https://dl.google.com/linux/chrome-stable/deb/ stable main\ndeb http://archive.ubuntu.com/ubuntu/ noble main\n' \
    >"$t/commented.list"
  is_own_source "$t/commented.list"; got=$?
  selftest_check "a commented-out foreign URI does not make a file foreign" "0" "$got"

  if [ "$SELFTEST_FAILS" -gt 0 ]; then
    printf '%s selftest assertion(s) failed\n' "$SELFTEST_FAILS" >&2
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
