#!/usr/bin/env bash
# viewer-module-kinds.sh — the viewer crate's vocabulary/driver rule,
# checked. ONE home; ci.yml's "viewer module kinds" step in the `mirror`
# job runs it and local-scripts/ci-local.sh runs it in `tier_blind_rows`
# (and again in the directory loop on a building change set).
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
# silently makes the README's vocabulary tables false.
#
# WHY THIS GATE IS SITED IN THE `mirror` JOB. *A gate must be sited
# where it can fire on its own inputs* (Ev, 2026-08-20, on S61;
# `.github/workflows/ci.yml` states it above that job).
# `crates/viewer/README.md` is the subject of the table parse above
# check 1 and of checks 3, 4, 6 and 6b; `crates/viewer/Cargo.toml` is
# check 5's. A change set of only the README classifies TIER=docs,
# `RUN_BUILD=false`, and every `if: run_build` job — `discipline`
# included — is skipped. Sited there, a docs-only PR renaming a table
# heading, adding a ghost row or deleting a table would merge with the
# table parse and checks 3-4 unrun. `scripts/check-ci-mirror
# -parity.py`'s TIER_BLIND names this gate, so the siting is enforced
# rather than remembered.
#
# WHERE A MODULE'S KIND IS DECLARED, and why it is not the README.
# Each module says its own kind, once, in its own doc header:
#
#     //! Module kind: **vocabulary** — …
#     //! Module kind: **driver** …
#
# The subject of the rule is a module's `use` block, so the
# declaration sits in the same file as the thing it classifies: an
# author changing a module's role meets the contradiction on screen
# rather than in a document they may not open, and a new module cannot
# be added without answering the question.
#
# WHAT IS DERIVED AND WHAT IS WRITTEN DOWN. Four things in this file
# are hand-kept and three of them are checked against a document on
# every pass; the fourth is named below with what bounds it instead.
# Everything else is READ from the documents it enforces:
#
#   * the DRIVER ROSTER is the README's `### The drivers` table;
#   * the VOCABULARY roster is the README's two vocabulary tables;
#   * the FORBIDDEN CRATES are `crates/viewer/Cargo.toml`'s `app`
#     feature — every `dep:` in it, and nothing else. That is the right
#     population rather than a curated "toolkit" list, and the manifest
#     already says why (`Cargo.toml`, the app-graph comment): every
#     entry there is optional and reached only through `app`, so a
#     vocabulary — which is compiled in a DEFAULT-feature build — naming
#     one is naming something that is not there. A hand-kept list of
#     this got it wrong in both directions on its first day: it carried
#     `egui_dock`, which this crate does not depend on, and omitted
#     `rfd`, which opens a native file dialog — the literal thing "can
#     be read and tested without a window existing" forbids;
#   * the DRIVER MODULE PATHS are the driver table's top-level module
#     names MINUS any that host a vocabulary in the vocabulary tables.
#     `session` is a driver AND the parent of six vocabularies, so
#     `crate::session::SessionOp` must stay green while
#     `crate::app::…` reds — and which modules those are is read off
#     the same tables rather than carved out by hand.
#
# THE HAND-KEPT FOUR, and the rule that produces this list is
# `grep -nE '^[A-Z_]+=' $0` read by hand against what each name is held
# to:
#
#   * FORBIDDEN_TYPE_NAMES — two names, cross-checked against the
#     README's own rule text by check 6;
#   * VOCAB_EXCEPTIONS (below) — held to the tree site by site by
#     check 8, which is also what retires an entry;
#   * TABLE_FIRST_COLUMN and CITED_SECTION — held to the README by the
#     table reader and by check 6b respectively;
#   * VIEWER_FENCE_AWK — a PATH, held only by its own existence test.
#     A path is not a cross-reference into a document's prose, which is
#     the class check 6b exists to refuse: it cannot go stale silently,
#     because the load fails loudly the moment it is wrong. No gate
#     names the other gate by filename any more — the callers of the
#     tracker are `grep -l viewer-readme-fence.awk scripts/gates/*.sh`,
#     which is the sweep rule rather than a list to keep.
#
# THE EXCEPTION LIST IS EMPTY, AND IT RETIRED THE WAY IT WAS BUILT TO.
# It held two entries — `pick.rs|DocSession|2` and
# `parts.rs|DocSession|3` — for the five read-only `&DocSession`
# arguments that made the README's rule false of the tree before this
# gate existed. Ev ruled on `#1883` to HOIST the read rather than widen
# the rule: the session hands out `pick::IndexInputs` and
# `parts::PartCensus`, the two vocabularies take those, and the rule
# stays unqualified. The count is what made the retirement mechanical —
# fixing a site without lowering it REDS, so the entries could not be
# left behind as a ratification for whatever the files gained next.
#
# THE EVIDENCE THIS OFFERED TO D103 STANDS, AND IS NOT WITHDRAWN BY THE
# ENTRIES GOING. `work/gates/D103.md` (unruled, track K, fenced
# to this directory) asks whether an allowlist should be file-granular
# or per-seam: *"the allowlist is file-granular while its
# justifications are per-seam, so later bounds inherit ratification"*.
# D103 lists "a count pinned per file" as one of three shapes a taker
# should weigh, and this was that shape, built inside D103's fence. Its
# RETIREMENT is evidence about the shape rather than a reason to stop
# offering it: the per-seam entry ended when its seam did, in the same
# PR, without anyone having to notice — which is the property a
# file-granular entry does not have.
# `interval-square-allowlist.sh:125-133` argues the same about its own
# retired entries (*"an allowlist entry with nothing behind it is a
# ratification waiting to be inherited by the next line added to the
# file"*).
#
# The machinery stays, empty, for the next seam that needs it. An entry
# is `FILE|NEEDLE|COUNT` and all three parts are load-bearing: the
# NEEDLE covers the reason the exemption was granted and nothing else
# (an exempted file that gains `use eframe::egui;` still reds); the
# COUNT covers the sites argued for and no more (a further one reds);
# and the count is also what retires the entry. An exempted file must
# say so in its own header, and check 2 requires it: a module whose
# header denies naming a driver type nine lines above naming one is
# false, and rustdoc publishes it.
#
# WHAT IT CANNOT CATCH (stated because a sweep whose blind spot is
# unstated is an unverified claim):
#
#   * ROLE. This decides what a module NAMES, never what it IS. A
#     module that owns mutable state and dispatches but imports nothing
#     forbidden passes as a vocabulary; the semantic half of the
#     README's definition is not mechanised and cannot be by a grep.
#   * A DRIVER TYPE REACHED WITHOUT NAMING IT: through a re-export
#     under another name, a generic parameter, a trait object, or a
#     macro that expands to the import. The scan reads source text,
#     never expansions.
#   * A USE TREE SPREAD OVER MORE THAN 12 LINES. The nested form
#     (`use crate::{app::x, camera::Camera};`) is read from a 12-line
#     window; rustfmt does not produce wider ones in this crate, but a
#     hand-written one would evade it.
#   * MODULES OUTSIDE `crates/viewer/src`. `tests/` is not scanned: a
#     suite is allowed to name the driver it drives.
#   * A CRATE REACHED THROUGH A RE-EXPORT OF A NON-`app` DEPENDENCY.
#     `pollster` is the live near-miss and it is correctly ABSENT from
#     the derived set: `Cargo.toml:244` makes it an unconditional
#     dependency, present in the default build, so a vocabulary naming
#     it compiles and breaks no rule this file enforces.
#   * A MARKDOWN BLOCK NEITHER THE FENCE TRACKER NOR THE INDENT STRIP
#     MODELS. The README reader below knows fenced code blocks and the
#     one-to-three-space indent, and nothing else, so a `#` inside an
#     HTML block or a block quote still ends a section, and a SETEXT
#     heading — a line of `===` or `---` under a paragraph — is not read
#     as a heading at all, so a section underlined that way does not end
#     where a renderer ends it. None of the three has ever appeared in a
#     scanned region; the sweeps are `grep -n '^<' `, `grep -n '^>' ` and
#     `grep -nE '^(=+|-+)$' ` over `crates/viewer/README.md`, and the
#     third is why the `|---|---|` delimiter test requires a `|`.
#     `viewer-readme-fence.awk` states the tracker's half of this.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

SRC=crates/viewer/src
README=crates/viewer/README.md
MANIFEST=crates/viewer/Cargo.toml
GATE_SCAN_NOUN='viewer module'

# THE FENCE TRACKER IS SHARED AND LOADED, NOT SPELLED HERE. This gate
# scopes a README region by `^#` and reads its roster off `^|` lines,
# and inside a fenced code block neither is markdown structure:
# `#[derive(Debug)]` and `#!/bin/sh` are content, and a worked example
# of a table row is content too. It is not the only gate reading this
# README that way, and two readers answering "is this line markdown
# structure" differently is a divergence nothing would catch — so the
# tracker is ONE file its callers load rather than a copy each, and who
# those callers are is `grep -l viewer-readme-fence.awk
# scripts/gates/*.sh` rather than a name kept here. Its header carries
# the CommonMark rule, the indent boundary, the three answers and the
# `mawk` constraint.
#
# EVERY LIVE INTERVAL IN THIS FILE IS CLEAR OF THE `mawk` SPELLING the
# tracker's header forbids — no `(` immediately after an interval, on
# pain of `REcompile() - panic` and a reader that decided nothing. The
# sweep rule is `grep -nE '[{][0-9]+,[0-9]*[}]' $0` read by hand rather
# than a count carried in prose. It returns FOUR lines and exactly ONE
# of them is live: the table reader's `sub(/^ {0,3}/, "", line)`, whose
# interval is followed by `/`. The other three are comments about that
# strip — this sentence, the reader's own paragraph and one planter's.
# The fatal adjacency — a closing brace of an interval with an opening
# parenthesis immediately after it — is written down once, in the
# sidecar's header, which is where its sweep lives too; spelling it a
# second time here would put it in the file that claims not to hold it.
VIEWER_FENCE_AWK=$GATE_REPO_ROOT/scripts/gates/viewer-readme-fence.awk

# THIS LOADER IS THE SECOND COPY OF ITSELF, disclosed rather than
# shared, and the reason it cannot be shared is one level down from the
# reason the tracker can. The tracker has ONE home because it is awk
# source; a loader is SHELL, and the only shared shell homes are
# `lib.sh` — code-quality's, read and called here and never edited —
# and a second `.sh` in this directory, which `gate-roster.sh` reads as
# a gate that runs nowhere (`viewer-readme-fence.awk`'s header argues
# that at length). So this function, `VIEWER_FENCE_AWK` and
# `selftest_fence_load` are spelled twice, here and in the directory's
# other viewer gate. What they can diverge about is bounded by what
# they are — a path, an existence test and a message — and the part
# where two readers disagreeing would be a defect nothing catches, the
# tracker, is not duplicated at all.
#
# THE LOAD IS A READ AND IT IS GUARDED, for `reader_failed`'s reason
# one layer down: a tracker that is not there leaves `FENCE_AWK` empty,
# every `md_fence` call becomes a call to an undefined function, and
# awk's exit-2 syntax error reaches a CI reader with no gate name on it
# and nothing said about what was not decided. The self-test calls this
# DIRECTLY, because no planted tree can express it — the tracker is
# this gate's own code and lives outside every fixture root.
load_fence_awk() {
  if [ ! -f "$1" ]; then
    gate_error "$(gate_name): $1 does not exist — that file IS the markdown fence tracker this gate reads $README through, shared with this directory's other viewer gate, and without it a fenced \`#\` ends a section and a fenced table row joins a roster. Restore it, or spell the tracker back into this gate deliberately"
    return 1
  fi
  printf '%s\n' "$(<"$1")"
}
FENCE_AWK=$(load_fence_awk "$VIEWER_FENCE_AWK") || exit 1

