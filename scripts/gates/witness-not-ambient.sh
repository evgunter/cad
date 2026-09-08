#!/usr/bin/env bash
# witness-not-ambient.sh — kernel library code RECEIVES the run
# tolerance, it does not mint it. ci.yml's "the witness is not an
# ambient read" step and local-scripts/ci-local.sh's discipline row
# both call this file.
#
# WHY THIS EXISTS. `Tol` (geom-core `tolerance.rs`) is a zero-sized
# witness that the run's tolerance is committed: it has exactly one
# inhabitant, so a `tol: Tol` parameter names a function's
# eps-dependence without carrying a value and without making a second
# eps constructible. That buys the signature. What it does NOT buy on
# its own is that the signature is the ONLY way in — a function can
# take `tol: Tol`, ignore it, and call `Tol::witness()` in its body,
# which is the ambient read the parameter was supposed to replace,
# now wearing the parameter as camouflage. This gate is the half the
# type system cannot do.
#
# THE RULE. `Tol::witness()` is an ENTRY-POINT act. It belongs where
# a run begins — a `main`, a test, the curated `pncad` door — and
# nowhere else. Library code under crates/*/src takes the witness as
# a parameter from its caller, up to whichever entry point minted it.
#
# WHAT IS NOT SCANNED, and why each is sound:
#  - `#[cfg(test)]` blocks (via --skip-cfg-test) and whole modules
#    declared `#[cfg(test)] mod x;` — a test IS an entry point, and
#    the suite's discipline is already one process per eps. WHERE such
#    a module lives is not decided here: `gate_test_only_mounts` places
#    it and `gate_filter_test_only_paths` takes it out of the scan,
#    under `lib.sh`'s §"WHERE A TEST-ONLY MODULE LIVES".
#  - crates/geom-core/src/tolerance.rs — it DEFINES `witness`.
#  - crates/pncad/src — the curated document/authoring door, whose
#    whole job is to be the place a program starts.
#  - crates/pncad-py/src/py — the pyo3 FFI boundary, which is where a
#    PYTHON program starts. This one is discharged by REACHABILITY
#    before the argument above is needed: `Tol` is a Rust ZST and
#    pyo3 cannot carry it across the boundary, so there is no caller
#    on the far side that could hold a witness to pass in. Note the
#    path is `src/py`, not the whole crate: pncad-py's non-FFI
#    modules are ordinary library code and are scanned.
#  - crates/*/src/bin/ — a BINARY TARGET's `main`, which THE RULE
#    above already names as an entry point ("a `main`, a test, the
#    curated `pncad` door"). Not an exemption so much as the rule's
#    first word finally having a resident: until `crates/viewer`
#    grew a bin target, every `main` in this repo lived under
#    `demos/` or `tools/`, which this gate never scanned, so the
#    case had never come up. It is sound for the reason the rule
#    gives: a bin target is not library code — nothing can call
#    into it, so nothing downstream can inherit an ambient read
#    from it — and it is where a run begins.
#    NARROW ON PURPOSE: only `src/bin/`, cargo's own convention for
#    "this file is a program". A `main` written anywhere else in
#    `src/` is scanned like the library code it sits beside.
#
# TWO SPELLINGS ARE MATCHED, and this is the whole of the gate's
# completeness argument. `Tol::witness` is the kernel's; and since
# GUI-0, `pncad::tolerance::witness` is the FAÇADE's — one thin
# wrapper over the same call, public, in the crate every consumer
# already depends on. Matching only the first left the second a
# general bypass: identical semantics, opposite verdicts, which is
# exactly what a GUI-0 reviewer's planted differential measured.
#
# WHAT THE PATTERN STILL CANNOT SEE, stated so a bypass hunt starts
# here rather than from scratch: a `use pncad::tolerance::witness;`
# followed by a bare `witness()` call. Both spellings matched are
# QUALIFIED forms, and matching the bare name would fire on every
# function in the tree called `witness`. Widening this is a real
# option (match the `use` line as well), deliberately not taken:
# it trades a false-negative nobody has written for a false-positive
# class, and the import itself is visible in review at the one place
# it would have to appear.
#
# THE COMPANION GATE is no-ambient-env.sh, which forbids the
# environment read; this one forbids the ambient tolerance read. They
# are the same principle at two layers: a value that decides shipped
# behaviour arrives through a reviewable call site, or it does not
# arrive.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# THE FILE that DEFINES `witness`, held once: the filter's exemption is
# built from it and the clean fixture plants it. The three exemptions
# below it are a DIRECTORY prefix twice and a path CLASS once —
# `crates/*/src/bin/` names a cargo convention and not a file — so they
# have no `FILE:LINE:` shape to pin and stay prefixes on purpose.#
# ONE LIST, AND ONE DIRECTION PROVED. The fixture->filter direction reds
# the clean fixture the moment a planted home stops being exempt; the
# filter->fixture direction is convention and not a check — a filter
# naming a home no fixture plants stays green here, and catching that is
# the subject-half residue's job
# (`work/gates/whole-file-skips-do-not-check-their-subject.md`).
HOME_FILE=crates/geom-core/src/tolerance.rs

