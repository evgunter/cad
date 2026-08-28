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
#    the suite's discipline is already one process per eps.
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

# Files whose whole content is test code because their `mod` line is
# `#[cfg(test)]`. Resolved from the DECLARING file's directory, both
# spellings (`x.rs` and `x/mod.rs`), so a renamed module cannot
# quietly re-enter the scan as shipped code.
cfg_test_modules() {
  # A `#[cfg(test)]` may be followed by further attributes before the
  # `mod` line, and `#[path = "..."]` renames the file outright — both
  # appear in this tree, so neither may be assumed away.
  awk '
    FNR == 1 { armed = 0; path = "" }
    /^[[:space:]]*#\[cfg\(test\)\][[:space:]]*$/ { armed = 1; path = ""; next }
    armed && /^[[:space:]]*#\[path[[:space:]]*=/ {
      if (match($0, /"[^"]+"/)) path = substr($0, RSTART + 1, RLENGTH - 2)
      next
    }
    armed && /^[[:space:]]*#\[/ { next }
    armed && /^[[:space:]]*(pub(\([a-z]+\))?[[:space:]]+)?mod[[:space:]]+[a-z_0-9]+[[:space:]]*;/ {
      name = $0
      sub(/^[[:space:]]*(pub(\([a-z]+\))?[[:space:]]+)?mod[[:space:]]+/, "", name)
      sub(/[[:space:]]*;.*$/, "", name)
      dir = FILENAME
      sub(/\/[^\/]*$/, "", dir)
      if (path != "") print dir "/" path
      else { print dir "/" name ".rs"; print dir "/" name "/" }
      armed = 0; next
    }
    { armed = 0 }
  ' "${GATE_SOURCE_FILES[@]}"
}

gate() {
  gate_require_crate_sources
  local excluded hits
  excluded=$(cfg_test_modules | sort -u)
  hits=$(gate_rust_code --skip-cfg-test "${GATE_SOURCE_FILES[@]}" \
    | gate_grep -E 'Tol::witness|tolerance::witness' \
    | gate_grep -vE '^crates/geom-core/src/tolerance\.rs:' \
    | gate_grep -vE '^crates/pncad/src/' \
    | gate_grep -vE '^crates/pncad-py/src/py/' \
    | gate_grep -vE '^crates/[^/]+/src/bin/' \
    | { if [ -n "$excluded" ]; then gate_grep -vF -f <(printf '%s\n' "$excluded" | sed 's#/$#/#; s#\.rs$#.rs:#'); else cat; fi })
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "kernel library code minted a tolerance witness instead of receiving one. Tol::witness() — and its façade spelling pncad::tolerance::witness() — commits the run's eps: it is an entry-point act (a main under src/bin, a test, the pncad door). Take \`tol: Tol\` as a parameter and pass it down — a witness minted mid-library is the ambient read the parameter exists to replace."
    exit 1
  fi
  gate_ok "no kernel library code mints a tolerance witness"
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

gate_selftest() {
  local want="kernel library code minted a tolerance witness"
  gate_selftest_clean
  gate_selftest_case "$want" plant
  gate_selftest_case "$want" plant_facade_spelling
  gate_selftest_case "$want" plant_main_outside_bin
  gate_selftest_passes "the call named in prose, a block comment and a string literal" plant_prose_only
  gate_selftest_passes "the same call inside a #[cfg(test)] module" plant_in_cfg_test
  gate_selftest_passes "a bin target's main under src/bin" plant_in_bin
  printf '%s selftest OK: passes a clean fixture, prose/block-comment/string-literal mentions of the call, the same call inside a #[cfg(test)] module, and a bin target under src/bin; fires on a witness minted in library code, on the pncad::tolerance::witness facade spelling, and on a main written outside src/bin\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
