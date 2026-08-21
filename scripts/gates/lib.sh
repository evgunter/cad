#!/usr/bin/env bash
# lib.sh — shared plumbing for the mirrored discipline gates.
#
# THE INVARIANT: every gate in this directory has exactly ONE home, and
# both halves of CI call it — `.github/workflows/ci.yml`'s `discipline`
# job (one step per gate, keeping the step name the Actions UI shows)
# and `local-scripts/ci-local.sh`'s `discipline` row. A gate implemented
# twice drifts: the dual-maintained allowlists produced live drift in
# BOTH directions (a `separation.rs` entry hosted-only, a
# `test_support.rs` paragraph stale locally, a `chart_region.rs` entry
# hosted-only before that), and two gates existed hosted-only with no
# local mirror at all.
#
# Sharing the BODIES is only half of that. The two halves still each
# name which gates to run, so `gate-roster.sh` closes the other half:
# it derives the roster from this directory and fails if either half
# runs a different set. Between them, neither the gate logic nor the
# gate list is maintained twice.
#
# WHY `scripts/` AND NOT `local-scripts/`: every workflow job runs
# `rm -rf local-scripts` right after checkout, so hosted CI cannot read
# anything there. `scripts/**` is also unscopable to a workspace member,
# so `scripts/ci-filter.py` classifies a change here as TIER=all — a
# gate edit re-runs everything, which is the conservative answer.
#
# Each gate script takes `--root DIR` (the tree to scan; default is this
# repo) and `--selftest` (assert the gate passes a clean fixture and
# fires on a planted one, then exit). Both halves run `--selftest`
# before the real pass, the way the sibling python gates do.

# Repo root, derived from this file's location (scripts/gates/lib.sh).
GATE_REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
GATE_ROOT=$GATE_REPO_ROOT
GATE_SELFTEST=false
GATE_SCAN_FILES=0
# Filled in by gate_require_crate_sources, so a gate reads the same file
# set it counted rather than re-deriving one.
GATE_SOURCE_FILES=()
# Extra argv the self-test harness passes to each real invocation. A gate
# with more than one MODE needs it: the mode has to reach the subprocess
# through the command line, because setting a global here no longer
# reaches the gate — which is the point of running it as a subprocess.
GATE_SELFTEST_ARGS=()

gate_parse_args() {
  while [ $# -gt 0 ]; do
    case "$1" in
      --selftest) GATE_SELFTEST=true ;;
      --root) GATE_ROOT=$2; shift ;;
      *) printf 'usage: %s [--selftest] [--root DIR]\n' "$0" >&2; exit 2 ;;
    esac
    shift
  done
}

# One message text serves both halves: hosted CI wants the `::error::`
# annotation (it surfaces in the Actions UI against the failing step), a
# local run wants the plain form.
gate_error() {
  if [ -n "${GITHUB_ACTIONS:-}" ]; then
    printf '::error::%s\n' "$*"
  else
    printf 'ERROR: %s\n' "$*"
  fi
}

gate_name() { basename "$0" .sh; }

