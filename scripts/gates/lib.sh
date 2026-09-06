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

# The marker `gate_grep` leaves behind when a matcher could not run.
# Named from `$$`, which is the GATE's pid inside a pipeline stage and
# inside a process substitution too — so every subshell of one run names
# the same file, and two gates running at once never share one.
GATE_MATCHER_FAILED=${TMPDIR:-/tmp}/gate-matcher-failed.$$

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
#
# STDERR, NOT STDOUT, and it is not a style choice. A gate whose stdout
# is consumed — `probe-suite-census.sh` emits a crate list and a suite
# list that CI reads — has one diagnosis path and one data path, and
# writing the diagnosis into the data corrupts the data. Worse, a
# `gate_error` inside a command substitution had its message CAPTURED
# and thrown away: the caller then died at the failed assignment with
# nothing on screen, which is S157 wearing different clothes.
gate_error() {
  if [ -n "${GITHUB_ACTIONS:-}" ]; then
    printf '::error::%s\n' "$*" >&2
  else
    printf 'ERROR: %s\n' "$*" >&2
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

# A MATCHER THAT DID NOT RUN IS NOT A CLEAN SCAN — the rule above,
# applied to the scan instead of to the file set. That asymmetry was the
# hole: two guards proved the gate had files to read, and nothing proved
# the reading happened.
#
# `grep` already separates the two cases. Exit 1 is "I searched and
# nothing matched"; exit 2 is "I could not search" — a malformed
# pattern, an unreadable file, a missing `-f` list — and a shell that
# cannot find `grep` at all reports 127. Every gate here sets
# `pipefail`, so all of those reach the pipeline through the same
# channel, and the trailing `|| true` that an exclusion filter
# legitimately needs for exit 1 swallowed the rest along with it. The
# gate then printed `OK: … (337 source files scanned)` having matched
# nothing, and the reassuring count was REAL, because a different guard
# produced it.
#
# `pipefail` cannot draw the distinction either, which is why the fix is
# not "drop the `|| true` and let pipefail speak". It reports the
# RIGHTMOST non-zero stage; a matcher that died fed nothing downstream,
# so the exclusion filter exits 1 on empty input and 1 is what the
# pipeline reports — the exact status a clean scan produces.
#
# So the distinction is drawn per stage, here, and the `|| true` goes
# away with it: exit 1 becomes exit 0 (the scan ran, nothing survived),
# anything else is diagnosed and ends the gate. A scanning pipeline
# writes `gate_grep` everywhere it wrote `grep`.
#
# NOT FOR `grep -q` USED AS A PREDICATE. `gate_require_*`'s callers ask
# `if ! grep -qxF …` and mean exit 1 as the answer; folding it to 0 here
# would invert them. Those spellings fail RED on an unsearchable subject
# (a misdiagnosed red, not a green), so they are left alone.
gate_grep() {
  local status=0
  grep "$@" || status=$?
  # An `if`, not `[ … ] && return 0`: as a bare `&&` list a false test
  # is a failed statement, and errexit would leave this function by the
  # one path that skips the diagnosis below. That is the defect this
  # helper exists to close, re-minted inside it.
  if [ "$status" -le 1 ]; then
    return 0
  fi
  # The call is echoed so the diagnosis names the matcher that died, but
  # TRIMMED: a gate hands `grep` a few hundred file operands, and a
  # diagnosis that buries its own first line under them is the reason a
  # reader scrolls past it.
  local shown="$*"
  if [ "${#shown}" -gt 160 ]; then
    shown="${shown:0:160}... (arguments trimmed)"
  fi
  gate_error "$(gate_name): grep exited $status, which is not \"no match\" (exit 1) — it is grep saying it could not search, so the scan below it decided nothing. Call: grep $shown"
  # THE EXIT STATUS ALONE IS NOT ENOUGH, and this file is where that is
  # already known: a stage inside `< <(…)` feeding `mapfile` or a `while
  # read` cannot fail its caller, because a process substitution's
  # status is not the reader's. The marker crosses the boundary the
  # status cannot, and `gate_ok` — the single place a gate says green —
  # refuses to print over it.
  : >> "$GATE_MATCHER_FAILED"
  exit "$status"
}

# Subshells INHERIT this trap, so the marker may only be removed by the
# gate itself: a `gate_grep` exiting inside a pipeline stage would
# otherwise delete its own evidence on the way out.
gate_matcher_marker_cleanup() {
  if [ "${BASHPID:-$$}" = "$$" ]; then
    rm -f "$GATE_MATCHER_FAILED"
  fi
  return 0
}
trap gate_matcher_marker_cleanup EXIT

# Gates say what they proved, like their sibling
# `scripts/check-interval-cfg-additive.py`.
#
# GATE_SCAN_NOUN names what was counted. Most gates scan `crates/*/src`
# and inherit the default; a gate whose subject is something else sets
# it, so the count it prints says what it actually looked at.
: "${GATE_SCAN_NOUN:=source file}"
gate_ok() {
  # THE CHOKE POINT. Every gate in this directory ends here, so this is
  # the one line that has to hold for "a gate reports green only for
  # what it actually scanned" to be a property of the directory rather
  # than of each call site's luck.
  if [ -e "$GATE_MATCHER_FAILED" ]; then
    rm -f "$GATE_MATCHER_FAILED"
    gate_error "$(gate_name): a matcher failed to run during this pass (diagnosed above), so what it did not match is unknown — that is not a pass"
    exit 1
  fi
  local plural=s
  [ "$GATE_SCAN_FILES" = 1 ] && plural=
  printf '%s OK: %s (%s %s%s scanned)\n' \
    "$(gate_name)" "$1" "$GATE_SCAN_FILES" "$GATE_SCAN_NOUN" "$plural"
}


# --- THE TEST-ONLY `cfg` ATTRIBUTE ------------------------------------
#
# ONE SPELLING, read by the reader's `--skip-cfg-test` predicate and by
# the module resolver below it. Both ask the same question — is this
# item compiled ONLY under `cfg(test)` — and asking it in two dialects
# is how a gate comes to skip an item its neighbour scans.
#
# ONLY A TEST-ONLY ATTRIBUTE COUNTS: `test` alone or inside an `all(…)`.
# `not(test)` marks the most production code there is, and `any(test, …)`
# marks an item that also exists under the other condition — in this tree
# the topo crate has a `test_support_impl` module gated on
# `any(debug_assertions, test, feature = "test-support")`, which is every
# debug build. Dropping either would be blind in the one direction that
# matters; scanning an item that could have been skipped only cries wolf.
# The second pattern is a SUBTRACTION from the first, not an alternative
# reading of it: a caller matches the first and refuses the second.
#
# THEY REACH `awk` THROUGH `ENVIRON`, NEVER `-v`. A `-v` assignment is
# processed for escape sequences before it is a regex, so `\(` in one is
# an unknown escape the hosted runner's `awk` warns about and a value it
# may hand on changed; `ENVIRON` carries the bytes.
GATE_CFG_TEST_RE='#\[cfg\(([^]]*[(,][[:space:]]*)?test[,)]'
GATE_CFG_TEST_NOT_RE='#\[cfg\([^]]*(any|not)\('

# --- THE SHARED RUST READER -------------------------------------------
#
# WHY THIS IS HERE. Every grep gate in this directory carried its own
# copy of the same one-line comment strip -- `grep -vE
# ':[0-9]+:\s*(//|///|//!)'` -- and it is leading-`//` only, so it is
# wrong in BOTH directions:
#
#   * CRY WOLF. A trailing comment is not stripped, so a line of prose
#     naming the forbidden spelling fires the gate. That is not
#     hypothetical: an `interval-square-allowlist.sh` entry was
#     justified in writing partly by a false positive, and a false red
#     is a nudge toward the allowlist rather than the fix. (That entry
#     — `linalg/mat.rs` — has since been discharged by converting the
#     site, so the example no longer resolves in that file; the
#     mechanism it names is why this reader exists.)
#   * GO BLIND. A block comment is stripped on no line at all, so the
#     matcher reads commented-out prose as code -- and, in the other
#     direction, a violation written after `/* ... */` on one line is
#     read as a comment by nothing and as code by the matcher, while
#     prose inside a `/* ... */` block reds the gate forever.
#
# STRING LITERALS are the third direction and the one that decides the
# interface. A blanket strip that also removed literals would make a
# gate whose needle IS a string literal vacuous. S117 sorts the guards
# that read source text by which of `test_utils::source`'s three views
# their needle wants — `code_only`, `code_and_literals` (comments
# stripped, literals KEPT) and `comments_only` (the inverse view: the
# needle is prose) — and this reader builds TWO of them: the CODE-ONLY
# view by default, because that is what most gates converted to it need
# (their needles are bounds, calls and operators), and
# `code_and_literals` under `--keep-literals` for a needle that contains
# a literal. WHICH gates take which is DERIVED, not copied —
# `grep -l gate_rust_code scripts/gates/*.sh | grep -v /lib.sh`,
# and the exclusion is part of the derivation rather than a subtraction
# left to the reader: this file names the function because it DEFINES
# it. For the same reason the paragraph below gives.
#
# THAT POPULATION HAS ONE HOME, AND IT IS NOT THIS COMMENT.
# `crates/test-utils/tests/reader_census.rs` carries one ledger line
# per site in the tree that reads Rust source as text, naming the view
# each takes, and a second quantity beside it — the sites still reading
# through something other than a shared lexer. Both live there and
# nowhere else: a count copied into prose goes stale in the silent
# direction, because the population grows by a reader arriving.
#
# `--keep-literals` IS ONE VIEW OF THE THREE, NOT A FOURTH. A needle
# whose own text contains a literal — `#[cfg(feature = "probe")]`, say —
# is looking for `feature = ""` under the code-only view and matches
# every cfg gate in the tree, so that view is not merely unhelpful for
# it but wrong. WHICH gates take this view is derived the way the
# paragraph above derives the rest, and is not named here. What keeping
# literals costs is stated where the needle is, because it is a property
# of the needle: a literal SPELLING the needle is indistinguishable from
# the code, and only a raw literal can spell one whose own quotes are
# unescaped.
#
# `--keep-literals` WITH `--skip-cfg-test` IS NOT REFUSED AND MEANS
# SOMETHING PARTICULAR, so it is written down rather than left to be
# discovered. The test-skip predicate and its brace counting run over
# the RECORD, so with literals kept a string spelling `#[cfg(test)]`
# opens a skip and braces inside literals count toward closing it — a
# skip that starts and ends in the wrong places. No caller combines
# them today. A caller that wants both owes the skip a literal-blind
# view of the same line, which is a change to this function and not a
# flag on it.
#
# THE THIRD VIEW IS NOT BUILT HERE. `comments_only` — the needle is
# prose — is the inverse selection, and its one caller in this
# directory (the census's `^//!` disposition sentences) reads the raw
# file for it. Building it would mean emitting what this scanner
# discards, which is a second traversal of the same lexer states rather
# than a flag on this one; the row that wants it is the row that adds
# a second such caller.
#
# THREE RECORD SHAPES, one lexer, because two hand-rolled Rust readers
# under `scripts/gates/` is how the leading-`//` strip came to be
# copied into every grep gate in this directory:
#
#   (default)      one record per CODE line
#   --statements   one record per STATEMENT, cut at `{`, `}` and `;`,
#                  whitespace collapsed. `rustfmt` wraps a long bound
#                  list as `T: Real\n    + PartialOrd,`, so a matcher
#                  anchored on a LINE is blind to the form the formatter
#                  converges on (S158's ruling). `{}`/`;` is where a
#                  generic list and its `where` clause end, so the
#                  statement is the unit those matchers actually mean.
#   --window N     one record per CODE line, joined with the next N-1 of
#                  them, whitespace collapsed — for a needle that spans
#                  a construct rather than ending at a delimiter
#                  (`signed-zero-one-home.sh`'s `== 0.0 { 0.0 }`).
#
# A LINE THAT CARRIES NO CODE IS NOT A RECORD, in any of the three, and
# a line holding only a comment is such a line: what survives the strip
# is its own indentation. Emitted, it would start a window over the code
# BELOW it and report that site twice, once at a line that holds none of
# it — so a matcher counting sites counts one too many, one line early.
# A blank line and a comment-only line are the same line to every needle
# here, and the reader says so.
#
# All three emit `FILE:LINE:TEXT`, the shape `grep -rn` emits, so a
# gate's downstream pipeline (its allowlist filters, its message) is
# unchanged by the swap. LINE is the line the record's own first CODE
# sits on: a statement or a window opening under a comment is reported
# at the code, never at the comment above it, so a hit a gate prints
# names a line that holds what it matched.
#
# WHAT IT KNOWS. `//`, `/* */` INCLUDING NESTING TO ANY DEPTH AND ACROSS
# ANY NUMBER OF LINES (Rust allows nested block comments; the `*/` that
# BALANCES the opener closes here, as it does in rustc and in
# `test_utils::source`), `"..."`, `r#"..."#`, `b"..."`, and char
# literals as distinct from lifetimes.
#
# WHAT IT CANNOT DO, ONE BLIND SPOT AT A TIME, because a list of names
# is not a statement of what each one is blind to. It is a lexer, not a
# parser, and each of these is left rather than fixed:
#
#   * `macro_rules!` BODIES ARE ORDINARY CODE. A body is lexed and
#     emitted like any other text, so every matcher reads what a macro
#     is WRITTEN as and never what it EXPANDS to. Both directions are
#     live: a forbidden spelling assembled from token fragments is
#     invisible, and one written literally inside a body nothing
#     invokes reds a gate. ACCEPTED because the fix is expanding Rust,
#     which is a compiler and not a reader — and the one gate whose
#     subject this actually is has its own row
#     (`clippy-panic-gate-blind-in-macros`) rather than a flag here.
#
#   * `include!`d TEXT IS NOT FOLLOWED, and the reason is that this
#     reader's unit is a file path its CALLER hands it. An included
#     file inside the caller's scan set is read in its own right; one
#     outside it is read by nothing. ACCEPTED because which files are
#     the subject is the caller's decision — `gate_require_crate_sources`
#     is where that set is fixed and proved non-empty — and a reader
#     that followed `include!` would scan files its caller never
#     counted, so the count a gate prints would stop naming what it
#     read.
#
#   * AN UNTERMINATED `/*` SWALLOWS THE REST OF ITS FILE, silently and
#     in the blind direction: from the opener to EOF nothing is
#     emitted, so every matcher reads that file as empty while the
#     count the gate prints — a FILE count, not a record count — does
#     not move. ACCEPTED because `rustc` REFUSES such a file, so only
#     source that does not compile can carry one and a gate is a claim
#     about code that compiles. Worth writing down for its direction: a
#     reader that force-closed the comment at EOF would cry wolf
#     instead, and this one cannot.
#
#   * `#[cfg]` OTHER THAN THE `test` SKIP IS SCANNED AS LIVE CODE, and
#     that is the deliberate half: an item behind `feature = "x"` or
#     `target_os = "…"` compiles under some configuration, so reading
#     it is right and skipping it would be blind exactly where a gate
#     is load-bearing. Even the `test` skip is opt-in per caller
#     (`--skip-cfg-test`) and refuses `any(…)`/`not(…)` for the reason
#     the attribute's own block above gives. The residue is one-directional:
#     an item behind a cfg that is false everywhere is still scanned,
#     which cries wolf and cannot go blind.
#
# AND WHAT IS NOT THIS READER'S BLIND SPOT. A needle that cannot match
# a shape the view faithfully carries is the MATCHER's blind spot, not
# the lexer's: `v[i] * v[i]` reaches the code view exactly as written,
# and a square matcher that only pairs bare identifiers is where that
# gap is stated. Keeping the two apart is what stops a fix landing in
# the wrong file.
gate_rust_code() {
  local skip_cfg_test=0 mode=lines window=0 keep_literals=0 status=0
  while [ $# -gt 0 ]; do
    case "$1" in
      --skip-cfg-test) skip_cfg_test=1; shift ;;
      --keep-literals) keep_literals=1; shift ;;
      --statements) mode=statements; shift ;;
      --window) mode=window; window=$2; shift 2 ;;
      *) break ;;
    esac
  done
  [ $# -gt 0 ] || return 0
  GATE_CFG_TEST_RE="$GATE_CFG_TEST_RE" GATE_CFG_TEST_NOT_RE="$GATE_CFG_TEST_NOT_RE" \
  awk -v SKIPTEST="$skip_cfg_test" -v MODE="$mode" -v WIN="$window" \
      -v KEEPLIT="$keep_literals" '
    # A single quote cannot be written inside this program, which is
    # itself single-quoted; CODEBRK is built rather than spelled.
    BEGIN { Q = sprintf("%c", 39); CODEBRK = "[\"" Q "/]" }
    FNR == 1 { state = 0; cdepth = 0; depth = 0; skipping = 0; seen_open = 0 }
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
      # BOTH FIGURES ARE ONE UNDATED READING ON ONE BOX and nothing
      # re-takes either — no register carries the wall clock of a gate,
      # and a threshold here would be a timing assertion inside a
      # correctness gate, which is the shape this repo keeps out of its
      # gates. What the choice rests on is the ORDER OF MAGNITUDE (a
      # rewrite that gave back the decade would be visible in any local
      # run of the discipline row), not on either number staying true.
      while (i <= n) {
        rest = substr(s, i)
        if (state == 1) {                      # inside /* ... */, nested
          # NESTING, the way rustc reads it and the way
          # `test_utils::source` models it: a `/*` inside a block
          # comment opens another, and the comment ends at the `*/` that
          # balances them. The EARLIER of the two tokens decides, so
          # `/*/` opens (its `/*` starts before its `*/`) and `*/*`
          # closes. Reading the first `*/` as the end instead put the
          # rest of an outer comment back into the code view — text a
          # matcher then reads as live source.
          #
          # THE DEPTH IS A STATE, NOT A PER-LINE TALLY, and that is
          # where this was wrong once: an opener was counted only when
          # a closer sat on the SAME line, so `/* outer` / `/* inner` /
          # `*/` left depth at one and the next `*/` ended the outer
          # comment early — the multi-line inner comment being the
          # ordinary way one is written. `cdepth` spans lines because
          # every opener and every closer is counted as the scan
          # reaches it, wherever it sits.
          o = index(rest, "/*"); p = index(rest, "*/")
          if (o > 0 && (p == 0 || o < p)) { cdepth++; i += o + 1; continue }
          if (p == 0) { i = n + 1; continue }
          cdepth--
          if (cdepth <= 0) { cdepth = 0; state = 0 }
          i += p + 1
          continue
        }
        if (state == 2) {                      # inside "..." (or b"...")
          p = match(rest, /["\\]/)
          if (p == 0) { if (KEEPLIT == 1) out = out rest; i = n + 1; continue }
          if (substr(rest, p, 1) == "\\") {
            if (KEEPLIT == 1) out = out substr(rest, 1, p + 1)
            i += p + 1; continue
          }
          if (KEEPLIT == 1) out = out substr(rest, 1, p - 1)
          out = out "\""; state = 0; i += p
          continue
        }
        if (state == 3) {                      # inside r#*"..."#*
          p = index(rest, "\"" rawhashes)
          if (p == 0) { if (KEEPLIT == 1) out = out rest; i = n + 1; continue }
          if (KEEPLIT == 1) out = out substr(rest, 1, p - 1)
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
          if (two == "/*") { state = 1; cdepth = 1; i += 2; continue }
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
          if (KEEPLIT == 1) out = out substr(s, i, j - i + 1)
          else out = out Q Q
          i = j + 1; continue
        }
        if (substr(s, i + 2, 1) == Q) {
          if (KEEPLIT == 1) out = out substr(s, i, 3)
          else out = out Q Q
          i += 3; continue
        }
        out = out Q; i++
      }

      # `#[cfg(test)]` items, dropped as whole brace-balanced blocks when
      # the caller asks. WHICH attribute is test-only is `GATE_CFG_TEST_RE`
      # and its subtraction, one spelling for this predicate and for the
      # module resolver — see their block above for what each says.
      # BRACE COUNTING IS PAID FOR ONLY BY THE CALLER THAT ASKED. `gsub`
      # over every line of crates/*/src costs ~8 s on its own, so the
      # gates that do not skip test modules never run it.
      if (SKIPTEST == 1) {
        opens = 0; closes = 0
        if (index(out, "{") > 0) opens = gsub(/\{/, "{", out)
        if (index(out, "}") > 0) closes = gsub(/\}/, "}", out)
        if (skipping == 0 && out ~ ENVIRON["GATE_CFG_TEST_RE"] &&
            out !~ ENVIRON["GATE_CFG_TEST_NOT_RE"]) {
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
      # A line whose code content is whitespace is not a record: the
      # indentation of a comment-only line is all that survives the
      # strip, and a record made of it starts a window one line above
      # the code that window carries.
      if (out ~ /^[ \t]*$/) next
      if (MODE == "lines") { print FILENAME ":" FNR ":" out; next }
      if (MODE == "window") {
        # Buffered per file, flushed when the file changes: a window
        # starting at line i needs the lines after it.
        if (FILENAME != wfile) { flushwin(); wfile = FILENAME; wn = 0 }
        wn++; WT[wn] = out; WL[wn] = FNR
        next
      }
      # --- statements ---------------------------------------------
      # NO CONTIGUITY ASSUMPTION. An earlier version of this joined
      # records only while their line numbers ran consecutively, which a
      # blank line or a column-zero block comment silently broke — the
      # statement reset and the matcher went blind mid-`where` clause.
      # A statement ends at a delimiter and at nothing else.
      if (FILENAME != sfile) { flushstmt(); sfile = FILENAME; stmt = ""; sline = 0 }
      code = out
      while (length(code) > 0) {
        if (match(code, /[{};]/)) {
          cut = RSTART
          if (stmt == "") sline = FNR
          stmt = stmt " " substr(code, 1, cut - 1)
          emitstmt()
          code = substr(code, cut + 1)
        } else {
          if (stmt == "") sline = FNR
          stmt = stmt " " code
          code = ""
        }
      }
    }
    function emitstmt(  t) {
      t = stmt; stmt = ""
      gsub(/[ \t]+/, " ", t); sub(/^ /, "", t); sub(/ $/, "", t)
      if (t != "") print sfile ":" sline ": " t
      sline = 0
    }
    function flushstmt() { if (stmt != "") emitstmt() }
    function flushwin(  i, j, w) {
      for (i = 1; i <= wn; i++) {
        w = WT[i]
        for (j = i + 1; j <= i + WIN - 1 && j <= wn; j++) w = w " " WT[j]
        gsub(/[ \t]+/, " ", w); sub(/^ /, "", w); sub(/ $/, "", w)
        if (w != "") print wfile ":" WL[i] ": " w
      }
      wn = 0
    }
    END {
      if (MODE == "statements") flushstmt()
      else if (MODE == "window") flushwin()
    }
  ' "$@" || status=$?
  # A READER THAT DIED IS NOT AN EMPTY FILE SET — `gate_grep`'s rule,
  # applied to the other half of the scan. Every gate that reads Rust
  # through this function pipes its output into a matcher, so an `awk`
  # that cannot open a file, or is not on PATH at all, delivers NO
  # RECORDS and every matcher downstream then finds nothing: the gate
  # reads a dead reader as a clean tree. The marker is what crosses the
  # process substitutions and command substitutions the status cannot,
  # and `gate_ok` refuses to print over it.
  #
  # WHAT THIS HOLDS IS A NON-ZERO STATUS, AND NOTHING MORE. An `awk`
  # that writes half the view and exits 0 is not seen here: the caller
  # gets a short view, its matchers find nothing in what is missing, and
  # the count the gate prints comes from a different guard, so the green
  # is exactly as reassuring as it was wrong. Nothing in this directory
  # cross-checks record count against file count, and that is the shape
  # of the guard that would.
  [ "$status" -eq 0 ] && return 0
  gate_error "$(gate_name): the shared Rust reader exited $status, so the code view it was asked for is short of what the caller handed it and every matcher reading it decided less than the gate claims — that is not a clean scan"
  : >> "$GATE_MATCHER_FAILED"
  exit "$status"
}

# --- WHERE A TEST-ONLY MODULE LIVES -----------------------------------
#
# A module whose `mod` declaration is `#[cfg(test)]`-gated is test-only
# code that happens to live in its own file, and the reader's per-item
# skip cannot see across files. Resolved from the declaration rather
# than from a list of names, so a new one needs no edit — and resolved
# HERE, beside the reader, because three gates ask this question and a
# resolution answered per caller drifts per caller: a textual reading
# names the sibling `dir/x.rs` for every declarer, which is rustc's
# answer only in a crate root or a `mod.rs`.
#
# THE RESOLUTION IS RUSTC'S, and nothing else resolves both directions at
# once. `mod bar;` names the SIBLING `dir/bar.rs` (or `dir/bar/mod.rs`)
# only when the declaring file is a crate root or a `mod.rs`; declared in
# any other file `dir/foo.rs` it names `dir/foo/bar.rs` (or
# `dir/foo/bar/mod.rs`); an enclosing inline `mod y { … }` adds `y/` to
# whichever of those two the declarer chooses; and a `#[path = "P"]`
# attribute overrides the file name with `P`, relative to the declaring
# file's directory at top level and to the inline module's directory
# inside one. Reading every declaration as a sibling drops the production
# `dir/bar.rs` from the scan because an unrelated file declared a test
# module of its name, and scans the test module that was actually
# declared as production.
#
# ROOTNESS IS READ FROM THE BASENAME, which is a proxy for what Cargo
# actually says: a `[[bin]]` or `[lib]` `path =` can make any name a
# crate root. The tree's one root outside the three names —
# `crates/viewer/src/bin/viewer.rs` — declares no module at all, so the
# proxy decides nothing today; a root named otherwise that declares a
# gated module would be resolved as a non-root and mount its module one
# directory too deep.
#
# WHERE THIS READER IS BLIND, each with the direction it errs in:
#
#   * A DECLARATION SPLIT OVER LINES (`mod` and its name on separate
#     ones) is not matched by the raw narrowing below, which is
#     line-scoped, so a file whose only gated declaration takes that
#     shape never becomes a candidate and its module file is scanned as
#     production. OVER-SCAN — a false red, not a silent hole. Once the
#     file IS a candidate the same spelling is refused loudly instead.
#   * `#[cfg_attr(…, path = "…")]` is not read as a mount, so the
#     declaration falls through to the positional rule. BOTH DIRECTIONS
#     at once: the positional path is excluded though nothing mounts
#     there (under-scan if production code sits at it) and the real
#     target is scanned as production (over-scan).
#   * A DECLARATION WRITTEN BY A MACRO or pulled in by `include!` is
#     invisible to the shared reader, so its module file is scanned as
#     production. OVER-SCAN.

# A relative path with its `.` and `..` segments taken out. An exclusion
# is matched against the scan set as text, so an unnormalised
# `crates/mesh/src/../tests/common/x.rs` excludes nothing at all — a
# silent no-op wearing the shape of an exclusion.
gate_norm_path() {
  printf '%s' "$1" | awk -F/ '
    {
      n = 0
      for (i = 1; i <= NF; i++) {
        if ($i == "" || $i == ".") continue
        if ($i == ".." && n > 0 && seg[n] != "..") { n--; continue }
        seg[++n] = $i
      }
      out = ""
      for (i = 1; i <= n; i++) out = (i == 1) ? seg[i] : out "/" seg[i]
      print out
    }'
}

# WHERE A GATED DECLARATION MOUNTS, read from the CODE-ONLY view that
# `gate_rust_code` already builds — comments and string bodies are gone
# before this sees a line, so no comment rule is re-minted beside the
# shared reader's. One pass over the declaring file carries the three
# things the statement view drops:
#
#   * `{` VS `;`. An inline `mod x { … }` declares no file at all, and
#     the statement view cuts at both delimiters, so an inline module and
#     a file declaration arrive as the same record.
#   * THE ENCLOSING INLINE-MODULE CHAIN. `mod y { #[cfg(test)] mod x; }`
#     mounts at `y/x.rs`; read as top-level it excludes the unrelated
#     production `x.rs` instead, which is the unsafe direction.
#   * WHERE a `#[path]` attribute sits. Its payload is a string literal
#     and this view blanks it, so only the raw line can supply it — the
#     line, the occurrence on it and how many the code view sees there
#     are named here, so the raw read is one line long and cannot pick up
#     a mount the code view does not have.
#
# Prints `KIND|CHAIN|PATH_LINE|PATH_INDEX|PATH_COUNT` — `|` and not a
# tab, because a tab is IFS whitespace and `read` folds a run of it, so
# an empty CHAIN would shift every field after it by one.
# KIND is `attr` (a `#[path]` mount), `default` (rustc's positional
# rule), `inline` (no file) or `refuse` (an enclosing brace that is not a
# module, where rustc mounts a non-inline module only through `#[path]`).
# CHAIN is the enclosing module names, `/`-joined. NO OUTPUT means the
# code view does not place the declaration the statement view reported —
# also a refusal, and the caller says so.
gate_declaration_shape() {
  gate_rust_code "$1" | awk -v start="$2" -v name="$3" '
    # THE ANSWER IS HELD TO `END`, NOT PRINTED AND EXITED ON. This `awk`
    # reads a pipe, and exiting at the declaration closes it while the
    # shared reader upstream is still writing: that write fails, the
    # reader dies of it, and `pipefail` reports a pipeline that did its
    # job as a pipeline that broke. Whether it lands is a race between
    # two processes, which is the worst way for a gate to be wrong —
    # green on one machine and red on the next over the same tree.
    function emit(kind,   i, c) {
      if (done) return
      done = 1
      c = ""
      for (i = 1; i <= depth; i++) {
        if (chain[i] == "") { out = "refuse||0|0|0"; return }
        c = (c == "") ? chain[i] : c "/" chain[i]
      }
      out = sprintf("%s|%s|%d|%d|%d", kind, c, pline, pidx, pcount)
    }
    done { next }
    {
      s = $0
      if (!match(s, /^[^:]*:[0-9]+:/)) next
      ln = substr(s, RSTART, RLENGTH); sub(/^[^:]*:/, "", ln); sub(/:$/, "", ln)
      s = substr(s, RSTART + RLENGTH)
      ln += 0
      if (ln >= start && pline == 0) {
        k = 0; t = s
        while (match(t, /#\[[ \t]*path[ \t]*=/)) { k++; t = substr(t, RSTART + RLENGTH) }
        if (k > 0) { pline = ln; pidx = 1; pcount = k }
      }
      i = 1; L = length(s)
      while (i <= L) {
        c = substr(s, i, 1)
        if (c == "{") {
          if (want) { emit("inline"); next }
          depth++; chain[depth] = pending; pending = ""; i++; continue
        }
        if (c == ";") {
          if (want) { emit(pline > 0 ? "attr" : "default"); next }
          pending = ""; i++; continue
        }
        if (c == "}") {
          # The declaration was found and neither delimiter followed it —
          # a shape this reader cannot place, so it says nothing and the
          # caller refuses.
          if (want) { done = 1; out = ""; next }
          if (depth > 0) { chain[depth] = ""; depth-- }
          pending = ""; i++; continue
        }
        r = substr(s, i)
        if (match(r, /^mod[ \t]+[A-Za-z_][A-Za-z0-9_]*/) &&
            (i == 1 || substr(s, i - 1, 1) !~ /[A-Za-z0-9_]/)) {
          w = substr(r, RSTART, RLENGTH); sub(/^mod[ \t]+/, "", w)
          i += RLENGTH
          pending = w
          if (ln >= start && w == name) want = 1
          continue
        }
        i++
      }
    }
    END { if (out != "") print out }'
}

# THE MOUNT'S PAYLOAD, from the one raw line the code view named. This
# one reads the file directly rather than a pipe, so stopping at that
# line closes nothing behind it. Prints
# nothing when that line does not carry the same attributes the view saw
# — a `#[path]` written inside a comment beside a live one, or a payload
# that is not a string literal on that line — because a payload the two
# views disagree about is not a decision.
gate_path_payload() {
  awk -v ln="$2" -v idx="$3" -v want="$4" '
    NR == ln {
      k = 0; s = $0; p = ""
      while (match(s, /#\[[ \t]*path[ \t]*=[ \t]*"[^"]*"/)) {
        k++
        if (k == idx) {
          p = substr(s, RSTART, RLENGTH)
          sub(/^[^"]*"/, "", p); sub(/"$/, "", p)
        }
        s = substr(s, RSTART + RLENGTH)
      }
      if (k == want) print p
      exit
    }' "$1"
}

# A refusal, and it is loud in the one way that crosses the boundary it
# is written behind: `gate_test_only_mounts` is read through a process
# substitution, so an `exit` here cannot fail the caller and the list
# printed so far would be read as the whole answer. The marker is what
# `gate_production_sources` checks the moment the list is in.
gate_refuse_declaration() {
  gate_error "$(gate_name): $1 — that is not a pass"
  : >> "$GATE_MATCHER_FAILED"
  exit 1
}

# gate_test_only_mounts FILE... — the set of test-only files and
# directories the given sources mount, one entry per line: a FILE entry
# is a whole path, a DIRECTORY entry ends in `/` and is a path prefix.
gate_test_only_mounts() {
  local decl file rest line name shape kind chain pline pidx pcount
  local base payload target narrowed=() cands=()
  [ $# -gt 0 ] || return 0
  # TWO STAGES, because reading every source twice costs more than any
  # gate here is worth: raw `grep` narrows to the handful of files that
  # carry both a test-only attribute and a `mod` declaration, and only
  # those are read properly. A file that fails the raw narrowing carries
  # no test-only `cfg` text at all, so it cannot carry a gated
  # declaration.
  #
  # The declaration is matched over the STATEMENT view, which is what
  # makes `#[cfg(test)] mod probes;` on ONE line and the same split over
  # two the same record — a line-scoped narrowing sees only the split
  # one, and cries wolf on the other.
  #
  # NO `xargs` BETWEEN THE STAGES. `xargs` reports 123 for any child that
  # exited 1-125, which folds `grep`'s "nothing matched" and its "I could
  # not search" into one status before anything here can tell them apart.
  # The narrowed list is small by construction, so the second stage takes
  # it as arguments.
  mapfile -t narrowed < <(gate_grep -lE "$GATE_CFG_TEST_RE" "$@")
  if [ "${#narrowed[@]}" -gt 0 ]; then
    mapfile -t cands < <(gate_grep -lE '(^|[[:space:]])mod [a-z_][a-z0-9_]*;' "${narrowed[@]}")
  fi
  [ "${#cands[@]}" -gt 0 ] || return 0
  while IFS= read -r decl; do
    [ -n "$decl" ] || continue
    file=${decl%%:*}; rest=${decl#*:}; line=${rest%%:*}; name=${rest#*:}
    # A READER THAT DIED IS NOT AN ANSWER either, and captured in a
    # command substitution it would otherwise die under errexit with
    # its status thrown away and no diagnosis at all.
    if ! shape=$(gate_declaration_shape "$file" "$line" "$name"); then
      gate_refuse_declaration "reading $file to place its \`mod $name;\` failed, so where that module lives was not decided"
    fi
    IFS='|' read -r kind chain pline pidx pcount <<<"$shape"
    case "$kind" in
      inline) continue ;;
      refuse)
        gate_refuse_declaration "$file:$line declares mod $name inside a brace that is not a module, where rustc mounts a non-inline module only through #[path]; where that module lives was not decided" ;;
      attr|default)
        case "${file##*/}" in
          mod.rs|lib.rs|main.rs) base=${file%/*} ;;
          *) base=${file%.rs} ;;
        esac
        if [ "$kind" = attr ]; then
          # A top-level `#[path]` is relative to the declaring file's
          # DIRECTORY whatever the file is named; inside an inline
          # module it is relative to that module's directory, which is
          # the positional base plus the chain.
          [ -n "$chain" ] || base=${file%/*}
          [ -z "$chain" ] || base=$base/$chain
          payload=$(gate_path_payload "$file" "$pline" "$pidx" "$pcount") || payload=
          if [ -z "$payload" ]; then
            gate_refuse_declaration "$file:$line mounts mod $name with a #[path] whose payload line $pline does not read back as the code view sees it, so where that module lives was not decided"
          fi
          target=$(gate_norm_path "$base/$payload")
        else
          [ -z "$chain" ] || base=$base/$chain
          target=$base/$name.rs
        fi ;;
      *)
        gate_refuse_declaration "$file:$line declares mod $name in the statement view and the code view does not place it, so where that module lives was not decided" ;;
    esac
    # A declaration resolving onto its own declarer (`mod lib;` in
    # `lib.rs`) names no other file, and excluding the declarer takes
    # the whole tree with it. The directory form stays: `dir/lib/mod.rs`
    # is a different file and a real resolution of that declaration.
    [ "$target" = "$file" ] || printf '%s\n' "$target"
    printf '%s\n' "${target%.rs}/"
  done < <(gate_rust_code --statements "${cands[@]}" \
    | gate_grep -E "$GATE_CFG_TEST_RE" \
    | gate_grep -vE "$GATE_CFG_TEST_NOT_RE" \
    | gate_grep -oE '^[^:]*:[0-9]+:.*[[:space:]]mod [a-z_][a-z0-9_]*$' \
    | sed -E 's/:([0-9]+):.*[[:space:]]mod /:\1:/')
}

# gate_filter_test_only_paths PATH... — the paths `GATE_TEST_ONLY_MOUNTS`
# does not name, in the order given.
#
# AN EXCLUSION IS A PATH, NOT A SUBSTRING, and that is why this is a
# comparison rather than a `grep -F` over the list: `-F` matches anywhere
# in the line, so an excluded `foo/bar.rs` also takes `foo/bar.rs_old.rs`,
# and an excluded `crates/p/src/foo/bar.rs` takes a
# `crates/q/src/crates/p/src/foo/bar.rs` under another crate. A file
# entry is the WHOLE path; a directory entry is a path PREFIX, which is
# what its trailing `/` says. Nothing here can fail to run, so nothing
# here needs the marker.
gate_filter_test_only_paths() {
  local f e skip keep=()
  for f in "$@"; do
    skip=false
    for e in ${GATE_TEST_ONLY_MOUNTS[@]+"${GATE_TEST_ONLY_MOUNTS[@]}"}; do
      if [ "${e%/}" != "$e" ]; then
        if [ "${f#"$e"}" != "$f" ]; then skip=true; break; fi
      elif [ "$f" = "$e" ]; then
        skip=true; break
      fi
    done
    [ "$skip" = true ] || keep+=("$f")
  done
  [ "${#keep[@]}" -eq 0 ] || printf '%s\n' "${keep[@]}"
}

# gate_production_sources — `GATE_SOURCE_FILES` less the test-only
# mounts, in `GATE_PRODUCTION_FILES`, with `GATE_SCAN_FILES` counting
# what the gate will actually read. Both refusals below are the caller's
# boundary and not a convenience: a gate that does not know which files
# are its subject has not cleared any of them.
gate_production_sources() {
  mapfile -t GATE_TEST_ONLY_MOUNTS < <(gate_test_only_mounts "${GATE_SOURCE_FILES[@]}")
  # THE BOUNDARY, READ BEFORE ANY GUARD READS THE LIST. The resolver
  # prints its answer only once it is complete, so a refusal inside it
  # arrives here as an EMPTY list — the same shape a tree of nothing but
  # test-only sources has, and the guard below would then answer a
  # question nobody asked, with the true diagnosis scrolled off above it.
  # The marker is what crosses the process substitution; the status
  # cannot.
  if [ -e "$GATE_MATCHER_FAILED" ]; then
    rm -f "$GATE_MATCHER_FAILED"
    gate_error "$(gate_name): the scan's file set was not decided (diagnosed above), so what it does not contain is unknown — that is not a pass"
    exit 1
  fi
  mapfile -t GATE_PRODUCTION_FILES < <(gate_filter_test_only_paths "${GATE_SOURCE_FILES[@]}")
  GATE_SCAN_FILES=${#GATE_PRODUCTION_FILES[@]}
  if [ "$GATE_SCAN_FILES" -eq 0 ]; then
    gate_error "$(gate_name): every source under crates/*/src in $PWD is test-only — the gate scanned no production code, which is not a pass"
    exit 1
  fi
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
  # AN EMPTY TREE IS NOT A CLEAN TREE, proved on every gate. This file
  # makes a paragraph of `gate_require_crate_sources` and `gate_require_
  # file`, and a trace of `gate_error` across every self-test in this
  # directory found both of their diagnoses UNREACHED — `gate_plant_clean`
  # always writes a source file, so no fixture ever asked. `lib.sh` says a
  # guard never shown to fire is not a guard; that sentence had not been
  # applied inside this file. The two cases below are the rest of it, and
  # the way to check the claim is the trace, not this comment:
  # instrument `gate_error` to record `BASH_SOURCE`/`BASH_LINENO` AND
  # THE MESSAGE, run every `--selftest`, and diff what fired against the
  # declared population, `grep -nE '(^|[[:space:];&|])gate_error "'
  # scripts/gates/*.sh`.
  #
  # THE MESSAGE IS NOT BELT AND BRACES. A site inside a command
  # substitution reports the line of the ENCLOSING FUNCTION CALL, not
  # its own: bash resets the call stack in the subshell, so
  # `BASH_LINENO` names a line that holds no `gate_error` at all. A
  # line-only trace therefore reports such a site as unreached however
  # many fixtures fire it — and this directory has one, the census's
  # nested-suite refusal. Match a fired message to a site by ALL of the
  # site's literal fragments in ONE message: two sites here share a
  # fragment, so any-of scores one of the pair reached for free.
  tmp=$(mktemp -d)
  if out=$("$0" --root "$tmp" ${GATE_SELFTEST_ARGS[@]+"${GATE_SELFTEST_ARGS[@]}"} 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the gate PASSED on an EMPTY tree — a gate that scanned nothing is not a pass\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
  gate_selftest_assert_diagnosed "an empty tree" "$out"
  # A TREE THAT HAS THE DIRECTORIES AND NONE OF THE FILES IS NOT A
  # CLEAN TREE, and it is a DIFFERENT tree from the empty one above.
  # The empty tree has no `crates/*/src` to glob at all, so it stops at
  # the FIRST of `gate_require_crate_sources`'s two guards and the
  # second — a `crates/*/src` that exists and holds no `.rs` — was
  # reached by no case in this directory. The manifest gates read the
  # same tree the other way: a `crates/` that exists with no
  # `crates/*/Cargo.toml` under it. One fixture, because it is one
  # question — did the gate mistake an empty subject for a clean one.
  tmp=$(mktemp -d)
  mkdir -p "$tmp/crates/scanned/src"
  if out=$("$0" --root "$tmp" ${GATE_SELFTEST_ARGS[@]+"${GATE_SELFTEST_ARGS[@]}"} 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the gate PASSED on a tree whose crates/ directories are all EMPTY — a gate that scanned nothing is not a pass\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
  gate_selftest_assert_diagnosed "a crates/ tree with no files in it" "$out"
  # THE WINDOW VIEW STARTS AT CODE, proved on every gate rather than
  # asserted in the reader's comment. This one is an assertion about the
  # READER and not about the gate around it, because that is where the
  # defect is: every gate reads through this function, and a comment-only
  # line emitted as a record makes the window view report the site below
  # it a second time, one line early. The fixture pins the whole view of
  # a four-line file, so it fails in both directions — a record for the
  # comment line, and a join that stops at it.
  tmp=$(mktemp -d)
  printf 'fn f() {\n    // a comment-only line\n    let x = 1;\n}\n' > "$tmp/win.rs"
  out=$(gate_rust_code --window 2 "$tmp/win.rs" | sed "s#^$tmp/##")
  rm -rf "$tmp"
  if [ "$out" != "win.rs:1: fn f() { let x = 1;
win.rs:3: let x = 1; }
win.rs:4: }" ]; then
    printf 'SELFTEST FAILED: the --window view over a file whose second line holds only a comment is not what the reader claims — a comment-only line is not a record, and a window joins the CODE lines after it:\n%s\n' "$out" >&2
    exit 1
  fi
  # THE MARKER'S OWN GUARD, proved on every gate rather than asserted in
  # its comment. `gate_main` refuses to scan when it cannot create
  # `$GATE_MATCHER_FAILED`, because a marker that cannot be written
  # cannot report a matcher that died — and that guard was itself the
  # thing no fixture had ever reached, which is the sentence this file
  # keeps repeating at other people's guards. The clean fixture is
  # planted so the ONLY reason to fail is the unwritable TMPDIR.
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  if out=$(TMPDIR="$tmp/no-such-tmpdir" "$0" --root "$tmp" ${GATE_SELFTEST_ARGS[@]+"${GATE_SELFTEST_ARGS[@]}"} 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the gate PASSED with an unwritable TMPDIR — the marker a dead matcher writes could not have been created, so the green means nothing\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
  gate_selftest_assert_diagnosed "an unwritable TMPDIR" "$out"
  case "$out" in
    *"cannot create"*) ;;
    *) printf 'SELFTEST FAILED (an unwritable TMPDIR): the gate failed for some OTHER reason than the marker it could not create:\n%s\n' "$out" >&2
       exit 1 ;;
  esac
  # `gate_main`'s OWN no-self-test guard, and the fixture is not a gate
  # in this directory. The guard was recorded as unreachable on the
  # argument that every gate defines a `gate_selftest` and one that did
  # not would red `gate-roster.sh` — which is an argument about the
  # DIRECTORY, not about the guard. The guard's subject is anything that
  # sources this file, so six lines of scratch reach it: a `gate`, a
  # `gate_parse_args`, a `gate_main`, and no `gate_selftest`. Written
  # here rather than in one gate because the guard is this file's, so
  # every caller of this function carries it exactly as it carries the
  # unreadable `--root` and the unwritable TMPDIR above.
  tmp=$(mktemp -d)
  {
    printf '#!/usr/bin/env bash\n'
    printf 'set -euo pipefail\n'
    printf '. %s\n' "${BASH_SOURCE[0]}"
    printf 'gate() { gate_ok "nothing"; }\n'
    printf 'gate_parse_args "$@"\n'
    printf 'gate_main\n'
  } > "$tmp/no-selftest.sh"
  if out=$(bash "$tmp/no-selftest.sh" --selftest 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: a script sourcing lib.sh with NO gate_selftest passed --selftest — a guard that has never been shown to fire is not a guard, which is the sentence this file keeps repeating at other guards\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
  gate_selftest_assert_diagnosed "a caller defining no gate_selftest" "$out"
  case "$out" in
    *"defines no gate_selftest"*) ;;
    *) printf 'SELFTEST FAILED (a caller defining no gate_selftest): it failed for some OTHER reason:\n%s\n' "$out" >&2
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
  # The PLANTER name, captured before the planter runs. `$1` after the
  # shift is the planter, which is what a reader wants in the failure
  # line — but only until the planter takes arguments, at which point
  # `$1` starts naming the wrong thing at a distance.
  local case_name=$1
  local tmp out
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  "$@" "$tmp"
  if out=$("$0" --root "$tmp" ${GATE_SELFTEST_ARGS[@]+"${GATE_SELFTEST_ARGS[@]}"} 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: the gate PASSED on a planted violation (%s)\n%s\n' "$case_name" "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
  gate_selftest_assert_diagnosed "$case_name" "$out"
  case "$out" in
    *"$want"*) ;;
    *) printf 'SELFTEST FAILED (%s): the gate fired with an unexpected message:\n%s\n' "$case_name" "$out" >&2
       exit 1 ;;
  esac
}

# gate_selftest_passes WHAT PLANTER [ARGS...] — gate_selftest_case's
# NEAR-MISS twin, and the case that keeps a widening honest. The only
# passing fixture the harness had was the empty clean tree, which proves
# nothing about a spelling that must not fire; every widened matcher in
# this directory needs a fixture saying where it stops.
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

# --- THE RESOLVER'S OWN CASES -----------------------------------------
#
# WHY THEY LIVE HERE AND NOT IN ONE GATE. The resolution has one home,
# so one gate's fixtures prove the RESOLVER; what no fixture of one
# gate's can prove is that the gate BESIDE it is wired to the answer. A
# gate whose exclusion could be deleted outright with its self-test
# still green has no evidence for that exclusion — the sentence this
# file keeps repeating at other people's guards — so every caller of
# `gate_production_sources` runs these and carries all of them.
#
# THE GATE SUPPLIES ONE THING, its own breach: `PLANT FILE` APPENDS a
# line the gate fires on to FILE, whose directory exists. Everything
# else here is module plumbing, which is the same text for every caller.
# A file that must be READ and stay quiet is planted EMPTY rather than
# with benign code, so a case that fires can only have fired from the
# file it is about.
gate_plant_home_ungated() {
  mkdir -p "$2/crates/planted/src"
  printf 'mod probes;\n' > "$2/crates/planted/src/lib.rs"
  "$1" "$2/crates/planted/src/probes.rs"
}

# `any(test, …)` is NOT test-only: an `any(debug_assertions, test, …)`
# module is every debug build, so its code is production code.
gate_plant_home_any_gated() {
  mkdir -p "$2/crates/planted/src"
  printf '#[cfg(any(debug_assertions, test))]\nmod probes;\n' \
    > "$2/crates/planted/src/lib.rs"
  "$1" "$2/crates/planted/src/probes.rs"
}

# A DECLARATION THAT RESOLVES ONTO ITS OWN DECLARER excludes nothing:
# `mod lib;` in `lib.rs` names the file it is written in, and dropping
# that file would take a whole crate out of the scan on the strength of
# one line.
gate_plant_home_self_naming() {
  mkdir -p "$2/crates/planted/src"
  printf '#[cfg(test)]\nmod lib;\n' > "$2/crates/planted/src/lib.rs"
  "$1" "$2/crates/planted/src/lib.rs"
}

# THE UNDER-SCAN DIRECTION, and it is the silent one: `bar.rs` is
# production code that no declaration gates, while the module `foo.rs`
# declares lives at `foo/bar.rs`. Resolving the declaration to the
# SIBLING drops this file from the scan entirely and its breach is never
# read — the file at the resolved path is empty, so only the sibling can
# decide this case.
gate_plant_home_production_sibling() {
  mkdir -p "$2/crates/planted/src/foo"
  printf 'mod bar;\nmod foo;\n' > "$2/crates/planted/src/lib.rs"
  printf '#[cfg(test)]\nmod bar;\n' > "$2/crates/planted/src/foo.rs"
  : > "$2/crates/planted/src/foo/bar.rs"
  "$1" "$2/crates/planted/src/bar.rs"
}

# AN INLINE `mod x { … }` DECLARES NO FILE. The statement view cuts at
# `{` and `;` alike, so an inline test module reaches the matcher as the
# same record a file declaration does — and read as one it excludes a
# production file that has nothing to do with it.
gate_plant_home_inline_beside_named_file() {
  mkdir -p "$2/crates/planted/src/foo"
  printf 'mod foo;\n' > "$2/crates/planted/src/lib.rs"
  printf 'mod bar;\n#[cfg(test)]\nmod probes {\n    fn t() {}\n}\n' \
    > "$2/crates/planted/src/foo.rs"
  "$1" "$2/crates/planted/src/foo/probes.rs"
}

# AN INLINE MODULE IN A CRATE ROOT, which the non-root rule never
# reaches: here the inline check is the only thing standing between
# `mod probes { … }` and the production `probes.rs` beside it.
gate_plant_home_inline_in_a_root() {
  mkdir -p "$2/crates/planted/src"
  printf 'mod other;\n#[cfg(test)]\nmod probes {\n    fn t() {}\n}\n' \
    > "$2/crates/planted/src/lib.rs"
  "$1" "$2/crates/planted/src/probes.rs"
}

# A GATED DECLARATION INSIDE AN INLINE MODULE mounts under that module:
# `mod y { #[cfg(test)] mod x; }` in a crate root is `y/x.rs`, so the
# top-level `x.rs` beside it is production code and has to be read.
gate_plant_home_nested_sibling() {
  mkdir -p "$2/crates/planted/src/y"
  printf 'mod x;\npub mod y {\n    #[cfg(test)]\n    mod x;\n}\n' \
    > "$2/crates/planted/src/lib.rs"
  : > "$2/crates/planted/src/y/x.rs"
  "$1" "$2/crates/planted/src/x.rs"
}

# AN EXCLUSION IS A PATH, NOT A SUBSTRING. `foo/bar.rs.rs` is a different
# file from the excluded `foo/bar.rs` and it is production code; a
# substring filter drops it with the module it merely starts with.
gate_plant_home_extends_an_exclusion() {
  mkdir -p "$2/crates/planted/src/foo"
  printf 'mod foo;\n' > "$2/crates/planted/src/lib.rs"
  printf '#[cfg(test)]\nmod bar;\n' > "$2/crates/planted/src/foo.rs"
  : > "$2/crates/planted/src/foo/bar.rs"
  "$1" "$2/crates/planted/src/foo/bar.rs.rs"
}

# EVERY SOURCE EXCLUDED, WITH NO CYCLE IN THE TREE ITSELF. A crate root
# resolves its declarations as siblings, so two roots in one directory
# each declaring a test module of the OTHER's name exclude each other:
# the production list comes back empty and the guard fires. Neither
# declaration names its own file, which is the case above it covers.
# The one case that plants no breach: what it is about is the tree
# having no production file left, so its argument is the root alone.
gate_plant_home_every_source_excluded() {
  mkdir -p "$1/crates/clean/src"
  printf '#[cfg(test)]\nmod main;\n' > "$1/crates/clean/src/lib.rs"
  printf '#[cfg(test)]\nmod lib;\n' > "$1/crates/clean/src/main.rs"
}

# WHERE THE RAW READER STOPS, and it stops LOUDLY. `mod` and its name on
# separate lines are one statement to the code view and no `mod NAME;`
# line to the raw one, so the mount point is not decided — and a gate
# that cannot decide where a module lives has not cleared the tree.
# (The file needs a `mod x;` of its own to reach this stage at all: the
# raw narrowing is line-scoped, so a lone split declaration is invisible
# and its file is scanned as production.)
gate_plant_home_split_declaration() {
  mkdir -p "$2/crates/planted/src"
  printf 'mod other;\n#[cfg(test)]\nmod\nbar;\n' > "$2/crates/planted/src/lib.rs"
  "$1" "$2/crates/planted/src/bar.rs"
}

# The module file a gated declaration in a ROOT names — the two-line
# spelling and the one-line one, which are the same declaration and were
# read as different ones by a line-scoped resolver.
gate_plant_home_gated_two_line() {
  mkdir -p "$2/crates/planted/src"
  printf '#[cfg(test)]\nmod probes;\n' > "$2/crates/planted/src/lib.rs"
  "$1" "$2/crates/planted/src/probes.rs"
}

gate_plant_home_gated_one_line() {
  mkdir -p "$2/crates/planted/src"
  printf '#[cfg(test)] mod probes;\n' > "$2/crates/planted/src/lib.rs"
  "$1" "$2/crates/planted/src/probes.rs"
}

# THE OVER-SCAN DIRECTION: the file the declaration actually names.
# `#[cfg(test)] mod bar;` inside `foo.rs` is `foo/bar.rs`, so this is
# test-only code and reading it as production is crying wolf.
gate_plant_home_in_declarer_directory() {
  mkdir -p "$2/crates/planted/src/foo"
  printf 'mod foo;\n' > "$2/crates/planted/src/lib.rs"
  printf '#[cfg(test)]\nmod bar;\n' > "$2/crates/planted/src/foo.rs"
  "$1" "$2/crates/planted/src/foo/bar.rs"
}

# A `#[path]` MOUNT overrides both positional rules, relative to the
# declaring file's own directory — and the tree has live ones, so a
# resolver without it re-mints the over-scan it just fixed. The dead
# mount above the live one is why the attribute is LOCATED in the
# code-only view and only its payload read from the raw line: a reader
# that went to the raw text for both would follow `decoy.rs` and exclude
# nothing that exists.
gate_plant_home_path_attribute() {
  mkdir -p "$2/crates/planted/src"
  printf 'mod foo;\n' > "$2/crates/planted/src/lib.rs"
  printf '#[cfg(test)]\n// #[path = "decoy.rs"]\n#[path = "bar_impl.rs"]\nmod bar;\n' \
    > "$2/crates/planted/src/foo.rs"
  "$1" "$2/crates/planted/src/bar_impl.rs"
}

# THE DIRECTORY FORM of the same resolution: `mod bar;` names
# `foo/bar.rs` OR `foo/bar/mod.rs`, and a module big enough to be a
# directory is exactly the test module a gate would otherwise scan whole.
gate_plant_home_directory_form() {
  mkdir -p "$2/crates/planted/src/foo/bar"
  printf 'mod foo;\n' > "$2/crates/planted/src/lib.rs"
  printf '#[cfg(test)]\nmod bar;\n' > "$2/crates/planted/src/foo.rs"
  "$1" "$2/crates/planted/src/foo/bar/mod.rs"
}

# THE INLINE CHAIN, from the other side: the file that declaration really
# mounts is test-only and must stay out of the scan.
gate_plant_home_nested_target() {
  mkdir -p "$2/crates/planted/src/y"
  printf 'pub mod y {\n    #[cfg(test)]\n    mod x;\n}\n' \
    > "$2/crates/planted/src/lib.rs"
  "$1" "$2/crates/planted/src/y/x.rs"
}

# A DECLARATION NEAR THE TOP OF A LONG FILE, which is the shape that
# catches a reader that stops reading once it has its answer: the shared
# reader upstream is still writing, its write fails on the closed pipe,
# and `pipefail` turns a declaration the gate DID place into a refusal.
# The file is longer than a pipe buffer on purpose — that is the whole
# difference between the two outcomes, and it is why the same tree could
# pass here and fail on a runner.
gate_plant_home_early_declaration_in_a_long_file() {
  mkdir -p "$2/crates/planted/src"
  {
    printf '#[cfg(test)]\nmod probes;\n'
    seq 4000 | awk '{ printf "pub fn f%s(x: f64) -> f64 { x + %s.0 }\n", $1, $1 }'
  } > "$2/crates/planted/src/lib.rs"
  "$1" "$2/crates/planted/src/probes.rs"
}

# gate_selftest_test_module_homes WANT PLANT — every case above, in both
# directions, for one gate. WANT is the fragment that gate's own
# diagnosis carries; PLANT appends its breach to a file.
gate_selftest_test_module_homes() {
  local want=$1 plant=$2
  gate_selftest_case "$want" gate_plant_home_ungated "$plant"
  gate_selftest_case "$want" gate_plant_home_any_gated "$plant"
  gate_selftest_case "$want" gate_plant_home_self_naming "$plant"
  gate_selftest_case "$want" gate_plant_home_production_sibling "$plant"
  gate_selftest_case "$want" gate_plant_home_inline_beside_named_file "$plant"
  gate_selftest_case "$want" gate_plant_home_inline_in_a_root "$plant"
  gate_selftest_case "$want" gate_plant_home_nested_sibling "$plant"
  gate_selftest_case "$want" gate_plant_home_extends_an_exclusion "$plant"
  gate_selftest_case "is test-only — the gate scanned no production code" \
    gate_plant_home_every_source_excluded
  gate_selftest_case "where that module lives was not decided" \
    gate_plant_home_split_declaration "$plant"
  # THE SAME FIXTURE, ASSERTED ON THE OTHER HALF of the refusal: without
  # the marker read in `gate_production_sources` the list arrives empty
  # and the every-source guard answers instead, with a diagnosis that is
  # false about the tree.
  gate_selftest_case "the scan's file set was not decided" \
    gate_plant_home_split_declaration "$plant"
  gate_selftest_passes "a breach in the module file a gated declaration names" \
    gate_plant_home_gated_two_line "$plant"
  gate_selftest_passes "the same, declared on one line" \
    gate_plant_home_gated_one_line "$plant"
  gate_selftest_passes "a breach in the module file a non-root declarer actually names" \
    gate_plant_home_in_declarer_directory "$plant"
  gate_selftest_passes "a breach in the module file a #[path] attribute mounts" \
    gate_plant_home_path_attribute "$plant"
  gate_selftest_passes "a breach in the mod.rs of a resolved module directory" \
    gate_plant_home_directory_form "$plant"
  gate_selftest_passes "a breach in the file an inline module's own gated declaration mounts" \
    gate_plant_home_nested_target "$plant"
  gate_selftest_passes "a gated declaration near the top of a file longer than a pipe buffer" \
    gate_plant_home_early_declaration_in_a_long_file "$plant"
  printf '%s selftest OK (test-module homes): places a cfg(test) declaration where rustc mounts it, so a production sibling, a production file under an inline module, a file whose path merely extends an exclusion, an ungated declaration and an any(test, …) one all stay in the scan and red, while the file the declaration names — positional, one-line or two, #[path]-mounted, directory-form or nested in an inline module — does not; and it REFUSES, with its own diagnosis and never a second false one, a declaration it cannot place, while a declaration it CAN place stays placed however long the file under it runs and a tree whose sources exclude each other is not a clean tree\n' "$(gate_name)"
}
# gate_selftest_without_tool TOOL WANT — for a gate that shells out. A
# reader that fails is the SECOND half of S157: the gate dies at the
# assignment that captured it, so what a CI reader sees is whatever the
# tool wrote, with no gate name, no `::error::` framing and no statement
# of what was not decided. TOOL is shadowed by a stub that exits
# non-zero; the gate must fail WITH a diagnosis containing WANT.
gate_selftest_without_tool() {
  local tool=$1 want=$2
  local tmp bin out
  tmp=$(mktemp -d); bin=$(mktemp -d)
  gate_plant_clean "$tmp"
  printf '#!/bin/sh\nexit 9\n' > "$bin/$tool"
  chmod +x "$bin/$tool"
  if out=$(PATH="$bin:$PATH" "$0" --root "$tmp" ${GATE_SELFTEST_ARGS[@]+"${GATE_SELFTEST_ARGS[@]}"} 2>&1); then
    rm -rf "$tmp" "$bin"
    printf 'SELFTEST FAILED: the gate PASSED with %s failing — a gate that cannot read its subject has not cleared it\n%s\n' "$tool" "$out" >&2
    exit 1
  fi
  rm -rf "$tmp" "$bin"
  gate_selftest_assert_diagnosed "$tool failing" "$out"
  case "$out" in
    *"$want"*) ;;
    *) printf 'SELFTEST FAILED (%s failing): the gate fired with an unexpected message — wanted %s, got:\n%s\n' "$tool" "$want" "$out" >&2
       exit 1 ;;
  esac
}

# gate_selftest_with_broken_tool TOOL WANT SHIM [PLANTER [ARGS...]] —
# gate_selftest_without_tool's TARGETED twin. A stub that fails every
# call proves only that the FIRST matcher cannot die silently; a matcher
# deeper in the gate — one whose status a process substitution swallows,
# say — needs the calls before it to succeed. SHIM is the body of a
# /bin/sh script that shadows TOOL on PATH; it decides per call whether
# to pass through or fail, and reaches the real tool as
# "$GATE_REAL_TOOL". The gate must fail with a gate_error diagnosis
# containing WANT.
#
# THE PLANTER IS OPTIONAL AND IT IS NOT DECORATION. The clean fixture is
# the only tree this helper had, so a failure path a gate reaches ONLY
# on a non-empty or otherwise particular scan was unreachable from any
# self-test — the same gap `gate_selftest_case` closes for matchers, one
# layer down. With a planter the case says the stronger thing: the gate
# fails with the DEAD TOOL's diagnosis and not with the planted breach's,
# because a tool that died could not have seen the breach.
gate_selftest_with_broken_tool() {
  local tool=$1 want=$2 shim=$3
  shift 3
  local tmp bin out real
  real=$(command -v "$tool")
  tmp=$(mktemp -d); bin=$(mktemp -d)
  gate_plant_clean "$tmp"
  # An `if`, not `[ … ] && …`: a false test is a failed statement and
  # errexit would leave this function before it planted anything.
  if [ $# -gt 0 ]; then "$@" "$tmp"; fi
  printf '#!/bin/sh\n%s\n' "$shim" > "$bin/$tool"
  chmod +x "$bin/$tool"
  if out=$(GATE_REAL_TOOL="$real" PATH="$bin:$PATH" "$0" --root "$tmp" ${GATE_SELFTEST_ARGS[@]+"${GATE_SELFTEST_ARGS[@]}"} 2>&1); then
    rm -rf "$tmp" "$bin"
    printf 'SELFTEST FAILED: the gate PASSED with %s broken mid-scan — a matcher that could not run decided nothing, and what it did not match is unknown\n%s\n' "$tool" "$out" >&2
    exit 1
  fi
  rm -rf "$tmp" "$bin"
  gate_selftest_assert_diagnosed "$tool broken mid-scan" "$out"
  case "$out" in
    *"$want"*) ;;
    *) printf 'SELFTEST FAILED (%s broken mid-scan): the gate fired with an unexpected message — wanted %s, got:\n%s\n' "$tool" "$want" "$out" >&2
       exit 1 ;;
  esac
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
    # NO DEFAULT SELF-TEST. There used to be one — clean fixture plus a
    # single planted violation, parameterised through `gate_main`'s
    # arguments — and every gate in this directory overrode it, while
    # two still passed it arguments naming a planter that call did not
    # run. A default that plants only what the matcher was written for
    # is the shape this whole directory is a reaction to, so a gate with
    # no self-test is a loud failure rather than a quiet minimum.
    if ! declare -F gate_selftest >/dev/null 2>&1; then
      gate_error "$(gate_name): defines no gate_selftest — a guard that has never been shown to fire is not a guard"
      exit 1
    fi
    gate_selftest
    exit 0
  fi
  # THE MARKER IS PROVED WRITABLE BEFORE ANYTHING DEPENDS ON IT, and
  # this is the same rule as the two above it: a marker that cannot be
  # created reports nothing, and a `gate_grep` inside a process
  # substitution would then fail exactly the way this file exists to
  # stop — quietly. Creating it also clears a marker left by a crashed
  # earlier run that happened to hold this pid.
  if ! (: > "$GATE_MATCHER_FAILED") 2>/dev/null; then
    gate_error "$(gate_name): cannot create $GATE_MATCHER_FAILED, so a matcher failing mid-scan could not be reported and a green would mean nothing — point TMPDIR at a writable directory"
    exit 1
  fi
  rm -f "$GATE_MATCHER_FAILED"
  if ! cd "$GATE_ROOT" 2>/dev/null; then
    gate_error "$(gate_name): cannot enter --root $GATE_ROOT from $PWD, so the gate scanned nothing — which is not a pass"
    exit 1
  fi
  gate
}
