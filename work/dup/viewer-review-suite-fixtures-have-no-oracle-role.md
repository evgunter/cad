---
id: viewer-review-suite-fixtures-have-no-oracle-role
kind: issue
title: Three viewer review suites keep fixtures no oracle reads; a mutation measured which
status: closed
opened: 2026-09-19
closed: 2026-09-20
branch: dup/viewer-fixture-oracles
pr: 2900
---


## Finding

- **Where**: `crates/viewer/tests/review_gui2_r1.rs`,
  `review_gui2_r2.rs`, `review_gui3_r1.rs` and `review_gui3_r2.rs`.
  PR #2886 folded what it could prove live — `xy_frame`, `len`, `scl`,
  `tempdir`, `GALLERY_RING` and the three `gallery_ring_at`
  re-spellings. **What is left** is the sugar whose fold is a
  judgement rather than a substitution: private `insert`/`inserted`
  wrappers in all four, `length`/`scalar` in `review_gui2_r2`, and
  `rectangle`/`rect`/`triangle` against `common::square`.
- **Importance**: medium. The blanket ground that used to protect them
  ("a promoted review suite's value is that it is an INDEPENDENT
  derivation") was withdrawn by Ev on 2026-09-04 and was removed from
  these three headers by PR carrying
  `viewer-review-suites-cite-the-withdrawn-independence-reading`. What
  replaced it is the surviving per-row test, and applying that test is
  what this row is.
- **Confidence**: sure about the duplicates; the oracle question is
  **measured for one file and open for the other two**.
- **Raised by**: the S-DUP lane for the citation census, 2026-09-19.

## The measurement, and what it decided

At merge base `5b4979ef2`, with `common::xy_frame` folded into all
three suites, two planted mutations in the shared helper:

| mutation | binary | `gui2_r1` | `gui2_r2` | `gui3_r1` |
| --- | --- | --- | --- | --- |
| none | 626 pass / 0 fail | — | — | — |
| `common::xy_frame` v axis y → z | 581 / 45 | **3 red** | **9 red** | **0 red** |
| `common::scl(v)` → `v + 1.0` | 561 / 65 | 1 red | 0 red | 1 red |

So the frame datum is live for the two GUI-2 suites and **inert for
`review_gui3_r1`**: none of its rows can see the world frame move.
That is consistent with what it asserts — history structure, refusals,
landing generations, file bytes — and it is the evidence for the
sentence now in that file's header saying its triangle is a
readability choice, not an independence claim.

## What is left to decide, per file

The surviving clause's test is: name the constant or helper the row
would otherwise read, and say whether a bug in it would be invisible
to the row if the row read it.

- **`review_gui3_r1`** — answered: no oracle reads the fixture's shape,
  so nothing here needs its own derivation. `common::framed_square`,
  `common::inserted` and `common::tempdir` are the candidates, and the
  triangle and `r1_depth` are worth keeping only if a reader values
  telling R1's document apart in the aggregated binary's output.
- **`review_gui2_r1` and `review_gui2_r2`** — their geometric oracles
  (cursor position → resolved face) genuinely need their own
  derivation, but **not for the reason first written here**: neither
  suite builds the plate, so `common`'s plate helpers were never a
  candidate. Both call `Camera::framing` directly, and what keeps them
  honest is that their expectations are hand-derived from their own
  fixtures' coordinates. The sugar that carries no oracle is not
  covered by that and is sharing-eligible. Each remaining one is a
  separate judgement, which is why this is a row and not more edits in
  the census PR.
- **`review_gui3_r2`** — the fifth carrier, found by the class-shaped
  ground sweep rather than the citation census. Its rows assert on
  panels and history; its `rect` fixture is a spelling. Same
  disposition as `review_gui3_r1`.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner to
file it with, and the subject — one fixture spelled four times, freed
by a withdrawn rule — is S-DUP's charter. Any of the five may claim it
by `git mv`.

## Closed 2026-09-20 — the census re-taken, the per-file test run, what folded

**The row's own count was wrong, and item 1 is why it is re-taken
rather than quoted.** The row said PR #2886 had already folded `len`
and `scl`. It folded them in `review_gui3_r1` and `review_gui3_r2`;
`review_gui2_r1` still carried a private `len`/`scl` pair, spelled
path-qualified across two lines, which is why the census that wrote
that sentence missed it. Re-taken at `b29fe8bd1` over every tracked
file with no path argument, by whole-function scan rather than by name.

### The class, re-measured

| construction | home | spellings before |
| --- | --- | --- |
| `apply` + `InsertNode` + take the minted id | `common::inserted` | 4 (`gui2_r1::inserted`, `gui2_r2::insert`, `gui3_r1::insert`, `gui3_r2::push`) |
| `apply` one edit and keep the document | `common::edited` | 2 (`gui3_r1::applied`, and `gui3_r2::slab` writing it inline) |
| an `Expr` literal at a dimension | `common::{len, scl, ang}` | 9 private wrappers in 5 files, plus 6 inline sites in the files folded |
| an axis-aligned rectangle profile | `common::rectangle` (minted here) | 4 (`common::square`, `gui2_r2::rectangle`, `gui3_r2::rect`, `gui2_r1::offset_square`) + 1 inline in `common/asm.rs` |

`common::rectangle(plane, origin, w, h)` is the new home: `square` is it
with two equal sides at the origin, and `offset_square` was it with an
origin. Three parameterisations of one polygon, now one.

### The per-file test, with the helper each row would otherwise read

The surviving clause: *name the helper the row would otherwise read,
and say whether a bug in it would be invisible to the row if the row
read it.* Every verdict below was checked against what the file
actually reaches — the parent unit's first pass justified two suites
against helpers they never touch.

| file | the helper it would read | invisible? | disposition |
| --- | --- | --- | --- |
| `review_gui2_r1` | `common::{inserted, len, scl, rectangle}` | no — its oracles are cursor positions hand-derived from the blocks' own coordinates, and those coordinates stay at the call site | shares |
| `review_gui2_r2` | same | no, same reason | shares |
| `review_gui3_r1` | `common::{inserted, edited, ang}` | no — its oracles are history structure, refusals, generations and file bytes | shares |
| `review_gui3_r2` | `common::{inserted, edited, ang, rectangle}` | no — its oracles are panel models and history | shares |
| `review_gui3_r1`'s `triangle` and `r1_depth` | — | **no second spelling exists** | **kept**: nothing else in the crate builds a triangle, so there is no duplicate to fold. Sharing here would mean deleting a readability distinction, which is not this program's charter |
| `review_gui2_r1::delta` (1.5e-4), `review_gui2_r2::delta` (3.0e-4) and `coarse` (2.0e-3) | — | — | **kept**, and the three are not one kind of thing: the two `delta`s are per-suite display tolerances chosen for each suite's own geometry, while `coarse` is `gui2_r2`'s SECOND constant and a COST choice — its own doc says it exists so the gallery ring tessellates cheaply. Neither a drift to reconcile nor one value spelled three times. The real duplication is a fourth value outside these files, filed as `viewer-tests-spell-one-display-tolerance-in-seven-places` |
| `review_gui2_r2::insert`, `tol` | `common::inserted` | — | **kept as adapters**: one line each, fixing this suite's tolerance. The construction is gone; what is left is a parameter binding |

`review_gui2_r2::rectangle` had exactly one caller and was inlined
rather than kept as an adapter, which is the same judgement read the
other way.

### The mutation table — both directions, at `ab086f8c1`

**Method item 16: the direction is argued before the result is read.**
Baseline **626 pass / 0 fail / 1 ignored**, `cargo test -p viewer --test
all`, default lane, own target dir. The harness restores the file's
pre-plant BYTES and then diffs against `HEAD`, per item 17; every run
below restored clean.

For the shape plants the two directions are not equivalent. **Shrinking
relaxes** every containment- and disjointness-shaped predicate these
suites use — a smaller block is further from its neighbour and further
inside its frame — so a row asserting "these two do not overlap" or
"this pick misses" is satisfied for free. **Growing tightens those and
relaxes the opposite family** ("this pick hits", "the frame contains
it"). Neither direction alone is a probe.

| planted in `tests/common/mod.rs` | direction | total | gui2_r1 | gui2_r2 | gui3_r1 | gui3_r2 | msolve5 | asm-fed suites |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `rectangle` both extents halved | **shrink** | 610 / 16 | 1 | 1 | 0 | **0** | 0 | assembly_display 1, mate_tool_flow 4, gui4_r1 2, gui4_r2 1, instance_authoring 1 |
| `rectangle` both extents doubled | **grow** | 621 / 5 | 0 | 0 | 0 | **0** | 0 | assembly_display 1, gui4_r1 1, gui4_r2 1 |
| `rectangle` height only doubled | **grow, one extent** | 623 / 3 | 0 | 0 | 0 | **0** | 0 | gui4_r1 1 |
| `len(m)` → `m + 1.0` | grow | 534 / 92 | 3 | 11 | 0 | 3 | 0 | assembly_display 2 |
| `len(m)` → `m * 0.5` | shrink | 575 / 51 | 1 | 4 | 0 | 2 | 0 | mate_tool_flow 7, gui4_r1 2, gui4_r2 2 |
| `scl(v)` → `v + 1.0` | — | 558 / 68 | 2 | 2 | 1 | 0 | 0 | assembly_display 1, gui4_r1 2, gui4_r2 7 |
| `ang(r)` → `r + 1.0` | — | 616 / 10 | 0 | 1 | 0 | 0 | 0 | — |
| `inserted` answers the document's FIRST node's id | — | 434 / 192 | 4 | 16 | 4 | 13 | **1** | assembly_display 16, gui4_r1 10, gui4_r2 11 |
| `edited` discards the edit | — | 358 / 268 | 4 | 19 | 9 | 16 | **1** | assembly_display 16, gui4_r1 10, gui4_r2 11 |
| `insert_into` does not write the document back | — | 545 / 81 | 0 | 0 | 0 | 0 | **1** | assembly_display 16, instance_authoring 14, gui4_r2 11, mate_tool_flow 10, gui4_r1 10, tree_badges 4, doc_io 1 |
| `edit_into` does not write the document back | — | 601 / 25 | 0 | 0 | 0 | 0 | 0 | mate_tool_flow 8, gui4_r2 7, assembly_display 3, gui4_r1 2, tree_badges 1 |

**What the second direction changed.** The two shape directions red
**different sets**. Shrinking reds `gui2_r1`, `gui2_r2`,
`mate_tool_flow` and `instance_authoring`; growing reds none of them.
Both red `profile_draw` (2) and `assembly_display` — and
**`assembly_display` is live on the folded polygon in both directions
and appeared in no row of the first table**, because the first table
was taken before `common/asm.rs`'s inline polygon and its insert
wrapper were folded. A one-extent grow is weaker again (3 red against
5): a symmetric plant cannot reach a row that only watches one axis,
and a one-axis plant cannot reach one that watches area.

**The columns are the wrong shape for what these plants measure, and
widening them again would not fix it.** The halved plant also reds
`combine_ops` (1) and `frame_policy` (2), both through `common::square`
— which `rectangle` now backs — and neither appears in a column or a
sentence above; `assembly_display` was the same omission one category
earlier. The columns were chosen for the five suites this row is ABOUT,
while a plant in a shared door reaches everything that door feeds, which
is most of the crate. Read the totals for whether a fold is live and the
per-suite figures only for the five named; the consumer set is wider
than any column list here will be.

**An instrument defect in the harness, named because it published a
wrong number once.** The first `insert_into` run printed *"0 pass / 1
fail"*: the plant makes `tree_badges` panic inside `common/mod.rs`, the
harness read the FIRST `test result:` line in the output, and a panicking
shard emits one of its own before the suite's. The true figure is **545
/ 81**, read off the last such line after a re-run. The per-suite counts
were right throughout; only the total was wrong — which is method item
18's shape, in the table whose job is to prove the folds live.

**A reader does not have to re-run anything to check for it.** Every row
of this table sums pass + fail to **626**, the baseline's row count, and
the defect violates that by construction: the figure it substitutes is
some other section's, so it sums to that section's total instead (the
artefact read 0 + 1 = 1). So the sum IS the complete test for this
defect, and it is cheaper than a re-run. The harness now reads the LAST
`test result:` line, which fixes it going forward; the sum is how any
table taken before that change gets confirmed.

### The zeros, answered by measurement rather than inference

- **`gui3_r1` on `rectangle`, `gui2_r1` on `ang`**: neither file
  reaches the helper. Zero by construction, not a coverage fact.
- **`gui3_r2` on `rectangle` — settled, and now by two directions.**
  It reads the helper and reds zero when the fixture is halved AND when
  it is doubled, in a binary where `inserted` reds 13 of its rows and
  `edited` 16. Two differently-directed shape perturbations leaving a
  demonstrably live suite green is *asserts nothing about that helper*,
  not *asserts nothing useful*. Closed here; it is not an S-TINT
  coverage finding.
- **`msolve5_read_below_a_root`** reds **1** under three plants
  (`inserted`, `edited`, `insert_into`). The first table recorded zero
  everywhere and said the fold there was not claimed as proved live;
  that was true of the literal doors and is no longer true of the
  insert door, which this fix pass routed it through. The literal
  folds in that file are still compiler-proved rather than plant-proved.

### What this unit deliberately did not do

- **`docm9_range_vs_probe.rs`'s `lit`/`scalar` AND its `insert`
  wrapper** are both members of classes folded here and both left: the
  file is `#![cfg(feature = "interval")]` and reaches `Expr`, `apply`
  and `ProfileDoc` through `editor_core` rather than the
  `pncad::document` façade, so a fold there is type-checked in one lane
  only. Both members named in
  `viewer-tests-bypass-the-shared-literal-doors`.
- The seven `crates/topo/src/review_m1_*` headers ending *"Promoted per
  Ev's request (PR #17 thread)."* — separately filed, waiting on Ev.
  (Measured seven: `review_m1_pr1.rs` and all six of
  `review_m1_pr2/*.rs`. A case-sensitive line-shaped grep returns six
  and misses `review_m1_pr2/mod.rs`, whose copy is lowercase,
  mid-sentence and wrapped between `Ev's` and `request`.)
- `a-viewer-error-arm-is-not-split-because-a-review-suite-pins-it` was
  not touched; nothing in this diff reaches that arm.

### Residue filed, one per class

- `viewer-tests-rebuild-the-pick-index-in-every-suite`
- `viewer-tests-each-spell-their-own-downward-pick-ray`
- `viewer-tests-bypass-the-shared-literal-doors`
- `suite-headers-instruct-on-ignored-rows-they-no-longer-have`
- `viewer-tests-spell-one-display-tolerance-in-seven-places`
- `the-rectangle-profile-is-still-written-longhand-beside-its-door`
- `three-doors-named-insert-mean-two-different-constructions`

### What the fix pass added, and the class it had left standing

The first pass counted the apply-plus-`InsertNode`-plus-take-the-id
class at **4** and folded 4. Re-censused by the structural needle
(`DocEdit::InsertNode` over every tracked file, no path argument) the
class in `crates/viewer/tests/` was **12 sites in 11 files** — eight
more, differing from `common::inserted` only by taking `&mut` and
assigning back, five of them carrying the same
`.expect("an insert mints an id")` string. **Two of the eight sat in
`common/asm.rs` and `msolve5_read_below_a_root.rs`, byte-identical to
each other, in the two files this unit had already opened to pull their
literal helpers and their inline polygon out.** Editing a file for one
member of a class and leaving another in it is the trap the program's
own log names; it was caught by the reviewer, not by the lane.

Folded: `common/asm.rs`, `msolve5_read_below_a_root.rs`, `doc_io.rs`,
and closures in `assembly_display.rs`, `mate_tool_flow.rs`,
`review_gui4_r1.rs`, `tree_badges.rs`, onto two new in-place doors
`common::insert_into` / `common::edit_into` (each three lines over
`inserted` / `edited`, no third construction). `asm.rs`'s `fn edit` was
the `edited` class's third spelling and went the same way. While
folding `review_gui4_r1`'s closure, two further copies of the same
construction were found written INLINE in the same file and folded too.

**And that was still not all of it.** A third pass over the same two
files found three longhand copies of `edit_into`'s construction —
`mate_tool_flow.rs` directly BELOW two `insert_into` calls this unit had
just written, and two in `review_gui4_r1.rs` SANDWICHED BETWEEN them —
all three `SetPlacement` edits, the same edit converted in
`common/asm.rs` in the same commit. **Three consecutive passes each left
a member of this class in a file the pass had open**, and the trap was
named in this row's own prose before the second and third happened. The
needle that closes it is the in-place half's own tell,
`git grep -E '= applied\.doc' -- crates/viewer/tests/`, which is the
converse of the `InsertNode` needle and finds what it cannot: a
re-spelling of the WRITE-BACK rather than of the edit.
Left: `docm9_range_vs_probe.rs`, for the interval-lane reason above.
**Re-taken after the last fold** (the figure first written here was
taken before it and was wrong): `git grep -n 'DocEdit::InsertNode' --
crates/viewer/tests/` returns **4 textual sites in 3 files**, and none
of them is a private re-spelling any more —
`common/mod.rs:125` is `inserted`'s own call, `common/mod.rs:400` is the
`matches!` pattern inside `common::insert` asserting the session
committed an insert (an assertion, not a construction),
`msolve3_placer_refused.rs:76` is a caller handing an `InsertNode` edit
to `common::edited`, and `docm9_range_vs_probe.rs:43` is the one member
deliberately left. **The class itself is down to that single member.**
The converse needle for the in-place half, `git grep -E '= applied\.doc'
-- crates/viewer/tests/`, returns the same one file and nothing else.

Not censused, and stated rather than implied: `crates/editor-core/tests/`
(~480 `InsertNode` sites) and `crates/pncad/tests/all.rs` were not
examined for this class. They are a different crate's home question.