# A GATE THAT SCANNED NOTHING IS NOT A PASS. `crates/*/src` is a glob:
# with no match bash hands the literal to grep, grep finds nothing, and
# the gate reports green for the wrong reason — green because it looked
# at an empty tree, not because the tree is clean. `--root` makes that
# reachable, so the scan target is proven before every scan.
gate_require_crate_sources() {
  local dirs=(crates/*/src)
  if [ ! -d "${dirs[0]}" ]; then
    gate_error "$(gate_name): no crates/*/src under $PWD — the gate scanned nothing, which is not a pass"
    exit 1
  fi
  mapfile -t GATE_SOURCE_FILES < <(find crates/*/src -type f -name '*.rs' | sort)
  GATE_SCAN_FILES=${#GATE_SOURCE_FILES[@]}
  if [ "$GATE_SCAN_FILES" -eq 0 ]; then
    gate_error "$(gate_name): no .rs files under crates/*/src in $PWD — the gate scanned nothing, which is not a pass"
    exit 1
  fi
}

# Same rule for a gate whose subject is one named file. Without this a
# renamed subject makes the gate pass green forever (see
# bit-identity-debug-only.sh's header for the live instance).
gate_require_file() {
  if [ ! -f "$1" ]; then
    gate_error "$(gate_name): $1 does not exist under $PWD — the gate's subject is gone, so it cannot decide; move the gate with the file or retire it deliberately"
    exit 1
  fi
  GATE_SCAN_FILES=1
}

# Gates say what they proved, like their sibling
# `scripts/check-interval-cfg-additive.py`.
#
# GATE_SCAN_NOUN names what was counted. Most gates scan `crates/*/src`
# and inherit the default; a gate whose subject is something else sets
# it, so the count it prints says what it actually looked at.
: "${GATE_SCAN_NOUN:=source file}"
gate_ok() {
  local plural=s
  [ "$GATE_SCAN_FILES" = 1 ] && plural=
  printf '%s OK: %s (%s %s%s scanned)\n' \
    "$(gate_name)" "$1" "$GATE_SCAN_FILES" "$GATE_SCAN_NOUN" "$plural"
}

# --- THE SHARED RUST READER -------------------------------------------
#
# WHY THIS IS HERE. Six grep gates in this directory each carried the
# same one-line comment strip -- `grep -vE ':[0-9]+:\s*(//|///|//!)'` --
# and it is leading-`//` only, so it is wrong in BOTH directions:
#
#   * CRY WOLF. A trailing comment is not stripped, so a line of prose
#     naming the forbidden spelling fires the gate. That is not
#     hypothetical: `interval-square-allowlist.sh`'s `linalg/mat.rs`
#     entry is justified in writing partly by a false positive, and a
#     false red is a nudge toward the allowlist rather than the fix.
#   * GO BLIND. A block comment is stripped on no line at all, so the
#     matcher reads commented-out prose as code -- and, in the other
#     direction, a violation written after `/* ... */` on one line is
#     read as a comment by nothing and as code by the matcher, while
#     prose inside a `/* ... */` block reds the gate forever.
#
# STRING LITERALS are the third direction and the one that decides the
# interface. A blanket strip that also removed literals would make a
# gate whose needle IS a string literal vacuous (S117 sorts eleven such
# guards three ways: code-only, comments-only, and the inverse). Nothing
# under `scripts/gates/` greps for a Rust string literal today -- every
# needle here is a bound, a call or an operator -- so this reader builds
# the CODE-ONLY view and only that one. The other two views have no
# caller in this directory and are not written here; S117 is the row
# that needs them.
#
# gate_rust_code [--skip-cfg-test] FILE... emits `FILE:LINE:CODE`, the
# same shape `grep -rn` emits, so a gate's downstream pipeline (its
# `cut -d: -f1`, its allowlist filters, its message) is unchanged by the
# swap. Lines whose code part is empty are dropped; line numbers are the
# real ones.
#
# WHAT IT CANNOT DO. It is a lexer, not a parser: it knows `//`, `/* */`
# (nesting NOT handled -- Rust allows nested block comments and the
# first `*/` closes here), `"..."`, `r#"..."#`, `b"..."`, and char
# literals as distinct from lifetimes. It does not know `macro_rules!`
# bodies, `include!`d text, or code behind `#[cfg]` other than the
# `test` skip below.
gate_rust_code() {
  local skip_cfg_test=0
  while [ $# -gt 0 ]; do
    case "$1" in
      --skip-cfg-test) skip_cfg_test=1; shift ;;
      *) break ;;
    esac
  done
  [ $# -gt 0 ] || return 0
  awk -v SKIPTEST="$skip_cfg_test" '
    # A single quote cannot be written inside this program, which is
    # itself single-quoted; CODEBRK is built rather than spelled.
    BEGIN { Q = sprintf("%c", 39); CODEBRK = "[\"" Q "/]" }
    FNR == 1 { state = 0; depth = 0; skipping = 0; seen_open = 0 }
    {
      s = $0; out = ""; i = 1; n = length(s)
      # THE FAST PATH, and it is worth its four lines: a line carrying no
      # quote, no apostrophe and no block-comment opener has nothing in
      # it but code and possibly a trailing `//`. That is most of the
      # tree, and taking it without entering the scan below is the
      # difference between ~14 s and ~4 s over crates/*/src.
      if (state == 0 && index(s, "\"") == 0 && index(s, Q) == 0 &&
          index(s, "/*") == 0) {
        cpos = index(s, "//")
        out = (cpos > 0) ? substr(s, 1, cpos - 1) : s
        i = n + 1
      }
      # CHUNK JUMPS, not a character loop. Only `"`, `\x27` and `/` can
      # start a comment or a literal, so the scan copies the run of
      # ordinary code between them in one step. A per-character loop over
      # crates/*/src measured 43 s; this measures ~4 s, and CI runs it
      # once per gate.
      while (i <= n) {
        rest = substr(s, i)
        if (state == 1) {                      # inside /* ... */
          p = index(rest, "*/")
          if (p == 0) { i = n + 1 } else { state = 0; i += p + 1 }
          continue
        }
        if (state == 2) {                      # inside "..." (or b"...")
          p = match(rest, /["\\]/)
          if (p == 0) { i = n + 1; continue }
          if (substr(rest, p, 1) == "\\") { i += p + 1; continue }
          out = out "\""; state = 0; i += p
          continue
        }
        if (state == 3) {                      # inside r#*"..."#*
          p = index(rest, "\"" rawhashes)
          if (p == 0) { i = n + 1; continue }
          out = out "\"" rawhashes; state = 0; i += p + rawh
          continue
        }
        p = match(rest, CODEBRK)
        if (p == 0) { out = out rest; i = n + 1; continue }
        out = out substr(rest, 1, p - 1); i += p - 1
        c = substr(s, i, 1)
        if (c == "/") {
          two = substr(s, i, 2)
          if (two == "//") { i = n + 1; continue }
          if (two == "/*") { state = 1; i += 2; continue }
          out = out "/"; i++
          continue
        }
        if (c == "\"") {
          # RAW AND BYTE PREFIXES ARE READ BACKWARDS from the quote,
          # because the scan above jumps to the quote and never sees the
          # `r`/`br`/`#` that came before it. Getting this wrong makes a
          # `r#"..."#` containing a quote desynchronise the whole file.
          k = i - 1; rawh = 0
          while (k >= 1 && substr(s, k, 1) == "#") { rawh++; k-- }
          israw = 0
          if (k >= 1 && substr(s, k, 1) == "r") {
            pk = k - 1
            if (pk >= 1 && substr(s, pk, 1) == "b") pk--
            if (pk < 1 || substr(s, pk, 1) !~ /[A-Za-z0-9_]/) israw = 1
          }
          out = out "\""
          if (israw == 1) { rawhashes = substr(s, i - rawh, rawh); state = 3 }
          else { rawh = 0; state = 2 }
          i++
          continue
        }
        # A char literal, or a lifetime/label. A quote-delimited single
        # character (escaped or not) is a literal; a quote followed by an
        # identifier and NOT closed is a lifetime, and reading that as a
        # literal is what corrupts a naive stripper for the whole rest of
        # the file.
        if (substr(s, i + 1, 1) == "\\") {
          j = i + 3          # the backslash escapes exactly one char
          while (j <= n && substr(s, j, 1) != Q) j++
          out = out Q Q; i = j + 1; continue
        }
        if (substr(s, i + 2, 1) == Q) { out = out Q Q; i += 3; continue }
        out = out Q; i++
      }

      # `#[cfg(test)]` items, dropped as whole brace-balanced blocks when
      # the caller asks. `not(test)` is deliberately NOT matched: a gate
      # scanning code it should have skipped only cries wolf, while one
      # skipping code it should have scanned goes blind.
      # BRACE COUNTING IS PAID FOR ONLY BY THE CALLER THAT ASKED. `gsub`
      # over every line of crates/*/src costs ~8 s on its own, so the
      # five gates that do not skip test modules never run it.
      if (SKIPTEST == 1) {
        opens = 0; closes = 0
        if (index(out, "{") > 0) opens = gsub(/\{/, "{", out)
        if (index(out, "}") > 0) closes = gsub(/\}/, "}", out)
        if (skipping == 0 && out ~ /#\[cfg\(([^]]*[(,][[:space:]]*)?test[,)]/) {
          skipping = 1; seen_open = 0; skip_depth = depth
        }
        if (skipping == 1) {
          if (opens > 0) seen_open = 1
          depth += opens - closes
          if (seen_open == 1 && depth <= skip_depth) skipping = 0
          else if (seen_open == 0 && index(out, ";") > 0) skipping = 0
          next
        }
        depth += opens - closes
      }
      if (out != "") print FILENAME ":" FNR ":" out
    }
  ' "$@"
}

# The clean fixture every self-test starts from. A gate whose subject is
# not `crates/*/src` overrides this.
gate_plant_clean() {
  mkdir -p "$1/crates/clean/src"
  printf 'pub fn identity(x: f64) -> f64 { x }\n' > "$1/crates/clean/src/lib.rs"
}

# --- THE SELF-TEST HARNESS --------------------------------------------
#
# EVERY CASE RUNS THE GATE AS A REAL SUBPROCESS, and that is the whole
# design. The harness used to run the gate as `if out=$(… gate …)` in
# this process — and bash SUPPRESSES errexit inside an `if` condition,
# which is exactly the condition under which a `set -euo pipefail` gate
# dies at a failing matcher pipeline BEFORE printing its own diagnosis.
# So the old harness passed a gate whose message never prints on a real
# run, and hosted CI reported the failure the gate was written to
# explain as a bare `Process completed with exit code 1`. Fifteen
# self-tests passed and none of them could observe it.
#
# Running the gate the way CI runs it — a subprocess, through `--root` —
# makes a diagnosis lost to errexit FAIL the self-test instead. Written
# by lane F-f in `gate-roster.sh` to be lifted here; lifted with one
# change, which is that it replaced `gate_selftest_case` rather than
# sitting beside it. A second helper would have left the blind path in
# place for the other thirteen gates, which is the finding, not the fix.
#
# gate_selftest_assert_diagnosed is the second half and it is not
# cosmetic: `$want` alone can be satisfied by a gate that PRINTS its hit
# lines and then dies before `gate_error`, if the wanted text appears in
# the hits. Requiring the `gate_error` framing means the case is
# satisfied by the diagnosis and not by an echo. Both spellings are
# accepted because both are real: hosted CI sets GITHUB_ACTIONS and gets
# `::error::`, a local run gets `ERROR: `, and CI runs the self-test on
# both halves, so each half exercises its own form.
gate_selftest_assert_diagnosed() {
  case "$2" in
    *"ERROR: "*|*"::error::"*) return 0 ;;
  esac
  printf 'SELFTEST FAILED (%s): the gate exited non-zero WITHOUT a gate_error diagnosis — a matcher that dies under `set -e` before its message looks exactly like this on CI:\n%s\n' \
    "$1" "$2" >&2
  exit 1
}

# gate_selftest_clean — the NEGATIVE CONTROL. Without it a positive
# result proves nothing: a gate that fires on everything would pass a
# plant-only self-test.
gate_selftest_clean() {
  local tmp out
  # THE HARNESS'S OWN TAIL, proved on every gate rather than in a
  # comment: a `--root` the gate cannot enter used to kill it at the
  # `cd`, before it could name itself or say what it had not decided.
  # Every gate calls this function, so every gate carries the case.
  out=$("$0" --root "/nonexistent-gate-root-$$" ${GATE_SELFTEST_ARGS[@]+"${GATE_SELFTEST_ARGS[@]}"} 2>&1) && out=
  case "$out" in
    *"cannot enter --root"*) ;;
    *) printf 'SELFTEST FAILED: an unreadable --root did not produce a gate diagnosis; a gate that cannot reach its tree must say so:\n%s\n' "$out" >&2
       exit 1 ;;
  esac
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  if ! out=$("$0" --root "$tmp" ${GATE_SELFTEST_ARGS[@]+"${GATE_SELFTEST_ARGS[@]}"} 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the gate FAILED on a clean fixture\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
}

# gate_selftest_case WANT PLANTER [ARGS...] — one positive case: the
# clean fixture plus whatever PLANTER writes must FAIL, with a
# gate_error diagnosis containing WANT. The gate body is unparameterised
# apart from its root, so every case exercises the real matcher, the
# scan-target guard, and the diagnostic path.
gate_selftest_case() {
  local want=$1; shift
  local tmp out
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  "$@" "$tmp"
  if out=$("$0" --root "$tmp" ${GATE_SELFTEST_ARGS[@]+"${GATE_SELFTEST_ARGS[@]}"} 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the gate PASSED on a planted violation (%s)\n%s\n' "$1" "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
  gate_selftest_assert_diagnosed "$1" "$out"
  case "$out" in
    *"$want"*) ;;
    *) printf 'SELFTEST FAILED (%s): the gate fired with an unexpected message:\n%s\n' "$1" "$out" >&2
       exit 1 ;;
  esac
}

# gate_selftest_passes WHAT PLANTER [ARGS...] — gate_selftest_case's
# NEAR-MISS twin, and the case that keeps a widening honest. The only
# passing fixture the harness had was the empty clean tree, which proves
# nothing about a spelling that must not fire; every widened matcher in
# this directory needs a fixture saying where it stops. Reported by lane
# F-e, which wrote a gate-local copy (`bounds_selftest_passes`) rather
# than reach into this file.
gate_selftest_passes() {
  local what=$1; shift
  local tmp out
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  "$@" "$tmp"
  if ! out=$("$0" --root "$tmp" ${GATE_SELFTEST_ARGS[@]+"${GATE_SELFTEST_ARGS[@]}"} 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the gate FIRED on %s, which is not a violation\n%s\n' "$what" "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
}

# The default self-test: one clean fixture, one planted violation. A
# gate with more than one known evasion overrides this and calls
# gate_selftest_case once per case.
gate_selftest() {
  gate_selftest_clean
  gate_selftest_case "$@"
  printf '%s selftest OK: passes a clean fixture, fires on a planted violation\n' "$(gate_name)"
}

# The common tail: --selftest runs both fixtures and exits; otherwise
# the gate runs against GATE_ROOT.
#
# THE `cd` IS CHECKED, and it is the harness's own instance of the class
# this file exists to catch: an unreadable `--root` made `cd` fail under
# errexit and killed the gate at this line, so a reader got bash's
# one-line `cd:` complaint and no gate_error, no gate name, and no
# statement of what was not decided.
gate_main() {
  if [ "$GATE_SELFTEST" = true ]; then
    gate_selftest "$@"
    exit 0
  fi
  if ! cd "$GATE_ROOT" 2>/dev/null; then
    gate_error "$(gate_name): cannot enter --root $GATE_ROOT from $PWD, so the gate scanned nothing — which is not a pass"
    exit 1
  fi
  gate
}
