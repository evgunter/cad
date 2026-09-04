#!/usr/bin/env bash
# viewer-module-kinds.sh — the viewer crate's vocabulary/driver rule,
# checked. ONE home; ci.yml's "viewer module kinds" step runs it and
# local-scripts/ci-local.sh reaches it through the loop that runs this
# whole directory.
#
# `crates/viewer/README.md`'s `## Module boundaries` ratifies the rule:
# **every module in that crate is a VOCABULARY or a DRIVER, and its
# `use` block says which** — a driver may name any vocabulary, and no
# vocabulary may name a driver or the toolkit. It sells the rule on
# being "mechanically checkable — read the `use` block — which is the
# property that makes it survive contact with the next unit", and for
# its first life NOTHING read one. Not a gate here, not `doc-gate.sh`
# (rustdoc links, not imports), not clippy (it has no lint for "this
# module names a type from that module"). The failure that bought
# nothing red: a `use crate::session::DocSession;` added to a
# vocabulary module compiles, passes clippy, passes the doc gate, and
# silently makes the README's vocabulary tables false. This is the
# machine that sentence claimed.
#
# WHERE A MODULE'S KIND IS DECLARED, and why it is not the README.
# Each module says its own kind, once, in its own doc header:
#
#     //! Module kind: **vocabulary** — …
#     //! Module kind: **driver** — …
#
# The subject of the rule is a module's `use` block, so the
# declaration sits in the same file as the thing it classifies: an
# author changing a module's role meets the contradiction on screen
# rather than in a document they may not open, and a new module cannot
# be added without answering the question. A rustdoc header is also
# already a built, linted artefact, where a Markdown table is read by
# nothing — making a bash gate the sole reader of one turns table
# syntax into an unversioned interface. What the header form gives up
# is that it can drift from the README's prose, so the README is
# cross-checked below in the direction that matters: every module the
# README's vocabulary TABLES name must declare `vocabulary`.
#
# WHAT FIRES IT
#
#   * a module under `crates/viewer/src` with no kind declaration, or
#     with more than one;
#   * a module declaring `driver` that is not on DRIVER_ROSTER, or a
#     roster entry that has stopped being a driver or stopped existing
#     — the README says there are exactly two drivers, so a third one
#     is a README amendment and not a header edit;
#   * a vocabulary module naming a driver type (`DocSession`,
#     `ViewerApp`), a toolkit crate (`egui`, `eframe`, `wgpu`, …), or a
#     driver module path (`crate::app`, `crate::pane`, `crate::widgets`,
#     `crate::gpu`) anywhere in its CODE — `use` block, signature or
#     body, since a fully-qualified name evades an import check and the
#     rule is about what a module NAMES;
#   * a README vocabulary-table row naming a module that does not exist
#     or that declares `driver`;
#   * either README section losing its rows, which would make the
#     cross-check pass by scanning nothing.
#
# THE RATIFIED EXCEPTIONS are VOCAB_EXCEPTIONS, and they are live
# rather than hypothetical: `pick.rs` and `parts.rs` take a
# `&DocSession` as a read-only argument, so the rule as ratified is
# already false of the tree at two sites. They are named here, recorded
# in the README, and carried by their own tracker item; a THIRD site
# reds. Each exception must still HIT the matcher, so the list retires
# itself: fix the site and the gate says the entry is inert and to
# delete it. That is what bounds a hand-written list the filesystem
# cannot derive.
#
# WHAT IT CANNOT CATCH (stated because a sweep whose blind spot is
# unstated is an unverified claim):
#
#   * ROLE. This decides what a module NAMES, never what it IS. A
#     module that owns mutable state and dispatches but imports nothing
#     forbidden passes as a vocabulary; the semantic half of the
#     README's definition is not mechanised and cannot be by a grep.
#   * A FALSE DRIVER DECLARATION on a roster entry — the roster is the
#     one hand-kept list here, bounded by "every entry exists and still
#     declares `driver`", so it cannot name nothing, but a module the
#     README retires from driverhood stays permitted until someone
#     edits this file.
#   * VOCABULARIES THE README DOES NOT TABULATE. The cross-check covers
#     only the rows the two vocabulary tables carry; `camera`, `input`,
#     `frame` and the rest are classified by their own headers alone.
#   * A DRIVER TYPE REACHED WITHOUT NAMING IT: through a re-export
#     under another name, a generic parameter, a trait object, or a
#     macro that expands to the import. The scan reads source text,
#     never expansions.
#   * MODULES OUTSIDE `crates/viewer/src`. `tests/` is not scanned: a
#     suite is allowed to name the driver it drives.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