# A DEAD READER IS NOT AN EMPTY DOCUMENT, and this gate had no way to
# say so. Every stage below reads something off disk or off a pipe and
# can fail to run at all — a missing tool, an unreadable file, a
# malformed program — and until this apparatus landed each of them was
# a stage of a pipeline inside `mapfile … < <(…)` or a `$(…)`, so what
# a CI reader met was the NEXT check's conclusion about a scan that
# never happened. Both live misdiagnoses were reproduced before the
# repair: a dead `awk` reported *"the heading was renamed or the table
# was reshaped"* about a README that is fine, and a dead `sed` reported
# *"no modules under crates/viewer/src besides lib.rs and bin/"* about a
# tree holding forty-five.
#
# TWELVE STAGES THIS FILE GUARDS, which is NOT every reader stage in it.
# The rule that produces the list is *every stage on PATH whose status
# this file must read for itself* — so `gate_grep`, `gate_rust_code` and
# `gate_record_awk` are here only where a guard of this file's own sits
# beside them, and their other call sites — the rule is
# `grep -nE 'gate_grep|gate_rust_code|gate_record_awk' $0` with the
# comment lines dropped, read by hand, and no count is carried here
# because it moves whenever a filter is added — are absent because each
# diagnoses ITSELF in `lib.sh` and no case here is owed for one. A case
# IS owed for every row below:
#
#   `mapfile … < <(viewer_modules)`
#     1. the module enumerator        `find "$SRC"`
#     2. the module path trimmer      `sed "s#^$SRC/##"`
#     3. the module sorter            `sort`
#   `mapfile … < <(readme_table_block …)`
#     4. the table reader             `awk … "$README"`
#   `mapfile … < <(table_rows …)`
#     5. the table row reader         `sed -nE …`
#   `kind_of` (in `$(…)`)
#     6. the kind extractor           `sed -E …`, under a `gate_grep`
#        that diagnoses itself
#   `mapfile … < <(app_only_crates)`
#     7. the manifest feature reader  `awk … "$MANIFEST"`
#     8. the dep extractor            `sed -nE …`
#     9. the dep speller              `tr`
#    10. the dep sorter               `sort -u`
#   `hits=$(…)`
#    11. the hit deduplicator         `gate_record_awk`
#    12. the hit sorter               `sort`
#
# The two stages that FEED that union — `gate_rust_code | gate_grep`,
# in `lines_hits=$(…)` and `tree_hits=$(…)` one line above it — are the
# self-diagnosing kind and are named here only so a reader does not
# take their absence for an oversight.
#
# A GUARD IS ON A STAGE AND NEVER ON A PIPELINE. `pipefail` reports the
# RIGHTMOST non-zero stage, so a guard written `a | b || reader_failed
# "a"` diagnoses a dead `b` as a dead `a` — a message naming a repair to
# something that is fine, which is the class this apparatus exists to
# remove. Each stage is in its own brace group, so the status its guard
# reads is that stage's own, and `gate_rust_code` and `gate_grep`
# diagnose themselves in `lib.sh` rather than being spoken for here.
#
# WHAT IS NOT A READER, by the same rule. `contains`, `join_alt`,
# `module_path` and `module_head` are pure parameter expansion and
# `printf`; `printf` is a bash BUILTIN reading the argument list its
# caller already holds, so a `printf … | …` stage is not a reader even
# where it opens a pipeline. Checks 6 and 6b use a plain `grep -q` as a
# PREDICATE, where exit 1 IS the answer — `lib.sh` carves that case out
# by name and both sites say so.
#
# A STATUS THE SHELL KEEPS STILL BUYS NO DIAGNOSIS. Stages 7, 14 and 15
# sit in `$(…)`, so errexit does end the gate on them — what it does
# not do is say which reader died, or that anything was left undecided.
# The guard buys the diagnosis, not the redness, and that is the whole
# of `lib.sh`'s S157 second half.
reader_failed() {
  # THE TEXT IS `lib.sh`'s, and it is one text for every reader in this
  # directory that could not run. What stays here is WHICH reader — the
  # caller names it and the self-test aims at that name.
  gate_reader_died_refusal "the $1" "$2"
}

# THE MARKER IS READ WHERE THE CALLER RESUMES, not only at `gate_ok`.
# `gate_ok` refusing to print over it is what makes a green impossible;
# this is what keeps the diagnosis honest in between. Without it the
# checks below run over an empty read and print their own conclusions —
# "the heading was renamed or the table was reshaped" — about a scan
# that never happened, which is a misdiagnosis pointing at the wrong
# repair.
abort_if_reader_failed() {
  if [ -e "$GATE_MATCHER_FAILED" ]; then
    rm -f "$GATE_MATCHER_FAILED"
    gate_error "$(gate_name): a reader failed to run during this pass (diagnosed above), so what it did not read is unknown — the checks below it are not asked, because their answers would be about a scan that did not happen"
    exit 1
  fi
}

# The declaration, anchored at column zero so only a module-level doc
# comment can carry it. The trailing prose is free; the kind is not.
KIND_LINE='^//! Module kind: \*\*(vocabulary|driver)\*\*'
# What an exempted module's header must say, and what every other
# vocabulary's must not.
EXCEPTION_MARK='^//!.*recorded exception'

# The README tables this gate reads. The kind each asserts is the
# table's own; a row is a module in its first column.
DRIVER_TABLE='### The drivers'
VOCAB_TABLES=(
  "### The session's vocabularies"
  "### The app's vocabularies"
)

# EVERY ROSTER TABLE'S FIRST COLUMN IS THE MODULE, and this reader
# reads that column. A header row that no longer opens with it means
# the columns were reordered or renamed under a reader that would go on
# parsing them perfectly — the divergence between what a human sees and
# what this gate reads, which is the thing it exists to refuse.
TABLE_FIRST_COLUMN='| Module |'

# The README section two diagnoses send a reader to. Named once and
# CHECKED to exist (check 6b): a gate whose thesis is that hand-kept
# cross-references rot must not carry one itself. Both diagnoses used
# to spell this heading into their message text, and both had to be
# hand-edited when #1883 renamed the section — the demonstration, not
# the counterexample.
CITED_SECTION='### What a vocabulary reads, it is handed'

# The driver types, by name. The only needle set not derived, and check
# 6 holds it against the README's rule text so deleting one there
# cannot silently narrow this.
FORBIDDEN_TYPE_NAMES=(DocSession ViewerApp)

# FILE|NEEDLE|COUNT. See the header: the needle is what the exemption
# is FOR and the count is how many sites it covers.
VOCAB_EXCEPTIONS=()

# THE LIST IS OVERRIDABLE FOR A PLANTED TREE, AND ONLY FOR ONE. The
# self-test's exception arms need an entry to aim at and the tree has
# none, so they supply their own; the zero-hit control needs to be sure
# there is none, whatever the tree currently carries. Both read this.
#
# It is honoured only when `GATE_ROOT` is set, which happens only via
# `--root`, which only the self-test passes — so no environment can
# exempt the REPO from this gate, which is the reason an env-var
# override would otherwise be a bad idea in a file whose job is to
# refuse.
vocab_exceptions_from_env() {
  [ -n "${GATE_ROOT:-}" ] || return 0
  [ -n "${GATE_SELFTEST_VOCAB_EXCEPTIONS+x}" ] || return 0
  read -r -a VOCAB_EXCEPTIONS <<< "$GATE_SELFTEST_VOCAB_EXCEPTIONS"
}

# LIST MEMBERSHIP IN BASH, not `printf … | grep -qxF`. `grep -q` as a
# predicate is sanctioned by `lib.sh`, but only where exit 1 IS the
# answer — and here a grep that could not run (exit 2) would read as
# "not in the list", which is a FALSE RED on the roster checks and a
# silently DROPPED exemption on the exception check. The lists are a
# handful of short strings in memory; a search that cannot fail cannot
# fail wrong.
contains() {
  local needle=$1 item
  shift
  for item in "$@"; do
    [ "$item" = "$needle" ] && return 0
  done
  return 1
}

join_alt() {
  local out="" item
  for item in "$@"; do
    out="${out:+$out|}$item"
  done
  printf '%s' "$out"
}

kind_count() { gate_grep -cE "$KIND_LINE" "$1"; }
# THE GUARD IS ON THE `sed`, not on the pipeline: `gate_grep` diagnoses
# itself as `grep` in `lib.sh`, and a guard here would name this stage
# for that stage's death.
kind_of() {
  gate_grep -m1 -oE "$KIND_LINE" "$1" |
    { sed -E 's/.*\*\*(vocabulary|driver)\*\*/\1/' \
        || reader_failed "kind extractor over $SRC" "$?"; }
}

# THE MODULE ROSTER IS THE TREE, and the order is a read rather than a
# cosmetic — it is what makes the scan and the counts the gate prints
# the same on every box, so `sort` earns a guard of its own.
viewer_modules() {
  { find "$SRC" -type f -name '*.rs' ! -name lib.rs ! -path "$SRC/bin/*" \
      || reader_failed "module enumerator over $SRC" "$?"; } |
    { sed "s#^$SRC/##" || reader_failed "module path trimmer over $SRC" "$?"; } |
    { sort || reader_failed "module sorter over $SRC" "$?"; }
}