gate() {
  gate_require_crate_sources
  local hits
  # A FILE the scan never reads, rather than a record filtered after it:
  # the test-only modules leave the file set, so the count this gate
  # prints names what it actually read.
  gate_production_sources
  hits=$(gate_rust_code --skip-cfg-test "${GATE_PRODUCTION_FILES[@]}" \
    | gate_grep -E 'Tol::witness|tolerance::witness' \
    | gate_grep -vE "$(gate_record_anchor "$HOME_FILE")" \
    | gate_grep -vE '^crates/pncad/src/' \
    | gate_grep -vE '^crates/pncad-py/src/py/' \
    | gate_grep -vE '^crates/[^/]+/src/bin/')
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "kernel library code minted a tolerance witness instead of receiving one. Tol::witness() — and its façade spelling pncad::tolerance::witness() — commits the run's eps: it is an entry-point act (a main under src/bin, a test, the pncad door). Take \`tol: Tol\` as a parameter and pass it down — a witness minted mid-library is the ambient read the parameter exists to replace."
    exit 1
  fi
  gate_ok "no kernel library code mints a tolerance witness"
}

# THE DEFINING FILE IS IN THE CLEAN FIXTURE, which is `lib.sh`'s
# exact-skip contract read for a whole-file skip: a skip no fixture
# exercises is dead in every case, and an anchor that over-narrows is
# then noticed by nobody. The home mints the witness it is the home OF,
# so the clean case reds the moment the exemption stops covering it.
gate_plant_clean() {
  gate_plant_clean_sources "$1"
  mkdir -p "$1/${HOME_FILE%/*}"
  printf 'pub fn witness() -> Tol { Tol::witness() }\n' > "$1/$HOME_FILE"
}

# The home followed by a colon that is not a line number — one of the
# three shapes `gate_record_anchor`'s header enumerates, and the one a
# skip that ends at `:` exempts.
plant_colon_after_the_home_that_is_not_a_line_number() {
  printf 'pub fn eps() -> f64 { geom_core::Tol::witness().eps() }\n' \
    > "$1/$HOME_FILE:x.rs"
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn eps() -> f64 { geom_core::Tol::witness().eps() }\n' \
    > "$1/crates/planted/src/lib.rs"
}

# THE NEAR MISSES: the call named in prose and in a string literal is
# how this gate's own documentation spells the thing it forbids, and a
# gate reading literals as code reds on its own header.
plant_prose_only() {
  mkdir -p "$1/crates/planted/src"
  {
    printf '//! Never call Tol::witness here - take a tol: Tol parameter.\n'
    printf '/*\n * Nor Tol::witness() inside a block comment.\n */\n'
    printf 'pub const WHY: &str = "Tol::witness()";\n'
    printf 'pub fn ok(a: f64) -> f64 { a } // nor Tol::witness() in a trailing one\n'
  } > "$1/crates/planted/src/lib.rs"
}

# A test module IS an entry point, so the same call inside one passes.
plant_in_cfg_test() {
  mkdir -p "$1/crates/planted/src"
  {
    printf 'pub fn ok(a: f64) -> f64 { a }\n'
    printf '#[cfg(test)]\nmod tests {\n'
    printf '    #[test]\n    fn t() { let _ = geom_core::Tol::witness(); }\n'
    printf '}\n'
  } > "$1/crates/planted/src/lib.rs"
}

# THE FAÇADE SPELLING. `pncad::tolerance::witness()` is the same act
# through one thin wrapper; before GUI-0's fix pass this passed the
# gate while the line above it fired, which is a general bypass rather
# than a near miss.
plant_facade_spelling() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn eps() -> f64 { pncad::tolerance::witness().get().eps }\n' \
    > "$1/crates/planted/src/lib.rs"
}

# A BIN TARGET's main IS an entry point (THE RULE's first word), so the
# same call there passes — and only under `src/bin/`.
plant_in_bin() {
  mkdir -p "$1/crates/planted/src/bin"
  printf 'pub fn ok(a: f64) -> f64 { a }\n' > "$1/crates/planted/src/lib.rs"
  printf 'fn main() { let _ = pncad::tolerance::witness(); }\n' \
    > "$1/crates/planted/src/bin/prog.rs"
}

# ... but a `main` written OUTSIDE `src/bin/` is library-adjacent code
# and stays scanned: the exclusion is cargo's path convention, not the
# word `main`.
plant_main_outside_bin() {
  mkdir -p "$1/crates/planted/src"
  printf 'fn main() { let _ = pncad::tolerance::witness(); }\n' \
    > "$1/crates/planted/src/lib.rs"
}

# THE BREACH `lib.sh`'s test-module cases plant, and the only thing this
# gate supplies to them: a minted witness, appended to a file whose
# directory they have already made.
plant_witness_at() {
  printf 'pub fn eps() -> f64 { geom_core::Tol::witness().eps() }\n' >> "$1"
}

gate_selftest() {
  local want="kernel library code minted a tolerance witness"
  gate_selftest_clean
  # A `grep` that cannot run is the failure this gate cannot see for
  # itself: it produces no hits, and no hits is what a clean tree
  # produces. Proved here rather than asserted, because before
  # `gate_grep` this exact fixture printed OK and exited 0.
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant
  gate_selftest_case "$want" plant_facade_spelling
  gate_selftest_case "$want" plant_main_outside_bin
  gate_selftest_case "$want" plant_colon_after_the_home_that_is_not_a_line_number
  gate_selftest_passes "the call named in prose, a block comment and a string literal" plant_prose_only
  gate_selftest_passes "the same call inside a #[cfg(test)] module" plant_in_cfg_test
  gate_selftest_passes "a bin target's main under src/bin" plant_in_bin
  gate_selftest_test_module_homes "$want" plant_witness_at
  printf '%s selftest OK: passes a clean fixture carrying the file that DEFINES witness, prose/block-comment/string-literal mentions of the call, the same call inside a #[cfg(test)] module, and a bin target under src/bin; fires on a witness minted in library code, on the pncad::tolerance::witness facade spelling, on a main written outside src/bin, and at the colon-carrying path a home skip that ends at `:` exempts; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
