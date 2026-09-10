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
# decide nothing. It also reads the KINDS from the bolded bullets of the
# ONE list the section announces — the paragraph beginning "Three kinds
# of list stay hand-written", `KIND_ANCHOR` below — so the vocabulary of
# reasons is the ratification itself and not a copy of it: deleting a
# bullet while leaving a row that claims it reds. That is
# `viewer-module-kinds.sh`'s contract and the reason its rosters have
# not gone stale.
#
# THE ANCHOR IS THE PARAGRAPH AND NOT THE SECTION, and the difference is
# a defect this gate shipped with. The section is ~150 lines of
# expository prose; a scan for `- **…**` across all of it does not read
# "the ratified kinds", it reads "every bolded bullet in the section",
# which is a rule saying an author may never write a bulleted list in
# any paragraph of it. Nobody ratified that, and the red misdiagnosed
# it: it told an author who bulleted a paragraph about something else
# that they had added a KIND — a claim about a ratification they never
# touched. It cost #2143's author every bulleted list in a README
# rewrite. Anchoring on the announcing sentence is what makes this
# gate's stated subject and its actual subject the same thing.
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
#     anchored list's bullets spell, that list holds `KIND_COUNT` of
#     them, every row has exactly one list behind it, no two rows name
#     one list — never its fidelity to the argument Ev approved. A bullet
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

# THE PARAGRAPH THAT ENUMERATES THE KINDS, and it is an anchor for the
# same reason `TABLE` is one. The kinds are the bolded bullets of ONE
# list, and that list is announced by one sentence; the scan is anchored
# to that sentence rather than to the section, because the section is
# ~150 lines of expository prose whose paragraphs also open in bold. A
# section-wide scan does not read "the ratified kinds", it reads "every
# bolded bullet anywhere in the section" — a constraint nobody wrote
# down, whose red says an author ADDED A RATIFIED KIND when what they
# added was a bulleted list to a paragraph about something else. That
# cost a README rewrite its bullets once, and a misdiagnosis pointing at
# the wrong repair is the class `reader_failed` below exists for.
#
# A PREFIX OF THE LINE, NOT THE WHOLE LINE, unlike the four constants
# above. Those name headings and a table's header row, which are whole
# lines by construction. This names a PARAGRAPH, and where a paragraph's
# prose wraps is an artifact of the fill column: pinning the whole line
# would make an edit to the tail of the sentence — which cannot change
# which paragraph it is — a red. The prefix is the identifying half.
#
# THE COUNT WORD IS INSIDE IT, deliberately, and BOTH copies are kept.
# `KIND_COUNT` below is the gate's number and this sentence carries the
# README's. They are not one copy too many: `KIND_COUNT` is what makes
# an amendment cost an edit to THIS file, so deriving the number from
# this string and deleting that constant would close the divergence by
# giving up the thing it exists for.
#
# WHAT THE ANCHOR ALONE DOES NOT BUY, stated because it was claimed
# here. Carrying the word holds the README against the GATE: a section
# amended to four kinds leaves no line starting "Three kinds of list
# stay hand-written" and the anchor reds. It does not hold the gate
# against ITSELF, and the missing-anchor red used to offer the way
# around as a co-equal repair — take "change `KIND_ANCHOR`" alone, to
# `Four kinds of list stay hand-written` with `KIND_COUNT` left at 3,
# and a section saying Four over three bullets went GREEN, with `3
# kinds read from "Four kinds of list stay hand-written"` on the OK
# line. `anchor_states_count` below is the third edge of that triangle:
# this string's first word and `KIND_COUNT` are ONE assertion in two
# spellings, checked against each other before the README is opened, so
# the pair cannot be walked apart one edit at a time.
KIND_ANCHOR='Three kinds of list stay hand-written'

