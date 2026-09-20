---
id: viewer-review-suite-fixtures-have-no-oracle-role
kind: issue
title: Three viewer review suites keep fixtures no oracle reads; a mutation measured which
status: closed
opened: 2026-09-19
closed: 2026-09-20
branch: dup/viewer-fixture-oracles
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
| `review_gui2_r1::delta` (1.5e-4), `review_gui2_r2::delta` (3.0e-4), `coarse` (2.0e-3) | — | — | **kept**: three different values, one per suite's geometry. Not one thing spelled three times |
| `review_gui2_r2::insert`, `tol` | `common::inserted` | — | **kept as adapters**: one line each, fixing this suite's tolerance. The construction is gone; what is left is a parameter binding |

`review_gui2_r2::rectangle` had exactly one caller and was inlined
rather than kept as an adapter, which is the same judgement read the
other way.

### The mutation table, re-planted at `b29fe8bd1` after the folds

Baseline **626 pass / 0 fail / 1 ignored**, `cargo test -p viewer
--test all`, default lane. Each mutation is planted in
`crates/viewer/tests/common/mod.rs` and reverted after the run.

| planted | total | `gui2_r1` | `gui2_r2` | `gui3_r1` | `gui3_r2` | `msolve5` |
| --- | --- | --- | --- | --- | --- | --- |
| `inserted` answers the document's FIRST node's id | 514 / 112 | 4 | 16 | 4 | 13 | 0 |
| `edited` discards the edit and answers the input document | 438 / 188 | 4 | 19 | 9 | 16 | 0 |
| `len(m)` → `m + 1.0` | 534 / 92 | 3 | 11 | 0 | 3 | 0 |
| `scl(v)` → `v + 1.0` | 558 / 68 | 2 | 2 | 1 | 0 | 0 |
| `ang(r)` → `r + 1.0` | 616 / 10 | 0 | 1 | 0 | 0 | 0 |
| `rectangle`'s height halved | 616 / 10 | 1 | 2 | 0 | 0 | 0 |

Every folded door is live in at least one suite that newly reads it.
The `rectangle` mutation also reds `profile_draw` (2, through
`common::square`), `review_gui4_r1` (2), `review_gui4_r2` (1),
`mate_tool_flow` (1) and `instance_authoring` (1) — the last four
through `common/asm.rs`'s `box_part`, whose inline polygon this unit
folded onto the same door.

### The zeros, answered by measurement rather than inference

- **`gui3_r1` on `rectangle`, `gui2_r1` on `ang`**: neither file
  reaches the helper. Zero by construction, not a coverage fact.
- **`gui3_r2` on `rectangle`**: it DOES read it, and reds nothing. The
  second question — *asserts nothing about that helper* or *asserts
  nothing useful* — is settled by the same binary: `inserted` reds 13
  of its rows and `edited` reds 16. The suite is emphatically live; its
  rows assert on panels, history, seam generations and file bytes, and
  none of them on the profile's shape. That is *asserts nothing about
  that helper*, which is this row's to close, not S-TINT's.
- **`msolve5_read_below_a_root` on everything**: one row, and its
  three assertions are a session refusal being absent, a product fault
  being absent, and the at-rest badge's message being the gate's own
  `Display` word for word. No literal in its fixture reaches any of
  them, so a changed length, scalar or angle cannot move it. The fold
  there is a substitution and is NOT claimed as proved live; what
  proves it is the compiler, since `common::len` and the deleted local
  `len` are the same call.

### What this unit deliberately did not do

- **`docm9_range_vs_probe.rs`'s `lit`/`scalar`** are the same class and
  were left: the file is `#![cfg(feature = "interval")]` and reaches
  `Expr` through `editor_core` rather than the `pncad::document`
  façade, so a fold there is type-checked in one lane only. Filed as
  `viewer-tests-bypass-the-shared-literal-doors`.
- The seven `crates/topo/src/review_m1_*` headers ending *"Promoted per
  Ev's request (PR #17 thread)."* were left alone: separately filed and
  waiting on Ev.
- `a-viewer-error-arm-is-not-split-because-a-review-suite-pins-it` was
  not touched; nothing in this diff reaches that arm.

### Residue filed, one per class

- `viewer-tests-rebuild-the-pick-index-in-every-suite`
- `viewer-tests-each-spell-their-own-downward-pick-ray`
- `viewer-tests-bypass-the-shared-literal-doors`
- `suite-headers-instruct-on-ignored-rows-they-no-longer-have`
