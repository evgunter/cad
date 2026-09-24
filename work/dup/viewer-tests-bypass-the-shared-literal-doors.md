---
id: viewer-tests-bypass-the-shared-literal-doors
kind: issue
title: Thirty inline Expr::literal spellings in crates/viewer/tests beside the common::len/scl/ang doors
status: closed
opened: 2026-09-20
priority: P4
cost: E
closed: 2026-09-20
branch: dup/viewer-shared-doors
pr: 2929
---


## Finding

- **Where**: `crates/viewer/tests/`, thirty inline
  `Expr::literal(v, Dimension::{Length, Scalar, Angle})` spellings in
  fifteen files — `combine_ops` 6, `creation_ops` 3, `index_memo` 3,
  `blend_authoring`, `docm9_range_vs_probe`, `gesture_table`,
  `pick3_acceptance`, `review_pick2_r1`, `review_pick_r2` 2 each, and
  `cascade_delete`, `doc_io`, `docm1_face_frame`, `landing_gathers`,
  `path_authoring`, `pick_windows` 1 each. Split by dimension:
  Angle 15, Length 12, Scalar 3.
- **The home already exists**: `common::{len, scl, ang}`, which these
  suites' own binary mounts. The private WRAPPERS around the same call
  were folded on 2026-09-20 by the unit that measured this; what is
  left is the arm that writes the call out at the use site instead.
- **What makes the fold a judgement**: `docm9_range_vs_probe.rs` is
  `#![cfg(feature = "interval")]` and reaches `Expr` through
  `editor_core` rather than through the `pncad::document` façade, so
  its two sites are only type-checked in one lane and were left out of
  that unit deliberately. The rest are plain substitutions.
- **Importance**: low. No oracle: `common::len` and a hand-written
  `Expr::literal(_, Dimension::Length)` are the same call, and a bug in
  the shared door reds ninety-two rows (measured, 2026-09-20).
- **Instrument, and its blind spot**: a regex for
  `Expr::literal(<no comma>, Dimension::X)` over every tracked
  `crates/viewer/tests/*.rs`, `common/mod.rs` excluded as the home.
  It misses a literal whose value expression contains a comma
  (`Expr::literal(f(a, b), …)`), `Expr::literal_with_unit`, and every
  other crate's suites — the same class certainly runs wider, and
  `git grep -l 'Dimension::Length).expect'` alone names forty-five
  files outside this crate.
- **Raised by**: the S-DUP lane closing
  `viewer-review-suite-fixtures-have-no-oracle-role`, 2026-09-20,
  measured at `b29fe8bd1`.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner,
and one call spelled thirty times beside its own door is S-DUP's
charter. Any of the five may claim it by `git mv`.

## Closed 2026-09-20 — re-taken at `cd9fdfd6b`, folded onto `common::{len, scl, ang}`

### The census, re-taken, and the count that moved

