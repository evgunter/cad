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
# not about the character `+`, and not about a LINE. Rust spells the
# compound four ways and this gate used to see one of them:
#
#   PLUS, either order       `T: Real + PartialOrd`, `T: PartialOrd + Real`
#   PLUS, WRAPPED            what `rustfmt` makes of a long list:
#                            `T: Real` on one line, `+ PartialOrd,` on
#                            the next
#   TWO PREDICATES           `fn f<T: Real>(..) where T: PartialOrd`
#   TWO PREDICATES, WRAPPED  the same, after rustfmt puts each `where`
#                            predicate on its own line
#
# **Two of the four are what the FORMATTER PRODUCES**, so writing the
# evasion is not deliberate work — it is what happens to a bound list
# that grows. That is S158's ruling, which was recorded against the
# compound-`Bounds` gate one file over and reached this one late: a
# first version of this rewrite built a statement view and applied it
# only to the two-predicate spelling, leaving the plus line-scoped, and
# passed verbatim rustfmt output.
#
# So BOTH matchers run over `lib.sh`'s STATEMENT view — the code-only
# text cut at `{`, `}` and `;`, which is exactly where a generic list
# and its `where` clause end. A parameter bounded twice inside one such
# statement is the compound written without a plus; a `+ Real` anywhere
# in one is the compound written with one, however it is wrapped.
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
# TEXT costs one line and cannot grow.
#
# THE SKIP IS ANCHORED TO ITS FILE, and that is what makes the file
# moving LOUD rather than silent. An unanchored skip would exempt the
# same declaration copied anywhere in the tree, and would turn
# `locate.rs` being renamed into a quiet no-op. Anchored, a moved file
# carries the declaration to a path the skip does not match and the gate
# reds on it. The subject check is the other half, and it speaks in both
# of the remaining directions: the file still there with the text gone,
# and the file gone from the tree — which is a red and not an
# abstention, for the reasons `lib.sh` states at `gate_exact_skip`.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

SEALED_HOME=crates/geom-core/src/spline/locate.rs
SEALED_DECL='pub trait SpanLocate: sealed::Sealed + Real {'
# THE SKIP IS `lib.sh`'s, declared and never spelled: the anchored
# filter, the subject check and the fixtures on both sides of the anchor
# are one mechanism, and the pattern is derived from the plain
# declaration above through the same statement view the scan reads — so
# there is no hand-escaped twin of this line to drift from it.
gate_exact_skip --statements \
  --subject 'the SpanLocate declaration this gate skips by exact text is' \
  --repair "It may have been reformatted or renamed — or given a bound that is NOT the empty sealing marker, which would make \`T: SpanLocate\` an extra-bounds parameter at every use site in the kernel. Re-derive the skip against what the file now says; do NOT allowlist the file" \
  "$SEALED_HOME" "$SEALED_DECL"

