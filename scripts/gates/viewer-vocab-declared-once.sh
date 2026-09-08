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
# it. `static` opens an item in both arms for the same reason: a `pub
# static ALL: [Kind; 3]` is the same list with a different keyword on
# it, and the diagnosis names whichever it found.
#
# WHERE THE ALLOWLIST LIVES, AND WHY IT IS NOT IN THIS FILE. The README
# section ratifies THREE kinds of list that stay hand-written, and its
# `#### The lists that stay hand-written` table is the roster: one row
# per list, naming the module it is declared in and which of the three
# kinds it is. This file reads that table, and reads it only INSIDE that
# section, because the README's own word for where the roster sits is
# "below" and a reader finding it anywhere on the page makes that word
# decide nothing. It also reads the KINDS from the section's own bolded
# bullets, so the vocabulary of reasons is the ratification itself and
# not a copy of it — deleting a bullet while leaving a row that claims
# it reds. That is `viewer-module-kinds.sh`'s contract and the reason
# its rosters have not gone stale.
#
# HOW MANY KINDS is held here and nowhere else, and it is the one number
# this file does keep. Reading the bullets makes a row's kind be the
# ratification; it does not make the SET of kinds be one, because a
# fourth bullet plus a row claiming it is internally consistent and both
# green. `KIND_COUNT` below is the assertion that the section still
# ratifies three, so an amendment costs an edit to this file as well —
# which is right for an amendment to a ratification and wrong for the
# docs-tier table edit it would otherwise arrive as.
#
# THE ROSTER RETIRES ITSELF, AND IT DOES NOT SPREAD. A row naming a list
# the scan does not find reds: an allowlist entry with nothing behind it
# is a ratification waiting to be inherited by whatever is written at
# that name next (`interval-square-allowlist.sh:185-192` argues the same
# about its own retired entries), and a converted vocabulary leaves no
# array literal behind — so converting a list and forgetting its row is
# a red, not a silent inheritance. A row naming a module and a name that
# TWO lists answer to reds for the mirror reason: a row is keyed on the
# module and the name, never on the type, so `Theme::ALL` would
# otherwise ratify a second `impl Badge { pub const ALL: … }` appended
# to `theme.rs` — inheritance in the other direction, by an author who
# never touched the roster. That is the class
# `interval-square-allowlist.sh:102-105` names as its KNOWN GAP 4 and
# leaves open — a file-granular entry inheriting a second unrelated site
# silently — closed here rather than inherited, because a roster of four
# rows can afford the count. One row ratifies one list, and two rows for
# one list red as well, because the count this gate reports has to name
# as many lists as the crate holds. The price is stated where it is
# paid: two same-named lists in one module cannot both be rostered, and
# the red says to move or rename one.
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
#   * A LIST THAT IS NOT A `const` OR A `static` ITEM.
#     `Seats::new([Seat::A, Seat::B])` is a hand-written membership list
#     and this gate does not see it — which is correct for the seat
#     lists, whose partiality the README ratifies, and blind for
#     whatever else is written that way. The item anchor is what keeps
#     the un-named arm's population small; a scan over every array
#     literal in the crate would hit match arms, builder calls and test
#     fixtures.
#   * A SECOND ITEM AFTER THE FIRST ONE'S `;` ON THE SAME LINE, per the
#     item reader's own header. One-directional: a miss, never a false
#     red, and not a spelling rustfmt produces.
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
#   * WHETHER THE ROSTER STILL SAYS WHAT WAS RATIFIED. What is held is
#     the roster's INTERNAL consistency — every row claims a kind the
#     section's bullets spell, the section spells `KIND_COUNT` of them,
#     every row has exactly one list behind it, no two rows name one
#     list — never its fidelity to the argument Ev approved. A bullet
#     and the cells claiming it, REWORDED TOGETHER, are invisible here;
#     that is a review's job and this gate does not pretend otherwise.
#
# AND WHAT IT HITS ON THAT IS NOT A VOCABULARY, because "cannot catch"
# is only half a sweep's blind spot. The un-named arm's population is
# `const`/`static` items whose initialiser is an array or slice literal
# holding TWO OR MORE `Type::CONST`-shaped paths — a shape a list of
# associated CONSTANTS also has. `const AXES: [Vec3; 3] = [Vec3::X,
# Vec3::Y, Vec3::Z]` reds, and the behaviour is kept: the message's
# second repair — ratify it with a row saying why — is the right answer
# for a list of constants that nothing holds against its type, and the
# roster is where the argument for keeping one belongs. The COST is
# `lib.sh:246-254`'s, and it is the reason this is written down rather
# than left implied: a false red is a nudge toward the allowlist rather
# than the fix, and an `interval-square-allowlist.sh` entry was once
# justified in writing partly by one. Four rows is a roster a reader can
# audit; the sentence this replaces claimed a false-positive rate of
# NIL, which was not true of the population above. `Theme::ALL`'s own
# entries are bare constants and reach only the named arm, which is why
# the two arms are not redundant.
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
# And its separator, because a markdown table without one is not a
# table: every renderer shows the header and the rows as a paragraph,
# while this reader — which only wants lines starting `|` — goes on
# reading the roster perfectly. A roster the machine reads and the human
# cannot is this gate's own thesis inverted, and the header row above is
# already asserted for the neighbouring reason.
TABLE_SEPARATOR='|---|---|---|'