# HOW MANY KINDS THE SECTION RATIFIES. The README says the kinds a row
# may claim are the bolded bullets of the list `KIND_ANCHOR` announces,
# and until this constant
# nothing held that number: a fourth bullet plus a row claiming it both
# green, and the gate checked the roster's internal CONSISTENCY rather
# than its fidelity to what was ratified. It cannot check the fidelity —
# rewording a bullet and its matching cell together is invisible to it,
# and that is stated in WHAT IT CANNOT CATCH above — but the COUNT it
# can hold, and holding it means a fourth kind requires an edit to this
# file as well as to that section. That is the right price for an
# amendment to a ratification, and the wrong one for a docs-tier edit
# that arrives alone.
#
# ITS WORD IN `KIND_ANCHOR` MOVES WITH IT — `anchor_states_count` below
# refuses a pair that disagrees, so raising this number is two edits in
# one diff and neither of them is optional.
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
  # WHERE THE FILE COLUMN ENDS comes from `gate_record_split`, which
  # `gate_record_awk` prepends to this program (lib.sh, section THE
  # COLUMNS OF A RECORD; no apostrophe may appear here, since the
  # program is single-quoted where it is run, so possessives are written
  # around). An
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
# every stage of every pipeline inside a process substitution. STAGES,
# not pipelines, is the load-bearing half of the rule: the hit
# classifier is the stage AFTER `const_items`, in the same process
# substitution, and it once had no guard at all. It survived an
# immediate death only by SIGPIPE upstream, which reds naming the wrong
# reader; a classifier that consumes its input and THEN fails produces
# no SIGPIPE, and the gate then printed one "delete this row, its list
# is gone" per ratified row — or, with no data rows in the table, `OK`
# and exit 0 over two planted breaches. Both were reproduced before that
# guard existed.
#
# THE RULE YIELDS NINE, and each stage has a guard that names IT. There
# are five process substitutions in `gate()` and this is every stage of
# every one of them:
#
#   `mapfile … < <(viewer_sources)`
#     1. the source enumerator        `find`
#     2. the source sorter            `sort`
#   `mapfile … < <(readme_kinds)`
#     3. the kinds scanner            `awk … "$README"`
#     4. the kinds bullet extractor   `sed -nE …`
#   `mapfile … < <(readme_table)`
#     5. the table reader             `awk … "$README"`
#   `mapfile … < <(table_rows …)`
#     6. the row reader               `sed -nE …`
#   `mapfile … < <(const_hits …)`
#     7. the shared Rust reader       `gate_rust_code` (guarded in `lib.sh`)
#     8. the const-item reader        `gate_record_awk "$ITEM_AWK"`
#     9. the hit classifier           `awk "$HIT_AWK"`
#
# A GUARD IS ON A STAGE AND NEVER ON A PIPELINE, which is the shape
# `const_hits` argued for first and the rest of this file has now been
# brought to. `pipefail` reports the RIGHTMOST non-zero stage, so a
# guard written `a | b || reader_failed "a"` diagnoses a dead `b` as a
# dead `a` — a message naming a repair to something that is fine. That
# was live at three sites and every one was reproduced: a dead `sort`
# was reported as *the source enumerator*, a dead kinds `sed` as *the
# kinds reader* (one name covering two stages, so the log could not say
# which died), and a dead `gate_rust_code` drew a SECOND diagnosis
# naming *the const-item reader* on top of `lib.sh`'s own correct one.
# Each stage is now wrapped in its own brace group, so the status the
# guard reads is that stage's own.
#
# WHAT IS NOT A READER, by the same rule. `module_path`, `const_name`
# and `contains` are pure parameter expansion and `printf`, so they
# invoke nothing that can be missing and read nothing but their own
# arguments; `$(gate_name)` is the same. `table_rows`'s leading `printf`
# is a stage and still not a reader: it is a bash BUILTIN — nothing on
# PATH can shadow it away — and what it reads is the argument list the
# caller already holds, not `$README`. Guarding it would name a stage
# that cannot die of the thing these guards exist for. A command whose
# status the shell KEEPS is not one either — errexit and `pipefail`
# already end the gate on it.
#
# SEVEN OF THE NINE HAVE A CASE, and the two that do not are named
# rather than left to a count: stage 5 (the table reader) and stage 6
# (the row reader). The reason is NOT that they were already
# stage-guarded — stage 6's guard sat on a two-stage `printf | sed`
# pipeline and moved into a brace group here, as the block above
# `table_rows` says. It is that neither guard's NAME changed, and a case
# can only assert that a name is PRESENT: `the table reader over` and
# `the row reader over` are what both revisions print, so a case for
# either is green on both sides — coverage, not a control. The
# population above is what a later lane should read, not the number of
# rows below.
#
# A CASE CAN ASSERT A NAME IS PRESENT AND NEVER THAT ONE IS ABSENT, so
# a repair whose effect is to REMOVE a wrong name has no case that can
# see it (`work/issues/gate-selftest-cannot-observe-the-identity-a-gate-
# names`). Stage 7's is such a repair, so its case is coverage rather
# than a control; stages 2 and 4 gained names that did not exist before,
# so theirs are controls.
reader_failed() {
  # THE TEXT IS `lib.sh`'s, and it is one text for every reader in this
  # directory that could not run: a reader of a CI log met the same
  # event under four descriptions before, and none of them was the
  # canonical one. What stays here is WHICH reader — the caller names it
  # and the self-test aims at that name.
  gate_reader_died_refusal "the $1" "$2"
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

# THE ORDER IS A READ AND NOT A COSMETIC, which is why `sort` carries a
# guard of its own rather than riding the enumerator's: it is what makes
# the scan, the hit list and the count the gate prints the same on every
# box.
viewer_sources() {
  { find "$SRC" -type f -name '*.rs' || reader_failed "source enumerator over $SRC" "$?"; } |
    { sort || reader_failed "source sorter over $SRC" "$?"; }
}

# `gate_rust_code` DIAGNOSES ITSELF, in `lib.sh`, as "the shared Rust
# reader" — so a guard on this pipeline spoke for a stage it does not
# own: a dead code view drew that correct refusal AND this one, and a
# reader of the log met two names for one death. The brace group leaves
# stage 7 to its own guard and makes this one speak only for the item
# reader, which is the stage this file wrote.
const_items() {
  gate_rust_code "$@" |
    { gate_record_awk "$ITEM_AWK" || reader_failed "const-item reader over $SRC" "$?"; }
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

# THE FENCE TRACKER IS SHARED AND LOADED, NOT SPELLED HERE.
# `crates/viewer/README.md`'s sections are scoped by `^#` and its
# rosters are `^|` lines, and inside a fenced code block neither is
# markdown structure: `#[derive(Debug)]` and `#!/bin/sh` are content,
# and a worked example of a table row is content too. Both README
# readers below therefore ask `md_fence` of EVERY rule they have rather
# than only of `^#`, and `readme_kinds`'s `opens` asks for its THIRD
# answer — the tracker and every argument about it live in the sidecar
# named below, which is where a reader of either gate is sent.
#
# ONE FILE, NOT A COPY EACH. `viewer-module-kinds.sh` reads the same
# README through the same tracker, and two readers answering "is this
# line markdown structure" differently is a divergence nothing would
# catch — the reason this file prepends ONE helper to its own two
# programs, one level up.
#
# EVERY LIVE INTERVAL IN THIS FILE IS CLEAR OF THE `mawk` SPELLING the
# sidecar forbids — no `(` immediately after an interval, on pain of
# `REcompile() - panic` and a reader that decided nothing. The sweep
# rule is `grep -nE '[{][0-9]+,[0-9]*[}]' $0` read by hand rather than
# a count carried in prose: it returns THREE LINES carrying THREE
# intervals, all in `readme_kinds` — two bullet patterns and the `sed`
# extractor — each followed by `- ` or `- \*\*`, and `sed` is not `awk`
# besides.
VIEWER_FENCE_AWK=$GATE_REPO_ROOT/scripts/gates/viewer-readme-fence.awk

# THE LOAD IS A READ AND IT IS GUARDED, for `reader_failed`'s reason
# one layer down: a tracker that is not there leaves `FENCE_AWK` empty,
# every `md_fence` call becomes a call to an undefined function, and
# awk's exit-2 syntax error reaches a CI reader with no gate name on
# it and nothing said about what was not decided. The self-test calls
# this DIRECTLY, because no planted tree can express it — the tracker
# is this gate's own code and lives outside every fixture root.
load_fence_awk() {
  if [ ! -f "$1" ]; then
    gate_error "$(gate_name): $1 does not exist — that file IS the markdown fence tracker this gate reads $README through, shared with viewer-module-kinds.sh, and without it a fenced \`#\` ends a section and a fenced table row joins a roster. Restore it, or spell the tracker back into this gate deliberately"
    return 1
  fi
  printf '%s\n' "$(<"$1")"
}
FENCE_AWK=$(load_fence_awk "$VIEWER_FENCE_AWK") || exit 1

# The ratified kinds, as `@` (the anchor paragraph was found) followed
# by one line per bolded bullet of the list it announces: `- **A
# deliberately partial list** claims no completeness…` yields `A
# deliberately partial list`.
#
# EMITTING THE ANCHOR AS A RECORD, like `readme_table` below, is what
# lets the caller tell "the paragraph is gone" from "the reader died"
# from "the list is empty" — three answers a bare count folds into one,
# and the first of them is a NEW way this half of the subject can go
# wrong now that a sentence carries the scan.
#
# SCOPED TO THE SECTION FIRST, then to the paragraph. The section scope
# is `readme_table`'s and is there for the same reason: the README says
# these bullets are the ones "above" its table, so an anchor sentence
# found under someone else's heading would make that word decide
# nothing. The paragraph scope is what makes the gate's subject and its
# stated subject the same thing — see `KIND_ANCHOR`.
#
# WHERE THE LIST ENDS, stated as a rule because a scan whose stopping
# point is implied is the defect this reader is a fix for. Three states,
# all inside the section:
#
#   0  looking for the anchor; the anchor line starts the paragraph.
#   1  inside the anchor paragraph — its own sentence may wrap over as
#      many lines as the fill column gives it, and none of them is a
#      bullet. It ends at the blank line that ends any paragraph, or at
#      a bullet: a list may interrupt a paragraph with no blank line
#      between them, every renderer draws that as a list, and a reader
#      that required the blank would red on a spelling markdown allows.
#      The bullet is not consumed by the transition — the rule sets the
#      state without a `next`, so state 2's own rules see the same line.
#   2  at the list the paragraph announced: a blank line, an indented
#      continuation line and a bullet keep it open; ANY other line
#      closes it, and the bullets of the section's later prose are
#      therefore not read. The list must be what follows the paragraph:
#      a prose paragraph in between closes the scan with nothing
#      collected, which the caller reds on.
#
# A MARKER AT ONE TO THREE SPACES IS A BULLET, and reading one at
# column 0 only is the way this reader could be wrong AND quiet.
# CommonMark — and therefore GitHub — allows a list marker up to three
# spaces in and lets the list interrupt a paragraph, so
#
#       - **A fourth kind** …
#
# two spaces in, on the line after the announcing sentence or above the
# real bullets, renders to every human reader as the first item of the
# announced list. Read at column 0, state 1 swallowed it as the
# announcing sentence's own wrap and state 2 swallowed it as a
# continuation line: a ratified kind every renderer draws and the gate
# does not count, printing OK over four. That is this gate's own thesis
# inverted, and the reason the boundary is encoded exactly rather than
# approximated with `[[:space:]]*`.
#
# FOUR IS THE OTHER SIDE OF IT, and each position is a different
# renderer answer, none of them a fourth top-level kind: four spaces
# after the announcing paragraph is a LAZY CONTINUATION of it and the
# marker is drawn as literal text; four after a bullet is a NESTED item
# of that bullet; four after a blank line is an INDENTED CODE BLOCK;
# and a leading TAB advances to column four, so it is one of those
# three too. All of them reach the continuation rule, which is why the
# bullet patterns spell literal spaces and the continuation rule keeps
# `[[:space:]]`. Every rendering in this paragraph was checked with
# `markdown-it-py` in CommonMark mode, and both sides of the boundary
# are planted below.
#
# THE ANCHOR OPENS A PARAGRAPH, which is what tells a MENTION of the
# sentence from a second announcement of the list. The README quotes
# this sentence in the prose that describes the gate, and where a
# quotation falls on a line is an artifact of the fill column: at
# column 1 of a wrapped line it was a second `@`, and the red — the
# section "announces the ratified kinds more than once" — names a
# repair that is wrong for a quotation. The rule is the renderer's
# rather than a heuristic: a non-blank line following a paragraph line
# is a lazy continuation of that paragraph and never a new one, so a
# line that does not follow a blank line or a heading announces
# nothing. `opens` is that predicate, computed for every line before
# any other rule runs, because a rule that ends in `next` would skip a
# recorder placed after it.
readme_kinds() {
  {
    awk -v sec="$SECTION" -v anchor="$KIND_ANCHOR" "$FENCE_AWK"'
      BEGIN { opens = 1 }
      { fence = md_fence($0); fenced = (fence != "") }
      { prevopens = opens
        opens = (fence == "close" || (fence == "" && ($0 ~ /^[[:space:]]*$/ || $0 ~ /^#/))) }
      !fenced && $0 == sec { insec = 1; st = 0; next }
      insec && !fenced && /^#/ { insec = 0; st = 0; next }
      insec && !fenced && prevopens && index($0, anchor) == 1 { print "@"; st = 1; next }
      st == 1 && !fenced && /^[[:space:]]*$/ { st = 2; next }
      st == 1 && !fenced && /^ {0,3}- / { st = 2 }
      st == 1 { next }
      st == 2 && !fenced && /^ {0,3}- \*\*/ { print; next }
      st == 2 && !fenced && /^[[:space:]]*$/ { next }
      st == 2 && !fenced && /^([[:space:]]|- )/ { next }
      st == 2 { st = 0 }
    ' "$README" || reader_failed "kinds scanner over $README" "$?"
  } | {
    sed -nE -e 's/^ {0,3}- \*\*(.+)\*\*.*/\1/p' -e '/^@$/p' \
      || reader_failed "kinds bullet extractor over $README" "$?"
  }
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
  awk -v sec="$SECTION" -v want="$TABLE" "$FENCE_AWK"'
    { fenced = (md_fence($0) != "") }
    !fenced && $0 == sec { insec = 1; next }
    insec && !fenced && $0 == want { inside = 1; print "@"; next }
    insec && !fenced && /^####/ { inside = 0; next }
    insec && !fenced && /^#/ { insec = 0; inside = 0; next }
    inside && !fenced && /^\|/ { print }
  ' "$README" || reader_failed "table reader over $README" "$?"
}

# One `LIST|MODULE|KIND` per data row. A prose line inside the table is
# not one, which is what requiring two backticked cells buys; the header
# and separator rows are asserted by the caller and dropped by position,
# so this pattern is not what keeps them out.
# THE GUARD IS ON THE `sed`, NOT ON THE PIPELINE, and the leading
# `printf` is deliberately unguarded: it is a bash BUILTIN reading the
# argument list this function was handed, so it is a stage of a
# pipeline in a process substitution and still not a READER by the rule
# above — nothing on PATH can shadow it and it never opens `$README`.
# One reader, one guard, and the name it carries is the stage it is on.
table_rows() {
  printf '%s\n' ${1:+"$@"} |
    { sed -nE 's/^\|[[:space:]]*`([A-Za-z0-9_:]+)`[[:space:]]*\|[[:space:]]*`([A-Za-z0-9_:]+)`[[:space:]]*\|[[:space:]]*(.+[^[:space:]])[[:space:]]*\|[[:space:]]*$/\1|\2|\3/p' \
        || reader_failed "row reader over $README" "$?"; }
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

# THE ENGLISH WORD FOR A COUNT, so `KIND_ANCHOR`'s first word and
# `KIND_COUNT` can be COMPARED rather than trusted to have been moved
# together. Nine is past anything this section will ratify; a count
# with no word is its own red rather than a quiet pass, because a guard
# that cannot answer must not answer green.
count_word() {
  case "$1" in
    1) printf 'One\n' ;;
    2) printf 'Two\n' ;;
    3) printf 'Three\n' ;;
    4) printf 'Four\n' ;;
    5) printf 'Five\n' ;;
    6) printf 'Six\n' ;;
    7) printf 'Seven\n' ;;
    8) printf 'Eight\n' ;;
    9) printf 'Nine\n' ;;
    *) return 1 ;;
  esac
}