# A README table, as `@` (the heading was found) followed by every line
# of the table under it, and `!fence` if that table ENDED at a fenced
# block. Emitting the heading as a record is what lets the caller tell
# "the heading is gone" from "the reader died" from "there is no table
# under it" — three answers a bare row count folds into one, and this
# gate reds identically on all three today.
#
# COLUMN ZERO IS NOT WHERE MARKDOWN PUTS A ROW, and this reader learned
# it the expensive way: its first version anchored every rule at `^`,
# and a row indented TWO SPACES — which every renderer draws as a row of
# the table — was seen by no rule at all. Indenting
# `crates/viewer/README.md`'s last `session::probe` row dropped that
# module out of the roster with the gate at exit 0 and its OK line
# `cmp`-identical to the clean run: the silent short roster this reader
# was rewritten to remove, re-minted inside the rewrite.
# `work/view/plan.md` carries the rule from #2172 — *a `^`-anchored
# pattern over markdown is a claim about column zero that markdown does
# not make* — and it is the same boundary one sentinel over.
#
# SO THE INDENT IS STRIPPED ONCE, and every rule reads the stripped
# line. CommonMark allows a heading, a table row and a table's
# delimiter row up to THREE spaces in and makes FOUR a code block, so
# `sub(/^ {0,3}/, "", line)` encodes the boundary exactly rather than
# approximating it with `[[:space:]]*`: a four-space line keeps one
# space and matches no rule, and a leading TAB advances to column four
# and keeps its tab. Both sides are planted, and all six positions were
# settled with `markdown-it-py` 4.2.0 in CommonMark mode with tables
# on — the header, the delimiter row and a body row each accept one to
# three spaces, and four spaces on the delimiter row leaves no table at
# all. One call per line, before any rule, because a rule ending in
# `next` would skip a normaliser placed after it — `md_fence`'s own
# convention, one line up.
#
# THE TABLE IS THE FIRST CONTIGUOUS RUN OF TABLE LINES under the
# heading, which is what a renderer draws and what the old scan did
# not: that one collected every `^|` line between the heading and the
# next heading, so a `|` line in the section's prose below the table —
# or a second table — joined the roster, and the rows were read as one
# table nothing draws as one.
#
# EVERY TABLE LINE IN THE SECTION IS ACCOUNTED FOR, and what produces
# that population is the strip above plus the two markers below: a line
# is a table line when its first character after at most three spaces
# is `|`. Under that rule each one is the header, the delimiter, a row
# of the roster, or a line the caller reds on — which is what lets the
# roster's length be held against the table's own. A `|` line after the
# table has ended comes back as `!stray <line>`; without it a fence
# with a blank line above it ends the table silently and the rows below
# are simply not the roster, reproduced on the real tree where four of
# six vocabulary rows vanished and the OK line came back identical.
#
# ENDING AT A FENCE IS ITS OWN ANSWER, and it is the other half of that
# silence. A fenced block opening inside the table body ends the table
# there: markdown agrees, so the rows below the fence are not the
# roster and no renderer shows them as one — but the callers red only
# on an EMPTY roster and never on a SHORT one, so the region narrowed
# and the gate went on printing OK over fewer modules than the README
# lists. Reproduced before the repair: a `rust` fence four rows into
# `### The session's vocabularies` left the cross-check covering two of
# six vocabularies with the OK line byte-identical.
#
# THE OTHER SIDE OF THE BOUNDARY IS ITS OWN ANSWER TOO, `!indent`, and
# it is there because being RIGHT about a four-space line is not the
# same as being loud about it. A line the author wrote as a row and
# indented four spaces or a tab is a code block, so dropping it from
# the roster is the correct read — and if it is the table's LAST row,
# nothing follows it to become a `!stray`, the roster is one shorter
# and every other figure is unchanged. That is the silent shortening
# again, arriving from the README's side rather than the reader's, so
# the record is emitted without `next`: the line is reported AND still
# not read as a row. A fenced one is not reported, because inside a
# fence it is content. No line in this README matches it today — the
# sweep is `grep -nE '^[[:space:]]+[|]' crates/viewer/README.md`, which
# returns nothing over the whole page, not merely over the three
# scanned sections.
#
# THE FENCE TEST HERE ASKS FOR `open` AND DOES NOT NEED THE TRACKER'S
# THIRD ANSWER, which is worth saying because the opposite reads as
# obvious. `close` and `inside` are both unreachable at that rule: a
# table body cannot be read inside a fence, so the only delimiter that
# can end one is the delimiter that opens it. `fence != ""` decides
# identically here and this gate's self-test cannot tell the two apart
# — checked by mutation. The third answer is load-bearing for
# `viewer-vocab-declared-once.sh`'s `opens`, whose question is *did the
# previous line END a block*, and that gate's self-test is what holds
# it. `open` is written here because it is the true predicate — the
# table ran into a block that OPENS — and not because a boolean would
# fail.
readme_table_block() {
  awk -v want="$1" "$FENCE_AWK"'
    { fence = md_fence($0); fenced = (fence != "") }
    { line = $0; sub(/^ {0,3}/, "", line) }
    !fenced && line == want { insec = 1; intable = 0; done = 0; print "@"; next }
    !insec { next }
    !fenced && line ~ /^#/ { insec = 0; next }
    !fenced && line !~ /^[|]/ && $0 ~ /^[[:space:]]+[|]/ { print "!indent " $0 }
    done && !fenced && line ~ /^[|]/ { print "!stray " line; next }
    done { next }
    intable && !fenced && line ~ /^[|]/ { print line; next }
    intable { if (fence == "open") { print "!fence" } intable = 0; done = 1; next }
    !fenced && line ~ /^[|]/ { intable = 1; print line; next }
  ' "$README" || reader_failed "table reader over $README" "$?"
}

# One record per table line, and NEVER fewer: a line that does not read
# as a data row comes back as `!row <line>` rather than being dropped.
# That is the length check this gate had no way to make — the callers
# below hold the roster's length against the table's own, so a reshaped
# cell narrows the roster loudly instead of quietly. `t` is what keeps
# the two expressions from both firing on one line.
#
# THE GUARD IS ON THE `sed` and the leading `printf` is deliberately
# unguarded: it is a bash BUILTIN reading the argument list this
# function was handed, so it is a stage of a pipeline and still not a
# READER — nothing on PATH can shadow it and it never opens `$README`.
table_rows() {
  printf '%s\n' ${1:+"$@"} |
    { sed -nE -e 's/^\|[[:space:]]*`([A-Za-z0-9_:]+)`[[:space:]]*\|.*/\1/p;t' \
        -e 's/^.*$/!row &/p' \
        || reader_failed "table row reader over $README" "$?"; }
}

# ONE TABLE'S ROSTER, with every way the read can go wrong told apart.
# Fills `TABLE_ROSTER` and returns non-zero having diagnosed, so the
# caller decides whether one bad table ends the pass or joins its `rc`.
# WHAT IS NAMED IS WHAT THE TABLE IS FOR — `$1` — because the two
# callers enforce different things off the same shape and a diagnosis
# that says only "a table" leaves the reader to guess which.
TABLE_ROSTER=()
read_table_roster() {
  local what=$1 heading=$2 line
  local -a block=() lines=() rows=() strays=() indented=()
  local anchors=0 fence_cut=false
  TABLE_ROSTER=()
  mapfile -t block < <(readme_table_block "$heading")
  abort_if_reader_failed
  for line in ${block[@]+"${block[@]}"}; do
    case $line in
      '@') anchors=$((anchors + 1)) ;;
      '!fence') fence_cut=true ;;
      '!stray '*) strays+=("${line#!stray }") ;;
      '!indent '*) indented+=("${line#!indent }") ;;
      *) lines+=("$line") ;;
    esac
  done
  if [ "$anchors" -eq 0 ]; then
    gate_error "$(gate_name): $README carries no \"$heading\" heading, so $what came from nowhere. That heading is where this gate reads the table, matched whole and at column zero — restore it, or move the heading in this file in the same diff, because a roster that scans nothing is not a pass"
    return 1
  fi
  if [ "$anchors" -ne 1 ]; then
    gate_error "$(gate_name): $README carries $anchors \"$heading\" headings, and this gate would read the table under whichever one it met last. One heading, one roster — delete the duplicate, or give the second section a heading of its own"
    return 1
  fi
  # ASKED BEFORE THE TABLE'S OWN SHAPE, because a row the author
  # indented out of the table changes what every check below is looking
  # at — including which line is the header.
  if [ "${#indented[@]}" -ne 0 ]; then
    gate_error "$(gate_name): $README's \"$heading\" section carries ${#indented[@]} line(s) indented FOUR or more spaces that would otherwise be table rows, the first being \`${indented[0]}\`. Markdown draws four spaces as a code block and three as a row, so this line is not part of the table for this gate OR for any renderer — and if it is the last row, nothing else changes and $what silently covers one module fewer. Unindent it to at most three spaces, or move it out of the table"
    return 1
  fi
  if [ "$fence_cut" = true ]; then
    gate_error "$(gate_name): $README's \"$heading\" table is INTERRUPTED by a fenced code block, so the rows below the fence are not part of it — for this gate and for every renderer alike. That is how $what silently covers fewer modules than the page appears to list: the roster simply gets shorter and nothing else changes. Move the fenced example out of the table, or put a blank line above it if the table really ends there"
    return 1
  fi
  if [ "${#lines[@]}" -eq 0 ]; then
    gate_error "$(gate_name): $README's \"$heading\" heading is there and no table follows it, so $what came from nowhere. The roster is the FIRST run of \`|\` lines under that heading — either the table was moved out of the section or something was written between the two — and a roster that reads nothing is not a pass"
    return 1
  fi
  case ${lines[0]} in
    "$TABLE_FIRST_COLUMN"*) ;;
    *) gate_error "$(gate_name): $README's \"$heading\" table does not open with a header row whose first column is \`$TABLE_FIRST_COLUMN\` (it opens with \`${lines[0]}\`), so this gate would read $what off a column that no longer means what it thinks. Restore the header, or change this reader in the same diff"
       return 1 ;;
  esac
  # THE SEPARATOR IS WHAT MAKES IT A TABLE, and the pattern is held in
  # a variable because a bracket expression inside `[[ =~ ]]` is read
  # once by the shell and once by the regex engine otherwise.
  local sep_re='^\|[-:| ]*-[-:| ]*\|[[:space:]]*$'
  if [ "${#lines[@]}" -lt 2 ] || ! [[ ${lines[1]} =~ $sep_re ]]; then
    gate_error "$(gate_name): $README's \"$heading\" table has no \`|---|---|\` row under its header (the next line is \`${lines[1]:-<nothing>}\`), so no markdown renderer draws it as a table and a human reader sees $what as a paragraph. This reader would go on parsing it perfectly, which is the divergence this gate exists to refuse — restore the separator"
    return 1
  fi
  # GUARDED, because `printf '%s\n'` with no operands prints ONE empty
  # line and the row reader's fallback would report it as a line it
  # cannot read — a diagnosis about a line the table does not have,
  # where the answer is that the table has no rows at all.
  if [ "${#lines[@]}" -gt 2 ]; then
    mapfile -t rows < <(table_rows "${lines[@]:2}")
    abort_if_reader_failed
  fi
  # THE LENGTH CHECK, and it is the roster's length against the TABLE's
  # own rather than against a number kept here. Every line under the
  # header and separator is a row of this roster or it is a line this
  # gate could not read as one, and there is no third answer: a cell
  # reshaped so the first column stops being a backticked module used
  # to leave the roster one shorter with nothing said.
  #
  # WHAT IT CANNOT DO, stated because a length check reads as more than
  # it is: nothing here knows how long the table is SUPPOSED to be. Two
  # counts derived from the same read cannot catch a read that stopped
  # early, so a region truncated part-way down shortens the roster and
  # the check with it. What removes that mechanism is the fence tracker
  # above and the `!fence` answer, not this.
  for line in ${rows[@]+"${rows[@]}"}; do
    if [ "${line#!row }" != "$line" ]; then
      gate_error "$(gate_name): $README's \"$heading\" table carries a line this gate cannot read as a row of $what: \`${line#!row }\`. Every line under the header and its separator is one module of the roster — the first column is the module, in backticks — so a line that is not one makes the roster SHORTER than the table it is read from, which is a narrowing nothing else here would report. Fix the cell, or move the line out of the table"
      return 1
    fi
    TABLE_ROSTER+=("$line")
  done
  if [ "${#TABLE_ROSTER[@]}" -eq 0 ]; then
    gate_error "$(gate_name): $README's \"$heading\" table has a header and a separator and no rows under them, so $what decided nothing — a roster that scans nothing is not a pass"
    return 1
  fi
  # THE OTHER HALF OF THE LENGTH CHECK, and the direction that is
  # quiet. A `|` line further down the section is a table line this
  # roster does not contain — a second table, a row separated from the
  # first by a blank line, or a row below a fenced block — and every
  # renderer draws it as something other than a row of the table above.
  # Asked LAST because the diagnoses above name a repair and this one
  # names a line.
  if [ "${#strays[@]}" -ne 0 ]; then
    gate_error "$(gate_name): $README's \"$heading\" section carries ${#strays[@]} table line(s) BELOW the table this gate reads, the first being \`${strays[0]}\`. $what is the first run of \`|\` lines under that heading, which is the one thing a renderer draws as that table — a row cut off from it by a blank line, a fenced block or a second table is not part of it, and the roster is SHORTER than the page looks by exactly those lines. Rejoin the rows to the table, or move them under a heading of their own"
    return 1
  fi
  return 0
}

