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
#     refusal is counted in the session's receipt.
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
# impl).
CALLERS_RE='^crates/sweep/src/(swept|revolve/(surfaces|full))\.rs:'
DEFINITIONS_RE='^crates/geom-core/src/(real|sym|interval|dual|k_stats)\.rs:'

gate() {
  gate_require_crate_sources
  local hits
  # `.register_equal(` as a CALL, over the code-only view — a doc
  # comment naming the method, or a string literal, is not a call.
  hits=$(gate_rust_code "${GATE_SOURCE_FILES[@]}" \
    | gate_grep -E '\.register_equal\(' \
    | gate_grep -vE "$DEFINITIONS_RE" \
    | gate_grep -vE "$CALLERS_RE" \
    | cut -c1-160)
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "$(gate_name): a call to Real::register_equal outside the ratified constructor sites (this file's header lists them and what a site owes). The door states an AXIOM the tier then believes; a new site is a spec revision, not a call"
    exit 1
  fi
  gate_ok "Real::register_equal is called only from the ratified constructor sites"
}

# The clean fixture carries the shapes that must NOT fire: the trait's
# own default body, a scalar's impl, the ratified caller, and prose that
# names the method.
gate_plant_clean() {
  mkdir -p "$1/crates/geom-core/src" "$1/crates/sweep/src" "$1/crates/topo/src"
  cat > "$1/crates/geom-core/src/real.rs" <<'RS'
pub trait Real {
    fn register_equal(self, _other: Self) -> u8 {
        0
    }
}
RS
  cat > "$1/crates/geom-core/src/interval.rs" <<'RS'
impl Real for Interval {
    fn register_equal(self, other: Self) -> u8 {
        self.meets(other)
    }
}
RS
  cat > "$1/crates/sweep/src/swept.rs" <<'RS'
pub(crate) fn register_rim_identity<T: Real>(rim: Vec3<T>, radius: T) {
    let _ = rim.norm().register_equal(radius);
}
RS
  cat > "$1/crates/topo/src/carrier_eq.rs" <<'RS'
// Never call `a.register_equal(b)` here: this is a decide site.
pub const WHY: &str = "x.register_equal(y)";
RS
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
  gate_selftest_passes "the trait default, a scalar impl, the ratified caller and prose" gate_plant_clean
  printf '%s selftest OK: passes the trait default, a scalar impl, the ratified caller and prose that names the method; fires on a call from another crate, on one hidden behind a block comment, and on a second file inside sweep; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