# THE TWO CONSTANTS ARE ONE ASSERTION, and this is where that is
# enforced rather than asked for in a diagnosis. Returns 0 when
# ANCHOR's first word is the word for COUNT, 1 when it is not, and 2
# when COUNT has no word at all — three answers, because the caller's
# two messages point at different repairs. It reads nothing outside its
# arguments, which is what lets the self-test exercise both directions:
# a fixture plants a TREE, and this pair lives in this file.
anchor_states_count() {
  local want
  want=$(count_word "$2") || return 2
  [ "${1%% *}" = "$want" ]
}

gate() {
  local rc=0 row hit kind
  # THE GATE'S OWN TWO COPIES OF THE NUMBER, BEFORE THE README'S. Every
  # check below is about "the count this section ratifies", and this
  # file spells that count twice — as `KIND_COUNT` and as the first
  # word of `KIND_ANCHOR`. Split, they make the section's prose number
  # and its bullets free to disagree in silence again, one edit at a
  # time, with each edit taken from a diagnosis that named it. Asked
  # first because it needs no tree and because a pass over a broken
  # pair is a pass about the wrong number.
  # `|| pair=$?`, NOT a bare call: under `set -e` a bare call returning
  # non-zero kills the gate before its own diagnosis, which is the
  # shape `gate_selftest_assert_diagnosed` refuses.
  local pair=0 anchor_word=${KIND_ANCHOR%% *} want_word=
  want_word=$(count_word "$KIND_COUNT") || want_word=
  anchor_states_count "$KIND_ANCHOR" "$KIND_COUNT" || pair=$?
  case $pair in
    0) ;;
    2) gate_error "$(gate_name): \`KIND_COUNT\` in $0 is $KIND_COUNT and \`count_word\` has no English word for it, so the number this gate holds cannot be checked against the number word in \`KIND_ANCHOR\` — extend that table in the same diff that raises the count, because an unchecked pair is how the two spellings drift apart"
       exit 1 ;;
    *) gate_error "$(gate_name): this gate's own two copies of the count disagree — \`KIND_ANCHOR\` in $0 opens \"$anchor_word\" and \`KIND_COUNT\` is $KIND_COUNT (\"$want_word\"). They are ONE assertion in two spellings: the anchor is the only reading anything gives $README's prose number, and \`KIND_COUNT\` is what the bullets are counted against, so a pair like this is exactly how a section goes green announcing \"$anchor_word\" over $KIND_COUNT bullets. Move both, in the diff that amends the ratification"
       exit 1 ;;
  esac
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
  # `@` FIRST, THEN THE KINDS. The marker separates the three answers a
  # bare count folds into: the anchor paragraph is gone, the anchor is
  # there and announces no list, or the reader died. It is dropped by
  # value rather than by position because a SECOND anchor is its own
  # answer — two paragraphs announcing the ratified kinds, of which this
  # reader would read only the list under the first.
  local -a kindblock=()
  mapfile -t kindblock < <(readme_kinds)
  abort_if_reader_failed
  local -a kinds=()
  local anchors=0 entry
  for entry in ${kindblock[@]+"${kindblock[@]}"}; do
    if [ "$entry" = '@' ]; then
      anchors=$((anchors + 1))
    else
      kinds+=("$entry")
    fi
  done
  if [ "$anchors" -eq 0 ]; then
    gate_error "$(gate_name): no line inside $README's \"$SECTION\" section begins \"$KIND_ANCHOR\", so the paragraph that announces the ratified kinds is gone and this gate has no anchor to read them under. It reads that paragraph's bullets and NOT the section's, because the section is prose and a bolded bullet in it is not a ratified kind — restore the sentence, or change \`KIND_ANCHOR\` in $0 in the same diff — and its NUMBER WORD moves with \`KIND_COUNT\` there, which this gate refuses to let disagree: the anchor alone is not the repair, because rewording it to match a section that ratifies a different number of kinds is how the two go green over a count nobody approved"
    exit 1
  fi
  if [ "$anchors" -ne 1 ]; then
    gate_error "$(gate_name): $anchors lines inside $README's \"$SECTION\" section begin \"$KIND_ANCHOR\", so the section announces the ratified kinds more than once and this gate reads the bullets under the first. One paragraph enumerates them. A line that QUOTES the sentence inside a paragraph is not one of these — only a line OPENING a paragraph is read as an announcement — so what this found is a second paragraph starting with it: delete the duplicate, or reword it so it does not open with the announcing sentence. If the ratification itself is being amended, that is \`KIND_ANCHOR\` in $0 and its number word's partner \`KIND_COUNT\`, moved together"
    exit 1
  fi
  if [ "${#kinds[@]}" -eq 0 ]; then
    gate_error "$(gate_name): $README's \"$KIND_ANCHOR\" paragraph is followed by no \`- **kind**\` bullets, so the vocabulary of reasons a hand-written list may be kept came from nowhere. The list this gate reads is the one directly under that paragraph — either the bullets were reshaped or something was written between the two — and a roster that reads nothing is not a pass"
    exit 1
  fi
  if [ "${#kinds[@]}" -ne "$KIND_COUNT" ]; then
    gate_error "$(gate_name): the two sides of this count disagree. \`KIND_COUNT\` in $0 says the section ratifies $KIND_COUNT kinds of hand-written list; the bullets under \"$KIND_ANCHOR\" in $README are ${#kinds[@]}: $(printf '"%s" ' "${kinds[@]}"). Which side is wrong decides the repair and this gate cannot know which: a kind added to or removed from that list is an AMENDMENT to the ratification — argue it there, and move \`KIND_COUNT\` and the anchor's number word in $0 in the same diff, so the amendment cannot arrive as a table cell nobody had to approve. Adding a bullet to make $README match this file is the repair only when this file was the side that was right"
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
  # THE COUNT AND THE WORD ON THIS LINE CANNOT DISAGREE, and that is a
  # property of two guards rather than of this `printf`: `${#kinds[@]}`
  # has been checked equal to `KIND_COUNT`, and `KIND_ANCHOR`'s first
  # word was checked to be the word for `KIND_COUNT` before the README
  # was opened. `3 kinds read from "Four kinds of list stay
  # hand-written"` was reachable, and a green line contradicting itself
  # is read by nobody — so it is made unreachable, not diagnosed.
  gate_ok "no hand-written membership list under $SRC that \"$TABLE\" does not ratify (${#rows[@]} ratified, one list each; ${#kinds[@]} kinds read from \"$KIND_ANCHOR\")"
}

