#!/usr/bin/env bash
# no-extra-real-bounds.sh — no extra bounds on `Real`. ONE home;
# ci.yml's "no extra bounds on Real" step and
# local-scripts/ci-local.sh's discipline row both call this file.
#
# Evaluation-code discipline tripwire (see the `real.rs` module docs):
# a type parameter written `T: Real + PartialOrd` (or any other extra
# bound) is the escape hatch that lets generic evaluation code smuggle
# raw comparisons past the trait — it compiles at f64 and only dies at
# the interval instantiation.
#
# THE RULE IS ABOUT A TYPE PARAMETER CARRYING `Real` AND SOMETHING ELSE,
# not about the character `+`. Rust spells that three ways and this gate
# used to see one of them (`Real +`, that operand order, on one line):
#
#   PLUS, either order       `T: Real + PartialOrd`, `T: PartialOrd + Real`
#   TWO PREDICATES           `fn f<T: Real>(..) where T: PartialOrd`
#   TWO PREDICATES, WRAPPED  the same, after rustfmt puts each `where`
#                            predicate on its own line
#
# The second and third are the ones a matcher anchored on `+` cannot
# reach at all, and the third is what the formatter CONVERGES on — so
# writing the evasion is not even deliberate work. Both are matched
# here, over a statement rather than a line: the code-only text is cut
# at `{`, `}` and `;`, which is exactly where a generic list and its
# `where` clause end, and a parameter bounded twice inside one such
# statement is the compound bound written without a plus.
#
# WHAT THIS STILL CANNOT MATCH, and none of it is closed by widening:
#  - a compound reached through a NAME (`trait Both: Real + Ord`, then
#    `T: Both`). The declaration below fires; the use sites do not.
#    That is S124/D68's class, one gate over, and the answer there is
#    not a regex.
#  - a bound introduced by a macro, or in `include!`d text.
#  - a predicate whose subject is not a bare identifier
#    (`where Vec3<T>: Clone`), which the second matcher does not parse.
#
# When a legitimate extra bound first appears, refine this into an
# allowlist as a design decision — do not silently delete the check, and
# read the note on the ratified skip below before reaching for a file
# entry.
#
# THE RATIFIED SKIP, and why it is a line and not a file. `SpanLocate`
# is declared `sealed::Sealed + Real`, which this gate matches and
# should: it is a compound bound on `Real`. It is also ratified in two
# module headers (`geom-core/src/spline/locate.rs`, `geom/src/lib.rs`)
# on a reason that is exactly the rule's own: the extra operand is a
# pub-in-private SEALING marker with no methods and no nameable
# existence downstream, so it adds no comparison surface — the thing the
# escape hatch is an escape to. Allowlisting the FILE would un-guard
# every other line of `locate.rs`; skipping the declaration as EXACT
# TEXT costs one line and cannot grow. It is brittle in the other
# direction — a reformat or a rename stops the skip matching — so the
# skip's subject is proved before the scan rather than discovered as a
# confusing red, the way `bounds-allowlist.sh` proves its own.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

SEALED_HOME=crates/geom-core/src/spline/locate.rs
SEALED_DECL='pub trait SpanLocate: sealed::Sealed + Real {'

# The skip's subject, proved before the scan that depends on it.
gate_sealed_skip_subject() {
  [ -f "$SEALED_HOME" ] || return 0
  if ! grep -qxF "$SEALED_DECL" "$SEALED_HOME"; then
    gate_error "$(gate_name): the SpanLocate declaration this gate skips by exact text is no longer in $SEALED_HOME verbatim. It may have been reformatted or renamed — or given a bound that is NOT the empty sealing marker, which would make \`T: SpanLocate\` an extra-bounds parameter at every use site in the kernel. Re-derive the skip against what the file now says; do NOT allowlist the file"
    exit 1
  fi
}

