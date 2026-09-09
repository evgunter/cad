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
# a run begins — a test, the curated `pncad` door, and the one
# allowlisted binary `main` — and nowhere else. Library code under
# crates/*/src takes the witness as a parameter from its caller, up to
# whichever entry point minted it. WHICH `main`s ARE ENTRY POINTS HERE
# IS A LIST AND NOT A SHAPE, by Ev's ruling: being a `main`, or sitting
# under cargo's `src/bin/`, buys nothing on its own — the exemption
# names a file. The argument for that, and the way back to a shape, is
# at `BIN_HOME` below.
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
#  - crates/viewer/src/bin/viewer.rs — the one binary `main` THE RULE
#    above allowlists. It is sound for the reason the rule gives: a
#    bin target is not library code — nothing can call into it, so
#    nothing downstream can inherit an ambient read from it — and it
#    is where a run's eps is chosen. It is a NAME and not a shape:
#    a `main` written anywhere else — elsewhere under `src/`, or
#    under another crate's `src/bin/` — is scanned like the library
#    code it sits beside, and its own entry-point argument is made
#    by adding it here. Argued at `BIN_HOME` below, which is also
#    where the subject check that files this skip beside the other
#    three lives.
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

# EVERY SUBJECT THIS GATE'S EXEMPTIONS ANSWER FOR, held once: the
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
HOME_FILE=crates/geom-core/src/tolerance.rs
HOME_SUBJECT='the file that DEFINES witness, where minting one is the definition and not an ambient read'
DOOR_DIR=crates/pncad/src
DOOR_HOME=$DOOR_DIR/lib.rs
DOOR_SUBJECT="everything under $DOOR_DIR, the curated document/authoring door whose whole job is to be the place a program starts"
FFI_DIR=crates/pncad-py/src/py
FFI_HOME=$FFI_DIR/mod.rs
FFI_SUBJECT="everything under $FFI_DIR, the pyo3 boundary where a PYTHON program starts"

# THE FOURTH SUBJECT IS ONE FILE BY RULING, where it was cargo's
# `src/bin/` convention before. `^crates/[^/]+/src/bin/` matched
# whatever directory that convention names rather than a place this
# tree has, so it had nothing for `gate_require_homes` to prove and was
# the one skip here that could never say it had gone stale. Today it
# covers exactly one program, and Ev ruled (PR #2171): "i'd somewhat be
# inclined to switch it to allowlist just that one file but put a
# comment to go back to the glob version if we end up having many such
# entrypoints".
#
# SO: IF ENTRY POINTS UNDER `src/bin/` MULTIPLY, GO BACK TO THE GLOB —
# `^crates/[^/]+/src/bin/` in place of this home's anchor — and take
# back what the glob costs, which is this whole row: a convention-class
# skip names no place, so there is no subject to check, and the day the
# last `src/bin/` target is retired the exemption stands over the path
# for whatever is written there next. One file per entry point is the
# price of the check; a list of them is the point at which the price
# stops being worth paying.
BIN_HOME=crates/viewer/src/bin/viewer.rs
BIN_SUBJECT="a binary's \`main\`, where a run's eps is chosen and which nothing can call into, so nothing downstream can inherit an ambient read from it"

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
  gate_require_homes "$BIN_SUBJECT" "$BIN_HOME"
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
    | gate_grep -vE "$(gate_record_anchor "$BIN_HOME")")
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "kernel library code minted a tolerance witness instead of receiving one. Tol::witness() — and its façade spelling pncad::tolerance::witness() — commits the run's eps: it is an entry-point act (a test, the pncad door, or the one binary main this gate allowlists, $BIN_HOME). Take \`tol: Tol\` as a parameter and pass it down — a witness minted mid-library is the ambient read the parameter exists to replace."
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
# curated door's prefix, the module root for the pyo3 boundary's, and
# the viewer's binary for the one this gate allowlists by name.
#
# ONE BODY FOR ALL FOUR, and that is not a shortcut: every skip here is
# anchored on a PATH, so the only thing the fixture needs from a plant
# is that it mints the witness at that path. Nothing about the shape of
# the file is read — the door plant is not a door and the bin plant is
# not a `main` — and a plant dressed up as the thing it stands for would
# be claiming a fidelity the anchor does not have.
gate_plant_clean() {
  gate_plant_clean_sources "$1"
  local home
  for home in "$HOME_FILE" "$DOOR_HOME" "$FFI_HOME" "$BIN_HOME"; do
    mkdir -p "$1/${home%/*}"
    printf 'pub fn witness() -> Tol { Tol::witness() }\n' > "$1/$home"
  done
}

