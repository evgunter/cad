#!/usr/bin/env bash
# viewer-vocab-declared-once.sh — a closed vocabulary's membership list
# is declared ONCE, and a hand-written one under `crates/viewer/src` is
# a hit unless the README ratifies it. ONE home; ci.yml's "viewer
# vocabularies are declared once" step in the `mirror` job runs it and
# local-scripts/ci-local.sh runs it in `tier_blind_rows` (and again in
# the directory loop on a building change set).
#
# THE RULE, from `crates/viewer/README.md`'s **Closed vocabularies are
# declared once**: nine enums in that crate are closed vocabularies, and
# each used to carry a hand-written `const ALL` beside its declaration —
# a second copy of the membership, free to fall behind the first. Adding
# a variant compiled, the radio row silently lost a button, and every
# sweep keyed on the list quietly narrowed. `src/vocab.rs`'s
# `vocabulary!` now expands ONE list of variants into the enum AND its
# `ALL`, so a variant cannot reach the enum without reaching the list.
#
# WHAT THE COMPILER HOLDS AND WHAT IT DOES NOT. The nine are held: their
# `ALL` is projected from the declaration. **Nothing holds the next
# one.** An author who writes
#
#     impl NewChoice {
#         pub const ALL: [Self; 3] = [Self::A, Self::B, Self::C];
#     }
#
# meets no compile error, no clippy lint and — before this file — no
# gate: only a README section, if they read it. That is the standard
# this crate rejected when it built `scripts/gates/viewer-module-kinds.sh`,
# whose header exists because a rule sold as "mechanically checkable"
# spent its first life with nothing reading it.
#
# THE TWO SHAPES THIS HITS ON, and why it is two. The NAMED shape is a
# `const ALL` — the spelling the nine carried and the one an author
# reaches for. The UN-NAMED shape is any `const` array literal holding
# two or more `Type::Variant` entries, whatever it is called. The second
# arm is not padding: on the tree this gate was written against, THREE
# of the four hand-written lists are un-named (`BOOLEAN_OPS`,
# `MATE_PRIMITIVES`, `SUBJECTS_WITH_AN_EXPIRY_ISSUER`) and one is named
# (`Theme::ALL`), so a named-only gate would be evaded by calling the
# next table `KINDS` — which is the same list with a different word on
# it.
#
# WHERE THE ALLOWLIST LIVES, AND WHY IT IS NOT IN THIS FILE. The README
# section ratifies THREE kinds of list that stay hand-written, and its
# `#### The lists that stay hand-written` table is the roster: one row
# per list, naming the module it is declared in and which of the three
# kinds it is. This file reads that table. It also reads the KINDS from
# the section's own bolded bullets, so the vocabulary of reasons is the
# ratification itself and not a copy of it — deleting a bullet while
# leaving a row that claims it reds. That is `viewer-module-kinds.sh`'s
# contract and the reason its rosters have not gone stale.
#
# THE ROSTER RETIRES ITSELF. A row naming a list the scan does not find
# reds. An allowlist entry with nothing behind it is a ratification
# waiting to be inherited by whatever is written at that name next
# (`interval-square-allowlist.sh:125-133` argues the same about its own
# retired entries), and a converted vocabulary leaves no array literal
# behind — so converting a list and forgetting its row is a red, not a
# silent inheritance.
#
# WHY THIS GATE IS SITED IN THE `mirror` JOB. *A gate must be sited
# where it can fire on its own inputs* (Ev, 2026-08-20, on S61;
# `.github/workflows/ci.yml` states it above that job). Half this gate's
# subject is `crates/viewer/README.md`: the allowlist rows, the kind
# vocabulary and the table's own shape. A change set of only the README
# classifies TIER=docs, `RUN_BUILD=false`, and every `if: run_build` job
# — `discipline` included — is skipped. Sited there, a docs-only PR
# adding a row, inventing a fourth kind or deleting the table would
# merge with every one of those arms unrun.
# `scripts/check-ci-mirror-parity.py`'s TIER_BLIND names this gate, so
# the siting is enforced rather than remembered.
#
# WHY `crates/viewer/tests/` IS NOT SCANNED, which is a decision and not
# an oversight. The suites hold hand-written complete variant lists of
# their own — `work/view/viewer-suites-hold-hand-written-complete-variant-
# lists.md` names four — and this scan sees NONE of them, because they
# are a different shape: inline arrays in a `for … in [ … ]` row, not
# `const` items. Widening the scan to `tests/` on the tree this gate was
# written against adds exactly one hit, `crates/viewer/tests/theme.rs`'s
# `KINDS`, which is a deliberately partial list already argued in place
# at seventeen lines of doc — so it buys one allowlist row and no
# defect. What it would cost is the claim: a reader seeing `tests/` in
# the scan set would believe the suites are covered while all four
# instances stay invisible. The suites' item stays open and is taken by
# a unit that decides, per enum, whether the vocabulary earns a
# projected `ALL`.
#
# WHAT IT CANNOT CATCH (stated because a sweep whose blind spot is
# unstated is an unverified claim):
#
#   * A LIST THAT IS NOT A `const`. `Seats::new([Seat::A, Seat::B])` is
#     a hand-written membership list and this gate does not see it —
#     which is correct for the seat lists, whose partiality the README
#     ratifies, and blind for whatever else is written that way. The
#     `const` anchor is what makes the un-named arm's false-positive
#     rate nil; a scan over every array literal in the crate would hit
#     match arms, builder calls and test fixtures.
#   * A LIST WRITTEN AFTER `use Enum::*`, so no `Type::` prefix reaches
#     the text. The un-named arm is a text matcher over a lexed view and
#     resolves nothing.
#   * A LIST BUILT BY AN EXPRESSION — `vec!`, an iterator chain, a
#     `matches!` ladder, a `const fn` — rather than written as a
#     literal.
#   * WHETHER A LIST IS ACTUALLY COMPLETE. This decides that a
#     membership list is hand-written, never that it is right. A row in
#     the README table claiming "a deliberately partial list" is prose
#     and nothing computes with it; what the table is checked for is
#     that the kind it claims is one of the three the section ratifies.
#   * MACRO BODIES AND `include!`d TEXT, per `lib.sh`'s reader block:
#     the view is lexed, never expanded. `vocabulary!`'s own `pub const
#     ALL;` carries no `:` and no initialiser, so a converted
#     vocabulary is not a hit — the property that makes this gate
#     quiet on nine enums it would otherwise name.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

