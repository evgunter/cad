#!/usr/bin/env bash
# evalscalar-allowlist.sh — the Bounds gate, over the compound bound's
# NAME. ONE home; ci.yml's "EvalScalar allowlist (the Bounds gate, over
# the name)" step and local-scripts/ci-local.sh's discipline row both
# call this file.
#
# The SAME gate, over the compound bound's NAME (ASM-2A review
# MINOR-4). `editor_core::EvalScalar` is the evaluation-service
# bound — ten supertraits at `editor-core/src/eval/mod.rs`, of which
# `geom_core::Bounds` is the bracket door: `Decide + ContentBits +
# geom_core::Bounds + Send + Sync + topo::AtRestPolicy` (which carries
# `topo::PropsQuadLane` as its own supertrait) `+
# crate::analysis::AxisScalar + crate::analysis::SeedScalar +
# crate::measure::MinClearanceLane + SectionScalar` — declared once at
# the seam the rule above already ratifies, so `eval/parts.rs` names
# the requirement instead of restating `+ Bounds`. THE LIST IS READ FROM
# THAT DECLARATION AND IS NOT CHECKED HERE: what this gate checks is
# where the NAME may be bound, never what the name gathers.
#
# The name is a NAMING, and it must
# stay one: the trait is `pub` (the integration suites need it),
# so without this step any file in any crate could acquire a
# compound Bounds bound invisibly to the grep above. Allowlist =
# exactly the seam real.rs already names, so the two steps agree
# about where the seam is; growing it needs the same ratification
# the Bounds allowlist does. Both bound positions are caught
# (`T: EvalScalar` and `+ EvalScalar`), over `lib.sh`'s code-only view:
# comments and string literals are gone in both directions, where the
# leading-`//` filter this gate carried stripped neither a trailing
# comment nor a block one — and over the STATEMENT view, because
# `rustfmt` wraps a long bound list as `T: Clone\n    + EvalScalar,` and
# a line matcher is blind to the form the formatter produces (S158).
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# THE EVALUATION-SERVICE SEAM, as paths and held once: the filter's
# exemption is built from this list and the clean fixture plants both
# entries, so a file cannot be exempt in one and absent from the other.
SEAM_HOMES=(
  crates/editor-core/src/eval/mod.rs
  crates/editor-core/src/eval/parts.rs
)

# THE EXEMPTION IS ONE PATH PER SEAM FILE, by construction rather than
# by coincidence: `gate_record_anchor` escapes the path and pins the
# `FILE:LINE:` shape, and which paths an anchor missing either part
# would exempt is argued once, at that function in `lib.sh`.
seam_re() {
  local home
  local -a alts=()
  for home in "${SEAM_HOMES[@]}"; do
    alts+=("$(gate_record_anchor "$home")")
  done
  local IFS='|'
  printf '%s' "${alts[*]}"
}

gate() {
  gate_require_crate_sources
  local hits
  hits=$(gate_rust_code --statements "${GATE_SOURCE_FILES[@]}" \
    | gate_grep -E '(:|\+)[[:space:]]*(editor_core::)?EvalScalar([^A-Za-z0-9_]|$)' \
    | gate_grep -vE "$(seam_re)")
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "EvalScalar (a compound Bounds bound by another name) outside the evaluation-service seam — see geom-core/src/real.rs (Bounds scope rule) and editor-core/src/eval/mod.rs; ratify before allowlisting"
    exit 1
  fi
  gate_ok "no EvalScalar bound outside the evaluation-service seam"
}

# THE SEAM IS IN THE CLEAN FIXTURE, which is `lib.sh`'s exact-skip
# contract read for a whole-file skip: a skip no fixture exercises is
# dead in every case, and an anchor that over-narrows is then noticed by
# nobody. Each seam file carries the bound it is exempted FOR, so the
# clean case reds the moment one of the two stops being covered.
gate_plant_clean() {
  gate_plant_clean_sources "$1"
  local home
  for home in "${SEAM_HOMES[@]}"; do
    mkdir -p "$1/${home%/*}"
    printf 'pub fn f<T: editor_core::EvalScalar>(_t: T) {}\n' > "$1/$home"
  done
}

# THE `FILE:LINE:` SHAPE, which a skip ending at `:` does not pin: a
# file whose own path carries a colon after the home reads as the home
# plus a line number and rides the exemption. The path is legal on this
# filesystem and in git, and the anchor's `[0-9]+` is what refuses it —
# `gate_record_anchor` in `lib.sh` argues the reachable set once. One
# seam file stands for both: the anchor is built the same way for each.
plant_colon_after_the_home_that_is_not_a_line_number() {
  printf 'pub fn f<T: editor_core::EvalScalar>(_t: T) {}\n' \
    > "$1/${SEAM_HOMES[0]}:x.rs"
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: editor_core::EvalScalar>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_after_block_comment() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f/* why */<T: editor_core::EvalScalar>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

plant_plus_position() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn f<T: Clone + EvalScalar>(_t: T) {}\n' > "$1/crates/planted/src/lib.rs"
}

# The same bound as rustfmt leaves a long list.
plant_plus_wrapped() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub fn f<T>(_t: T)\n'
    printf 'where\n'
    printf '    T: Clone\n'
    printf '        + EvalScalar,\n'
    printf '{\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

plant_prose_only() {
  mkdir -p "$1/crates/planted/src"
  {
    printf '//! `T: EvalScalar` is the evaluation-service bound.\n'
    printf '/// Never write `+ EvalScalar` outside the seam.\n'
    printf '/*\n * Nor T: editor_core::EvalScalar in a block comment.\n */\n'
    printf 'pub const WHY: &str = "T: EvalScalar";\n'
    printf 'pub fn ok<T: Real>(_t: T) {} // nor : EvalScalar in a trailing one\n'
    printf 'pub fn near<T: EvalScalarish>(_t: T) {}\n'
  } > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="EvalScalar (a compound Bounds bound by another name)"
  gate_selftest_clean
  # A `grep` that cannot run is the failure this gate cannot see for
  # itself: it produces no hits, and no hits is what a clean tree
  # produces. Proved here rather than asserted, because before
  # `gate_grep` this exact fixture printed OK and exited 0.
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant
  gate_selftest_case "$want" plant_plus_position
  gate_selftest_case "$want" plant_plus_wrapped
  gate_selftest_case "$want" plant_after_block_comment
  gate_selftest_case "$want" plant_colon_after_the_home_that_is_not_a_line_number
  gate_selftest_passes "prose, doc comments, a string literal and a longer name starting with EvalScalar" plant_prose_only
  printf '%s selftest OK: passes a clean fixture carrying both seam files, prose/doc/string mentions and `EvalScalarish`; fires in both bound positions, on a rustfmt-wrapped plus, on a bound hidden behind a block comment, and at the colon-carrying path a home skip that ends at `:` exempts; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
