#!/usr/bin/env bash
# viewer-numeric-field-door.sh — every numeric field in the viewer is
# built through the crate's one door. ONE home; ci.yml's "the viewer's
# numeric fields go through one door" step in the `discipline` job runs
# it and local-scripts/ci-local.sh's `discipline` row runs the same file.
#
# THE RULE. `crate::widgets::number_field` is the constructor every
# numeric field in `crates/viewer/src` is built with, and it carries two
# rules a bare `egui::DragValue` does not get: the crate's precision
# rule (`widgets::number_text` — the text a field shows reads back as
# the value it holds) and the echo veto (`props::echoed` — text the
# field itself produced is not an edit). Only the FIRST of those can
# travel as a default: `widgets::install_number_formatter` writes the
# render onto `egui::Style::number_formatter`, and `egui::Style` has a
# `number_formatter` and no counterpart for parsing
# (`egui-0.36.1/src/style.rs`). So a bare field shows the right text and
# still commits that text back when a click leaves it — and the render
# names its value only to `crate::readout::REL_TOLERANCE`, so the value
# MOVES.
#
# WHY A GATE AND NOT A SENTENCE. The claim that nothing in the tree is
# reachable by that defect rests on a MEASUREMENT — every numeric field
# in the crate goes through `number_field`, and the one bare
# `egui::DragValue::new` is that door's own test harness. A measurement
# taken once is true until the next lane, and the next lane has no way
# to know it was ever taken. This makes it standing.
# `work/vgeom/a-bare-field-still-commits-its-own-render.md` is the item;
# `widgets::field_tests::a_bare_field_commits_a_render_the_door_would_refuse`
# is the row that pins what the difference IS.
#
# THE HOME IS THE DOOR'S OWN FILE, and that is deliberate rather than
# two exemptions. `crates/viewer/src/widgets.rs` holds `number_field`
# itself — which IS an `egui::DragValue::new` — and, in its own
# `#[cfg(test)]` modules, the bare fields that prove what the door adds
# (`Built::Bare`, `Built::Counting`). A gate that exempted the
# constructor and not the harness would red on the rows that hold the
# very difference it exists to keep.
#
# WHAT THE MATCHER SEES, and what it does not:
#
# - IT SEES both constructors: `DragValue::new` and
#   `DragValue::from_get_set`. The second is the one a reader forgets,
#   and it is the same door with a different accessor.
# - IT DOES NOT SEE `egui::Slider`, which renders its own value through
#   a `DragValue` of its own and is therefore a bare field wearing
#   another name. The crate has none today; banning the name outright
#   would be a rule about a widget nobody has asked for rather than
#   about this door, so it is disclosed here instead of matched. A
#   slider that arrives with a numeric field in it is this gate's next
#   revision.
# - IT DOES NOT SEE a field built in another crate and drawn by the
#   viewer. Nothing in this workspace draws viewer chrome from outside
#   `crates/viewer`, and a gate cannot reason about a widget it is not
#   handed.
# - IT READS CODE ONLY (`gate_rust_code`), so the spelling in a doc
#   comment, a string or a block comment — this header included — is
#   not a hit.
# - IT SCANS `crates/viewer/src` ONLY, including `#[cfg(test)]` modules:
#   an in-module row that builds a bare field somewhere OTHER than the
#   door's own file is exactly the arrival this rule is about, and a
#   scan that skipped test code would let one land.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

SRC=crates/viewer/src
# THE DOOR'S PATH, held once: the scan's guard, the whole-file skip and
# the clean fixture all read this name.
HOME_FILE=crates/viewer/src/widgets.rs
HOME_SUBJECT='the chrome'"'"'s one numeric-field door and the rows that hold what it adds, the only place a bare egui::DragValue may be built'