SRC=crates/viewer/src
README=crates/viewer/README.md
GATE_SCAN_NOUN='viewer source file'

# The README section that ratifies the rule, and the table inside it
# that is the roster. Named once each and CHECKED to yield something: a
# gate whose thesis is that hand-kept copies rot must not carry one
# itself.
SECTION='### Closed vocabularies are declared once'
TABLE='#### The lists that stay hand-written'
# What that table's header row must read, so a reordered or renamed
# column is a diagnosis rather than a roster read off the wrong cells.
TABLE_HEADER='| List | Module | Kind |'

# The repair a diagnosis points at. One home for the two messages that
# name it.
MACRO='crates/viewer/src/vocab.rs'

# THE ITEM READER, AND IT IS A WORKAROUND. `gate_rust_code`'s statement
# view cuts at `;`, and an array TYPE carries one — `[(BooleanOp,
# &str); 3]` — so a statement record splits a `const` item in half and
# the initialiser lands in a record with no `const` in it. That is a
# defect in the shared reader, not a property of this rule, and it is
# `work/issues/gate-rust-reader-splits-an-array-type-at-its-semicolon`;
# when it is fixed there, this reader is what should go. It reassembles
# instead: from a line
# declaring a `const` with an upper-case name, accumulate the code view
# until the `;` that closes the item at bracket depth zero, then emit
# `FILE|LINE|NAME|INIT` with INIT the text after the first `=` at depth
# zero. Brackets, parens and braces all count, so `PlanarRest { offset:
# 0.0 }` inside a table does not end the item early.
#
# ONE ITEM PER OPENING LINE. A second `const` written after the first
# one's `;` ON THE SAME LINE is not read; rustfmt does not produce that
# spelling and the residue is one-directional — a miss, never a false
# red.
ITEM_AWK='
function flush(  i, ch, d, eq, init, n) {
  d = 0; eq = 0; n = length(item)
  for (i = 1; i <= n; i++) {
    ch = substr(item, i, 1)
    if (ch == "(" || ch == "[" || ch == "{") d++
    else if (ch == ")" || ch == "]" || ch == "}") d--
    else if (ch == "=" && d == 0) {
      if (substr(item, i + 1, 1) == "=" || substr(item, i + 1, 1) == ">") { i++; continue }
      if (substr(item, i - 1, 1) ~ /[=!<>]/) continue
      eq = i; break
    }
  }
  if (eq == 0) return
  init = substr(item, eq + 1)
  sub(/^[[:space:]]*/, "", init)
  printf "%s|%s|%s|%s\n", file, line, name, init
}
{
  rec = $0
  p = index(rec, ":"); f = substr(rec, 1, p - 1); rest = substr(rec, p + 1)
  p = index(rest, ":"); l = substr(rest, 1, p - 1); t = substr(rest, p + 1)
  if (acc == 0) {
    if (t !~ /(^|[^A-Za-z0-9_])const[[:space:]]+[A-Z_][A-Za-z0-9_]*[[:space:]]*:/) next
    acc = 1; file = f; line = l; item = ""
    name = t
    sub(/^.*(^|[^A-Za-z0-9_])const[[:space:]]+/, "", name)
    sub(/[^A-Za-z0-9_].*$/, "", name)
  } else if (f != file) {
    # An item that never closed. Its file ended, so nothing downstream
    # can be decided from it; start again at the new file rather than
    # splicing two files into one record.
    acc = 0; item = ""
    next
  }
  item = item " " t
  d = 0
  for (i = 1; i <= length(item); i++) {
    ch = substr(item, i, 1)
    if (ch == "(" || ch == "[" || ch == "{") d++
    else if (ch == ")" || ch == "]" || ch == "}") d--
    else if (ch == ";" && d == 0) { flush(); acc = 0; item = ""; break }
  }
}'

