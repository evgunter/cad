#!/usr/bin/env bash
# register-equal-allowlist.sh — `Real::register_equal` is called from
# RATIFIED CONSTRUCTOR SITES only. ONE home; ci.yml's
# "register_equal call-site allowlist" step and
# local-scripts/ci-local.sh's discipline row both call this file.
#
# WHY IT EXISTS. `Real::register_equal` (M10-9, ERROR-DESIGN E12's
# provenance reserve) hands every generic `T: Real` body a value
# COMPARISON — the exact capability `real.rs`'s evaluation-code
# discipline exists to keep out of that position, and one that
# `no-extra-real-bounds` structurally cannot see, because reaching it
# adds no bound: `T: Real` is already enough. Two independent reviews
# of M10-9 named that (R1 m5, R2 MINOR-3), and this gate is the answer:
# the capability stays, and the SITES that may use it are a list a
# reviewer can read.
#
# WHAT A RATIFIED SITE OWES, and the gate cannot check any of it — it
# checks only that the site is on the list, so the list is where the
# argument has to live:
#
#   * a doc comment carrying the THEOREM the registration states, in
#     terms of the construction (spec claim 9). The witness cannot tell
#     an identity from a coincidence (`Real::register_equal`'s own
#     contract says so in full), so the proof is the whole soundness
#     argument;
#   * a planted lie pinned typed, so the refusal path is exercised;
#   * the typed answer HANDLED — the method is `#[must_use]`, and a
#     refusal is counted in the session's receipt. HANDLED, and not
#     ASSERTED on: a registrant's proof is a theorem of the reals, and
#     a configuration at the edge of `f64` representability can
#     contradict it without anything being wrong
#     (`work/m10/the-span-identity-is-not-a-theorem-of-the-floats`).
#
# THE RATIFIED SITES (M10-9, the swept arc carrier's builder — spec §2
# and its amendment A1: the unit of scope is the CONSTRUCTOR, so one
# builder states every same-object identity it guarantees):
#
#   * `crates/sweep/src/swept.rs` — `register_rim_identity`
#     (`‖q_from − c‖ = r`, the sagitta closed form) and
#     `register_span_identity` (`carrier.eval(param_end) = q_to`, the
#     bulge as `tan(θ/4)`). These two functions are the only bodies
#     that call the method; `crates/sweep/src/extrude.rs` reaches the
#     rim identity THROUGH `register_rim_identity` and therefore needs
#     no entry.
#   * `crates/sweep/src/revolve/surfaces.rs` and
#     `crates/sweep/src/revolve/full.rs` — the latitude carriers, which
#     build the same `Circle { u_ref: (q − center).normalize(), radius }`
#     under the same guarantee (R2 MINOR-5: A1's "the unit of scope is
#     the constructor" was applied to one of two). RIM ONLY: neither
#     builder is handed the far endpoint, so the span identity has
#     nothing to be stated about
#     (`work/m10/revolve-carriers-state-only-the-rim`).
#
# WHAT THIS GATE DOES NOT COVER, stated rather than implied: it reads
# source text, so a call reached through a macro, a re-export under
# another name, or a trait object is invisible to it; and it says
# nothing about whether an allowlisted site's registration is TRUE.
# Adding a site here is a design decision — it widens what the tier
# will believe — and belongs in a spec revision, not in a fix pass.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# The files that may CALL the method, and the files that DEFINE or
# re-export it (a definition is not a call site, and the gate would
# otherwise fire on the trait's own default body and on every scalar's
# impl). Held as paths rather than as a hand-written ERE: the filter's
# exemption, `gate_require_homes`'s subject check and the clean fixture
# all read these lists, and what that buys is argued at that check. The
# two are checked as two, because their diagnoses differ in what the
# missing entry would have exempted.
CALLER_SUBJECT='the ratified constructor sites, the only files that may CALL Real::register_equal'
DEFINITION_SUBJECT='the files that DEFINE or re-export Real::register_equal, where a mention is a definition and not a call'
CALLER_HOMES=(
  crates/sweep/src/swept.rs
  crates/sweep/src/revolve/surfaces.rs
  crates/sweep/src/revolve/full.rs
)
DEFINITION_HOMES=(
  crates/geom-core/src/real.rs
  crates/geom-core/src/sym.rs
  crates/geom-core/src/interval.rs
  crates/geom-core/src/dual.rs
  crates/geom-core/src/k_stats.rs
)

