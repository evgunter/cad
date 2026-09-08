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
#    BOTH DIRECTORIES PROVE THEIR SUBJECT, on the argument at
#    `DOOR_HOME` below: the exemption is the crate's door and the
#    module's boundary, so what has to still be in the scan is the
#    crate root and the module root, not the directory name. How WIDE
#    either exemption should be is argued there too, and is not what
#    the check answers.
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

# THE THREE SUBJECTS THIS GATE'S EXEMPTIONS ANSWER FOR, held once: the
# filter is BUILT from them, the clean fixture plants them, and
# `gate_require_homes` proves each is a file this scan actually reads.
#
# A DIRECTORY PREFIX'S SUBJECT IS A FILE — the one the directory is
# ABOUT, not the directory itself, and this is the whole argument for
# the two names below. `crates/pncad/src` is a crate's curated door and
# `crates/pncad-py/src/py` a module's FFI boundary; what makes either
# path that thing, rather than a name a rename left behind, is that the
# crate root and the module root are still there.
#
# PROVING THE DIRECTORY INSTEAD — it exists, or some scanned file lives
# under it — is satisfied by the file the prefix exempts: take the
# directory away, write anything at the vacated path, and the exemption
# has conjured the resident that licenses it. Anchored at the root that
# reds, because the root is what left and the file that replaced it is
# not one.
#
# THE ROOT HAS TO LIE UNDER THE PREFIX IT LICENSES, and that is the
# condition under which anchoring is STRONGER than the directory test
# rather than merely different: a root inside the prefix is also a
# scanned resident of it, so membership comes with it. It holds BY
# CONSTRUCTION below — the directory is held once and the root is that
# directory plus rustc's name for its root — and the pair is written
# that way round because the other way round loses it. Derived from the
# root, the prefix is whatever directory the root sits in, so
# re-anchoring `py/mod.rs` at rustc's other spelling of the same module,
# `py.rs`, would silently widen the exemption from `src/py/` to `src/`
# instead of reding. A reorganisation to `py.rs` moves the prefix too,
# and the pair is re-argued rather than retyped.
#
# HOW WIDE EACH EXEMPTION SHOULD BE is a different question, argued per
# seam and not answered here. The door prefix is crate-granular by its
# own argument — the façade crate is where a user's tolerance BECOMES a
# witness, so any file in it can be the place that happens — while the
# pyo3 prefix's reachability argument holds alike for every file under
# `src/py`. This check says the place is still there; it does not say
# the place is the right size.
#
# THE THIRD EXEMPTION, `^crates/[^/]+/src/bin/`, IS NOT IN THIS LIST
# and is checked by nothing: it names a cargo CONVENTION — anything
# under `src/bin/` is a bin target by construction — so a demand that
# some crate HAVE a `src/bin/` resident would red on a correct tree with
# no bin target. Whether a convention class owes a subject check at all,
# and what it would be anchored at, is an open ruling:
# `work/gates/directory-prefix-skips-have-no-subject-check.md`, asked of
# Ev in PR #2171. Until it is ruled the prefix is exercised by a fixture
# (`plant_in_bin`) and proved by no check.
HOME_FILE=crates/geom-core/src/tolerance.rs
HOME_SUBJECT='the file that DEFINES witness, where minting one is the definition and not an ambient read'
DOOR_DIR=crates/pncad/src
DOOR_HOME=$DOOR_DIR/lib.rs
DOOR_SUBJECT="everything under $DOOR_DIR, the curated document/authoring door whose whole job is to be the place a program starts"
FFI_DIR=crates/pncad-py/src/py
FFI_HOME=$FFI_DIR/mod.rs
FFI_SUBJECT="everything under $FFI_DIR, the pyo3 boundary where a PYTHON program starts"

# THE SKIP A LICENSED DIRECTORY IS: its path, escaped as an ERE and
# anchored at the start of the record, which every view emits as
# `FILE:LINE:TEXT`. Built here so the directory is spelled once and the
# filter, the subject and the fixtures all read the same name — the twin
# spelling `lib.sh`'s exact-skip contract refuses.
gate_licensed_prefix() {
  printf '^%s/' "$(gate_ere_escape "$1")"
}