# `FILE|LINE|NAME|SHAPE` for every item the two arms hit. The un-named
# arm counts `Type::Variant` occurrences INSIDE the initialiser and
# wants two: one is a constant being named, two or more is a list.
HIT_AWK='
BEGIN { FS = "|" }
{
  init = $4; n = 0; tmp = init
  while (match(tmp, /[A-Z][A-Za-z0-9_]*::[A-Z][A-Za-z0-9_]*/)) {
    n++; tmp = substr(tmp, RSTART + RLENGTH)
  }
  if ($3 == "ALL") { printf "%s|%s|%s|named\n", $1, $2, $3; next }
  if (init ~ /^&?\[/ && n >= 2) printf "%s|%s|%s|unnamed\n", $1, $2, $3
}'

# A READER THAT DID NOT RUN IS NOT AN EMPTY DOCUMENT, and every reader
# below crosses a process substitution, where a status cannot reach its
# caller. That is `lib.sh`'s `|| true` lesson with a different tool in
# the pipeline: folded to "no rows", a dead `awk` would report the
# README's table as absent and this gate's own diagnosis would name the
# wrong thing. `gate_grep` writes `$GATE_MATCHER_FAILED` for exactly
# this crossing and `gate_ok` refuses to print over it; these readers
# write the same marker.
reader_failed() {
  gate_error "$(gate_name): the $1 exited $2, so what it did not read is unknown and the checks below it decided nothing — that is not a clean scan"
  : >> "$GATE_MATCHER_FAILED"
  exit "$2"
}

# THE MARKER IS READ WHERE THE CALLER RESUMES, not only at `gate_ok`.
# `gate_ok` refusing to print over it is what makes a green impossible;
# this is what keeps the diagnosis honest in between. Without it the
# checks below run over an empty read and print their own conclusions —
# "delete this row, its list is gone" — about a scan that never
# happened, which is a misdiagnosis pointing at the wrong repair.
abort_if_reader_failed() {
  if [ -e "$GATE_MATCHER_FAILED" ]; then
    rm -f "$GATE_MATCHER_FAILED"
    gate_error "$(gate_name): a reader failed to run during this pass (diagnosed above), so what it did not read is unknown — the checks below it are not asked, because their answers would be about a scan that did not happen"
    exit 1
  fi
}

const_items() {
  local status=0
  gate_rust_code "$@" | awk "$ITEM_AWK" || status=$?
  if [ "$status" -ne 0 ]; then
    reader_failed "const-item reader over $SRC" "$status"
  fi
}

# The README section's own bolded bullets: `- **A deliberately partial
# list** claims no completeness…` yields `A deliberately partial list`.
# Read between the section heading and the next heading of any level, so
# the table below cannot contribute a kind to the vocabulary it is
# checked against.
readme_kinds() {
  local status=0
  awk -v want="$SECTION" '
    $0 == want { inside = 1; next }
    inside && /^#/ { inside = 0 }
    inside && /^- \*\*/ { print }
  ' "$README" |
    sed -nE 's/^- \*\*(.+)\*\*.*/\1/p' || status=$?
  if [ "$status" -ne 0 ]; then
    reader_failed "kinds reader over $README" "$status"
  fi
}

# The roster table, as `@` (the heading was found) followed by every
# table line under it. Emitting the heading as a record is what lets the
# caller tell "the heading is gone" from "the reader died" from "the
# table is empty" — three answers a bare row count folds into one.
readme_table() {
  local status=0
  awk -v want="$TABLE" '
    $0 == want { inside = 1; print "@"; next }
    inside && /^#/ { inside = 0 }
    inside && /^\|/ { print }
  ' "$README" || status=$?
  if [ "$status" -ne 0 ]; then
    reader_failed "table reader over $README" "$status"
  fi
}

# One `LIST|MODULE|KIND` per data row. The header and separator rows are
# dropped by requiring two backticked cells, which is also what stops a
# prose line inside the table being read as a row.
table_rows() {
  local status=0
  printf '%s\n' ${1:+"$@"} |
    sed -nE 's/^\|[[:space:]]*`([A-Za-z0-9_:]+)`[[:space:]]*\|[[:space:]]*`([A-Za-z0-9_:]+)`[[:space:]]*\|[[:space:]]*(.+[^[:space:]])[[:space:]]*\|[[:space:]]*$/\1|\2|\3/p' || status=$?
  if [ "$status" -ne 0 ]; then
    reader_failed "row reader over $README" "$status"
  fi
}

# `session::select` is `session/select.rs`; `forms` is `forms.rs` — the
# spelling the README's tables already use for a module.
module_path() { printf '%s.rs\n' "${1//:://}"; }
# `Theme::ALL` is declared as `const ALL`; `BOOLEAN_OPS` as itself.
const_name() { printf '%s\n' "${1##*::}"; }

# LIST MEMBERSHIP IN BASH, not `printf … | grep -qxF`, for
# `viewer-module-kinds.sh`'s reason: a grep that could not run would
# read as "not in the list", which is a false red here and a silently
# dropped ratification next door. The lists are a handful of short
# strings in memory; a search that cannot fail cannot fail wrong.
contains() {
  local needle=$1 item
  shift
  for item in "$@"; do
    [ "$item" = "$needle" ] && return 0
  done
  return 1
}

gate() {
  local rc=0 row hit kind
  gate_require_file "$README"
  if [ ! -d "$SRC" ]; then
    gate_error "$(gate_name): $SRC does not exist under $PWD — the gate's subject is gone, so it scanned nothing, which is not a pass"
    exit 1
  fi
  local -a sources=()
  mapfile -t sources < <(find "$SRC" -type f -name '*.rs' | sort)
  GATE_SCAN_FILES=${#sources[@]}
  if [ "$GATE_SCAN_FILES" -eq 0 ]; then
    gate_error "$(gate_name): no .rs files under $SRC in $PWD — the gate scanned nothing, which is not a pass"
    exit 1
  fi

  # --- what the README ratifies ---------------------------------------
  local -a kinds=()
  mapfile -t kinds < <(readme_kinds)
  abort_if_reader_failed
  if [ "${#kinds[@]}" -eq 0 ]; then
    gate_error "$(gate_name): $README's \"$SECTION\" section yielded no \`- **kind**\` bullets, so the vocabulary of reasons a hand-written list may be kept came from nowhere. Either the heading was renamed or the bullets were reshaped — a roster that reads nothing is not a pass"
    exit 1
  fi

  local -a block=()
  mapfile -t block < <(readme_table)
  abort_if_reader_failed
  if [ "${#block[@]}" -eq 0 ]; then
    gate_error "$(gate_name): $README carries no \"$TABLE\" heading, so this gate's allowlist has no home and every hand-written list under $SRC would red. That table IS the allowlist — restore it, or retire this gate deliberately"
    exit 1
  fi
  # `@` is the heading; the first table line after it must be the header
  # row, so a reordered or renamed column is a diagnosis rather than an
  # allowlist read off the wrong cells.
  if [ "${block[1]:-}" != "$TABLE_HEADER" ]; then
    gate_error "$(gate_name): $README's \"$TABLE\" table does not open with the header row \`$TABLE_HEADER\` (it opens with \`${block[1]:-<nothing>}\`), so this gate would read its allowlist off columns that no longer mean what it thinks. Restore the header, or change this reader in the same diff"
    exit 1
  fi
  local -a rows=()
  mapfile -t rows < <(table_rows "${block[@]:2}")
  abort_if_reader_failed

  # A row may claim only a kind the section ratifies. Without this a
  # fourth kind arrives as a table edit — which is a docs-tier change
  # set, which is why this gate is sited where a docs-tier change set
  # still runs it.
  for row in ${rows[@]+"${rows[@]}"}; do
    kind=${row##*|}
    if ! contains "$kind" "${kinds[@]}"; then
      gate_error "$README's \"$TABLE\" row \`${row%%|*}\` claims the kind \"$kind\", which is not one the \"$SECTION\" section ratifies. Its bolded bullets are the ratified kinds: $(printf '"%s" ' "${kinds[@]}")— a fourth kind is an amendment to that section, argued there, not a new word in this table"
      rc=1
    fi
  done
  [ "$rc" -eq 0 ] || exit 1

  # --- what the tree holds --------------------------------------------
  local -a hits=()
  mapfile -t hits < <(const_items "${sources[@]}" | awk "$HIT_AWK")
  abort_if_reader_failed

  local file lineno name shape want_file list module ratified seen
  # --- 1. EVERY HIT IS RATIFIED ----------------------------------------
  for hit in ${hits[@]+"${hits[@]}"}; do
    IFS='|' read -r file lineno name shape <<<"$hit"
    ratified=false
    for row in ${rows[@]+"${rows[@]}"}; do
      IFS='|' read -r list module kind <<<"$row"
      want_file=$SRC/$(module_path "$module")
      if [ "$file" = "$want_file" ] && [ "$name" = "$(const_name "$list")" ]; then
        ratified=true
        break
      fi
    done
    if [ "$ratified" = true ]; then
      continue
    fi
    if [ "$shape" = named ]; then
      gate_error "$file:$lineno declares a hand-written \`const $name\` — a closed vocabulary's membership list is declared ONCE ($README, \"$SECTION\"), and a second copy of it is free to fall behind the enum: adding a variant compiles, the row that walks the list silently narrows. Declare the enum and its \`ALL\` together with \`vocabulary!\` ($MACRO), or add a row to \"$TABLE\" naming one of the kinds that section ratifies and say why in the list's own doc"
    else
      gate_error "$file:$lineno declares \`const $name\`, a hand-written array of two or more \`Type::Variant\` entries — that is a membership list whatever it is called, and nothing holds it against the enum it lists ($README, \"$SECTION\"). Declare the enum and its \`ALL\` together with \`vocabulary!\` ($MACRO), or add a row to \"$TABLE\" naming one of the kinds that section ratifies and say why in the list's own doc"
    fi
    rc=1
  done

  # --- 2. EVERY ROW STILL HAS A LIST BEHIND IT -------------------------
  # An allowlist entry with nothing behind it is a ratification waiting
  # to be inherited by the next thing written at that name, and a
  # converted vocabulary leaves no array literal at all — so a row that
  # outlives its list is the shape this check exists to retire.
  for row in ${rows[@]+"${rows[@]}"}; do
    IFS='|' read -r list module kind <<<"$row"
    want_file=$SRC/$(module_path "$module")
    seen=false
    for hit in ${hits[@]+"${hits[@]}"}; do
      IFS='|' read -r file lineno name shape <<<"$hit"
      if [ "$file" = "$want_file" ] && [ "$name" = "$(const_name "$list")" ]; then
        seen=true
        break
      fi
    done
    if [ "$seen" = true ]; then
      continue
    fi
    gate_error "$README's \"$TABLE\" row \`$list\` says \`$module\` declares a hand-written list, and $want_file declares no such \`const\` this gate can see. Either the list was converted — in which case delete the row, because an allowlist entry with nothing behind it is a ratification the next thing written at that name inherits — or it moved, and the row moves with it"
    rc=1
  done

  [ "$rc" -eq 0 ] || exit 1
  gate_ok "no hand-written membership list under $SRC that \"$TABLE\" does not ratify (${#rows[@]} ratified, ${#kinds[@]} kinds read from \"$SECTION\")"
}

# --- THE FIXTURES ------------------------------------------------------
#
# The subject is a crate directory AND a README, so the clean tree
# carries both. `gate_plant_clean_sources` keeps `lib.sh`'s own
# `crates/*/src` line in one place rather than writing it a third time.
readme_fixture() {
  mkdir -p "$1/crates/viewer/src"
  printf 'pub fn identity(x: f64) -> f64 { x }\n' > "$1/crates/viewer/src/lib.rs"
  cat > "$1/crates/viewer/README.md" <<'MD'
### Closed vocabularies are declared once

Prose.

- **A registry of struct constants** is not an enumeration of variants.
- **A deliberately partial list** claims no completeness.
- **A mirror of an enum declared in another crate** cannot be projected.

#### The lists that stay hand-written

| List | Module | Kind |
|---|---|---|

### Something else
MD
}

gate_plant_clean() {
  gate_plant_clean_sources "$1"
  readme_fixture "$1"
}

# Append a row to the fixture's roster table. The separator row is the
# anchor, so a row lands inside the table wherever the table sits.
add_row() {
  sed -i "s#^|---|---|---|\$#|---|---|---|\n| \`$2\` | \`$3\` | $4 |#" \
    "$1/crates/viewer/README.md"
}

plant_named_all() {
  cat > "$1/crates/viewer/src/choice.rs" <<'RS'
pub enum NewChoice { A, B, C }
impl NewChoice {
    pub const ALL: [Self; 3] = [Self::A, Self::B, Self::C];
}
RS
}

plant_unnamed_one_line() {
  printf 'const KINDS: [Kind; 2] = [Kind::A, Kind::B];\n' \
    > "$1/crates/viewer/src/kinds.rs"
}

plant_unnamed_multiline() {
  cat > "$1/crates/viewer/src/kinds.rs" <<'RS'
pub(crate) const KIND_LABELS: [(Kind, &str); 3] = [
    (Kind::A, "a"),
    (Kind::B, "b"),
    (Kind::C, "c"),
];
RS
}

# The un-named arm with the entries in a nested literal, which is where
# a `;`-cut statement view lost the initialiser entirely.
plant_unnamed_nested() {
  cat > "$1/crates/viewer/src/kinds.rs" <<'RS'
pub const SHAPES: [(Kind, &str); 2] = [
    (Kind::Rest { offset: 0.0 }, "rest"),
    (Kind::Move { offset: 1.0 }, "move"),
];
RS
}

plant_ratified_named() {
  plant_named_all "$1"
  add_row "$1" 'NewChoice::ALL' choice 'A registry of struct constants'
}

plant_ratified_unnamed() {
  plant_unnamed_one_line "$1"
  add_row "$1" KINDS kinds 'A deliberately partial list'
}

plant_row_with_no_list() {
  add_row "$1" GHOSTS ghosts 'A deliberately partial list'
}

plant_row_of_an_unratified_kind() {
  plant_unnamed_one_line "$1"
  add_row "$1" KINDS kinds 'because I said so'
}

# The allowlist is keyed on the MODULE as well as the name, so a
# ratified `Theme::ALL` in `theme` does not ratify an `ALL` next door.
plant_named_all_in_another_module() {
  plant_ratified_named "$1"
  cat > "$1/crates/viewer/src/other.rs" <<'RS'
pub enum Other { A, B }
impl Other {
    pub const ALL: [Self; 2] = [Self::A, Self::B];
}
RS
}

plant_table_heading_gone() {
  sed -i '/^#### The lists that stay hand-written$/d' \
    "$1/crates/viewer/README.md"
}

plant_table_header_reordered() {
  sed -i 's/^| List | Module | Kind |$/| Module | List | Kind |/' \
    "$1/crates/viewer/README.md"
}

plant_kind_bullets_gone() {
  sed -i '/^- \*\*/d' "$1/crates/viewer/README.md"
}

plant_readme_gone() { rm -f "$1/crates/viewer/README.md"; }

plant_src_gone() { rm -rf "$1/crates/viewer/src"; }

# --- THE NEAR MISSES ---------------------------------------------------
#
# Every one of these is a spelling the gate must NOT fire on, and each
# is a direction a widening of either arm would break. Without them a
# passing fixture proves only that the empty tree is quiet.
pass_converted_vocabulary() {
  cat > "$1/crates/viewer/src/vocab_user.rs" <<'RS'
vocabulary! {
    pub enum Kind {
        A,
        B,
        C,
    }
    pub const ALL;
}
RS
}

pass_single_variant() {
  printf 'const PRESETS: &[(&str, Map)] = &[("", Map::DEFAULT)];\n' \
    > "$1/crates/viewer/src/presets.rs"
}

pass_scalar_array() {
  printf 'const PITCH_STEPS: [f64; 3] = [1.0, 2.0, 5.0];\n' \
    > "$1/crates/viewer/src/steps.rs"
}

pass_prose_naming_the_shape() {
  cat > "$1/crates/viewer/src/doc.rs" <<'RS'
//! An example, in prose:
//!
//!     pub const ALL: [Self; 3] = [Self::A, Self::B, Self::C];
//!
/// The `const ALL: [Self; 2] = [Self::A, Self::B];` shape, described.
pub fn documented() {}
RS
}

pass_not_a_const() {
  printf 'fn seats() -> Seats { Seats::new([Seat::A, Seat::B]) }\n' \
    > "$1/crates/viewer/src/seats.rs"
}

pass_tests_are_not_scanned() {
  mkdir -p "$1/crates/viewer/tests"
  printf 'const KINDS: [Kind; 2] = [Kind::A, Kind::B];\n' \
    > "$1/crates/viewer/tests/suite.rs"
}

gate_selftest() {
  gate_selftest_clean
  # THE SCAN-TARGET GUARDS, each shown to fire. A gate that reports
  # green over a subject that is gone is the failure this directory is
  # a reaction to.
  gate_selftest_case "does not exist under" plant_readme_gone
  gate_selftest_case "does not exist under" plant_src_gone
  # THE TWO ARMS.
  gate_selftest_case 'declares a hand-written `const ALL`' plant_named_all
  gate_selftest_case 'two or more `Type::Variant` entries' plant_unnamed_one_line
  gate_selftest_case 'two or more `Type::Variant` entries' plant_unnamed_multiline
  gate_selftest_case 'two or more `Type::Variant` entries' plant_unnamed_nested
  # THE ALLOWLIST IS KEYED ON THE MODULE TOO, so a ratified `ALL` in one
  # module does not ratify the `ALL` next door.
  gate_selftest_case 'declares a hand-written `const ALL`' \
    plant_named_all_in_another_module
  # THE README HALF OF THE SUBJECT, all five ways it can go wrong.
  gate_selftest_case "carries no \"$TABLE\" heading" plant_table_heading_gone
  gate_selftest_case "does not open with the header row" plant_table_header_reordered
  gate_selftest_case 'yielded no `- **kind**` bullets' plant_kind_bullets_gone
  gate_selftest_case "which is not one the" plant_row_of_an_unratified_kind
  gate_selftest_case 'declares no such `const` this gate can see' plant_row_with_no_list
  # A DEAD READER IS NOT AN EMPTY DOCUMENT. Every reader here crosses a
  # process substitution, so its status cannot reach the caller and a
  # fold to "no rows" would report the README as empty instead of the
  # reader as dead. The first case kills `awk` outright; the second
  # kills it only for the SHARED RUST READER — the deepest reader, past
  # two that must succeed first — with a breach planted in the tree, so
  # a green there could only come from a scan that did not happen.
  gate_selftest_without_tool awk "the kinds reader over"
  gate_selftest_with_broken_tool awk "the const-item reader over" \
    'case "$*" in *SKIPTEST*) exit 9 ;; esac
exec "$GATE_REAL_TOOL" "$@"' plant_named_all
  # THE NEAR MISSES.
  gate_selftest_passes "a ratified named list" plant_ratified_named
  gate_selftest_passes "a ratified un-named list" plant_ratified_unnamed
  gate_selftest_passes "a vocabulary! invocation's projected ALL" \
    pass_converted_vocabulary
  gate_selftest_passes "a const array naming one associated constant" \
    pass_single_variant
  gate_selftest_passes "a const array of scalars" pass_scalar_array
  gate_selftest_passes "the shape written in a doc comment" \
    pass_prose_naming_the_shape
  gate_selftest_passes "a membership list that is not a const" pass_not_a_const
  gate_selftest_passes "a const list under tests/, which is out of scope" \
    pass_tests_are_not_scanned
  printf '%s selftest OK: passes a clean fixture and eight near misses, fires on both arms (one-line, multi-line and nested), on a ratified name in an unratified module, on all five ways the README half can go wrong, and on a reader that could not run — outright and mid-scan\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