# `session::select` is `session/select.rs`; `forms` is `forms.rs`.
module_path() { printf '%s.rs\n' "${1//:://}"; }
# `session::select` is hosted by `session`.
module_head() { printf '%s\n' "${1%%::*}"; }

# THE `app` FEATURE'S `dep:` ENTRIES, in the spelling Rust source uses
# (cargo's `-` is the crate's `_`). Read from the manifest rather than
# restated: see the header. Four stages, four guards — the sort is
# `-u` and drops duplicates, so it decides the count check 5 reports.
app_only_crates() {
  { awk '
      /^app[[:space:]]*=[[:space:]]*\[/ { inside = 1; next }
      inside && /^\]/ { inside = 0 }
      inside { print }
    ' "$MANIFEST" || reader_failed "manifest feature reader over $MANIFEST" "$?"; } |
    { sed -nE 's/.*"dep:([A-Za-z0-9_-]+)".*/\1/p' \
        || reader_failed "dep extractor over $MANIFEST" "$?"; } |
    { tr '-' '_' || reader_failed "dep speller over $MANIFEST" "$?"; } |
    { sort -u || reader_failed "dep sorter over $MANIFEST" "$?"; }
}

gate() {
  local rc=0 f rel n kind
  gate_require_file "$README"
  gate_require_file "$MANIFEST"
  if [ ! -d "$SRC" ]; then
    gate_error "$(gate_name): $SRC does not exist under $PWD — the gate's subject is gone, so it scanned nothing, which is not a pass"
    exit 1
  fi

  # THE MODULE ROSTER IS THE TREE. `lib.rs` declares the modules rather
  # than being one, and `bin/` is a binary rather than a module of the
  # library; everything else answers the question.
  local -a modules=()
  mapfile -t modules < <(viewer_modules)
  # BEFORE the count, not after it. A dead enumerator yields no paths,
  # and "no modules under crates/viewer/src" is the wrong answer to give
  # about a directory nobody managed to list — it was the live
  # misdiagnosis a dead `sed` produced here.
  abort_if_reader_failed
  GATE_SCAN_FILES=${#modules[@]}
  if [ "$GATE_SCAN_FILES" -eq 0 ]; then
    gate_error "$(gate_name): no modules under $SRC in $PWD besides lib.rs and bin/ — the gate scanned nothing, which is not a pass"
    exit 1
  fi

  # --- what the README says ------------------------------------------
  local -a driver_rows=() vocab_rows=() table
  read_table_roster "the driver roster this gate enforces" "$DRIVER_TABLE" || exit 1
  driver_rows=(${TABLE_ROSTER[@]+"${TABLE_ROSTER[@]}"})
  for table in "${VOCAB_TABLES[@]}"; do
    if ! read_table_roster "the cross-check over it" "$table"; then
      rc=1
      continue
    fi
    vocab_rows+=("${TABLE_ROSTER[@]}")
  done
  [ "$rc" -eq 0 ] || exit 1

  local -a driver_roster=() row
  for row in "${driver_rows[@]}"; do
    driver_roster+=("$(module_path "$row")")
  done

  # --- 1. EVERY MODULE DECLARES EXACTLY ONE KIND ----------------------
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

  # CHECK 7 HAS A SUBJECT, and the constraint on where this is asked is
  # check 4's exit. Below that exit every row of the README's
  # vocabulary tables has been matched to a module declaring
  # `vocabulary`, over tables proved non-empty, so an empty vocabulary
  # set is unreachable there and a guard against it decides nothing. It
  # is asked under check 1 because that is where the kinds are read and
  # `vocab` — which check 7's `scanned` is built from — is complete.
  # The answer is the tree's own, not the README's: every module
  # declared itself a driver, so the import scan would read no file and
  # the gate would report green over a scan of nothing.
  if [ "${#vocab[@]}" -eq 0 ]; then
    gate_error "$(gate_name): no vocabulary modules under $SRC — every module declares itself a driver, which is not a pass"
    exit 1
  fi

  # --- 2. THE EXEMPTED MODULES SAY SO IN THEIR OWN HEADERS ------------
  # A header that denies naming a driver type above a line that names
  # one is the defect this unit is about, and rustdoc publishes it.
  local ex spec exfile exneedle excount
  local -a exception_files=()
  for spec in "${VOCAB_EXCEPTIONS[@]}"; do
    exception_files+=("${spec%%|*}")
  done
  for rel in "${modules[@]}"; do
    f=$SRC/$rel
    n=$(gate_grep -cE "$EXCEPTION_MARK" "$f")
    if contains "$rel" ${exception_files[@]+"${exception_files[@]}"}; then
      if [ "$n" -eq 0 ]; then
        gate_error "$SRC/$rel is on this gate's exception list and its doc header does not say so. A module header that reads \"names no driver type\" nine lines above naming one is false, and rustdoc publishes it — state the exception in the \`Module kind:\` declaration and point at $README's \"${CITED_SECTION#\#\#\# }\""
        rc=1
      fi
    elif [ "$n" -gt 0 ]; then
      gate_error "$SRC/$rel claims a \"recorded exception\" in its doc header and this gate grants it none — an exemption written in prose is not one, and a reader of that header would believe it. Either add the entry here with its needle and site count, or drop the claim"
      rc=1
    fi
  done
  [ "$rc" -eq 0 ] || exit 1

  # --- 3. THE DRIVER ROSTER, BOTH WAYS -------------------------------
  local d
  for rel in ${driver[@]+"${driver[@]}"}; do
    if ! contains "$rel" "${driver_roster[@]}"; then
      gate_error "$SRC/$rel declares itself a DRIVER and $README's \"$DRIVER_TABLE\" table does not list it. That table IS the roster and not a summary of one — $README says so, and it lists ${#driver_roster[@]} modules — so a further driver is an amendment there and not a header edit, because a module that declares \`driver\` is exempt from every import check below"
      rc=1
    fi
  done
  for d in "${driver_roster[@]}"; do
    if [ ! -f "$SRC/$d" ]; then
      gate_error "$README's \"$DRIVER_TABLE\" lists a module whose file $SRC/$d does not exist — the table outran the code"
      rc=1
    elif ! contains "$d" ${driver[@]+"${driver[@]}"}; then
      gate_error "$README's \"$DRIVER_TABLE\" lists $d but $SRC/$d does not declare \`//! Module kind: **driver**\` — a module demoted in its own header while the README still calls it a driver is exempt from every import check and classified as a vocabulary by every reader"
      rc=1
    fi
  done
  # --- 4. THE VOCABULARY TABLES, AGAINST THE MODULES THEY NAME -------
  local path
  for row in "${vocab_rows[@]}"; do
    path=$(module_path "$row")
    if [ ! -f "$SRC/$path" ]; then
      gate_error "$README's vocabulary tables list \`$row\`, which is not a module in the tree ($SRC/$path does not exist) — the table outran the code"
      rc=1
    elif ! contains "$path" ${vocab[@]+"${vocab[@]}"}; then
      gate_error "$README's vocabulary tables list \`$row\` as a vocabulary, but $SRC/$path declares itself a DRIVER — the README and the module disagree about what the module is, which is the drift a per-module declaration buys and this check pays for"
      rc=1
    fi
  done
  [ "$rc" -eq 0 ] || exit 1

  # --- 5. THE NEEDLES, DERIVED ---------------------------------------
  local -a crates_list=() heads=() vocab_heads=() path_mods=()
  mapfile -t crates_list < <(app_only_crates)
  # BEFORE the count. "The `app` feature yielded no `dep:` entries" is
  # the wrong answer to give about a manifest no stage managed to read.
  abort_if_reader_failed
  if [ "${#crates_list[@]}" -eq 0 ]; then
    gate_error "$(gate_name): $MANIFEST's \`app\` feature yielded no \`dep:\` entries, so the forbidden-crate set came from nowhere. Either the feature was renamed or its shape changed — a needle set derived from nothing matches nothing, which is not a pass"
    exit 1
  fi
  for row in "${vocab_rows[@]}"; do vocab_heads+=("$(module_head "$row")"); done
  for row in "${driver_rows[@]}"; do
    d=$(module_head "$row")
    contains "$d" ${path_mods[@]+"${path_mods[@]}"} && continue
    # A driver that HOSTS a vocabulary cannot be a forbidden path:
    # `crate::session::SessionOp` is the rule working. Read off the
    # vocabulary tables, never carved out by hand.
    contains "$d" ${vocab_heads[@]+"${vocab_heads[@]}"} && continue
    path_mods+=("$d")
  done
  # AN EMPTY SET IS NOT A NARROWER PATTERN, IT IS A DIFFERENT ONE. With
  # no driver module path left, `join_alt` yields the empty string and
  # the path arm below reads `crate::()\b`, which matches ANY
  # `crate::` import — so without this the gate does not fall quiet, it
  # reds on the first vocabulary that imports a vocabulary under a
  # driver, naming a file that broke nothing. What is wrong is the
  # roster the needles came from, and that is what this says.
  if [ "${#path_mods[@]}" -eq 0 ]; then
    gate_error "$(gate_name): every driver in $README's table also hosts a vocabulary, so no driver module path is forbidden and check 7's path arm matches nothing — that is not a pass"
    exit 1
  fi

  local types_alt crates_alt mods_alt
  types_alt=$(join_alt "${FORBIDDEN_TYPE_NAMES[@]}")
  crates_alt=$(join_alt "${crates_list[@]}")
  mods_alt=$(join_alt "${path_mods[@]}")
  local pat_types="\\b($types_alt)\\b"
  local pat_crates="\\b($crates_alt)\\b"
  # THREE PATH SPELLINGS, and the third is the one the README's own
  # slogan names. `crate::app` is the bare import; `app::` is the
  # segment-with-children form, which also catches a nested
  # `crate::{app::x, …}`; and the use-tree arm below catches
  # `use crate::{app, camera::Camera};`, where the driver appears as a
  # bare leaf inside braces and neither of the first two sees it. That
  # idiom is written in 22 places elsewhere in this workspace.
  local pat_paths="crate::($mods_alt)\\b|\\b($mods_alt)::"
  local pat_tree="use[[:space:]]+(crate|self|super)::\\{[^;]*\\b($mods_alt)\\b"
  local pat_all="$pat_types|$pat_crates|$pat_paths"

  # --- 6. THE README'S RULE STILL NAMES THE TYPES --------------------
  local t
  for t in "${FORBIDDEN_TYPE_NAMES[@]}"; do
    # PLAIN `grep -q`, NOT `gate_grep`: lib.sh folds exit 1 to 0 so a
    # scan that matched nothing reads as a clean scan, which is right
    # for a matcher and WRONG for a predicate — folded, this arm could
    # never fire. lib.sh says so at gate_grep; this is the case it means.
    if ! grep -qF "\`$t\`" "$README"; then
      gate_error "$(gate_name): $README no longer names \`$t\` anywhere, and this gate forbids it in every vocabulary on that document's authority. A needle whose ratification has left the README is a rule this gate invented — restore the rule text or drop the name here"
      rc=1
    fi
  done
  [ "$rc" -eq 0 ] || exit 1

  # --- 7. NO VOCABULARY NAMES A DRIVER, A DRIVER MODULE, OR A CRATE
  # THAT ONLY EXISTS BEHIND `app` ------------------------------------
  # Read over the shared CODE-ONLY view, so a header that spells
  # `DocSession` out to say it does not name one — `forms.rs` and
  # `drafts.rs` both do — stays green, and a string literal cannot fire
  # it either.
  local -a scanned=()
  for rel in ${vocab[@]+"${vocab[@]}"}; do scanned+=("$SRC/$rel"); done
  local lines_hits tree_hits hits
  lines_hits=$(gate_rust_code "${scanned[@]}" | gate_grep -E "$pat_all")
  tree_hits=$(gate_rust_code --window 12 "${scanned[@]}" | gate_grep -E "$pat_tree")
  # ONE RECORD PER SITE. The window arm restates a line the line arm
  # may already have matched, and both spell the site `FILE:LINE:`, so
  # the union is deduplicated on that key — otherwise an exception's
  # site count would depend on which arms fired.
  #
  # THE KEY IS THE RECORD'S OWN FILE AND LINE, read by `lib.sh`'s
  # `gate_record_split` (§"THE COLUMNS OF A RECORD"). It used to be `awk
  # -F: '{ k = $1 ":" $2 }'`, which is the FILE column read as
  # everything before the first colon — so for a module at `a:b.rs`
  # every record in the file shared ONE key whatever line it sat on,
  # and all but the first were DROPPED. The direction is blind: sites
  # vanish from the count an exception entry is compared against, so a
  # count reads low, an entry the tree has outgrown reads as still
  # exact, and the gate goes green over a site nobody argued.
  #
  # `gate_grep` ON THE FILTER, NOT `|| true` ON THE PIPELINE. With no
  # vocabulary naming anything forbidden — the tree since #1883 hoisted
  # the last two reads — both arms are empty, the blank-line filter
  # matches nothing and exits 1, and under `set -euo pipefail` a plain
  # `grep` here takes the ASSIGNMENT and the gate with it: exit 1, no
  # diagnosis, indistinguishable on CI from a real finding. The fix is
  # `lib.sh`'s and it is not `|| true`: that spelling folds exit 2
  # (could not search) and 127 (no `grep` at all) into "nothing
  # matched" and greens over a scan that never ran. `gate_grep` draws
  # the distinction per stage — 1 becomes 0, anything else is diagnosed
  # and marks `GATE_MATCHER_FAILED` so `gate_ok` refuses to print.
  hits=$(printf '%s\n%s\n' "$lines_hits" "$tree_hits" | gate_grep -v '^[[:space:]]*$' |
    { gate_record_awk '
        { k = gate_record_split($0) ? GR_FILE ":" GR_LINE : $0 }
        !(k in seen) { seen[k] = 1; print }' \
        || reader_failed "hit deduplicator over $SRC" "$?"; } |
    { sort || reader_failed "hit sorter over $SRC" "$?"; })

  # --- 8. THE EXCEPTIONS, SITE BY SITE -------------------------------
  local found kept exre
  for spec in "${VOCAB_EXCEPTIONS[@]}"; do
    exfile=${spec%%|*}
    excount=${spec##*|}
    exneedle=${spec#*|}; exneedle=${exneedle%|*}
    # THE ANCHOR IS `lib.sh`'s, and it is the ONE property this gate
    # shares with the skips built on `gate_exact_skip`: the exemption
    # applies to sites in the entry's own file and nowhere else. Built
    # once and read three times below, so the three cannot disagree, and
    # ESCAPED there — exactly, not because `formsXrs` is reachable,
    # since every module scanned here is a `.rs` file this gate found
    # by name.
    # THE NEEDLE IS NOT EXACT TEXT and is not escaped: an entry's needle
    # is a pattern by design (see the entry format above), which is why
    # this gate keeps its own exemption rather than calling
    # `gate_exact_skip_filter` — that mechanism drops a whole record it
    # can name in full, and this one counts SITES a pattern names.
    exre="$(gate_record_anchor "$SRC/$exfile").*$exneedle"
    if [ ! -f "$SRC/$exfile" ]; then
      gate_error "$(gate_name): the exception list names $SRC/$exfile, which is not a file under $PWD — drop the entry"
      rc=1
      continue
    fi
    if ! contains "$exfile" ${vocab[@]+"${vocab[@]}"}; then
      gate_error "$(gate_name): the exception list names $exfile, which no longer declares \`//! Module kind: **vocabulary**\` — an exception to the vocabulary rule on a module that is not a vocabulary exempts nothing and hides the module from every check here"
      rc=1
      continue
    fi
    # `gate_grep`, and this one is the sharpest of the three: the
    # pattern is INTERPOLATED from an exception entry, so a malformed
    # needle is a grep that cannot search. Under `|| true` that read as
    # a count of zero and the gate went green over an exemption whose
    # own pattern was broken.
    found=$(printf '%s\n' "$hits" | gate_grep -c -E "$exre")
    if [ "$found" -gt "$excount" ]; then
      printf '%s\n' "$hits" | grep -E "$exre" | cut -c1-160
      gate_error "$SRC/$exfile names $exneedle at $found sites and its recorded exception covers $excount. The exception is SITE-granular on purpose ($README, ${CITED_SECTION#\#\#\# }): the sites that were argued for are exempt and a new one is not, so a later unit cannot inherit the ratification by adding a line to an allowlisted file (work/gates/D103.md's class). Take the driver's answer as a value, or argue the new site and raise the count with it"
      rc=1
      continue
    fi
    if [ "$found" -lt "$excount" ]; then
      gate_error "$SRC/$exfile names $exneedle at $found sites and its recorded exception covers $excount — the exception has outlived part of its reason. An entry with nothing behind it is a ratification waiting to be inherited by the next line added to the file (interval-square-allowlist.sh says the same of its own entries), so lower the count or delete the entry"
      rc=1
      continue
    fi
    hits=$(printf '%s\n' "$hits" | gate_grep -v -E "$exre")
  done
  [ "$rc" -eq 0 ] || exit 1

  # --- 6b. THE SECTION THE DIAGNOSES CITE ----------------------------
  # PLAIN `grep -q`, NOT `gate_grep`: this is a predicate where exit 1
  # IS the answer, and `gate_grep` folds 1 to 0 — which would invert it
  # into "always present". `lib.sh` carves that case out by name, and an
  # unsearchable README fails RED here, which is the right direction.
  if ! grep -qxF "$CITED_SECTION" "$README"; then
    gate_error "$(gate_name): two of this gate's diagnoses send a reader to $README's \"${CITED_SECTION#\#\#\# }\" and the README no longer carries that heading. A gate whose subject is hand-kept cross-references rotting must not ship one — rename CITED_SECTION with the section, or restore the heading"
    exit 1
  fi

  kept=$(printf '%s\n' "$hits" | gate_grep -v '^[[:space:]]*$')
  if [ -n "$kept" ]; then
    printf '%s\n' "$kept" | cut -c1-160
    gate_error "a vocabulary module names a driver type, a driver module, or a crate that only exists behind the \`app\` feature. $README's rule: a vocabulary holds values, their wording and pure functions over them, and can be read and tested without a session or a window existing. Move the code to the driver, or take the driver's answer as a value — do not reclassify the module to silence this"
    exit 1
  fi

  # THE README-DERIVED COUNTS ARE ON THE LINE, and that is the reason
  # they are: every other figure here is read off the tree or the
  # manifest, so a roster that came back SHORT left this line
  # byte-identical to a clean run and no reader of a log could see it.
  # These two move when the rosters do. They gate nothing — the checks
  # above do — they make a narrowing visible where it does not gate.
  gate_ok "every module under $SRC declares a kind, ${#driver[@]} drivers match $README's own table of ${#driver_rows[@]} rows, ${#scanned[@]} vocabularies name none of ${#FORBIDDEN_TYPE_NAMES[@]} driver types, ${#path_mods[@]} driver module paths or ${#crates_list[@]} \`app\`-only crates read from $MANIFEST (${#VOCAB_EXCEPTIONS[@]} recorded exceptions, each still live at exactly its recorded site count), and the README's ${#vocab_rows[@]} tabulated vocabularies agree with the modules they name"
}

# This gate's subject is one crate's src tree plus its README and
# manifest, not `crates/*/src`, so the fixture is a miniature viewer
# crate. THE MANIFEST IS THE REAL ONE, copied: the derived needle set in
# the fixture is then the derived needle set in the tree, so the
# coverage cases below cannot fall behind a dependency someone adds to
# the `app` feature. The rosters and the exception list are planted from
# themselves for the same reason.
gate_plant_clean() {
  local root=$1 d ex spec exfile exneedle excount i
  mkdir -p "$root/$SRC/session" "$root/$SRC/pane" "$root/$SRC/bin"
  cp "$GATE_REPO_ROOT/$MANIFEST" "$root/$MANIFEST"

  # lib.rs and bin/ carry NO declaration, and stay green: they are the
  # two exclusions, asserted by the clean fixture rather than assumed.
  printf 'pub mod camera;\n' > "$root/$SRC/lib.rs"
  printf 'fn main() {}\n' > "$root/$SRC/bin/viewer.rs"

  # A vocabulary with both near misses baked in: a COMMENT naming the
  # driver type and the toolkit, and a real import of `session::op`,
  # which is a vocabulary living under a driver's module path.
  cat > "$root/$SRC/camera.rs" <<'RS'
//! The camera value.
//!
//! Names no `DocSession` and no `egui`; both words are prose here.
//!
//! Module kind: **vocabulary** — it names no driver type.
use crate::session::SessionOp;
pub fn apply(op: SessionOp) -> SessionOp { op }
RS
  for d in forms drafts; do
    {
      printf '//! An app vocabulary.\n//!\n'
      printf '//! Module kind: **vocabulary** — it names no driver type.\n'
      printf 'pub struct Thing%s;\n' "$d"
    } > "$root/$SRC/$d.rs"
  done
  {
    printf '//! What is selected.\n//!\n'
    printf '//! Module kind: **vocabulary** — it names no driver type.\n'
    printf 'pub struct Selection;\n'
  } > "$root/$SRC/session/select.rs"

  for d in "${FIXTURE_DRIVERS[@]}"; do
    mkdir -p "$root/$SRC/$(dirname "$d")"
    {
      printf '//! A driver.\n//!\n'
      printf '//! Module kind: **driver** (README, Module boundaries).\n'
      printf 'use eframe::egui;\n'
      printf 'pub struct DocSession;\n'
    } > "$root/$SRC/$d"
  done

  # The exceptions, planted from their own entries: the file names its
  # needle exactly COUNT times and its header states the exception.
  for spec in "${VOCAB_EXCEPTIONS[@]}"; do
    exfile=${spec%%|*}
    excount=${spec##*|}
    exneedle=${spec#*|}; exneedle=${exneedle%|*}
    mkdir -p "$root/$SRC/$(dirname "$exfile")"
    {
      printf '//! A ratified exception.\n//!\n'
      printf '//! Module kind: **vocabulary**, with a recorded exception.\n'
      i=0
      while [ "$i" -lt "$excount" ]; do
        printf 'pub fn read_%s(_s: &%s) {}\n' "$i" "$exneedle"
        i=$((i + 1))
      done
    } > "$root/$SRC/$exfile"
  done

  fixture_readme > "$root/$README"
}

# The fixture's driver roster. Named here rather than read from the real
# README so the fixture is a miniature crate rather than a copy of this
# one, but it carries the shapes that matter: a driver that hosts
# vocabularies (`session`), a driver split into a parent and children
# (`pane`), and plain ones.
FIXTURE_DRIVERS=(app.rs gpu.rs pane.rs pane/create.rs session.rs widgets.rs)

fixture_readme() {
  local d
  printf '## Module boundaries\n\n'
  printf 'Every module is a vocabulary or a driver. A vocabulary names\n'
  printf 'no `DocSession`, no `ViewerApp` and no `egui`.\n\n'
  # The section two diagnoses cite, so the cross-reference is asserted
  # by the fixture rather than trusted.
  printf '%s\n\nProse the diagnoses send a reader to.\n\n' "$CITED_SECTION"
  printf '%s\n\n' "$DRIVER_TABLE"
  printf '| Module | Is |\n|---|---|\n'
  for d in "${FIXTURE_DRIVERS[@]}"; do
    d=${d%.rs}
    printf '| `%s` | a driver |\n' "${d////::}"
  done
  printf '\n'
  printf "%s\n\n" "${VOCAB_TABLES[0]}"
  printf '| Module | Holds |\n|---|---|\n'
  printf '| `session::select` | what is selected |\n\n'
  printf "%s\n\n" "${VOCAB_TABLES[1]}"
  printf '| Module | Holds |\n|---|---|\n'
  printf '| `forms` | what the panels offer |\n'
  printf '| `drafts` | in-flight form state |\n\n'
  printf '### What a vocabulary reads, it is handed\n\nProse.\n'
}

# --- planters -------------------------------------------------------

# THE EMPTY-SET PLANTERS. Each of the four below empties a set this
# gate derives, and a matcher over an empty set decides nothing about
# the tree: three of them leave a green that says the tree is clean
# when the gate never read it, and the fourth leaves a red against the
# wrong file (see its planter). `lib.sh` states the rule for the file
# set; these are it for the rosters this gate derives.

plant_src_tree_gone() { rm -rf "$1/$SRC"; }

# lib.rs and bin/ are the two exclusions the clean fixture asserts. A
# tree holding ONLY those has no module to classify, and the scan set
# is empty for a reason no later check can see.
plant_only_lib_and_bin() {
  find "$1/$SRC" -type f -name '*.rs' ! -name lib.rs ! -path "$1/$SRC/bin/*" -delete
}

# Every module promotes itself out of the rule. Derived from the
# declaration rather than from a list of the fixture's files, so a
# vocabulary added to the fixture is converted too.
plant_every_module_is_a_driver() {
  local f
  while IFS= read -r f; do
    sed -i 's|^//! Module kind: \*\*vocabulary\*\*.*$|//! Module kind: **driver** (README, Module boundaries).|' "$f"
  done < <(grep -rlE "$KIND_LINE" "$1/$SRC" | sort)
}

# The driver roster reduced to the one driver that HOSTS a vocabulary,
# in the README and in the tree at once, so the derived forbidden-path
# set is empty and check 7's path arm has no alternate to match.
# `session` is that driver in the fixture as it is in the crate.
#
# WHAT THIS CASE HOLDS is the guard and not a silence: with the guard
# backed out the empty alternation reds on `camera.rs`'s
# `use crate::session::SessionOp;`, an import that breaks no rule. The
# case says the gate names the roster that came out empty rather than
# the first file the degenerate pattern happens to hit.
plant_every_driver_hosts_a_vocabulary() {
  local root=$1 d m
  for d in "${FIXTURE_DRIVERS[@]}"; do
    if [ "$d" = session.rs ]; then continue; fi
    rm -f "$root/$SRC/$d"
    m=${d%.rs}; m=${m////::}
    grep -vxF "| \`$m\` | a driver |" "$root/$README" > "$root/$README.new"
    mv "$root/$README.new" "$root/$README"
  done
}

plant_undeclared_module() {
  printf '//! A new module with no kind.\npub struct Thing;\n' > "$1/$SRC/thing.rs"
}

plant_two_kinds() {
  cat > "$1/$SRC/thing.rs" <<'RS'
//! Two answers.
//!
//! Module kind: **vocabulary** — it names no driver type.
//! Module kind: **driver** (README, Module boundaries).
pub struct Thing;
RS
}

# ONE PLANTER FOR EVERY NEEDLE, driven by the derived sets rather than
# by a hand-written case list. `lib.sh` requires a fixture saying where
# a matcher stops; this is the other half — a fixture per alternate
# saying that it starts. Nine of thirteen alternates had no case behind
# them when this gate first landed, and deleting them from the pattern
# left `--selftest` green.
plant_named() {
  local text=$1 root=$2
  {
    printf '//! A vocabulary that names something it may not.\n//!\n'
    printf '//! Module kind: **vocabulary** — it names no driver type.\n'
    printf '%s\n' "$text"
  } > "$root/$SRC/thing.rs"
}

plant_type_use() { plant_named "use crate::session::$1; pub fn peek(_s: &$1) {}" "$2"; }
# NOT AN IMPORT: a fully-qualified name in a signature evades any check
# that reads only `use` lines, which is what the README's slogan sells
# the rule on.
plant_type_inline() { plant_named "pub fn peek(_s: &crate::session::$1) {}" "$2"; }
plant_crate_use() { plant_named "use $1::Thing; pub fn go(_t: Thing) {}" "$2"; }

# THE PATH ARMS, ONE ISOLATING FIXTURE EACH. A fixture that trips two
# arms proves neither: the first version of this gate had four path
# plants and every one of them was caught by a second arm, so deleting
# either the segment arm or the whole use-tree scan left `--selftest`
# green. Each planter below names its driver in exactly ONE of the
# three spellings, so deleting that arm turns this file red.
#
#   bare      `use crate::app as chrome;`  — `crate::app`, and the
#             alias means no `app::` anywhere else in the file.
#   segment   `self::app::run()`           — `app::`, with no `crate::`
#             prefix and no brace group.
#   use tree  `use crate::{app, …};`       — the driver as a bare leaf
#             inside braces, which neither of the other two sees. The
#             import stands alone: naming it in a body would trip the
#             segment arm and hide a broken tree scan.
plant_path_bare_aliased() {
  plant_named "use crate::$1 as chrome; pub fn go() { chrome::run() }" "$2"
}
plant_path_segment_via_self() {
  plant_named "pub fn go() { self::$1::run() }" "$2"
}
# rustfmt wraps a long tree, so the wrapped form is what the window
# scan actually has to read.
plant_path_use_tree() {
  plant_named "use crate::{
    $1,
    camera::Camera,
};
pub fn go(_c: Camera) {}" "$2"
}
plant_path_use_tree_oneline() {
  plant_named "use crate::{$1, camera::Camera}; pub fn go(_c: Camera) {}" "$2"
}
# The realistic spelling, kept because it is what a real edit looks
# like even though it trips two arms at once.
plant_path_child() { plant_named "use crate::$1::helper; pub fn go() { helper() }" "$2"; }

# A TEST MODULE IS IN SCOPE. The README's definition is that a
# vocabulary can be read AND TESTED without a session existing.
plant_test_module_names_driver() {
  plant_named "pub fn go() {}
#[cfg(test)]
mod tests {
    use crate::session::DocSession;
    #[test]
    fn t() { let _ = DocSession; }
}" "$1"
}

plant_self_promoted_driver() {
  cat > "$1/$SRC/thing.rs" <<'RS'
//! A module that promoted itself out of the rule.
//!
//! Module kind: **driver** (README, Module boundaries).
use eframe::egui;
pub fn draw(_ui: &mut egui::Ui) {}
RS
}

plant_driver_row_is_a_ghost() {
  sed -i 's#^| `app` | a driver |$#&\n| `retired` | a driver |#' "$1/$README"
}

plant_driver_demoted() {
  cat > "$1/$SRC/app.rs" <<'RS'
//! Demoted in its own header while the README still calls it a driver.
//!
//! Module kind: **vocabulary** — it names no driver type.
pub fn go() {}
RS
}

plant_driver_table_renamed() {
  sed -i "s|^### The drivers\$|### Drivers|" "$1/$README"
}

plant_vocab_table_renamed() {
  sed -i "s|^### The session's vocabularies\$|### Session vocabularies|" "$1/$README"
}

plant_vocab_row_is_a_ghost() {
  sed -i 's#^| `session::select` | what is selected |$#&\n| `session::retired` | gone |#' \
    "$1/$README"
}

plant_readme_calls_a_driver_a_vocabulary() {
  sed -i 's#^| `forms` | what the panels offer |$#| `widgets` | drawn helpers |#' "$1/$README"
}

# THE EXCEPTION'S THREE FAILURES, one per part of the entry.
plant_exception_gains_a_site() {
  local spec=${VOCAB_EXCEPTIONS[0]}
  local exfile=${spec%%|*} exneedle=${spec#*|}
  exneedle=${exneedle%|*}
  printf 'pub fn brand_new_session_read(_s: &%s) {}\n' "$exneedle" >> "$1/$SRC/$exfile"
}

plant_exception_loses_a_site() {
  local spec=${VOCAB_EXCEPTIONS[0]}
  local exfile=${spec%%|*} exneedle=${spec#*|}
  exneedle=${exneedle%|*}
  grep -v "$exneedle" "$1/$SRC/$exfile" > "$1/$SRC/$exfile.new"
  mv "$1/$SRC/$exfile.new" "$1/$SRC/$exfile"
}

# THE HOLE THE FILE-GRANULAR VERSION LEFT: an exempted file loses no
# `DocSession` and gains an entirely different forbidden name. Under a
# union-matched, file-granular exemption this was green.
plant_exception_file_gains_another_needle() {
  local spec=${VOCAB_EXCEPTIONS[0]}
  local exfile=${spec%%|*}
  printf 'use eframe::egui;\nuse crate::app::ViewerApp;\n' >> "$1/$SRC/$exfile"
}

# AN ENTRY NAMING A DRIVER. The header mark is what makes this case the
# check-8 guard's and not check 2's: without it the module is on the
# list and denies it, which a different arm already covers.
plant_exception_names_a_driver() {
  sed -i 's|^//! Module kind: \*\*driver\*\* (README, Module boundaries).$|//! Module kind: **driver**, with a recorded exception.|' \
    "$2/$SRC/$1"
}

plant_exception_header_denies_it() {
  local spec=${VOCAB_EXCEPTIONS[0]}
  local exfile=${spec%%|*}
  sed -i 's|^//! Module kind: \*\*vocabulary\*\*, with a recorded exception.$|//! Module kind: **vocabulary** — it names no driver type.|' \
    "$1/$SRC/$exfile"
}

# The zero-hit control's planter: the clean fixture, unaltered.
plant_nothing() { :; }

# THE ENTRY IS THE PLANT. The tree is the clean fixture untouched — the
# exception list reaches the gate through the environment and names a
# file the fixture never writes, which is the one way a `--root` tree
# can carry an entry pointing outside itself.
plant_entry_names_no_file() { :; }

plant_unexempted_module_claims_an_exception() {
  cat > "$1/$SRC/thing.rs" <<'RS'
//! A module writing itself a permission.
//!
//! Module kind: **vocabulary**, with a recorded exception for the
//! session read below.
pub fn go() {}
RS
}

# The cross-reference two diagnoses carry, deleted from the README the
# way #1883 deleted the section they used to name.
plant_readme_drops_the_cited_section() {
  grep -vxF "$CITED_SECTION" "$1/$README" > "$1/$README.new"
  mv "$1/$README.new" "$1/$README"
}

plant_readme_drops_a_type_name() {
  sed -i 's|no `ViewerApp` and ||' "$1/$README"
}

plant_manifest_app_feature_renamed() {
  sed -i 's|^app = \[|application = [|' "$1/$MANIFEST"
}

# --- THE README'S TABLE, AS A TABLE ----------------------------------
#
# Each of these is a way the roster can be read off something that is
# not the table a renderer draws. The old scan collected every `^|`
# line between the heading and the next heading and dropped whatever
# its `sed` could not parse, so all six were silent: four of them
# GREEN over a roster shorter than the page, and two red with a
# message about the wrong thing.

# A FENCED BLOCK OPENING INSIDE THE TABLE BODY. Markdown ends the table
# there and so does this reader; what the reader now does is say so.
plant_a_fence_interrupts_the_table() {
  sed -i 's%^| `app` | a driver |$%&\n```\nan example, mid-table\n```%' "$1/$README"
}

# THE SAME CUT WITH A BLANK LINE ABOVE IT, which is the shape a fence
# tracker alone does not catch: the table ends normally and the rows
# below it are simply not in the roster.
plant_a_row_below_a_blank_line() {
  sed -i 's%^| `gpu` | a driver |$%&\n\n| `retired` | a driver |%' "$1/$README"
}

# A SECOND TABLE UNDER ONE HEADING, the same defect with a header row
# of its own.
plant_a_second_table_in_the_section() {
  sed -i 's%^| `gpu` | a driver |$%&\n\n| Module | Is |\n|---|---|\n| `retired` | a driver |%' \
    "$1/$README"
}

plant_the_table_separator_gone() {
  sed -i '0,/^|---|---|$/{/^|---|---|$/d}' "$1/$README"
}

# THE COLUMNS REORDERED. This reader reads the FIRST column as the
# module, and a header that no longer says so is a reader parsing cells
# that mean something else.
plant_the_table_header_reordered() {
  sed -i 's%^| Module | Is |$%| Is | Module |%' "$1/$README"
}

# A ROW WHOSE MODULE CELL IS NOT A BACKTICKED MODULE. Dropped in
# silence before, so the roster came back one shorter than the table.
plant_a_row_the_reader_cannot_parse() {
  sed -i 's%^| `gpu` | a driver |$%&\n| the retired one | a driver |%' "$1/$README"
}

# A HEADER AND A SEPARATOR AND NOTHING UNDER THEM. A table a renderer
# draws with no rows in it, which is not the same answer as no table.
plant_a_table_with_no_rows() {
  sed -i '/^### The drivers$/,/^### /{/^| `/d}' "$1/$README"
}

plant_the_table_gone_but_the_heading_stays() {
  sed -i '/^### The drivers$/,/^### /{/^|/d}' "$1/$README"
}

plant_the_driver_heading_twice() {
  sed -i 's%^### The drivers$%&\n\nProse.\n\n### The drivers%' "$1/$README"
}

# THE INDENT BOUNDARY, both sides, and the silent half is the FIRST of
# them. A row indented one to three spaces is a row every renderer
# draws, and the reader that anchored at column zero saw none of them —
# so this case plants a GHOST module there: if the row is read the gate
# reds naming a module the tree does not hold, and if it is not read the
# gate is green over a roster one shorter than the table. Nothing else
# can observe the difference, because every other figure the gate prints
# is derived from the tree.
plant_an_indented_ghost_row() {
  sed -i 's%^| `session::select` | what is selected |$%&\n  | `session::retired` | gone |%' \
    "$1/$README"
}

# FOUR SPACES IS A CODE BLOCK, so this row is correctly NOT read — and
# being right about it is not the same as being loud about it, which is
# what `!indent` is for. It plants a REAL module's row rather than a
# ghost, so a reader widened to `[[:space:]]*` would read it and go
# green: this case is what keeps that widening out.
plant_a_four_space_row() {
  sed -i 's%^| `drafts` | in-flight form state |$%    &%' "$1/$README"
}

# A LEADING TAB ADVANCES TO COLUMN FOUR, so it is the same answer by a
# different spelling and the `{0,3}` strip must not touch it.
plant_a_tab_indented_row() {
  sed -i 's%^| `drafts` | in-flight form state |$%\t&%' "$1/$README"
}

# A HEADING TAKES THE SAME THREE SPACES, and the strip is what makes
# this green rather than "carries no heading" about a heading that is
# there and indented.
pass_a_three_space_indented_heading() {
  sed -i 's%^### The drivers$%   &%' "$1/$README"
}

# --- WHAT MARKDOWN DRAWS AS CODE -------------------------------------
#
# Every one of these is content a renderer draws inside a code block,
# and each was structure to the old scan: a `#` ended the section and a
# `|` joined the roster.
pass_a_fenced_hash_above_the_table() {
  sed -i 's%^### The drivers$%&\n\nProse, with an example:\n\n```rust\n#[derive(Debug)]\nstruct Example;\n```%' \
    "$1/$README"
}

pass_a_fenced_table_row_below_the_table() {
  sed -i 's%^| `drafts` | in-flight form state |$%&\n\nA worked example of a row, which is not a row:\n\n```\n| `ghost` | not a row |\n```%' \
    "$1/$README"
}

# A BACKTICK LINE INSIDE A TILDE FENCE DOES NOT CLOSE IT, which is what
# a bare toggle gets wrong: the toggle hands the rest of the block back
# to the heading and row rules, and the `#` and the `|` below become
# structure again.
pass_a_tilde_fence_holding_a_backtick_line() {
  sed -i 's%^### The drivers$%&\n\nProse, with an example:\n\n~~~\n# not a heading\n```\n| `ghost` | not a row |\n~~~%' \
    "$1/$README"
}

# A FENCE THAT OPENS ALSO CLOSES, which is the direction a repair can
# be wrong in silently. The decoy row is under a LATER heading, so it
# is read only if the fence above swallowed the heading that ends this
# section — and a fence that never closes hides the real tables too.
pass_a_fence_closes_so_the_section_still_ends() {
  sed -i 's%^### The drivers$%&\n\n```sh\n# a comment\n```%' "$1/$README"
  printf '\n| `ghost` | a driver |\n' >> "$1/$README"
}

# GREEN cases, asserted rather than assumed.
plant_prose_and_literals() {
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

plant_imports_a_session_vocabulary() {
  plant_named "use crate::session::{OpOutcome, SessionOp};
pub fn go(_o: OpOutcome, _s: SessionOp) {}" "$1"
}

# A NESTED USE TREE THAT NAMES NO DRIVER. The tree arm must stop here,
# or every wrapped `use crate::{…}` in the crate reds.
plant_innocent_use_tree() {
  plant_named "use crate::{
    camera::Camera,
    session::SessionOp,
};
pub fn go(_c: Camera, _s: SessionOp) {}" "$1"
}

# `pollster` is a DEFAULT-feature dependency (Cargo.toml), not an
# `app`-only one, so a vocabulary may name it. The derived set says so;
# a curated "toolkit" list would have had to remember.
plant_names_a_default_feature_dependency() {
  plant_named "use pollster::block_on; pub fn go() { let _ = block_on; }" "$1"
}

# THE TRACKER'S OWN LOAD, AS A DIRECT CALL, and it is here because no
# fixture can be: every case below plants a TREE and runs this file
# over it, while `viewer-readme-fence.awk` sits beside this file and outside
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

gate_selftest() {
  local want t c m spec exfile
  gate_selftest_clean
  gate_selftest_without_tool grep "it is grep saying it could not search"

  # NOTHING TO DECIDE OVER IS NOT A PASS, four times.
  # `plant_src_tree_gone` empties the subject, `plant_only_lib_and_bin`
  # the module roster, `plant_every_module_is_a_driver` the vocabulary
  # set and `plant_every_driver_hosts_a_vocabulary` the forbidden-path
  # set; each case stands beside the arm that fills the set it empties.
  # An empty set matches nothing, so the gate would otherwise print OK
  # having read no module or no vocabulary — or red against a file that
  # broke nothing, which is the fourth.
  gate_selftest_case "the gate's subject is gone, so it scanned nothing" plant_src_tree_gone
  gate_selftest_case "besides lib.rs and bin/ — the gate scanned nothing" plant_only_lib_and_bin

  gate_selftest_case "declares no module kind" plant_undeclared_module
  gate_selftest_case "declares 2 module kinds" plant_two_kinds
  gate_selftest_case "every module declares itself a driver" plant_every_module_is_a_driver

  # NEEDLE COVERAGE, derived from the same sets the matcher is built
  # from, so a name added to the `app` feature or to the driver table
  # gets a fixture without anyone remembering to write one.
  want="names a driver type, a driver module, or a crate that only exists behind"
  for t in "${FORBIDDEN_TYPE_NAMES[@]}"; do
    gate_selftest_case "$want" plant_type_use "$t"
    gate_selftest_case "$want" plant_type_inline "$t"
  done
  for c in $(app_only_crates); do
    gate_selftest_case "$want" plant_crate_use "$c"
  done
  for m in app gpu pane widgets; do
    gate_selftest_case "$want" plant_path_bare_aliased "$m"
    gate_selftest_case "$want" plant_path_segment_via_self "$m"
    gate_selftest_case "$want" plant_path_use_tree "$m"
    gate_selftest_case "$want" plant_path_use_tree_oneline "$m"
    gate_selftest_case "$want" plant_path_child "$m"
  done
  gate_selftest_case "$want" plant_test_module_names_driver

  gate_selftest_case "declares itself a DRIVER and" plant_self_promoted_driver
  gate_selftest_case "does not exist — the table outran the code" plant_driver_row_is_a_ghost
  gate_selftest_case "does not declare" plant_driver_demoted
  gate_selftest_case "carries no \"$DRIVER_TABLE\" heading" plant_driver_table_renamed
  gate_selftest_case "carries no \"${VOCAB_TABLES[0]}\" heading" plant_vocab_table_renamed
  gate_selftest_case "is not a module in the tree" plant_vocab_row_is_a_ghost
  gate_selftest_case "declares itself a DRIVER — the README and the module disagree" \
    plant_readme_calls_a_driver_a_vocabulary
  gate_selftest_case "so no driver module path is forbidden" \
    plant_every_driver_hosts_a_vocabulary

  # THE EXCEPTION ARMS PLANT THEIR OWN ENTRY. They used to read
  # `VOCAB_EXCEPTIONS[0]` — whatever the tree was currently wrong about
  # — so when #1883's hoist retired the last entry they lost their
  # subject and would have stopped running. Borrowing a live defect was
  # the defect: the coverage of the exemption machinery depended on the
  # tree still needing an exemption. They now supply one, and are
  # exercised whether or not the tree carries any.
  local -a live_exceptions=(${VOCAB_EXCEPTIONS[@]+"${VOCAB_EXCEPTIONS[@]}"})
  export GATE_SELFTEST_VOCAB_EXCEPTIONS='forms.rs|DocSession|2'
  VOCAB_EXCEPTIONS=('forms.rs|DocSession|2')
  spec=${VOCAB_EXCEPTIONS[0]}; exfile=${spec%%|*}
  gate_selftest_case "and its recorded exception covers" plant_exception_gains_a_site
  gate_selftest_case "has outlived part of its reason" plant_exception_loses_a_site
  gate_selftest_case "$want" plant_exception_file_gains_another_needle
  gate_selftest_case "its doc header does not say so" plant_exception_header_denies_it

  # A COLON-CARRYING PATH, WHOSE TWO SITES ARE TWO SITES. `a:b.rs` is a
  # legal path here and in git, and the union's dedupe key used to read
  # the FILE column as everything before the first colon: both records
  # then shared one key, all but the first were dropped, and the count
  # read 1 against an entry pinning 2 — so the entry read as having
  # outlived part of its reason while the site it no longer covered was
  # sitting in the file. The ENTRY is the whole fixture: the clean
  # fixture plants an exempted file from its own entry, at its own path,
  # with exactly the recorded count of sites in it. The second case is
  # the same path in the other direction, and it is what says the count
  # is still a count rather than a constant: a THIRD site must red as a
  # site the exception does not cover, which is a different diagnosis
  # from the one a collapsed key produces.
  export GATE_SELFTEST_VOCAB_EXCEPTIONS='a:b.rs|DocSession|2'
  VOCAB_EXCEPTIONS=('a:b.rs|DocSession|2')
  gate_selftest_passes "two sites in a path that carries a colon, counted as two" plant_nothing
  gate_selftest_case "and its recorded exception covers" plant_exception_gains_a_site
  # This one needs no entry: it is a module writing ITSELF a permission,
  # which an empty list must still refuse.
  export GATE_SELFTEST_VOCAB_EXCEPTIONS=''
  VOCAB_EXCEPTIONS=()
  gate_selftest_case "this gate grants it none" plant_unexempted_module_claims_an_exception

  # THE LIST'S OWN TWO BOUNDS. An entry naming a path the tree does not
  # hold, or a module that is not a vocabulary, exempts nothing and
  # hides its module from every check here — and an exception list
  # bounded by a check that has never been shown to fire is bounded by
  # nothing, which is this gate's own thesis turned on itself. Both
  # cases plant the ENTRY rather than a file: it reaches the gate
  # through the environment while `gate_plant_clean` plants from the
  # in-process list, which is empty, so the entry can name a file the
  # fixture never writes.
  export GATE_SELFTEST_VOCAB_EXCEPTIONS='ghost.rs|DocSession|1'
  gate_selftest_case "which is not a file under" plant_entry_names_no_file
  # `gpu.rs` is one of FIXTURE_DRIVERS, so the tree writes it as a
  # driver; the planter gives its header the mark check 2 wants, which
  # is what leaves this case to the entry's own guard.
  export GATE_SELFTEST_VOCAB_EXCEPTIONS='gpu.rs|DocSession|1'
  gate_selftest_case "exempts nothing and hides the module" \
    plant_exception_names_a_driver gpu.rs
  export GATE_SELFTEST_VOCAB_EXCEPTIONS=''

  # THE ZERO-HIT CONTROL, and it is a control rather than an accident.
  # With no entry the clean fixture plants no exempted file, so nothing
  # in the tree names anything forbidden and every filter in the union
  # pipeline matches NOTHING. That is the path a plain `grep` turned
  # into exit 1 with no diagnosis, and the path a `|| true` would green
  # over even when the matcher had died. It is asserted here with the
  # list forced empty, so it stays exercised the day an entry comes
  # back — which is exactly what stopped being true of the four arms
  # above.
  gate_selftest_passes "a tree with nothing forbidden in it at all" plant_nothing
  unset GATE_SELFTEST_VOCAB_EXCEPTIONS
  VOCAB_EXCEPTIONS=(${live_exceptions[@]+"${live_exceptions[@]}"})

  gate_selftest_case "no longer carries that heading" plant_readme_drops_the_cited_section
  gate_selftest_case "no longer names" plant_readme_drops_a_type_name
  gate_selftest_case "yielded no \`dep:\` entries" plant_manifest_app_feature_renamed

  # THE ROSTER IS THE TABLE A RENDERER DRAWS, and every row here was
  # silent before: four of them GREEN over a roster shorter than the
  # page, and two red about something other than the cut.
  gate_selftest_case "is INTERRUPTED by a fenced code block" \
    plant_a_fence_interrupts_the_table
  gate_selftest_case "table line(s) BELOW the table this gate reads" \
    plant_a_row_below_a_blank_line
  gate_selftest_case "table line(s) BELOW the table this gate reads" \
    plant_a_second_table_in_the_section
  gate_selftest_case 'has no `|---|---|` row under its header' \
    plant_the_table_separator_gone
  gate_selftest_case "does not open with a header row whose first column is" \
    plant_the_table_header_reordered
  gate_selftest_case "cannot read as a row of" plant_a_row_the_reader_cannot_parse
  gate_selftest_case "heading is there and no table follows it" \
    plant_the_table_gone_but_the_heading_stays
  gate_selftest_case "has a header and a separator and no rows under them" \
    plant_a_table_with_no_rows
  gate_selftest_case "and this gate would read the table under whichever" \
    plant_the_driver_heading_twice

  # THE INDENT BOUNDARY, both sides. The first was a live silent
  # shortening in this reader's own first version; the other three keep
  # the boundary from being widened to `[[:space:]]*`.
  gate_selftest_case "is not a module in the tree" plant_an_indented_ghost_row
  gate_selftest_case "indented FOUR or more spaces" plant_a_four_space_row
  gate_selftest_case "indented FOUR or more spaces" plant_a_tab_indented_row
  # A DEAD READER IS NOT AN EMPTY DOCUMENT — the population and the rule
  # that produces it are stated once, above `reader_failed`, and not
  # restated here. Each row below kills ONE stage and wants that stage
  # by name. Killing a RIGHT-HAND stage needs a shim that CONSUMES its
  # input and then exits: a stub that dies at once takes the upstream
  # stage down with SIGPIPE, and the diagnosis then names the wrong
  # reader — which is the defect these guards exist to remove, so a
  # case that got it wrong would pass for the wrong reason. Each shim
  # keys on a fragment of its own stage's program text, except the two
  # `sort` stages, which are told apart by what they are READING: three
  # sorts run here and only one of them carries `-u`.
  gate_selftest_without_tool find "the module enumerator over"
  gate_selftest_without_tool awk "the table reader over"
  gate_selftest_without_tool tr "the dep speller over"
  gate_selftest_with_broken_tool sed "the module path trimmer over" \
    'case "$*" in *"s#^"*) cat > /dev/null; exit 9 ;; esac
exec "$GATE_REAL_TOOL" "$@"'
  gate_selftest_with_broken_tool sort "the module sorter over" \
    'f=$(dirname "$0")/stdin
cat > "$f"
case "$*" in *-u*) exec "$GATE_REAL_TOOL" "$@" < "$f" ;; esac
if grep -q "[.]rs:" "$f"; then exec "$GATE_REAL_TOOL" "$@" < "$f"; fi
exit 9'
  gate_selftest_with_broken_tool sed "the table row reader over" \
    'case "$*" in *"!row"*) cat > /dev/null; exit 9 ;; esac
exec "$GATE_REAL_TOOL" "$@"'
  gate_selftest_with_broken_tool sed "the kind extractor over" \
    'case "$*" in *vocabulary*) cat > /dev/null; exit 9 ;; esac
exec "$GATE_REAL_TOOL" "$@"'
  gate_selftest_with_broken_tool awk "the manifest feature reader over" \
    'case "$*" in *"app[[:space:]]"*) exit 9 ;; esac
exec "$GATE_REAL_TOOL" "$@"'
  gate_selftest_with_broken_tool sed "the dep extractor over" \
    'case "$*" in *"dep:"*) cat > /dev/null; exit 9 ;; esac
exec "$GATE_REAL_TOOL" "$@"'
  gate_selftest_with_broken_tool sort "the dep sorter over" \
    'case "$*" in *-u*) cat > /dev/null; exit 9 ;; esac
exec "$GATE_REAL_TOOL" "$@"'
  gate_selftest_with_broken_tool awk "the hit deduplicator over" \
    'case "$*" in *gate_record_split*) cat > /dev/null; exit 9 ;; esac
exec "$GATE_REAL_TOOL" "$@"' plant_type_use DocSession
  gate_selftest_with_broken_tool sort "the hit sorter over" \
    'f=$(dirname "$0")/stdin
cat > "$f"
if grep -q "[.]rs:" "$f"; then exit 9; fi
exec "$GATE_REAL_TOOL" "$@" < "$f"' plant_type_use DocSession

  # THE TRACKER'S OWN LOAD, in both answers. No planted TREE can
  # express a tracker that is gone — it is this gate's own code and
  # lives outside every fixture root — so the predicate the load is
  # made of is called directly.
  selftest_fence_load 0 "$VIEWER_FENCE_AWK"
  selftest_fence_load 1 "$VIEWER_FENCE_AWK.no-such-file"

  gate_selftest_passes "prose and string-literal mentions" plant_prose_and_literals
  gate_selftest_passes "an import of a vocabulary under session::" \
    plant_imports_a_session_vocabulary
  gate_selftest_passes "a nested use tree naming no driver" plant_innocent_use_tree
  gate_selftest_passes "a default-feature dependency (pollster)" \
    plant_names_a_default_feature_dependency
  # WHAT MARKDOWN DRAWS AS CODE, in every direction the roster can be
  # read wrong by one. The first two are the reported defect, one per
  # sentinel; the last two are the ways a repair could be wrong and
  # quiet.
  gate_selftest_passes "a fenced Rust attribute at column zero above the table" \
    pass_a_fenced_hash_above_the_table
  gate_selftest_passes "a worked table row written inside a fence" \
    pass_a_fenced_table_row_below_the_table
  gate_selftest_passes "a backtick line inside a tilde fence, which does not close it" \
    pass_a_tilde_fence_holding_a_backtick_line
  gate_selftest_passes "a closed fence, after which the section still ends" \
    pass_a_fence_closes_so_the_section_still_ends
  gate_selftest_passes "a section heading indented three spaces" \
    pass_a_three_space_indented_heading

  printf '%s selftest OK: every forbidden name has its own fixture, and the fixture LIST is derived from the same two documents the matcher is — one case per driver type, one per `dep:` in %s'"'"'s `app` feature, and five per driver module path — an ISOLATING fixture for each of the three spellings the matcher has (aliased bare import, `self::`-qualified segment, wrapped use tree, one-line use tree) plus the realistic child path that trips two arms at once, so deleting any one arm turns this self-test red. The clean fixture proves lib.rs and bin/ are excluded on purpose, and four cases leave the gate nothing to decide over — the src tree gone, a tree holding only lib.rs and bin/, every module declaring itself a driver, and a driver roster whose every entry hosts a vocabulary — each of which the gate REFUSES rather than reporting green over an empty set. The exception list is EMPTY since #1883 hoisted the last two reads, so every arm that needs an entry to aim at supplies its own: four over an entry the fixture honours (a SIXTH site, a lost site, a different forbidden name in the same file, a header that denies the exception), two more over an entry whose path CARRIES A COLON — its two sites counted as two, and a third one red — and two over one it deliberately does not — an entry naming a file outside the tree and one naming a module the tree writes as a driver, which are the two bounds on the list itself. A seventh, a module writing ITSELF a permission, needs no entry — and every driver-name case above is a vocabulary naming a driver with no exemption in force at all. The README arms fire on a ghost driver row, a demoted driver, either table heading renamed, a ghost vocabulary row, a driver listed as a vocabulary, and the rule text losing a type name; the manifest arm fires when the `app` feature can no longer be read. Prose, string literals, an import under `session::`, an innocent nested use tree and a default-feature dependency stay green; and the gate stays RED, with a diagnosis, when `grep` itself cannot run. The README'"'"'s table is read as the table a RENDERER draws — the first contiguous run of `|` lines under the heading, its header and separator asserted by position — and eight rows fire on the ways that can be false: a fenced block opening inside the table body, a row cut off from it by a blank line, a second table under one heading, the separator gone, the columns reordered, a row whose module cell is not a backticked module, a heading with no table under it, and one heading written twice. Four more stay QUIET where markdown draws code rather than structure: a column-zero `#[derive]` and a worked table row inside a fence, a backtick line inside a tilde fence that does not close it, and a fence whose close still lets the section end. Twelve rows kill ONE reader stage each and want that stage by name — outright, mid-scan, and after consuming its input — and two direct rows hold the shared fence tracker'"'"'s load in both of its answers\n' \
    "$(gate_name)" "$MANIFEST"
}

gate_parse_args "$@"
vocab_exceptions_from_env
gate_main