SRC=crates/viewer/src
README=crates/viewer/README.md
GATE_SCAN_NOUN='viewer module'

# The declaration, anchored at column zero so only a module-level doc
# comment can carry it. The trailing prose is free; the kind is not.
KIND_LINE='^//! Module kind: \*\*(vocabulary|driver)\*\*'

# The drivers, as `## Module boundaries` spells them: `session` (owns
# `DocSession`, dispatches `SessionOp`) and `app` (owns `ViewerApp`,
# drives the frame), the latter split across `app`, `pane` and its
# bodies, `widgets` and `gpu`.
DRIVER_ROSTER=(
  app.rs
  gpu.rs
  pane.rs
  pane/create.rs
  pane/features.rs
  pane/properties.rs
  pane/view.rs
  pane/viewport.rs
  session.rs
  widgets.rs
)

# What a vocabulary may not name. Three groups, because they fail for
# three different reasons: the driver's own types, the toolkit, and the
# driver modules by path. `\b` keeps `egui` from matching inside
# `egui_tiles`, so each toolkit crate is named rather than prefixed —
# a prefix match would make `egui_probe`-shaped vocabulary helpers
# unspellable without touching this line.
FORBIDDEN_TYPES='\b(DocSession|ViewerApp)\b'
FORBIDDEN_TOOLKIT='\b(egui|eframe|wgpu|winit|egui_tiles|egui_wgpu|egui_dock)\b'
FORBIDDEN_PATHS='crate::(app|pane|widgets|gpu)\b'

# Vocabulary modules that name a driver type today. See the header.
VOCAB_EXCEPTIONS=(pick.rs parts.rs)

# The README's vocabulary tables, by heading. Every module in a row's
# first column must declare `vocabulary`.
VOCAB_TABLES=(
  "### The session's vocabularies"
  "### The app's vocabularies"
)

# LIST MEMBERSHIP IN BASH, not `printf … | grep -qxF`. `grep -q` as a
# predicate is sanctioned by `lib.sh`, but only where exit 1 IS the
# answer — and here a grep that could not run (exit 2) would read as
# "not in the list", which is a FALSE RED on the roster checks and a
# silently DROPPED exemption on the exception check. The lists are five
# to forty short strings held in memory; nothing needs a subprocess to
# search them, and one that cannot fail cannot fail wrong.
contains() {
  local needle=$1 item
  shift
  for item in "$@"; do
    [ "$item" = "$needle" ] && return 0
  done
  return 1
}

# The kind a file declares, or the empty string. Count and value are
# read separately so "declares two kinds" is a distinct diagnosis from
# "declares one".
kind_count() {
  gate_grep -cE "$KIND_LINE" "$1"
}
kind_of() {
  gate_grep -m1 -oE "$KIND_LINE" "$1" | sed -E 's/.*\*\*(vocabulary|driver)\*\*/\1/'
}

# A README vocabulary table's first column: the rows between its
# heading and the next heading, `session::select` style.
readme_table_modules() {
  awk -v want="$1" '
    $0 == want { inside = 1; next }
    inside && /^#/ { inside = 0 }
    inside && /^\|/ { print }
  ' "$README" |
    sed -nE 's/^\|[[:space:]]*`([A-Za-z0-9_:]+)`[[:space:]]*\|.*/\1/p'
}