The row's instrument was a regex for `Expr::literal(<no comma>,
Dimension::X)` over `crates/viewer/tests/*.rs`. Re-run as
`git grep -n 'Expr::literal' -- crates/viewer/tests/`, then a read of
each hit and a second, multi-line pass (`git grep -A2
'Expr::literal($'`).

**That command carried a path argument, and this section first said it
did not.** The claim *"every tracked file, no path argument"* was false
of the command that produced the count below — it was scoped to the
class's home tree. The cross-crate row that ran next excluded
`crates/viewer/` as a whole, so `crates/viewer/src/` fell between the
two and neither looked at it. It holds **14 `.expect`-shaped members,
all inside `#[cfg(test)]` modules** (12 at the merge base, 2 more
added on main since), which cannot reach `tests/common`; filed as
`viewer-src-test-modules-restate-the-literal-doors`. A scope sentence
reads as completeness whatever the command under it did, which is
method item 3 failing in the prose rather than in the grep.

**31 sites in 16 files, not 30 in 15.** The extra is
`frame_policy.rs`'s `rotation_angle`, written path-qualified across
FOUR lines — the row's line-shaped instrument could not see it, and
`frame_policy` appears nowhere in the row's file list. By dimension:
**Angle 16** (the row said 15), Length 12, Scalar 3. Every other
per-file figure in the row held exactly.

Disposition of the 31:

| sites | disposition |
| --- | --- |
| 24 ordinary `.expect`-shaped calls in 13 files — `blend_authoring` 2, `cascade_delete` 1, `combine_ops` 5, `creation_ops` 2, `doc_io` 1, `docm1_face_frame` 1, `frame_policy` 1, `gesture_table` 2, `index_memo` 2, `landing_gathers` 1, `path_authoring` 1, `pick3_acceptance` 2, `pick_windows` 1, `review_pick2_r1` 2, `review_pick_r2` 2 | **folded**. `creation_ops`'s two were local CLOSURES named `len` and `scl`, shadowing the `common::len` the file already imports |
| `combine_ops:514`, `creation_ops:509` — `Expr::literal(f64::NAN, Dimension::Length)` with no `.expect` | **not members**: the refusal is what the row is about, and `common::len` expects |
| `index_memo:155` — `.ok()?` | **not a member**: the door panics where this site returns `None`, which is a different contract, not a different spelling |
| `docm9_range_vs_probe` 2 | **left**, as the row said and the parent unit settled: `#![cfg(feature = "interval")]`, reaching `Expr` through `editor_core`, so a fold is type-checked in one lane only |

`review_pick2_r1` and `review_pick_r2` reach `Expr` through
`editor_core` too, but are not feature-gated, so `common::len`'s
`pncad::document::Expr` type-checks there in every lane — the façade
re-exports the kernel's type rather than wrapping it. That is why those
four sites folded and `docm9`'s two did not.

**The blind spot the row named — `Expr::literal_with_unit` — run as a
second instrument.** Six byte-identical sites of one unit-carrying
extrude distance in two files, which `common::len` cannot serve because
it lowers canonically and carries no notation. Filed first, then folded in the merge pass onto a new
`common::len_mm`, the notation-keeping sibling of `len`: all six were
byte-identical down to the `.expect` string, so the door's shape was
not a design question and the fix was smaller than the row. The
other named blind spot, a value expression containing a comma, has no
hits: every `Expr::literal` in the crate takes a comma-free value.

### The proof

Baseline **626 passed / 0 failed / 1 ignored**; every row sums to 626.

| plant | direction | total | reds (top suites) |
| --- | --- | --- | --- |
| `ang(r) → r + 1.0` | **grow**: a `rotation_angle` of 0.0 becomes a radian of spin, which moves every body a row places; nothing is relaxed | 607 / 19 | `combine_ops` 4, `mate_tool_flow` 3, `frame_policy` 2, `index_memo` 2, `review_pick2_r1` 2, and one each in `creation_ops`, `pick3_acceptance`, `review_gui2_r2`, `review_pick_r2`, `story_authoring`, `story_parametric` |
| `len(m) → m + 1.0` | **grow**: every shared length literal gains a metre | 535 / 91 | `datum_draw` 14, `review_gui2_r2` 11, `mate_tool_flow` 10, `combine_ops` 7, `review_gui4_r2` 7, `select_pick` 5, `blend_authoring` 4, `gesture_table` 4, + 16 more suites |
| `scl(v) → v + 1.0` | **grow**: a unit axis stops being one and a 0.0 component becomes 1.0 | 559 / 67 | `datum_draw` 15, `mate_tool_flow` 10, `review_gui4_r2` 7, `combine_ops` 4, `landing_gathers` 2, `doc_io` 1, `docm1_face_frame` 1, + 18 more |

The `ang` plant is the one this row's folds move: 15 of the 24 folded
sites are Angle, and `combine_ops` (5 folded Angle sites) is its
largest red column. `cascade_delete`, `pick_windows` and
`path_authoring` fold Angle or Scalar sites and appear in **no** red
set — `path_authoring`'s is explained (its row asserts on the notation
a literal is written IN, not on its value, so a value plant cannot
reach it by construction); the other two are filed with the no-probe
finding on S-TINT's slate.

A note on the figures: these suites draw a fresh fuzz seed per run, so
a ±1 difference between two runs of one plant is noise. The parent
unit's `len(m) → m + 1.0` read 534 / 92 on a tree with fewer `len` call
sites than this one; 535 / 91 here is the same measurement, not a
smaller one.

### After merging main

Main landed seven new members of this class after the merge base, in
files this PR's merge touched: **four** in `docm1_face_frame.rs`
(AUTH-1 and AUTH-3, all `Expr::literal(_, Dimension::Angle)`), beside
the one this unit had folded there, and **three** in the new suite
`frame_labels.rs`. All seven are folded onto `common::{len, ang}`,
because the door is in the same binary and the fix was smaller than a
row. Re-taken on the merged tree, `git grep -n 'Expr::literal(' --
crates/viewer/tests/` returns only the home, the three `NAN` refusal
probes (`combine_ops`, `creation_ops` and a new one in `panel_edits`),
`docm9`'s two, and `index_memo`'s `.ok()?`, none of which is a member.

**And the merge pass nearly left a member behind in a file it had
open.** It minted `common::len_mm` for the six `8 mm` sites and folded
`frame_labels.rs`'s three plain `Expr::literal` sites onto `len` /
`ang` — and left **three longhand copies of `len_mm`'s own
construction** in that same file, `literal_with_unit(_, Length,
MM.def()).expect(..)` at other values. Found on the re-read the log
names as the strongest single instruction (*"re-read every file you
touched, for the class you are closing"*), not by any census the pass
had run: the `len_mm` census was keyed on the VALUE `0.008`, and these
are `0.0` and `0.010`. A census keyed on a value misses the same
construction at any other value — which is item 2, *grep the
construction, not the name*, one level down. All three folded.