# The home followed by a colon that is not a line number — one of the
# three shapes `gate_record_anchor`'s header enumerates, and the one a
# skip that ends at `:` exempts. ONE PER ANCHORED HOME, so the case
# takes the home as its own argument (`lib.sh`'s planter convention: own
# arguments first, the tree last).
plant_colon_after_the_home_that_is_not_a_line_number() {
  printf 'pub fn eps() -> f64 { geom_core::Tol::witness().eps() }\n' \
    > "$2/$1:x.rs"
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

# A `main` under ANOTHER crate's `src/bin/`, which FIRES. This is the
# fixture the ruling flipped, and it is what `BIN_HOME`'s trade costs
# when it is paid: an unlisted bin target is an ordinary hit.
plant_in_bin() {
  mkdir -p "$1/crates/planted/src/bin"
  printf 'fn main() { let _ = pncad::tolerance::witness(); }\n' \
    > "$1/crates/planted/src/bin/prog.rs"
}

# ... and a `main` written outside `src/bin/` altogether fires for the
# same reason one level out: the exemption is a NAMED FILE, and the word
# `main` in a scanned file buys nothing. The pair is kept because the two
# reds have different causes — this one would fire under the glob too.
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
  gate_selftest_case "$want" \
    plant_colon_after_the_home_that_is_not_a_line_number "$HOME_FILE"
  gate_selftest_case "$want" \
    plant_colon_after_the_home_that_is_not_a_line_number "$BIN_HOME"
  gate_selftest_case "$want" plant_in_bin
  gate_selftest_passes "the call named in prose, a block comment and a string literal" plant_prose_only
  gate_selftest_passes "the same call inside a #[cfg(test)] module" plant_in_cfg_test
  gate_selftest_test_module_homes "$want" plant_witness_at
  gate_selftest_case "$DOOR_HOME is not a file under" plant_door_renamed_away_then_rewritten
  # EACH HOME WITH THE SUBJECT ITS SKIP CARRIES, so a subject attached to
  # the wrong home reds here rather than passing on the path alone.
  #
  # THE OUT-OF-SCAN CASE IS A PROXY FOR A TARGET ROOT. `gate_plant_home_
  # unscanned` mounts a home test-only, and no target root can be mounted
  # that way at all — rustc reaches `lib.rs`, and a `src/bin/` file, as a
  # target's entry rather than through a `mod` declaration — so for
  # `$DOOR_HOME` and `$BIN_HOME` the fixture exercises the resolver's
  # basename rule rather than a shape a tree could take. What it proves
  # there is the guard and not the route: the gate reds whenever a home
  # leaves the scan set, which for a target root is reachable by a symlink
  # or a narrowing instead.
  gate_selftest_homes --narrowed \
    --subject "$HOME_SUBJECT" "$HOME_FILE" \
    --subject "$DOOR_SUBJECT" "$DOOR_HOME" \
    --subject "$FFI_SUBJECT" "$FFI_HOME" \
    --subject "$BIN_SUBJECT" "$BIN_HOME"
  printf '%s selftest OK: passes a clean fixture carrying the file that DEFINES witness, the two roots its directory prefixes exempt and the one binary whose main it allows, plus prose/block-comment/string-literal mentions of the call and the same call inside a #[cfg(test)] module; fires on a witness minted in library code, on the pncad::tolerance::witness facade spelling, on a main written outside src/bin AND on one under another crate'"'"'s src/bin, at each colon-carrying path a home skip that ends at `:` exempts, and where a directory prefix outlives the crate root it exempts and a new file mints the witness at the old path; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