# `session::select` is `session/select.rs`; `forms` is `forms.rs`.
module_path() {
  printf '%s.rs\n' "${1//:://}"
}

gate() {
  local rc=0 f rel n kind
  gate_require_file "$README"
  if [ ! -d "$SRC" ]; then
    gate_error "$(gate_name): $SRC does not exist under $PWD — the gate's subject is gone, so it scanned nothing, which is not a pass"
    exit 1
  fi

  # THE ROSTER IS THE TREE. `lib.rs` declares the modules rather than
  # being one, and `bin/` is a binary rather than a module of the
  # library; everything else answers the question.
  local -a modules=()
  mapfile -t modules < <(find "$SRC" -type f -name '*.rs' \
    ! -name lib.rs ! -path "$SRC/bin/*" | sed "s#^$SRC/##" | sort)
  GATE_SCAN_FILES=${#modules[@]}
  if [ "$GATE_SCAN_FILES" -eq 0 ]; then
    gate_error "$(gate_name): no modules under $SRC in $PWD besides lib.rs and bin/ — the gate scanned nothing, which is not a pass"
    exit 1
  fi

  # 1. EVERY MODULE DECLARES EXACTLY ONE KIND.
  local -a vocab=() driver=()
  for rel in "${modules[@]}"; do
    f=$SRC/$rel
    n=$(kind_count "$f")
    if [ "$n" -eq 0 ]; then
      gate_error "$SRC/$rel declares no module kind — every module in this crate is a VOCABULARY or a DRIVER ($README, Module boundaries), and the declaration is one line in the module's own doc header: \`//! Module kind: **vocabulary**\` or \`//! Module kind: **driver**\`"
      rc=1
      continue
    fi
    if [ "$n" -gt 1 ]; then
      gate_error "$SRC/$rel declares $n module kinds — a module is one or the other, and two declarations means the gate would read whichever came first"
      rc=1
      continue
    fi
    kind=$(kind_of "$f")
    if [ "$kind" = driver ]; then driver+=("$rel"); else vocab+=("$rel"); fi
  done
  [ "$rc" -eq 0 ] || exit 1

  # 2. THE DRIVER ROSTER, BOTH WAYS. The README says there are exactly
  # two drivers and names the modules the second is split across, so a
  # module cannot promote itself by editing its own header.
  local d
  for rel in ${driver[@]+"${driver[@]}"}; do
    if ! contains "$rel" "${DRIVER_ROSTER[@]}"; then
      gate_error "$SRC/$rel declares itself a DRIVER and is not on this gate's driver roster. $README says there are exactly two drivers (\`session\` and \`app\`, the latter split across \`app\`, \`pane\`, \`widgets\` and \`gpu\`) — a third one is a README amendment plus an entry here, not a header edit, because a module that declares \`driver\` is exempt from every import check below"
      rc=1
    fi
  done
  for d in "${DRIVER_ROSTER[@]}"; do
    if [ ! -f "$SRC/$d" ]; then
      gate_error "$(gate_name): the driver roster names $SRC/$d, which is not a file under $PWD — this list has no filesystem to derive itself from, so a roster entry naming nothing is a roster entry watching nothing. Fix the path or drop the entry deliberately"
      rc=1
    elif ! contains "$d" ${driver[@]+"${driver[@]}"}; then
      gate_error "$(gate_name): the driver roster names $d but $SRC/$d does not declare \`//! Module kind: **driver**\` — a module demoted in its own header while this list still exempts it is exempt from every import check and classified as a vocabulary by every reader"
      rc=1
    fi
  done
  [ "$rc" -eq 0 ] || exit 1

  # 3. NO VOCABULARY NAMES A DRIVER OR THE TOOLKIT. Read over the
  # shared CODE-ONLY view, so a header that spells `DocSession` out to
  # say it does not name one — `forms.rs` and `drafts.rs` both do —
  # stays green, and a string literal cannot fire it either.
  local -a scanned=() ex
  for rel in ${vocab[@]+"${vocab[@]}"}; do
    if contains "$rel" "${VOCAB_EXCEPTIONS[@]}"; then continue; fi
    scanned+=("$SRC/$rel")
  done
  if [ "${#scanned[@]}" -eq 0 ]; then
    gate_error "$(gate_name): no vocabulary modules left to scan under $SRC — every module is either a driver or an exception, which is not a pass"
    exit 1
  fi
  local hits
  hits=$(gate_rust_code "${scanned[@]}" |
    gate_grep -E "$FORBIDDEN_TYPES|$FORBIDDEN_TOOLKIT|$FORBIDDEN_PATHS" |
    cut -c1-160)
  if [ -n "$hits" ]; then
    printf '%s\n' "$hits"
    gate_error "a vocabulary module names a driver type, a toolkit crate or a driver module. $README's rule: a vocabulary holds values, their wording and pure functions over them, and can be read and tested without a session or a window existing. Move the code to the driver, or take the driver's answer as a value — do not reclassify the module to silence this"
    exit 1
  fi

  # 4. THE EXCEPTIONS RETIRE THEMSELVES. An entry that no longer hits
  # is a permission nobody needs, and a list that can only be wrong by
  # naming a file that is clean is a list that cannot quietly grow.
  for ex in "${VOCAB_EXCEPTIONS[@]}"; do
    if [ ! -f "$SRC/$ex" ]; then
      gate_error "$(gate_name): the exception list names $SRC/$ex, which is not a file under $PWD — drop the entry"
      rc=1
      continue
    fi
    if ! contains "$ex" ${vocab[@]+"${vocab[@]}"}; then
      gate_error "$(gate_name): the exception list names $ex, which no longer declares \`//! Module kind: **vocabulary**\` — an exception to the vocabulary rule on a module that is not a vocabulary exempts nothing and hides the module from every check here"
      rc=1
      continue
    fi
    # MATERIALISED, never `| grep -q`: `-q` exits on its first match and
    # SIGPIPEs the reader upstream, which `pipefail` then reports as a
    # failed pipeline — so the live exceptions read as inert depending
    # on which side won the race. `gate-roster.sh`'s header banishes the
    # same shape from its own halves.
    local live
    live=$(gate_rust_code "$SRC/$ex" |
      gate_grep -E "$FORBIDDEN_TYPES|$FORBIDDEN_TOOLKIT|$FORBIDDEN_PATHS")
    if [ -z "$live" ]; then
      gate_error "$(gate_name): $SRC/$ex is on the exception list and no longer names a driver type — the exception is inert, so delete the entry and let the rule cover the file again"
      rc=1
    fi
  done
  [ "$rc" -eq 0 ] || exit 1

  # 5. THE README, CROSS-CHECKED. The declaration lives in the module,
  # so this is the direction it can drift: a table row calling a module
  # a vocabulary while the module says otherwise makes the README false
  # in exactly the way this unit exists to stop.
  local table row path
  for table in "${VOCAB_TABLES[@]}"; do
    local -a rows=()
    mapfile -t rows < <(readme_table_modules "$table")
    if [ "${#rows[@]}" -eq 0 ]; then
      gate_error "$(gate_name): $README's \"$table\" section yielded no module rows, so the cross-check below it decided nothing. Either the heading was renamed or the table was reshaped — a cross-check that scans nothing is not a pass"
      rc=1
      continue
    fi
    for row in "${rows[@]}"; do
      path=$(module_path "$row")
      if [ ! -f "$SRC/$path" ]; then
        gate_error "$README's \"$table\" lists \`$row\`, which is not a module in the tree ($SRC/$path does not exist) — the table outran the code"
        rc=1
      elif ! contains "$path" ${vocab[@]+"${vocab[@]}"}; then
        gate_error "$README's \"$table\" lists \`$row\` as a vocabulary, but $SRC/$path declares itself a DRIVER — the README and the module disagree about what the module is, which is the drift a per-module declaration buys and this check pays for"
        rc=1
      fi
    done
  done
  [ "$rc" -eq 0 ] || exit 1

  gate_ok "every module under $SRC declares a kind, ${#driver[@]} drivers match the ratified roster, ${#scanned[@]} vocabularies name no driver type, no toolkit crate and no driver module (${#VOCAB_EXCEPTIONS[@]} recorded exceptions, each still live), and the README's vocabulary tables agree with the modules they name"
}

# This gate's subject is one crate's src tree plus its README, not
# `crates/*/src`, so the fixture is a miniature viewer crate. The
# roster and the exception list are PLANTED FROM THEMSELVES, so a
# fixture cannot fall behind an entry someone adds here.
gate_plant_clean() {
  local root=$1 d ex
  mkdir -p "$root/$SRC/session" "$root/$SRC/pane" "$root/$SRC/bin"
  mkdir -p "$root/$(dirname "$README")"

  # lib.rs and bin/ carry NO declaration, and stay green: they are the
  # two exclusions, asserted by the clean fixture rather than assumed.
  printf 'pub mod camera;\n' > "$root/$SRC/lib.rs"
  printf 'fn main() {}\n' > "$root/$SRC/bin/viewer.rs"

  # A vocabulary with both near misses baked in: a COMMENT naming the
  # driver type and the toolkit, and a real import of `session::op`,
  # which is a vocabulary living under the driver's module path.
  cat > "$root/$SRC/camera.rs" <<'RS'
//! The camera value.
//!
//! Names no `DocSession` and no `egui`; both words are prose here.
//!
//! Module kind: **vocabulary** — it names no driver type.
use crate::session::SessionOp;
pub fn apply(op: SessionOp) -> SessionOp { op }
RS
  cat > "$root/$SRC/forms.rs" <<'RS'
//! What the panels offer.
//!
//! Module kind: **vocabulary** — it names no driver type.
pub struct FieldWriting;
RS
  cat > "$root/$SRC/drafts.rs" <<'RS'
//! In-flight form state.
//!
//! Module kind: **vocabulary** — it names no driver type.
pub struct Drafts;
RS
  cat > "$root/$SRC/session/select.rs" <<'RS'
//! What is selected.
//!
//! Module kind: **vocabulary** — it names no driver type.
pub struct Selection;
RS

  for d in "${DRIVER_ROSTER[@]}"; do
    mkdir -p "$root/$SRC/$(dirname "$d")"
    {
      printf '//! A driver.\n//!\n'
      printf '//! Module kind: **driver** — it may name the toolkit.\n'
      printf 'use eframe::egui;\n'
      printf 'pub struct DocSession;\n'
    } > "$root/$SRC/$d"
  done

  # The exceptions, each carrying the shape the real ones carry: a
  # vocabulary taking a `&DocSession` as a read-only argument.
  for ex in "${VOCAB_EXCEPTIONS[@]}"; do
    mkdir -p "$root/$SRC/$(dirname "$ex")"
    {
      printf '//! A ratified exception.\n//!\n'
      printf '//! Module kind: **vocabulary** — it names no driver type.\n'
      printf 'use crate::session::DocSession;\n'
      printf 'pub fn read(_s: &DocSession) {}\n'
    } > "$root/$SRC/$ex"
  done

  {
    printf '## Module boundaries\n\n'
    printf 'Every module is a vocabulary or a driver.\n\n'
    printf "### The session's vocabularies\n\n"
    printf '| Module | Holds |\n|---|---|\n'
    printf '| `session::select` | what is selected |\n\n'
    printf "### The app's vocabularies\n\n"
    printf '| Module | Holds |\n|---|---|\n'
    printf '| `forms` | what the panels offer |\n'
    printf '| `drafts` | in-flight form state |\n\n'
    printf '### The app driver, split for size\n\n'
    printf 'Prose, not a table.\n'
  } > "$root/$README"
}

plant_undeclared_module() {
  printf '//! A new module with no kind.\npub struct Thing;\n' \
    > "$1/$SRC/thing.rs"
}

plant_two_kinds() {
  cat > "$1/$SRC/thing.rs" <<'RS'
//! Two answers.
//!
//! Module kind: **vocabulary** — it names no driver type.
//! Module kind: **driver** — it may name the toolkit.
pub struct Thing;
RS
}

# THE HEADLINE CASE, in the exact spelling the item names.
plant_vocabulary_imports_docsession() {
  cat > "$1/$SRC/thing.rs" <<'RS'
//! A vocabulary that grew a session import.
//!
//! Module kind: **vocabulary** — it names no driver type.
use crate::session::DocSession;
pub fn peek(_s: &DocSession) {}
RS
}

# NOT AN IMPORT. A fully-qualified name in a signature evades any check
# that reads only `use` lines, which is what the README's "read the
# `use` block" sells the rule on — so the matcher reads the module.
plant_vocabulary_names_docsession_inline() {
  cat > "$1/$SRC/thing.rs" <<'RS'
//! No import at all.
//!
//! Module kind: **vocabulary** — it names no driver type.
pub fn peek(_s: &crate::session::DocSession) {}
RS
}

plant_vocabulary_imports_toolkit() {
  cat > "$1/$SRC/thing.rs" <<'RS'
//! A vocabulary that grew a widget.
//!
//! Module kind: **vocabulary** — it names no driver type.
use eframe::egui;
pub fn draw(_ui: &mut egui::Ui) {}
RS
}

plant_vocabulary_imports_driver_module() {
  cat > "$1/$SRC/thing.rs" <<'RS'
//! A vocabulary reaching into the app driver.
//!
//! Module kind: **vocabulary** — it names no driver type.
use crate::widgets::delete_button;
pub fn go() { delete_button() }
RS
}

# A TEST MODULE IS IN SCOPE. The README's definition is that a
# vocabulary can be read AND TESTED without a session existing, so a
# `#[cfg(test)]` block that builds one is the same violation.
plant_vocabulary_test_module_imports_docsession() {
  cat > "$1/$SRC/thing.rs" <<'RS'
//! Clean in production.
//!
//! Module kind: **vocabulary** — it names no driver type.
pub fn go() {}
#[cfg(test)]
mod tests {
    use crate::session::DocSession;
    #[test]
    fn t() { let _ = DocSession; }
}
RS
}

plant_self_promoted_driver() {
  cat > "$1/$SRC/thing.rs" <<'RS'
//! A module that promoted itself out of the rule.
//!
//! Module kind: **driver** — it may name the toolkit.
use eframe::egui;
pub fn draw(_ui: &mut egui::Ui) {}
RS
}

plant_roster_entry_missing() {
  rm -f "$1/$SRC/${DRIVER_ROSTER[0]}"
}

plant_roster_entry_demoted() {
  cat > "$1/$SRC/${DRIVER_ROSTER[0]}" <<'RS'
//! Demoted in its own header while the roster still exempts it.
//!
//! Module kind: **vocabulary** — it names no driver type.
use eframe::egui;
pub fn draw(_ui: &mut egui::Ui) {}
RS
}

plant_exception_gone_clean() {
  cat > "$1/$SRC/${VOCAB_EXCEPTIONS[0]}" <<'RS'
//! Fixed, and still on the exception list.
//!
//! Module kind: **vocabulary** — it names no driver type.
pub fn read() {}
RS
}

plant_readme_row_is_a_ghost() {
  sed -i 's#^| `session::select` | what is selected |$#&\n| `session::retired` | gone from the tree |#' \
    "$1/$README"
}

plant_readme_calls_a_driver_a_vocabulary() {
  sed -i 's#^| `forms` | what the panels offer |$#| `widgets` | drawn helpers |#' \
    "$1/$README"
}

plant_readme_table_renamed() {
  sed -i "s|^### The session's vocabularies\$|### Session vocabularies|" \
    "$1/$README"
}

# GREEN cases, asserted rather than assumed. A gate that fires on these
# is a gate people route around.
plant_vocabulary_mentions_driver_in_prose() {
  cat > "$1/$SRC/thing.rs" <<'RS'
//! Names `DocSession`, `ViewerApp` and `egui` in prose to say it does
//! not name them — which `forms.rs` and `drafts.rs` both do today.
//!
//! Module kind: **vocabulary** — it names no driver type.
/* A block comment about egui::Ui and crate::app. */
pub fn note() -> &'static str {
    // A trailing comment naming DocSession.
    "egui and DocSession as a string literal"
}
RS
}