# --- THE FIXTURES ------------------------------------------------------
#
# The subject is a crate directory AND a README, so the clean tree
# carries both. `gate_plant_clean_sources` keeps `lib.sh`'s own
# `crates/*/src` line in one place rather than writing it a third time.
#
# THE FIXTURE HAS THE PAGE'S SHAPE, not only its readable parts. The
# anchor paragraph WRAPS, as it does on the real page, so the reader's
# state-1 rule — the tail of the announcing sentence is not a line that
# closes the list — is exercised by every case rather than asserted in a
# comment. Prose sits on both sides of the list, so a planter can put a
# bolded bullet before the anchor and after the list, which is where the
# section-wide scan this reader replaced counted one as a ratified kind.
readme_fixture() {
  mkdir -p "$1/crates/viewer/src"
  printf 'pub fn identity(x: f64) -> f64 { x }\n' > "$1/crates/viewer/src/lib.rs"
  cat > "$1/crates/viewer/README.md" <<'MD'
### Closed vocabularies are declared once

Prose.

Three kinds of list stay hand-written, and each is a different answer
rather than an exception:

- **A registry of struct constants** is not an enumeration of variants.
- **A deliberately partial list** claims no completeness.
- **A mirror of an enum declared in another crate** cannot be projected.

More prose.

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

# THE ANCHOR IS HALF THIS HALF OF THE SUBJECT, so it can go wrong the
# same three ways the table heading can: reworded away, written twice,
# or separated from the list it announces. Reworded is the live one — an
# amendment to a fourth kind rewrites that sentence's first word — and
# it must red rather than fall back to reading the whole section, which
# is the behaviour this reader exists to end.
plant_anchor_reworded() {
  sed -i 's/^Three kinds of list stay hand-written,/Some kinds of list stay hand-written,/' \
    "$1/crates/viewer/README.md"
}

# TWICE: two paragraphs claiming to enumerate the kinds, of which the
# reader would read the first list only. The second here carries a
# fourth kind, so a fold to "read the first" would be green over an
# unratified bullet.
plant_the_anchor_twice() {
  sed -i 's#^More prose\.$#Three kinds of list stay hand-written, and each is a different answer\nrather than an exception:\n\n- **A fourth kind** announced a second time.#' \
    "$1/crates/viewer/README.md"
}

# SEPARATED: the list is no longer what follows the paragraph. The scan
# closes at the interposed prose with nothing collected, which is a red
# naming the paragraph — not a silent hunt down the section for the next
# bulleted list it can find.
plant_a_paragraph_between_the_anchor_and_the_list() {
  sed -i 's/^rather than an exception:$/rather than an exception:\n\nAn interposed paragraph./' \
    "$1/crates/viewer/README.md"
}

# A LIST MARKER ONE TO THREE SPACES IN IS A BULLET, and this is the
# hole a column-0 reader was WRONG AND QUIET about: CommonMark indents
# a marker up to three spaces and lets a list interrupt a paragraph, so
# both of these render as the FIRST ITEM of the announced list and both
# went green — the gate read three kinds and printed OK over four. Two
# planters because they are two rules: the first meets state 1's escape
# from the announcing paragraph (no blank line above it), the second
# meets state 2's own bullet and the `sed` that extracts the name.
plant_an_indented_fourth_kind_interrupting_the_paragraph() {
  sed -i 's/^rather than an exception:$/rather than an exception:\n  - **A fourth kind** two spaces in./' \
    "$1/crates/viewer/README.md"
}

plant_an_indented_fourth_kind_in_the_list() {
  sed -i 's/^- \*\*A registry of struct constants\*\*.*$/   - **A fourth kind** three spaces in.\n&/' \
    "$1/crates/viewer/README.md"
}

# A FENCED CODE BLOCK IS NOT MARKDOWN STRUCTURE, and these are the
# spellings a column-zero-`#` reader was WRONG AND LOUD about. Each
# planted line renders to every human as code and was read as a heading
# that ENDS the section, so the half of the page below it went
# invisible: the first of these red claiming the announcing paragraph
# was gone, with that paragraph two lines below the fence, and the
# second claiming the roster table has no heading. Both were reproduced
# against the unfixed reader before the helper existed.
pass_a_fence_carrying_a_rust_attribute_above_the_anchor() {
  sed -i 's%^Prose\.$%Prose, with an example:\n\n```rust\n#[derive(Debug)]\npub enum Kind { A }\n```%' \
    "$1/crates/viewer/README.md"
}

pass_a_fence_carrying_a_shebang_below_the_list() {
  sed -i 's%^More prose\.$%More prose, with a script:\n\n```sh\n#!/bin/sh\n# a comment\necho hi\n```%' \
    "$1/crates/viewer/README.md"
}

# THE CLOSE IS CHARACTER-AWARE, and a bare toggle is what this refuses.
# A ``` line inside a ~~~ block does NOT close it in CommonMark, so a
# toggle would hand the `#[derive]` two lines down back to the heading
# rule and end the section there — the repaired defect, re-minted by the
# cheaper spelling of the repair.
pass_a_tilde_fence_holding_a_backtick_line() {
  sed -i 's%^Prose\.$%Prose, with a tilde fence:\n\n~~~\n```\n#[derive(Debug)]\n~~~%' \
    "$1/crates/viewer/README.md"
}

# THE ROSTER IS STRUCTURE TOO, so the fence has to hold for `^|` and not
# only for `^#`. A worked example of a row, written in a fence below the
# real table, is inside the section and inside `inside` — read as a
# roster row it reds with "row `GHOSTS` says `ghosts` declares", which
# is a diagnosis about a list nobody claimed exists.
pass_a_fenced_table_row_below_the_roster() {
  sed -i 's%^### Something else$%An example of a row, which is not a row:\n\n```\n| `GHOSTS` | `ghosts` | A deliberately partial list |\n```\n\n### Something else%' \
    "$1/crates/viewer/README.md"
}

# A FENCE THAT OPENS ALSO CLOSES, which is the other side of the repair
# and the direction a fix could be wrong in silently. The decoy roster
# below `### Something else` is OUTSIDE the section, so it is read only
# if the fence above swallowed the heading that ends it — and a fence
# that never closes hides the REAL table too. Either way this reds; it
# is green only when the fence opens and closes exactly where markdown
# says it does.
pass_a_fence_closes_so_the_section_still_ends() {
  local md=$1/crates/viewer/README.md
  sed -i 's%^Prose\.$%Prose, with a fenced example:\n\n```sh\n# a comment\n```%' "$md"
  printf '\n%s\n\n%s\n%s\n| `GHOSTS` | `ghosts` | A deliberately partial list |\n' \
    "$TABLE" "$TABLE_HEADER" "$TABLE_SEPARATOR" >> "$md"
}

# A CLOSING FENCE ENDS A BLOCK, so the line under it OPENS one, and this
# is the pair a BOOLEAN fence answer got wrong in both directions at
# once. `md_fence` reports a delimiter as "not markdown structure",
# which is right for every rule that asks *is this a heading, a bullet,
# a row* and WRONG for `opens`, whose question is *did the previous line
# end a block*. An opening delimiter starts one, so the next line is
# content; a closing delimiter ENDS one, so the next line begins a
# paragraph — and CommonMark agrees, emitting `fence` then
# `paragraph_open` with no blank line between them.
#
# THE FALSE GREEN IS THE WORSE HALF and it is planted first. A second
# announcement sitting DIRECTLY under a closing fence was not read as
# opening a paragraph, so it was not counted as an announcement: the
# gate found one anchor, read three kinds under it and printed OK over a
# duplicate announcement AND the unratified fourth kind bulleted beneath
# it. Exit 0 over exactly what this gate exists to refuse.
plant_a_second_anchor_under_a_closing_fence() {
  local md=$1/crates/viewer/README.md
  sed -i 's%^Prose\.$%```sh\necho hi\n```\nThree kinds of list stay hand-written, and here is a fourth.\n\n- **A fourth kind** that nobody ratified.\n\nProse.%' "$md"
}

# THE FALSE RED IS THE SAME DEFECT, and it is the misdiagnosis this
# gate's fence work was filed to remove: the announcing paragraph
# DIRECTLY under a closing fence read as a lazy continuation of nothing,
# so the gate said the paragraph was gone about a sentence one line
# below the fence.
pass_the_anchor_directly_under_a_closing_fence() {
  local md=$1/crates/viewer/README.md
  # The blank line under `Prose.` goes first, so the anchor ends up
  # DIRECTLY under the closing fence that replaces it —
  # `pass_no_blank_line_before_the_list`'s idiom, for the same reason.
  sed -i '/^Prose\.$/{n;/^$/d}' "$md"
  sed -i 's%^Prose\.$%```sh\necho hi\n```%' "$md"
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

# A BOLDED BULLET IN THE SECTION'S PROSE IS NOT A RATIFIED KIND, in both
# directions, and this is the pair the section-wide scan got wrong. That
# scan collected every `- **…**` line between the heading and the next
# heading of any level — a hundred and fifty lines of ordinary prose —
# so a bulleted list written about anything else counted as a fourth
# kind, and the red told its author they had amended a ratification they
# had not touched. One README rewrite was written with no bulleted list
# anywhere to keep this gate green.
pass_a_bolded_bullet_before_the_anchor() {
  sed -i 's/^Prose\.$/Prose, with a list of its own:\n\n- **A bolded bullet** that ratifies nothing.\n\nAnd more prose./' \
    "$1/crates/viewer/README.md"
}

pass_a_bolded_bullet_after_the_list() {
  sed -i 's/^More prose\.$/More prose, with a list of its own:\n\n- **A bolded bullet** that ratifies nothing./' \
    "$1/crates/viewer/README.md"
}

# A LIST MAY INTERRUPT A PARAGRAPH. Markdown draws this as a list and so
# must the reader, or a legal spelling of the ratification is a red.
pass_no_blank_line_before_the_list() {
  sed -i '/^rather than an exception:$/{n;/^$/d}' "$1/crates/viewer/README.md"
}

# FOUR SPACES IS THE OTHER SIDE OF THAT BOUNDARY, and these are the
# cases that keep it from being widened to `[[:space:]]*`. Each is a
# different renderer answer and none of them is a fourth ratified kind:
# four spaces after the announcing paragraph is a LAZY CONTINUATION of
# it, drawn as literal text inside the sentence; four under a bullet is
# a NESTED item of that bullet; a leading TAB advances to column four
# and, after a blank line, is an INDENTED CODE BLOCK. Checked against
# `markdown-it-py` in CommonMark mode, one case per answer.
pass_a_four_space_marker_continuing_the_paragraph() {
  sed -i 's/^rather than an exception:$/rather than an exception:\n    - **not a bullet** four spaces in./' \
    "$1/crates/viewer/README.md"
}

pass_a_four_space_marker_nested_under_a_bullet() {
  sed -i 's/^- \*\*A deliberately partial list\*\*.*$/&\n    - **a nested item** four spaces in./' \
    "$1/crates/viewer/README.md"
}

pass_a_tab_indented_marker() {
  sed -i 's/^- \*\*A registry of struct constants\*\*.*$/\t- **a code block** after a tab.\n&/' \
    "$1/crates/viewer/README.md"
}

# A QUOTATION IS NOT AN ANNOUNCEMENT. This page quotes the announcing
# sentence in the prose describing the gate, and where a quotation
# falls on a line is an artifact of the fill column — at column 1 of a
# wrapped line the reader counted a second `@` and reported that the
# section "announces the ratified kinds more than once", whose repair
# (delete the duplicate) is the wrong one for a quotation. The line
# here continues a paragraph, and a lazy continuation is never a new
# one, so nothing is announced.
pass_the_anchor_quoted_inside_a_paragraph() {
  sed -i 's/^More prose\.$/More prose, which mentions the sentence\nThree kinds of list stay hand-written and does not announce a list./' \
    "$1/crates/viewer/README.md"
}

# THE GATE'S OWN CONSTANT PAIR, AS A DIRECT CALL. Every case above
# plants a TREE and runs this file over it, while `KIND_ANCHOR` and
# `KIND_COUNT` live IN this file: no fixture can express a pair that
# disagrees, and a test hook that let one would be a way to set the
# count from outside, which is what `KIND_COUNT` exists to prevent.
# What can be shown is the predicate the guard is made of — the guard
# is one `case` over exactly this call — in all three of its answers.
# THE TRACKER'S OWN LOAD, AS A DIRECT CALL, and for `selftest_anchor_
# pair`'s reason: every case above plants a TREE and runs this file over
# it, while `viewer-readme-fence.awk` sits beside this file and outside
# every fixture root, so no `--root` can express a tracker that is gone.
# What can be shown is the predicate the load is made of, in both of its
# answers — the real path yields the function, a path that is not there
# yields a gate_error and a non-zero status rather than an empty
# `FENCE_AWK` and awk's own syntax error.
selftest_fence_load() {
  local want=$1 path=$2 got=0 out=
  out=$(load_fence_awk "$path" 2>&1) || got=$?
  if [ "$got" != "$want" ]; then
    printf 'SELFTEST FAILED: load_fence_awk "%s" returned %s, wanted %s\n%s\n' \
      "$path" "$got" "$want" "$out" >&2
    exit 1
  fi
  # `gate_selftest_assert_diagnosed`, NOT a case of this function's own:
  # `gate_error` writes `ERROR: ` locally and `::error::` under Actions,
  # so a hand-written test for one spelling passes on a developer's box
  # and fails on the runner. `lib.sh` knows both and is the one place
  # that should.
  if [ "$want" = 0 ]; then
    case "$out" in
      *"function md_fence"*) ;;
      *) printf 'SELFTEST FAILED: load_fence_awk "%s" succeeded without yielding the tracker:\n%s\n' "$path" "$out" >&2
         exit 1 ;;
    esac
  else
    gate_selftest_assert_diagnosed "load_fence_awk over $path" "$out"
  fi
}