# The scan is the viewer's sources rather than `crates/*/src`, so the
# guard `gate_require_crate_sources` carries is spelled here for this
# subject: a directory that is gone, or that yields nothing, is a gate
# that read nothing, and reading nothing is not a pass.
scan_viewer_sources() {
  if [ ! -d "$SRC" ]; then
    gate_error "$(gate_name): $SRC does not exist under $PWD — the gate's subject is gone, so it scanned nothing, which is not a pass"
    exit 1
  fi
  mapfile -t GATE_SOURCE_FILES < <(find "$SRC" -type f -name '*.rs' | sort)
  GATE_SCAN_FILES=${#GATE_SOURCE_FILES[@]}
  if [ "$GATE_SCAN_FILES" -eq 0 ]; then
    gate_error "$(gate_name): no .rs files under $SRC in $PWD — the gate scanned nothing, which is not a pass"
    exit 1
  fi
}

gate() {
  scan_viewer_sources
  gate_require_homes "$HOME_SUBJECT" "$HOME_FILE"
  local hits
  hits=$(gate_rust_code "${GATE_SOURCE_FILES[@]}" \
    | gate_grep -E 'DragValue::(new|from_get_set)' \
    | gate_grep -vE "$(gate_record_anchor "$HOME_FILE")")
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a bare egui::DragValue outside crate::widgets::number_field — it takes the crate's render from the context's number_formatter and the echo veto from nothing, so it commits its own text back and moves the value (work/vgeom/a-bare-field-still-commits-its-own-render.md). Build the field with crate::widgets::number_field"
    exit 1
  fi
  gate_ok "every numeric field under $SRC is built through crate::widgets::number_field"
}

# THE DOOR ITSELF IS IN THE CLEAN FIXTURE, which is `lib.sh`'s
# exact-skip contract read for a whole-file skip: a skip no fixture
# exercises is dead in every case, and an anchor that over-narrows is
# then noticed by nobody. The home carries the construction it is the
# home OF — the constructor and a bare harness field beside it — so the
# clean case reds the moment the exemption stops covering either.
gate_plant_clean() {
  mkdir -p "$1/$SRC"
  printf 'pub fn identity(x: f64) -> f64 { x }\n' > "$1/$SRC/lib.rs"
  {
    printf 'pub fn number_field(v: &mut f64) -> egui::DragValue<%s> { egui::DragValue::new(v) }\n' "'_"
    printf '#[cfg(test)]\nmod field_tests {\n'
    printf '    fn bare(v: &mut f64) { let _ = egui::DragValue::new(v); }\n'
    printf '    fn get_set(v: &mut f64) { let _ = egui::DragValue::from_get_set(|_| *v); }\n'
    printf '}\n'
  } > "$1/$HOME_FILE"
}

plant() {
  mkdir -p "$1/$SRC/pane"
  printf 'pub fn row(ui: &mut egui::Ui, v: &mut f64) { ui.add(egui::DragValue::new(v)); }\n' \
    > "$1/$SRC/pane/create.rs"
}

plant_from_get_set() {
  mkdir -p "$1/$SRC"
  printf 'pub fn row(ui: &mut egui::Ui, v: &mut f64) { ui.add(egui::DragValue::from_get_set(|_| *v)); }\n' \
    > "$1/$SRC/camera.rs"
}

# A bare field in an in-module test somewhere OTHER than the door's own
# file. It is the arrival this rule is about — a row that proves
# something about a field the chrome does not build — and a gate that
# skipped `#[cfg(test)]` would call it clean.
plant_in_a_test_module_elsewhere() {
  mkdir -p "$1/$SRC"
  {
    printf 'pub fn ok(x: f64) -> f64 { x }\n'
    printf '#[cfg(test)]\nmod tests {\n'
    printf '    fn field(v: &mut f64) { let _ = egui::DragValue::new(v); }\n'
    printf '}\n'
  } > "$1/$SRC/readout.rs"
}

# The home followed by a colon that is not a line number — one of the
# three shapes `gate_record_anchor`'s header enumerates, and the one a
# skip that ends at `:` exempts.
plant_colon_after_the_home_that_is_not_a_line_number() {
  printf 'pub fn row(ui: &mut egui::Ui, v: &mut f64) { ui.add(egui::DragValue::new(v)); }\n' \
    > "$1/$HOME_FILE:x.rs"
}

plant_outside_the_viewer() {
  mkdir -p "$1/crates/other/src"
  printf 'pub fn row(ui: &mut egui::Ui, v: &mut f64) { ui.add(egui::DragValue::new(v)); }\n' \
    > "$1/crates/other/src/lib.rs"
}

plant_prose_only() {
  mkdir -p "$1/$SRC"
  {
    printf '//! A bare egui::DragValue::new takes the render and not the commit.\n'
    printf '/*\n * egui::DragValue::from_get_set is the other constructor.\n */\n'
    printf 'pub const WHY: &str = "egui::DragValue::new";\n'
    printf 'pub fn ok(a: f64) -> f64 { a } // and DragValue::new in a trailing one\n'
  } > "$1/$SRC/props.rs"
}

gate_selftest() {
  local want="a bare egui::DragValue outside crate::widgets::number_field"
  gate_selftest_clean
  # A `grep` that cannot run is the failure this gate cannot see for
  # itself: it produces no hits, and no hits is what a clean tree
  # produces.
  gate_selftest_without_tool grep "it is grep saying it could not search"
  gate_selftest_case "$want" plant
  gate_selftest_case "$want" plant_from_get_set
  gate_selftest_case "$want" plant_in_a_test_module_elsewhere
  gate_selftest_case "$want" plant_colon_after_the_home_that_is_not_a_line_number
  gate_selftest_passes "a bare field in a crate that is not the viewer" plant_outside_the_viewer
  gate_selftest_passes "prose, doc comments and a string literal naming the constructor" plant_prose_only
  gate_selftest_homes --subject "$HOME_SUBJECT" "$HOME_FILE"
  printf '%s selftest OK: passes a clean fixture carrying the door and its harness'"'"'s own bare fields, a bare field outside crates/viewer/src, and prose/doc/string mentions of the constructor; fires on a bare DragValue::new, on DragValue::from_get_set, on one inside a cfg(test) module of another viewer file, and at the colon-carrying path a home skip that ends at `:` exempts; and it stays RED, with a diagnosis, when `grep` itself cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