# A type parameter bounded TWICE inside one statement, at least one of
# those bounds naming `Real`. `IDENT:` introduces a predicate; `IDENT::`
# is a path and does not.
compound_without_plus() {
  awk '
    function report(stmt, line,   name, seen, dup, carries, rest, bound) {
      if (stmt !~ /(^|[^A-Za-z0-9_])Real([^A-Za-z0-9_]|$)/) return
      split("", seen); split("", dup); split("", carries)
      rest = stmt
      while (match(rest, /[A-Za-z_][A-Za-z0-9_]*[ \t]*:/)) {
        name = substr(rest, RSTART, RLENGTH)
        sub(/[ \t]*:$/, "", name)
        rest = substr(rest, RSTART + RLENGTH)
        if (substr(rest, 1, 1) == ":") { continue }        # a path, not a bound
        seen[name]++
        if (seen[name] == 2) dup[name] = 1
        # The bound list runs to the next separator that can end one.
        if (match(rest, /[,>({;]/)) bound = substr(rest, 1, RSTART - 1)
        else bound = rest
        if (bound ~ /(^|[^A-Za-z0-9_])Real([^A-Za-z0-9_]|$)/) carries[name] = 1
      }
      for (name in dup)
        if (carries[name] == 1) {
          print FNAME ":" line ": " substr(stmt, 1, 200)
          return
        }
    }
    {
      # `FILE:LINE:CODE` split by index, not by regex: a regex here costs
      # more than everything else this program does put together.
      # A line that is not in that shape is a `grep` context separator,
      # and it ENDS the statement — the input is a window around each
      # `Real`, so gluing across a gap would invent a statement.
      p1 = index($0, ":")
      r = substr($0, p1 + 1)
      p2 = index(r, ":")
      ln = substr(r, 1, p2 - 1)
      if (p1 == 0 || p2 == 0 || ln + 0 == 0) { stmt = ""; prev = 0; next }
      f = substr($0, 1, p1 - 1)
      code = substr(r, p2 + 1)
      if (f != FNAME || ln + 0 != prev + 1) { FNAME = f; stmt = ""; sline = 0 }
      prev = ln + 0
      while (length(code) > 0) {
        if (match(code, /[{};]/)) {
          # RSTART IS SAVED FIRST. `report` runs `match` itself, and RSTART
          # is a GLOBAL — reading it after the call cuts the line at
          # whatever the callee last matched, which is an infinite loop
          # the moment that lands at or before the delimiter.
          cut = RSTART
          if (stmt == "") sline = ln
          stmt = stmt " " substr(code, 1, cut - 1)
          report(stmt, sline)
          stmt = ""; sline = 0
          code = substr(code, cut + 1)
        } else {
          if (stmt == "") sline = ln
          stmt = stmt " " code
          code = ""
        }
      }
    }
    END { if (stmt != "") report(stmt, sline) }
  '
}

gate() {
  gate_require_crate_sources
  gate_sealed_skip_subject
  local near plus twice hits
  # THE PREFILTER IS A FIXED STRING, and it is why this gate costs
  # seconds rather than a minute: every spelling below needs the token
  # `Real` in the same statement, so the two real matchers only ever see
  # a window around one. Twelve lines of context is the claim — a `where`
  # clause more than twelve lines from its `Real` predicate is outside
  # what this gate matches, and rustfmt does not produce one.
  near=$(gate_rust_code "${GATE_SOURCE_FILES[@]}" | grep -F -B12 -A12 Real || true)
  plus=$(printf '%s\n' "$near" \
    | grep -E '(\+[[:space:]]*([A-Za-z0-9_]+::)*Real([^A-Za-z0-9_]|$))|((^|[^A-Za-z0-9_])Real[[:space:]]*\+)' \
    | grep -vF ":$SEALED_DECL" || true)
  twice=$(printf '%s\n' "$near" | compound_without_plus || true)
  hits=$(printf '%s\n%s\n' "$plus" "$twice" | grep -v '^$' || true)
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "found extra bound(s) on Real above — evaluation-code discipline forbids extra bounds on scalar type parameters, in either operand order and whether or not a \`+\` is written"
    exit 1
  fi
  gate_ok "no extra bounds on Real"
}

# THE OPERAND ORDERS ARE SEPARATE CASES, planted one at a time: a single
# fixture carrying both would still fire if only one spelling matched,
# which is exactly the blindness being guarded against.
plant_real_first() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Real + PartialOrd>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_real_second() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: PartialOrd + Real>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_path_qualified() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: PartialOrd + geom_core::Real>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_where_one_line() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T>(_t: T) where T: Real, T: PartialOrd {}\n' > "$1/crates/planted/src/lib.rs"
}

# What rustfmt converges on from the line above, and the reason the
# statement view exists at all.
plant_where_wrapped() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub fn f<T>(_t: T) -> T\n'
    printf 'where\n'
    printf '    T: Real,\n'
    printf '    T: PartialOrd,\n'
    printf '{\n'
    printf '    _t\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

# The generic list and the where clause, split across the two.
plant_split_predicate() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Real>(_t: T) where T: PartialOrd {}\n' > "$1/crates/planted/src/lib.rs"
}