selftest_anchor_pair() {
  local want=$1 anchor=$2 count=$3 got=0
  anchor_states_count "$anchor" "$count" || got=$?
  if [ "$got" != "$want" ]; then
    printf 'SELFTEST FAILED: anchor_states_count "%s" %s returned %s, wanted %s\n' \
      "$anchor" "$count" "$got" "$want" >&2
    exit 1
  fi
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
  gate_selftest_case 'is followed by no `- **kind**` bullets' \
    plant_kind_bullets_gone
  # THE ANCHOR, every way it can go wrong. It carries the scan now, so
  # each of these is a way the README half goes wrong that did not exist
  # while the scan was the whole section.
  gate_selftest_case 'begins "Three kinds of list stay hand-written"' \
    plant_anchor_reworded
  gate_selftest_case 'announces the ratified kinds more than once' \
    plant_the_anchor_twice
  gate_selftest_case 'is followed by no `- **kind**` bullets' \
    plant_a_paragraph_between_the_anchor_and_the_list
  gate_selftest_case "says the section ratifies $KIND_COUNT kinds of hand-written list" \
    plant_a_fourth_kind
  # THE INDENT BOUNDARY, in the direction that was silent. The want
  # string asserts the README SIDE of the count — "the bullets … are 4"
  # — because a message that named only this file's number is what sent
  # an author to add a fourth bullet.
  gate_selftest_case "under \"$KIND_ANCHOR\" in $README are 4" \
    plant_an_indented_fourth_kind_interrupting_the_paragraph
  gate_selftest_case "under \"$KIND_ANCHOR\" in $README are 4" \
    plant_an_indented_fourth_kind_in_the_list
  # A CLOSING FENCE ENDS A BLOCK. Both directions, because one boolean
  # answer was wrong in both: the false GREEN over a second announcement
  # and an unratified fourth kind, and the false RED at the anchor
  # itself. The near-miss half is with the other passing rows below.
  gate_selftest_case 'announces the ratified kinds more than once' \
    plant_a_second_anchor_under_a_closing_fence
  gate_selftest_case "which is not one the" plant_row_of_an_unratified_kind
  gate_selftest_case 'row `GHOSTS` says `ghosts` declares' plant_row_with_no_list
  gate_selftest_case 'more than one row for `kinds`' plant_two_rows_for_one_list
  # A DEAD READER IS NOT AN EMPTY DOCUMENT — the population and its rule
  # are stated once, above `reader_failed`, and not restated here. Each
  # row below kills ONE stage and wants that stage by name. Three ways
  # a reader can die, and the third is the one with a technique:
  # killing the RIGHT-HAND stage of a pipeline needs a shim that
  # CONSUMES its input and then exits, because a stub that dies at once
  # takes the upstream stage down with SIGPIPE and the diagnosis names
  # the wrong reader. Each such shim keys on a fragment of its own
  # stage's program text so it kills that stage and no other.
  gate_selftest_without_tool awk "the kinds scanner over"
  gate_selftest_without_tool find "the source enumerator over"
  gate_selftest_without_tool sort "the source sorter over"
  gate_selftest_with_broken_tool awk "the shared Rust reader" \
    'case "$*" in *SKIPTEST*) exit 9 ;; esac
exec "$GATE_REAL_TOOL" "$@"' plant_named_all
  gate_selftest_with_broken_tool awk "the const-item reader over" \
    'case "$*" in *gate_record_split*) cat > /dev/null; exit 9 ;; esac
exec "$GATE_REAL_TOOL" "$@"' plant_named_all
  gate_selftest_with_broken_tool sed "the kinds bullet extractor over" \
    'case "$*" in *"@"*) cat > /dev/null; exit 9 ;; esac
exec "$GATE_REAL_TOOL" "$@"'
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
  gate_selftest_passes "a bolded bullet in the section's prose above the anchor" \
    pass_a_bolded_bullet_before_the_anchor
  gate_selftest_passes "a bolded bullet in the section's prose below the list" \
    pass_a_bolded_bullet_after_the_list
  gate_selftest_passes "the kind list written with no blank line above it" \
    pass_no_blank_line_before_the_list
  gate_selftest_passes "a four-space marker continuing the announcing paragraph" \
    pass_a_four_space_marker_continuing_the_paragraph
  gate_selftest_passes "a four-space marker nested under a ratified bullet" \
    pass_a_four_space_marker_nested_under_a_bullet
  gate_selftest_passes "a tab-indented marker, which renders as a code block" \
    pass_a_tab_indented_marker
  gate_selftest_passes "the announcing sentence QUOTED inside a paragraph" \
    pass_the_anchor_quoted_inside_a_paragraph
  # A FENCED `#` IS NOT A HEADING, in every direction the section can be
  # cut by one. The first two are the reported defect; the last three
  # are the ways a repair could be wrong and quiet.
  gate_selftest_passes "a fenced Rust attribute at column zero above the anchor" \
    pass_a_fence_carrying_a_rust_attribute_above_the_anchor
  gate_selftest_passes "a fenced shebang and comment below the kind list" \
    pass_a_fence_carrying_a_shebang_below_the_list
  gate_selftest_passes "a backtick line inside a tilde fence, which does not close it" \
    pass_a_tilde_fence_holding_a_backtick_line
  gate_selftest_passes "a worked table row written inside a fence" \
    pass_a_fenced_table_row_below_the_roster
  gate_selftest_passes "a closed fence, after which the section still ends" \
    pass_a_fence_closes_so_the_section_still_ends
  gate_selftest_passes "the announcing paragraph DIRECTLY under a closing fence" \
    pass_the_anchor_directly_under_a_closing_fence
  # THE TWO CONSTANTS, which no fixture can vary. The middle row is the
  # green this gate used to have: `KIND_ANCHOR` moved to "Four" on the
  # missing-anchor red's own advice, `KIND_COUNT` left at 3, and a
  # section announcing Four over three bullets passing.
  selftest_anchor_pair 0 "$KIND_ANCHOR" "$KIND_COUNT"
  selftest_anchor_pair 1 'Four kinds of list stay hand-written' 3
  selftest_anchor_pair 1 'Three kinds of list stay hand-written' 4
  selftest_anchor_pair 2 'Ten kinds of list stay hand-written' 10
  # THE SHARED TRACKER'S LOAD, both answers.
  selftest_fence_load 0 "$VIEWER_FENCE_AWK"
  selftest_fence_load 1 "$VIEWER_FENCE_AWK.no-such-file"
  # THE COUNT IS THE `gate_selftest_passes` ROWS ABOVE, one per near
  # miss, so a reader can produce the population rather than trust the
  # number: `grep -c '^  gate_selftest_passes ' $0`. The constant-pair
  # rows are not among them and are counted separately, because they
  # call a predicate rather than run the gate over a tree. The SEVEN
  # dead-reader rows are its counterpart for the other population, and
  # they are produced the same way: `grep -cE '^  gate_selftest_(without|with_broken)_tool ' $0`.
  printf '%s selftest OK: passes a clean fixture and twenty-two near misses, fires on both arms and both keywords (one-line, multi-line, nested, and under a const generic), on a list in a file whose PATH carries a colon — named whole, at its own line, in the diagnosis — on a ratified name in an unratified module and on a second list under one row, on every way the README half can go wrong — its heading, its table, its kind bullets and the paragraph that announces them, including a bullet indented one to three spaces, which every renderer draws as a ratified kind — and stays quiet where markdown draws CODE rather than structure: a column-zero `#` and a table row inside a fence, a tilde fence a backtick line does not close, and a fence whose close still lets the section end. Seven rows kill ONE reader stage each and want that stage by name: outright, mid-scan, and after consuming its input. Four direct rows hold the two copies of the count in this file against each other\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