# HOW MANY KINDS THE SECTION RATIFIES. The README says "the KINDS they
# may claim are the three bolded bullets above", and until this constant
# nothing held that number: a fourth bullet plus a row claiming it both
# green, and the gate checked the roster's internal CONSISTENCY rather
# than its fidelity to what was ratified. It cannot check the fidelity —
# rewording a bullet and its matching cell together is invisible to it,
# and that is stated in WHAT IT CANNOT CATCH above — but the COUNT it
# can hold, and holding it means a fourth kind requires an edit to this
# file as well as to that section. That is the right price for an
# amendment to a ratification, and the wrong one for a docs-tier edit
# that arrives alone.
KIND_COUNT=3

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
# AN OPENING LINE IS A DECLARATION AT THE START OF ONE, and the anchor
# is the whole reason this reader can be trusted past its first hit.
# `const` also opens a GENERIC PARAMETER — `fn stack<const N: usize>`,
# `struct Stack<const N: usize>`, `impl<const N: usize>` — and the depth
# counter counts `()[]{}`, never `<>`. An unanchored opening therefore
# started accumulating mid-signature, the trailing `{` left it at depth
# one, and the item did not close until the next depth-zero `;`, which
# inside an `impl` block is never: the `const ALL` three lines below was
# swallowed and emitted under the name `N`, or not emitted at all. Both
# directions were live — a hand-written `ALL` under a const-generic
# `struct` went GREEN, and a const-generic `fn` above `BOOLEAN_OPS` on
# the real tree produced two reds naming the wrong repair. `DECL` is the
# anchored declaration prefix and `OPEN` is `DECL` plus the name and its
# `:`, so the test and the name extraction are ONE spelling rather than
# two that can drift — this gate's own thesis, applied to itself.
#
# `static` OPENS ONE TOO. A `pub static ALL: [Kind; 3] = […]` is the
# same hand-written membership list with a different keyword, and
# nothing about the rule is `const`-specific; the alternation is one
# word and closes it rather than documenting it. (No `static` item under
# `crates/*/src` carries an array literal today — 22 exist, every one a
# `thread_local!` cell, a `OnceLock` or an atomic — so the arm costs
# nothing on this tree and is there for the next one.)
#
# NO BACKSLASH APPEARS IN EITHER, per `loop-boundary-discards.sh:222-234`
# and `lib.sh:230-235`: `[(]` and `[)]`, not `\(` and `\)`. Both
# spellings of the population were derived under `gawk` 5.2.1 AND `mawk`
# 1.3.4 and agree line for line.
#
# ONE ITEM PER OPENING LINE. A second `const` written after the first
# one's `;` ON THE SAME LINE is not read; rustfmt does not produce that
# spelling and the residue is one-directional — a miss, never a false
# red. It is the anchor that makes that true in both halves: unanchored,
# the greedy name extraction took the LAST `const` on such a line while
# `flush()` took the FIRST `=`, so a ratified `KINDS` beside a scalar
# `STEPS` was a hit named `STEPS` AND a "row `KINDS` has no list" red.
ITEM_AWK='
BEGIN {
  DECL = "^[[:space:]]*(pub([[:space:]]*[(][^)]*[)])?[[:space:]]+)?(const|static)([[:space:]]+mut)?[[:space:]]+"
  OPEN = DECL "[A-Z_][A-Za-z0-9_]*[[:space:]]*:"
}
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
  printf "%s|%s|%s|%s|%s\n", file, line, kw, name, init
}
{
  # WHERE THE FILE COLUMN ENDS is gate_record_split, prepended to this
  # program by its caller (lib.sh, section THE RECORD S COLUMNS). An
  # item is accumulated ACROSS records and closed when the file changes,
  # so a FILE column read to the first colon spliced two files whose
  # paths agree up to a colon into one item — and put the line number at
  # the front of the text, where the declaration anchor reads it.
  if (!gate_record_split($0)) next
  f = GR_FILE; l = GR_LINE; t = GR_TEXT
  if (acc == 0) {
    if (t !~ OPEN) next
    acc = 1; file = f; line = l; item = ""
    name = t
    sub(DECL, "", name)
    sub(/[^A-Za-z0-9_].*$/, "", name)
    kw = (t ~ /^[[:space:]]*(pub([[:space:]]*[(][^)]*[)])?[[:space:]]+)?static/) ? "static" : "const" 
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

# `FILE|LINE|KEYWORD|NAME|SHAPE` for every item the two arms hit. The
# un-named arm counts `Type::Variant` occurrences INSIDE the initialiser
# and wants two: one is a constant being named, two or more is a list.
#
# THE INITIALISER IS THE REST OF THE RECORD, not `$5`. A Rust
# initialiser may contain `|` — a bitwise or, a closure's parameter
# list — and reading one field of it would count the variants in its
# first segment only, which is a miss in the un-named arm. The four
# fields before it cannot contain one, so the remainder is recoverable
# by length.
HIT_AWK='
BEGIN { FS = "|" }
{
  init = substr($0, length($1) + length($2) + length($3) + length($4) + 5)
  n = 0; tmp = init
  while (match(tmp, /[A-Z][A-Za-z0-9_]*::[A-Z][A-Za-z0-9_]*/)) {
    n++; tmp = substr(tmp, RSTART + RLENGTH)
  }
  if ($4 == "ALL") { printf "%s|%s|%s|%s|named\n", $1, $2, $3, $4; next }
  if (init ~ /^&?\[/ && n >= 2) printf "%s|%s|%s|%s|unnamed\n", $1, $2, $3, $4
}'

# A READER THAT DID NOT RUN IS NOT AN EMPTY DOCUMENT. That is `lib.sh`'s
# `|| true` lesson with a different tool in the pipeline: folded to "no
# rows", a dead `awk` would report the README's table as absent and this
# gate's own diagnosis would name the wrong thing. `gate_grep` writes
# `$GATE_MATCHER_FAILED` for exactly that crossing and `gate_ok` refuses
# to print over it; the readers below write the same marker.
#
# WHICH COMMANDS ARE READERS, AS A RULE AND NOT AS A LIST. A reader is
# any command that reads this gate's SUBJECT — the tree under `$SRC` or
# `$README` — and whose exit status the shell DISCARDS, which here means
# every stage of every pipeline inside a process substitution. The rule
# yields six, and each has a guard below:
#
#   1. the source enumerator   `find … | sort`
#   2. the const-item reader   `gate_rust_code … | awk "$ITEM_AWK"`
#   3. the hit classifier      `awk "$HIT_AWK"`
#   4. the kinds reader        `awk … | sed -nE …`
#   5. the table reader        `awk …`
#   6. the row reader          `printf … | sed -nE …`
#
# STAGES, not pipelines, is the load-bearing half of the rule, and it is
# where the sentence this block used to make was FALSE: the hit
# classifier is the stage AFTER `const_items`, in the same process
# substitution, and it had no guard at all. It survived an immediate
# death only by SIGPIPE upstream, which reds naming the wrong reader; a
# classifier that consumes its input and THEN fails produces no SIGPIPE,
# and the gate then printed one "delete this row, its list is gone" per
# ratified row — or, with no data rows in the table, `OK` and exit 0
# over two planted breaches. Both were reproduced before this guard
# existed.
#
# What is NOT a reader, by the same rule: `module_path`, `const_name`
# and `contains` are pure parameter expansion and `printf`, so they
# invoke nothing that can be missing and read nothing but their own
# arguments. `$(gate_name)` is the same. A command whose status the
# shell KEEPS is not one either — errexit and `pipefail` already end the
# gate on it.
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

viewer_sources() {
  local status=0
  find "$SRC" -type f -name '*.rs' | sort || status=$?
  if [ "$status" -ne 0 ]; then
    reader_failed "source enumerator over $SRC" "$status"
  fi
}

const_items() {
  local status=0
  gate_rust_code "$@" |
    GATE_RECORD_LINE_RE="$GATE_RECORD_LINE_RE" awk "$GATE_RECORD_AWK$ITEM_AWK" || status=$?
  if [ "$status" -ne 0 ]; then
    reader_failed "const-item reader over $SRC" "$status"
  fi
}

# THE CLASSIFIER'S OWN STATUS, NOT THE PIPELINE'S, and the brace group
# is what draws the distinction. `pipefail` reports the RIGHTMOST
# non-zero stage, so `const_items … | awk … || reader_failed` would
# diagnose an upstream death as the classifier failing — the misdiagnosis
# this whole block exists to prevent, re-minted one line inside it.
# `const_items` diagnoses itself; this guard speaks only for `awk`.
const_hits() {
  const_items "$@" |
    { awk "$HIT_AWK" || reader_failed "hit classifier over $SRC" "$?"; }
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
#
# SCOPED TO THE SECTION, like `readme_kinds`, because the README says
# the roster is the table "below" that heading and a reader that finds
# it anywhere in the file makes that word decide nothing: the table
# could be moved to the end of a 1200-line page, under someone else's
# `###`, and this gate would still read it while the two sentences
# pointing at it went stale. The two halves of the ratification are read
# under one scope or the roster and its vocabulary can drift apart.
# `####` is deeper than the section's own `###`, so the section ends at
# a heading of level three or shallower and the table heading is looked
# for only inside it.
readme_table() {
  local status=0
  awk -v sec="$SECTION" -v want="$TABLE" '
    $0 == sec { insec = 1; next }
    insec && $0 == want { inside = 1; print "@"; next }
    insec && /^####/ { inside = 0; next }
    insec && /^#/ { insec = 0; inside = 0; next }
    inside && /^\|/ { print }
  ' "$README" || status=$?
  if [ "$status" -ne 0 ]; then
    reader_failed "table reader over $README" "$status"
  fi
}

# One `LIST|MODULE|KIND` per data row. A prose line inside the table is
# not one, which is what requiring two backticked cells buys; the header
# and separator rows are asserted by the caller and dropped by position,
# so this pattern is not what keeps them out.
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
#
# THE TYPE IS DROPPED, AND THE ROW'S KEY IS THEREFORE MODULE AND NAME.
# That is worth saying at the site, because the `Theme::` half looks
# load-bearing and is not: nothing here resolves a type, and nothing
# can. An associated constant's item text says `[Self; 3]`, not
# `[Theme; 3]`, so the type it belongs to is the enclosing `impl`
# header — a scope the lexed view does not delimit and only a parse
# would. The uniqueness check in `gate()` is what makes the key sound
# instead: one row ratifies ONE list, and a module declaring two
# `const ALL`s reds rather than letting one row ratify both.
const_name() { printf '%s\n' "${1##*::}"; }

# The key a row and a hit are matched on. One spelling, so the two
# checks below cannot disagree about what "the same list" means.
list_key() { printf '%s|%s\n' "$SRC/$(module_path "$2")" "$(const_name "$1")"; }

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
  mapfile -t sources < <(viewer_sources)
  # BEFORE the count, not after it. A dead enumerator yields no paths,
  # and "no .rs files under crates/viewer/src" is the wrong answer to
  # give about a directory nobody managed to list.
  abort_if_reader_failed
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
  if [ "${#kinds[@]}" -ne "$KIND_COUNT" ]; then
    gate_error "$(gate_name): $README's \"$SECTION\" section ratifies $KIND_COUNT kinds of hand-written list and this pass read ${#kinds[@]}: $(printf '"%s" ' "${kinds[@]}"). A kind added or removed is an AMENDMENT to that ratification, not a docs edit — argue it there and change \`KIND_COUNT\` in $0 in the same diff, so the amendment cannot arrive as a table cell nobody had to approve"
    exit 1
  fi

  local -a block=()
  mapfile -t block < <(readme_table)
  abort_if_reader_failed
  if [ "${#block[@]}" -eq 0 ]; then
    gate_error "$(gate_name): $README carries no \"$TABLE\" heading inside \"$SECTION\", so this gate's allowlist has no home and every hand-written list under $SRC would red. That table IS the allowlist, and it is read only where the section that ratifies it can be read too — restore it there, or retire this gate deliberately"
    exit 1
  fi
  # `@` is the heading; the two table lines after it must be the header
  # and separator rows, so a reordered or renamed column is a diagnosis
  # rather than an allowlist read off the wrong cells, and a table that
  # no renderer will draw is one too.
  if [ "${block[1]:-}" != "$TABLE_HEADER" ]; then
    gate_error "$(gate_name): $README's \"$TABLE\" table does not open with the header row \`$TABLE_HEADER\` (it opens with \`${block[1]:-<nothing>}\`), so this gate would read its allowlist off columns that no longer mean what it thinks. Restore the header, or change this reader in the same diff"
    exit 1
  fi
  if [ "${block[2]:-}" != "$TABLE_SEPARATOR" ]; then
    gate_error "$(gate_name): $README's \"$TABLE\" table has no \`$TABLE_SEPARATOR\` row under its header (the next line is \`${block[2]:-<nothing>}\`), so no markdown renderer draws it as a table and a human reader sees the roster as a paragraph. This reader would go on parsing it perfectly, which is the divergence this gate exists to refuse — restore the separator"
    exit 1
  fi
  local -a rows=()
  mapfile -t rows < <(table_rows "${block[@]:3}")
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

  # ONE ROW PER LIST, IN THE TABLE'S OWN DIRECTION. Two rows keyed on
  # the same module and name ratify one list twice — and, when they
  # claim different kinds, say two different things about it while the
  # OK line counts both. `Theme::ALL` and `ALL` in `theme` are one key,
  # so the duplicate is caught on what the gate computes rather than on
  # how the cell is spelled.
  local -a seen_keys=()
  local key
  for row in ${rows[@]+"${rows[@]}"}; do
    IFS='|' read -r list module kind <<<"$row"
    key=$(list_key "$list" "$module")
    if contains "$key" ${seen_keys[@]+"${seen_keys[@]}"}; then
      gate_error "$README's \"$TABLE\" carries more than one row for \`$module\`'s \`$(const_name "$list")\` (this one says \`$list\` / \"$kind\"). One row ratifies one list: two of them ratify it twice, disagree about its kind if their cells differ, and make the count this gate reports name more lists than the crate holds. Delete the duplicate, or split the rows onto the two lists they meant"
      rc=1
    fi
    seen_keys+=("$key")
  done
  [ "$rc" -eq 0 ] || exit 1

  # --- what the tree holds --------------------------------------------
  local -a hits=()
  mapfile -t hits < <(const_hits "${sources[@]}")
  abort_if_reader_failed

  local file lineno kw name shape want_key list module ratified matched
  # --- 1. EVERY HIT IS RATIFIED ----------------------------------------
  for hit in ${hits[@]+"${hits[@]}"}; do
    IFS='|' read -r file lineno kw name shape <<<"$hit"
    ratified=false
    for row in ${rows[@]+"${rows[@]}"}; do
      IFS='|' read -r list module kind <<<"$row"
      if [ "$(list_key "$list" "$module")" = "$file|$name" ]; then
        ratified=true
        break
      fi
    done
    if [ "$ratified" = true ]; then
      continue
    fi
    if [ "$shape" = named ]; then
      gate_error "$file:$lineno declares a hand-written \`$kw $name\` — a closed vocabulary's membership list is declared ONCE ($README, \"$SECTION\"), and a second copy of it is free to fall behind the enum: adding a variant compiles, the row that walks the list silently narrows. Declare the enum and its \`ALL\` together with \`vocabulary!\` ($MACRO), or add a row to \"$TABLE\" naming one of the kinds that section ratifies and say why in the list's own doc"
    else
      gate_error "$file:$lineno declares \`$kw $name\`, a hand-written array of two or more \`Type::Variant\` entries — that is a membership list whatever it is called, and nothing holds it against the enum it lists ($README, \"$SECTION\"). Declare the enum and its \`ALL\` together with \`vocabulary!\` ($MACRO), or add a row to \"$TABLE\" naming one of the kinds that section ratifies and say why in the list's own doc"
    fi
    rc=1
  done

  # --- 2. EVERY ROW HAS EXACTLY ONE LIST BEHIND IT ---------------------
  # NONE is the retiring direction: an allowlist entry with nothing
  # behind it is a ratification waiting to be inherited by the next thing
  # written at that name, and a converted vocabulary leaves no array
  # literal at all.
  #
  # MORE THAN ONE is the other direction and it is why this counts
  # rather than breaking on the first match. A row names a module and a
  # NAME, never a type — `Theme::ALL` keys on `theme.rs` and `ALL` — so
  # a second, unrelated `impl Badge { pub const ALL: … }` appended to
  # `theme.rs` was ratified by the row that was written for `Theme`, and
  # the OK line went on saying four. One row cannot ratify two lists;
  # each needs its own argument, and two lists that cannot be told apart
  # by module and name need one of them moved or renamed.
  for row in ${rows[@]+"${rows[@]}"}; do
    IFS='|' read -r list module kind <<<"$row"
    want_key=$(list_key "$list" "$module")
    matched=0
    for hit in ${hits[@]+"${hits[@]}"}; do
      IFS='|' read -r file lineno kw name shape <<<"$hit"
      if [ "$want_key" = "$file|$name" ]; then
        matched=$((matched + 1))
      fi
    done
    if [ "$matched" -eq 1 ]; then
      continue
    fi
    if [ "$matched" -eq 0 ]; then
      gate_error "$README's \"$TABLE\" row \`$list\` says \`$module\` declares a hand-written list, and ${want_key%|*} declares no such \`const\` this gate can see. Either the list was converted — in which case delete the row, because an allowlist entry with nothing behind it is a ratification the next thing written at that name inherits — or it moved, and the row moves with it"
    else
      gate_error "$README's \"$TABLE\" row \`$list\` ratifies ONE list, and ${want_key%|*} declares $matched of them under the name \`${want_key##*|}\`. A row is keyed on the module and the name, never on the type, so this one ratifies every list that answers to both — including whichever was written after it was approved. Give each list its own row and its own name, or move one out of that module"
    fi
    rc=1
  done

  [ "$rc" -eq 0 ] || exit 1
  gate_ok "no hand-written membership list under $SRC that \"$TABLE\" does not ratify (${#rows[@]} ratified, one list each; ${#kinds[@]} kinds read from \"$SECTION\")"
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

# THE SAME LIST IN A FILE WHOSE PATH CARRIES A COLON, which is legal
# here and in git. The item reader accumulates ACROSS records and keys
# the accumulation on the FILE column, so read to the first colon this
# file shared a key with every sibling whose path begins the same way
# (one unclosed item then swallowed the next file) and the declaration
# line arrived with the line number in front of it, where the anchored
# `const` pattern reads it. The case is asserted on the PATH the
# diagnosis carries, which is what says the column was read whole.
plant_named_all_colon_path() {
  cat > "$1/crates/viewer/src/a:b.rs" <<'RS'
pub enum ColonChoice { A, B, C }
impl ColonChoice {
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

# MAJOR-2's shape: `const` also opens a generic parameter, and an
# unanchored reader started accumulating there. Both directions, because
# the fix has to hold both: the `struct` case swallowed the `ALL` below
# it into an item that never closed (green over the item's own worked
# example), and the `fn` case emitted its list under the name `N`.
plant_const_generic_struct_above_an_all() {
  cat > "$1/crates/viewer/src/choice.rs" <<'RS'
pub enum NewChoice { A, B, C }

pub struct Stack<const N: usize> {
    items: [NewChoice; N],
}

impl NewChoice {
    pub const ALL: [Self; 3] = [Self::A, Self::B, Self::C];
}
RS
}

plant_const_generic_fn_above_a_list() {
  cat > "$1/crates/viewer/src/kinds.rs" <<'RS'
pub fn stack<const N: usize>(v: [Kind; N]) -> usize {
    v.len()
}

pub const KIND_LABELS: [(Kind, &str); 2] = [(Kind::A, "a"), (Kind::B, "b")];
RS
}

# MINOR-1's shape: the same membership list under the other keyword.
plant_static_all() {
  printf 'pub static ALL: [Kind; 3] = [Kind::A, Kind::B, Kind::C];\n' \
    > "$1/crates/viewer/src/kinds.rs"
}

# MAJOR-3's shape: one row, two lists answering to its module and name.
plant_two_lists_under_one_row() {
  plant_ratified_named "$1"
  cat >> "$1/crates/viewer/src/choice.rs" <<'RS'

pub enum Badge { A, B, C }
impl Badge {
    pub const ALL: [Self; 3] = [Self::A, Self::B, Self::C];
}
RS
}

# MINOR-6's shape: one list, two rows. The second is spelled with a type
# so the check is shown keying on what it computes, not on the cell.
plant_two_rows_for_one_list() {
  plant_ratified_unnamed "$1"
  add_row "$1" 'Kind::KINDS' kinds 'A registry of struct constants'
}

plant_a_fourth_kind() {
  sed -i 's/^- \*\*A deliberately partial list\*\*.*$/&\n- **A list I felt like keeping** for no stated reason./' \
    "$1/crates/viewer/README.md"
  plant_unnamed_one_line "$1"
  add_row "$1" KINDS kinds 'A list I felt like keeping'
}

plant_table_separator_gone() {
  sed -i '/^|---|---|---|$/d' "$1/crates/viewer/README.md"
}

# The roster is read only inside the section that ratifies it, so a table
# moved out from under that heading is gone as far as this gate is
# concerned — which is what the README's word "below" has to mean.
plant_table_outside_the_section() {
  local md=$1/crates/viewer/README.md
  sed -i -e "/^$TABLE\$/d" -e '/^| List | Module | Kind |$/d' \
    -e '/^|---|---|---|$/d' "$md"
  printf '\n### Elsewhere\n\n%s\n\n%s\n%s\n' \
    "$TABLE" "$TABLE_HEADER" "$TABLE_SEPARATOR" >> "$md"
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

# ONE ITEM PER OPENING LINE, in the passing direction. Unanchored, the
# name extraction took the LAST `const` on this line and `flush()` the
# FIRST `=`, so the ratified `KINDS` was reported as an unratified list
# named `STEPS` AND its own row was reported as having no list — one
# spelling, two false reds. The second item is still not read, which is
# the one-directional miss the reader's header states.
pass_two_consts_on_one_line() {
  printf 'const KINDS: [Kind; 2] = [Kind::A, Kind::B]; const STEPS: usize = 3;\n' \
    > "$1/crates/viewer/src/kinds.rs"
  add_row "$1" KINDS kinds 'A deliberately partial list'
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
  # THE TWO ARMS. Three of these assert the NAME the gate reports, not
  # only the shape of the sentence: a name extraction that broke would
  # otherwise ship green over a diagnosis pointing at the wrong item,
  # which is what happened during review of this gate. The durable half
  # — a harness that cannot express "and the subject it named was X" —
  # is `work/issues/gate-selftest-cannot-observe-the-identity-a-gate-names`.
  gate_selftest_case 'declares a hand-written `const ALL`' plant_named_all
  gate_selftest_case 'crates/viewer/src/a:b.rs:3 declares a hand-written `const ALL`' \
    plant_named_all_colon_path
  gate_selftest_case 'declares `const KINDS`, a hand-written array' \
    plant_unnamed_one_line
  gate_selftest_case 'declares `const KIND_LABELS`, a hand-written array' \
    plant_unnamed_multiline
  gate_selftest_case 'declares `const SHAPES`, a hand-written array' \
    plant_unnamed_nested
  # THE OTHER KEYWORD. A `static` array is the same list, and the
  # diagnosis names the keyword it actually found.
  gate_selftest_case 'declares a hand-written `static ALL`' plant_static_all
  # A `const` GENERIC PARAMETER IS NOT AN ITEM OPENING. Unanchored, the
  # first of these went GREEN over the section's own worked example and
  # the second named `const N` at the wrong line.
  gate_selftest_case 'declares a hand-written `const ALL`' \
    plant_const_generic_struct_above_an_all
  gate_selftest_case 'declares `const KIND_LABELS`, a hand-written array' \
    plant_const_generic_fn_above_a_list
  # THE ALLOWLIST IS KEYED ON THE MODULE TOO, so a ratified `ALL` in one
  # module does not ratify the `ALL` next door — nor a SECOND `ALL` in
  # the same module, which the module key alone cannot tell from the
  # first.
  gate_selftest_case 'declares a hand-written `const ALL`' \
    plant_named_all_in_another_module
  gate_selftest_case 'row `NewChoice::ALL` ratifies ONE list' \
    plant_two_lists_under_one_row
  # THE README HALF OF THE SUBJECT, every way it can go wrong.
  gate_selftest_case "carries no \"$TABLE\" heading" plant_table_heading_gone
  gate_selftest_case "carries no \"$TABLE\" heading" plant_table_outside_the_section
  gate_selftest_case "does not open with the header row" plant_table_header_reordered
  gate_selftest_case "has no \`$TABLE_SEPARATOR\` row" plant_table_separator_gone
  gate_selftest_case 'yielded no `- **kind**` bullets' plant_kind_bullets_gone
  gate_selftest_case "ratifies $KIND_COUNT kinds of hand-written list and this pass read 4" \
    plant_a_fourth_kind
  gate_selftest_case "which is not one the" plant_row_of_an_unratified_kind
  gate_selftest_case 'row `GHOSTS` says `ghosts` declares' plant_row_with_no_list
  gate_selftest_case 'more than one row for `kinds`' plant_two_rows_for_one_list
  # A DEAD READER IS NOT AN EMPTY DOCUMENT. Six commands read this gate's
  # subject with their status discarded across a process substitution
  # (the block above `reader_failed` states the rule and enumerates
  # them), so a fold to "no rows" would report the README as empty, or
  # the tree as clean, instead of the reader as dead. Three cases, one
  # per way a reader can die:
  #
  #   * OUTRIGHT, before anything is read — the kinds reader, first in;
  #   * MID-SCAN, for the SHARED RUST READER only — the deepest reader,
  #     past two that must succeed first, with a breach planted so a
  #     green there could only come from a scan that did not happen;
  #   * AFTER CONSUMING ITS INPUT, for the HIT CLASSIFIER — the stage
  #     that decides what a hit IS, and the one that had no guard. It
  #     consumes and then exits, so nothing upstream sees SIGPIPE and
  #     nothing else fails: the only thing that can red is its own
  #     status being read. With the roster empty and two breaches
  #     planted, an unguarded classifier printed `OK` and exited 0.
  gate_selftest_without_tool awk "the kinds reader over"
  gate_selftest_without_tool find "the source enumerator over"
  gate_selftest_with_broken_tool awk "the const-item reader over" \
    'case "$*" in *SKIPTEST*) exit 9 ;; esac
exec "$GATE_REAL_TOOL" "$@"' plant_named_all
  gate_selftest_with_broken_tool awk "the hit classifier over" \
    'case "$*" in *unnamed*) cat > /dev/null; exit 9 ;; esac
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
  gate_selftest_passes "a ratified list beside a scalar const on one line" \
    pass_two_consts_on_one_line
  printf '%s selftest OK: passes a clean fixture and nine near misses, fires on both arms and both keywords (one-line, multi-line, nested, and under a const generic), on a list in a file whose PATH carries a colon — named whole, at its own line, in the diagnosis — on a ratified name in an unratified module and on a second list under one row, on every way the README half can go wrong, and on a reader that could not run — outright, mid-scan, and after consuming its input\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