# The comment strip has to be REAL, and each of these three is a way the
# leading-`//` filter got it wrong. A violation hidden behind a block
# comment on one line must still fire.
plant_after_block_comment() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f/* why */<T: Real + PartialOrd>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

# THE NEAR MISSES. Every one of these is correct code or plain prose,
# and a matcher that fires on any of them is the cry-wolf half of S63 —
# the half that already produced an allowlist entry justified in writing
# by a false positive. Bundled rather than planted one at a time: in the
# must-NOT-fire direction any single line firing fails the case, so a
# bundle is strictly stronger than separate fixtures.
plant_prose_and_sole_bounds() {
  mkdir -p "$1/crates/planted/src"
  {
    printf '// never write T: Real + PartialOrd here\n'
    printf '/// Nor `T: PartialOrd + Real` in a doc comment.\n'
    printf '/*\n * Nor T: Real + Ord inside a block comment.\n */\n'
    printf 'pub fn a<T: Real>(_t: T) {} // and not T: Real + Ord in a trailing one\n'
    printf 'pub const S: &str = "T: Real + PartialOrd";\n'
    printf 'pub const R: &str = r#"T: PartialOrd + Real"#;\n'
    printf 'pub fn b<T: Real>(_x: T, _y: T) -> T { _x }\n'
    printf 'pub fn c<T: Real, U: Real>(_x: T, _y: U) {}\n'
    printf 'pub struct P<T: Real> { pub x: T, pub y: T }\n'
    printf 'pub fn d<T: Real>(_t: T) where T::Assoc: Sized {}\n'
  } > "$1/crates/planted/src/lib.rs"
}

# The ratified skip is NARROW, and these two fixtures hold it narrow.
# The first is the real declaration beside an ordinary violation in the
# same file: the gate must still fire, so the skip costs one line rather
# than the file. The second is the edit the skip must NOT survive — the
# sealing marker replaced by a trait with a surface, which is the whole
# reason the entry was ratified.
plant_sealed_home_violation() {
  mkdir -p "$1/crates/geom-core/src/spline"
  {
    printf '%s\n' "$SEALED_DECL"
    printf '}\n'
    printf 'pub fn planted<T: Real + PartialOrd>(_t: T) {}\n'
  } > "$1/$SEALED_HOME"
}

plant_sealed_decl_changed() {
  mkdir -p "$1/crates/geom-core/src/spline"
  printf 'pub trait SpanLocate: PartialOrd + Real {\n}\n' > "$1/$SEALED_HOME"
}

plant_sealed_home_clean() {
  mkdir -p "$1/crates/geom-core/src/spline"
  printf '%s\n}\n' "$SEALED_DECL" > "$1/$SEALED_HOME"
}

gate_selftest() {
  local want="found extra bound(s) on Real above"
  gate_selftest_clean
  gate_selftest_case "$want" plant_real_first
  gate_selftest_case "$want" plant_real_second
  gate_selftest_case "$want" plant_path_qualified
  gate_selftest_case "$want" plant_where_one_line
  gate_selftest_case "$want" plant_where_wrapped
  gate_selftest_case "$want" plant_split_predicate
  gate_selftest_case "$want" plant_after_block_comment
  gate_selftest_case "$want" plant_sealed_home_violation
  gate_selftest_case "no longer in crates/geom-core/src/spline/locate.rs verbatim" plant_sealed_decl_changed
  gate_selftest_passes "prose, string literals and sole Real bounds" plant_prose_and_sole_bounds
  gate_selftest_passes "the ratified SpanLocate declaration" plant_sealed_home_clean
  printf '%s selftest OK: passes a clean fixture, prose/strings/sole bounds, and the ratified SpanLocate line; fires on both operand orders, on a path-qualified Real after the plus, on the one-line and rustfmt-wrapped two-predicate spellings, on a predicate split between the generic list and the where clause, on a bound hidden behind a block comment, on a violation beside the skipped declaration, and on that declaration being given a bound with a surface\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