gate() {
  gate_require_crate_sources
  local hits
  # A FILE the scan never reads, rather than a record filtered after it:
  # the test-only modules leave the file set, so the count this gate
  # prints names what it actually read.
  gate_production_sources
  # AFTER THE SCAN SET, BEFORE THE SCAN — `gate_require_homes` reads the
  # set the line above decided, and its header says why there.
  gate_require_homes "$HOME_SUBJECT" "$HOME_FILE"
  gate_require_homes "$DOOR_SUBJECT" "$DOOR_HOME"
  gate_require_homes "$FFI_SUBJECT" "$FFI_HOME"
  # THE TWO LICENSED PREFIXES, captured in statements of their own so a
  # builder that could not run ends the gate here rather than inside a
  # substitution (`lib.sh` §"A refusal a substitution would swallow").
  local door_prefix ffi_prefix
  door_prefix=$(gate_licensed_prefix "$DOOR_DIR")
  ffi_prefix=$(gate_licensed_prefix "$FFI_DIR")
  hits=$(gate_rust_code --skip-cfg-test "${GATE_PRODUCTION_FILES[@]}" \
    | gate_grep -E 'Tol::witness|tolerance::witness' \
    | gate_grep -vE "$(gate_record_anchor "$HOME_FILE")" \
    | gate_grep -vE "$door_prefix" \
    | gate_grep -vE "$ffi_prefix" \
    | gate_grep -vE '^crates/[^/]+/src/bin/')
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "kernel library code minted a tolerance witness instead of receiving one. Tol::witness() — and its façade spelling pncad::tolerance::witness() — commits the run's eps: it is an entry-point act (a main under src/bin, a test, the pncad door). Take \`tol: Tol\` as a parameter and pass it down — a witness minted mid-library is the ambient read the parameter exists to replace."
    exit 1
  fi
  gate_ok "no kernel library code mints a tolerance witness"
}

# EVERY SUBJECT IS IN THE CLEAN FIXTURE, which is `lib.sh`'s exact-skip
# contract read for a whole-file skip: a skip no fixture exercises is
# dead in every case, and an anchor that over-narrows is then noticed by
# nobody. Each plant MINTS the witness its exemption covers, so the
# clean case reds the moment that exemption stops covering it — the
# defining file for the anchored home skip, the crate root for the
# curated door's prefix, the module root for the pyo3 boundary's.
gate_plant_clean() {
  gate_plant_clean_sources "$1"
  local home
  for home in "$HOME_FILE" "$DOOR_HOME" "$FFI_HOME"; do
    mkdir -p "$1/${home%/*}"
    printf 'pub fn witness() -> Tol { Tol::witness() }\n' > "$1/$home"
  done
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

# THE VACATED-PATH ROUTE AS THE REVIEW WROTE IT, kept as a record: the
# refusal is terminal before the scan, so this reds for the same reason
# the home-gone case does and what it adds is the minting file the
# directory test would have been satisfied by.
plant_door_renamed_away_then_rewritten() {
  rm -rf "$1/$DOOR_DIR"
  mkdir -p "$1/$DOOR_DIR"
  printf 'pub fn eps() -> f64 { geom_core::Tol::witness().eps() }\n' \
    > "$1/$DOOR_DIR/new.rs"
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
  gate_selftest_case "$DOOR_HOME is not a file under" plant_door_renamed_away_then_rewritten
  # EACH HOME WITH THE SUBJECT ITS SKIP CARRIES, so a subject attached to
  # the wrong home reds here rather than passing on the path alone.
  #
  # THE OUT-OF-SCAN CASE IS A PROXY FOR A CRATE ROOT. `gate_plant_home_
  # unscanned` mounts a home test-only, and no crate root can be mounted
  # that way at all — rustc reaches `lib.rs` as the target's entry, not
  # through a `mod` declaration — so for `$DOOR_HOME` the fixture
  # exercises the resolver's basename rule rather than a shape a tree
  # could take. What it proves there is the guard and not the route: the
  # gate reds whenever a home leaves the scan set, which for a crate root
  # is reachable by a symlink or a narrowing instead.
  gate_selftest_homes --narrowed \
    --subject "$HOME_SUBJECT" "$HOME_FILE" \
    --subject "$DOOR_SUBJECT" "$DOOR_HOME" \
    --subject "$FFI_SUBJECT" "$FFI_HOME"
  printf '%s selftest OK: passes a clean fixture carrying the file that DEFINES witness and the two roots its directory prefixes exempt, prose/block-comment/string-literal mentions of the call, the same call inside a #[cfg(test)] module, and a bin target under src/bin; fires on a witness minted in library code, on the pncad::tolerance::witness facade spelling, on a main written outside src/bin, at the colon-carrying path a home skip that ends at `:` exempts, and where a directory prefix outlives the crate root it exempts and a new file mints the witness at the old path; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