gate() {
  gate_require_crate_sources
  gate_require_homes "$DEFINITION_SUBJECT" "${DEFINITION_HOMES[@]}"
  gate_require_homes "$CALLER_SUBJECT" "${CALLER_HOMES[@]}"
  local hits
  # `.register_equal(` as a CALL, over the code-only view — a doc
  # comment naming the method, or a string literal, is not a call.
  hits=$(gate_rust_code "${GATE_SOURCE_FILES[@]}" \
    | gate_grep -E '\.register_equal\(' \
    | gate_grep -vE "$(gate_record_anchor_any "${DEFINITION_HOMES[@]}")" \
    | gate_grep -vE "$(gate_record_anchor_any "${CALLER_HOMES[@]}")" \
    | cut -c1-160)
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "$(gate_name): a call to Real::register_equal outside the ratified constructor sites (this file's header lists them and what a site owes). The door states an AXIOM the tier then believes; a new site is a spec revision, not a call"
    exit 1
  fi
  gate_ok "Real::register_equal is called only from the ratified constructor sites"
}

# EVERY EXEMPTED FILE IS IN THE CLEAN FIXTURE, which is `lib.sh`'s
# exact-skip contract read for a whole-file skip: a skip no fixture
# exercises is dead in every case, and an anchor that over-narrows is
# then noticed by nobody. Each home carries a CALL — the shape the
# matcher reads — so the clean case reds the moment one of them stops
# being covered, and a definition file that today holds only the method
# is exercised as the skip it is. Beside them the fixture keeps the two
# shapes that must not fire on their own terms: the trait's default body
# and prose naming the method, the first two appended to the definition
# files that are their real homes.
gate_plant_clean() {
  local home
  mkdir -p "$1/crates/topo/src"
  for home in "${DEFINITION_HOMES[@]}" "${CALLER_HOMES[@]}"; do
    mkdir -p "$1/${home%/*}"
    printf 'fn f<T: Real>(a: T, b: T) { let _ = a.register_equal(b); }\n' \
      > "$1/$home"
  done
  cat >> "$1/crates/geom-core/src/real.rs" <<'RS'
pub trait Real {
    fn register_equal(self, _other: Self) -> u8 {
        0
    }
}
RS
  cat >> "$1/crates/geom-core/src/interval.rs" <<'RS'
impl Real for Interval {
    fn register_equal(self, other: Self) -> u8 {
        self.meets(other)
    }
}
RS
  cat > "$1/crates/topo/src/carrier_eq.rs" <<'RS'
// Never call `a.register_equal(b)` here: this is a decide site.
pub const WHY: &str = "x.register_equal(y)";
RS
}

# The home followed by a colon that is not a line number — one of the
# three shapes `gate_record_anchor`'s header enumerates, and the one a
# skip that ends at `:` exempts.
plant_colon_after_the_home_that_is_not_a_line_number() {
  local home
  for home in "${DEFINITION_HOMES[@]}" "${CALLER_HOMES[@]}"; do
    printf 'fn f<T: Real>(a: T, b: T) { let _ = a.register_equal(b); }\n' \
      > "$1/$home:x.rs"
  done
}

# An ordinary crate reaching for the door.
plant_call_outside() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f<T: Real>(a: T, b: T) { let _ = a.register_equal(b); }\n' \
    > "$1/crates/topo/src/carrier_eq.rs"
}

# The same, hidden behind a block comment on one line — the shape the
# leading-`//` filters in this directory used to walk past.
plant_call_after_block_comment() {
  mkdir -p "$1/crates/topo/src"
  printf 'fn f<T: Real>(a: T, b: T) { /* why */ let _ = a.register_equal(b); }\n' \
    > "$1/crates/topo/src/carrier_eq.rs"
}

# A second call site inside `sweep`, but not in the ratified file: the
# allowlist is per FILE, and a new file is a new site.
plant_call_in_another_sweep_file() {
  mkdir -p "$1/crates/sweep/src"
  printf 'fn f<T: Real>(a: T, b: T) { let _ = a.register_equal(b); }\n' \
    > "$1/crates/sweep/src/revolve.rs"
}

gate_selftest() {
  local want="a call to Real::register_equal outside the ratified constructor sites"
  gate_selftest_clean
  # A `grep` that cannot run produces no hits, and no hits is what a
  # clean tree produces — the failure this gate cannot see for itself.
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant_call_outside
  gate_selftest_case "$want" plant_call_after_block_comment
  gate_selftest_case "$want" plant_call_in_another_sweep_file
  gate_selftest_case "$want" plant_colon_after_the_home_that_is_not_a_line_number
  gate_selftest_passes "the trait default, a scalar impl, the ratified caller and prose" gate_plant_clean
  gate_selftest_homes "${DEFINITION_HOMES[@]}" "${CALLER_HOMES[@]}"
  printf '%s selftest OK: passes a clean fixture carrying every definition and caller file it exempts, the trait default and prose that names the method; fires on a call from another crate, on one hidden behind a block comment, on a second file inside sweep, and at the colon-carrying path a home skip that ends at `:` exempts; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