# A type parameter bounded TWICE inside one statement, at least one of
# those bounds naming `Real`. `IDENT:` introduces a predicate; `IDENT::`
# is a path and does not. Reads `lib.sh`'s statement view on stdin.
compound_without_plus() {
  GATE_RECORD_LINE_RE="$GATE_RECORD_LINE_RE" awk "$GATE_RECORD_AWK"'
    {
      # WHERE THE STATEMENT STARTS is where the FILE column ends, and
      # that is gate_record_split (lib.sh, section THE RECORD S
      # COLUMNS). Read to the first colon, a record from a path carrying
      # one of its own kept a tail of the PATH in the statement text —
      # and a path segment ending in a colon is exactly what the walk
      # below reads as a bound predicate, so the gate could fire on a
      # parameter no source declares.
      if (!gate_record_split($0)) next
      stmt = GR_TEXT
      if (stmt !~ /(^|[^A-Za-z0-9_])Real([^A-Za-z0-9_]|$)/) next
      split("", seen); split("", dup); split("", carries)
      rest = stmt
      while (match(rest, /[A-Za-z_][A-Za-z0-9_]*[ \t]*:/)) {
        name = substr(rest, RSTART, RLENGTH)
        sub(/[ \t]*:$/, "", name)
        rest = substr(rest, RSTART + RLENGTH)
        if (substr(rest, 1, 1) == ":") continue        # a path, not a bound
        seen[name]++
        if (seen[name] == 2) dup[name] = 1
        # The bound list runs to the next separator that can end one.
        if (match(rest, /[,>({;]/)) bound = substr(rest, 1, RSTART - 1)
        else bound = rest
        if (bound ~ /(^|[^A-Za-z0-9_])Real([^A-Za-z0-9_]|$)/) carries[name] = 1
      }
      for (name in dup)
        if (carries[name] == 1) { print $0; break }
    }
  '
}

gate() {
  gate_require_crate_sources
  gate_exact_skip_subject
  local near plus twice hits
  # THE PREFILTER IS A FIXED STRING over STATEMENTS, and it is why this
  # gate costs seconds rather than a minute: every spelling below needs
  # the token `Real` in the same statement, so the two matchers only
  # ever see statements that contain one. No line window, and therefore
  # no assumption about which lines are adjacent.
  near=$(gate_rust_code --statements "${GATE_SOURCE_FILES[@]}" | gate_grep -F Real)
  plus=$(printf '%s\n' "$near" \
    | gate_grep -E '(\+[[:space:]]*([A-Za-z0-9_]+::)*Real([^A-Za-z0-9_]|$))|((^|[^A-Za-z0-9_])Real[[:space:]]*\+)' \
    | gate_exact_skip_filter)
  # `compound_without_plus` is awk, which has no "no match" status: it
  # exits 0 having printed nothing. A non-zero from it is a reader that
  # died, so it is NOT tolerated here either.
  twice=$(printf '%s\n' "$near" | compound_without_plus)
  hits=$(printf '%s\n%s\n' "$plus" "$twice" | gate_grep -v '^$' | sort -u)
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "found extra bound(s) on Real above — evaluation-code discipline forbids extra bounds on scalar type parameters, in either operand order, whether or not a \`+\` is written, and however rustfmt wrapped it"
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

# WHAT RUSTFMT MAKES OF A LONG BOUND LIST, in both positions it can
# appear. These two are verbatim `rustfmt --edition 2021` output, and
# the first version of this gate's rewrite passed both: the plus matcher
# was line-scoped, so neither line carried both tokens. `+ ` at column
# zero is the resting state of this tree, not a corner case.
plant_plus_wrapped_where() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub fn f<T>(_t: T) -> T\n'
    printf 'where\n'
    printf '    T: Real\n'
    printf '        + PartialOrd,\n'
    printf '{\n'
    printf '    _t\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

plant_plus_wrapped_generics() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub fn f<\n'
    printf '    T: Real\n'
    printf '        + PartialOrd,\n'
    printf '>(_t: T) {}\n'
  } > "$1/crates/planted/src/lib.rs"
}

# A BLANK LINE inside the wrapped clause. The statement view used to
# require consecutive line numbers, and `lib.sh` drops an empty line —
# so a blank line reset the statement and the gate went blind mid-clause.
plant_wrapped_with_blank_line() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub fn f<T>(_t: T) -> T\n'
    printf 'where\n'
    printf '    T: Real,\n'
    printf '\n'
    printf '    T: PartialOrd,\n'
    printf '{\n'
    printf '    _t\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
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

# THE PATH READ AS CODE, which is the cry-wolf half of the file column.
# A `:` is legal in a path here and in git, and this walk reads
# `NAME:` as a bound predicate — so a statement text that begins inside
# the PATH hands it predicates the source never wrote. Read to the first
# colon, the text of this record began at `T:x.rs:…`, the walk counted a
# second predicate on `T` and the file's ONE sole `Real` bound was
# reported as a parameter bounded twice. The path needs three colons to
# reach it (the first two are consumed as the file and line columns),
# which is why this is the near-miss case and not a hit: it fires on a
# file whose only bound is sole.
plant_colon_path_read_as_code() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T>(_t: T) where T: Real {}\n' \
    > "$1/crates/planted/src/a:b:T:x.rs"
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

# The clean tree carries the skip's own subject, for the reason
# `lib.sh` gives at `gate_exact_skip`.
gate_plant_clean() {
  gate_plant_clean_sources "$1"
  gate_exact_skip_plant_home "$1"
}

gate_selftest() {
  local want="found extra bound(s) on Real above"
  gate_selftest_clean
  # A `grep` that cannot run is the failure this gate cannot see for
  # itself: it produces no hits, and no hits is what a clean tree
  # produces. Proved here rather than asserted, because before
  # `gate_grep` this exact fixture printed OK and exited 0.
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_real_first
  gate_selftest_case "$want" plant_real_second
  gate_selftest_case "$want" plant_path_qualified
  gate_selftest_case "$want" plant_where_one_line
  gate_selftest_case "$want" plant_where_wrapped
  gate_selftest_case "$want" plant_wrapped_with_blank_line
  gate_selftest_case "$want" plant_plus_wrapped_where
  gate_selftest_case "$want" plant_plus_wrapped_generics
  gate_selftest_case "$want" plant_split_predicate
  gate_selftest_case "$want" plant_after_block_comment
  gate_selftest_case "$want" plant_sealed_home_violation
  gate_selftest_case "no longer in crates/geom-core/src/spline/locate.rs verbatim" plant_sealed_decl_changed
  gate_selftest_passes "prose, string literals and sole Real bounds" plant_prose_and_sole_bounds
  gate_selftest_passes "a sole Real bound in a file whose PATH carries colons, which the statement text must not begin inside" \
    plant_colon_path_read_as_code
  gate_exact_skip_selftest "$want"
  printf '%s selftest OK: passes a clean fixture, prose/strings/sole bounds and a sole bound in a file whose PATH carries colons, which the statement text must not begin inside; fires on both operand orders, on a path-qualified Real after the plus, on rustfmt-wrapped plus in the where clause AND in the generic list, on the one-line and wrapped two-predicate spellings, across a blank line inside a where clause, on a predicate split between the generic list and the where clause, on a bound hidden behind a block comment, on a violation beside the skipped declaration, and on that declaration being given a bound with a surface; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