plant_vocabulary_imports_a_session_vocabulary() {
  cat > "$1/$SRC/thing.rs" <<'RS'
//! `session::op` is a vocabulary that happens to live under the
//! driver's module path; importing it is the rule working, not a
//! violation.
//!
//! Module kind: **vocabulary** — it names no driver type.
use crate::session::{OpOutcome, SessionOp};
pub fn go(_o: OpOutcome, _s: SessionOp) {}
RS
}

gate_selftest() {
  gate_selftest_clean
  # A matcher that cannot run must not read as a clean scan: before
  # `gate_grep` this fixture printed OK and exited 0.
  gate_selftest_without_tool grep "it is grep saying it could not search"

  gate_selftest_case "declares no module kind" plant_undeclared_module
  gate_selftest_case "declares 2 module kinds" plant_two_kinds

  local want="names a driver type, a toolkit crate or a driver module"
  gate_selftest_case "$want" plant_vocabulary_imports_docsession
  gate_selftest_case "$want" plant_vocabulary_names_docsession_inline
  gate_selftest_case "$want" plant_vocabulary_imports_toolkit
  gate_selftest_case "$want" plant_vocabulary_imports_driver_module
  gate_selftest_case "$want" plant_vocabulary_test_module_imports_docsession

  gate_selftest_case "declares itself a DRIVER and is not on this gate's driver roster" \
    plant_self_promoted_driver
  gate_selftest_case "which is not a file under" plant_roster_entry_missing
  gate_selftest_case "does not declare" plant_roster_entry_demoted
  gate_selftest_case "the exception is inert" plant_exception_gone_clean

  gate_selftest_case "is not a module in the tree" plant_readme_row_is_a_ghost
  gate_selftest_case "declares itself a DRIVER — the README and the module disagree" \
    plant_readme_calls_a_driver_a_vocabulary
  gate_selftest_case "yielded no module rows" plant_readme_table_renamed

  gate_selftest_passes "a vocabulary naming the driver only in prose and a literal" \
    plant_vocabulary_mentions_driver_in_prose
  gate_selftest_passes "a vocabulary importing a vocabulary under session::" \
    plant_vocabulary_imports_a_session_vocabulary

  printf '%s selftest OK: the clean fixture proves lib.rs and bin/ are excluded on purpose and that a ratified exception suppresses; 13 planted cases fire — an undeclared module, a doubly declared one, a vocabulary that imports the driver type, names it fully qualified with no import at all, imports the toolkit, imports a driver module, or reaches the driver from a #[cfg(test)] block; a module that promotes itself to driver; a roster entry that vanished or was demoted in its own header; an exception that went clean and is now inert; and a README table row naming a ghost module, calling a driver a vocabulary, or losing its rows to a renamed heading. Prose and string-literal mentions of `DocSession`/`egui`, and an import of a vocabulary under `session::`, stay green; and the gate stays RED, with a diagnosis, when `grep` itself cannot run\n' \
    "$(gate_name)"
}

gate_parse_args "$@"
gate_main
