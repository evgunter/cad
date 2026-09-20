---
id: viewer-tests-each-spell-their-own-horizontal-pick-ray
kind: issue
title: Seven private spellings of the horizontal axis-aligned pick ray in crates/viewer/tests
status: open
opened: 2026-09-20
---


## Finding

- **Where**: `crates/viewer/tests/`, seven spellings of an
  axis-aligned HORIZONTAL pick ray in six files, each starting at ±1 m
  on the axis it travels and aiming at a wall the downward rays cannot
  reach — `mate_tool_flow.rs` (−x), `review_gui2_r1.rs`'s `wall` (+x),
  `review_gui2_r2.rs` (+x), `review_gui4_r2.rs` (−x),
  `story_assembly.rs` twice (+y and −y), and `select_pick.rs`'s
  `at_the_wall` (+x), the one of the seven that is a named helper —
  minted by the unit that found this class, out of two byte-identical
  inline copies in that file.
- **The construction, which is one**: `Ray { origin: p offset to ∓1 on
  the travel axis, dir: the unit axis }`. It is the same shape as
  `common::{down_at, up_at}`, one axis over, and the natural home is
  the same module: `common::along(axis, sense, at)` or four named
  doors.
- **What makes the fold a judgement**: the offset. `down_at` could fix
  its height at one metre because the fixtures it serves are all
  plate- or assembly-scale; these seven each aim at a specific wall of
  a specific fixture, so the door takes the two in-plane coordinates
  AND the axis, which is three arguments rather than two, and a reader
  may find `Ray { .. }` clearer than a call with an axis enum in it.
  Answering *no* at a site is a result here.
- **Importance**: low. No oracle: a wrong ray misses the body and reds
  the row that aimed it. Measured 2026-09-20 — planting `at_the_wall`
  to aim AWAY from the plate reds 2 of `select_pick`'s rows, and
  planting `common::down_from` to start below the fixture reds 52
  across thirteen suites, so this family of rays is live.
- **Instrument, and its blind spot**: `git grep -n -A3 'Ray {' --
  crates/viewer/tests/`, read for a `dir` that is a unit axis other
  than ±z. It misses a ray built in a loop from a table (measured:
  `index_memo`, `pick3_acceptance`, `review_pick2_r1` and
  `review_pick_r2` each build one, and those are enumerations rather
  than fixed aims), a ray whose `dir` is a named constant, and any
  ray outside this crate's suites.
- **Raised by**: the S-DUP lane closing
  `viewer-tests-each-spell-their-own-downward-pick-ray`, 2026-09-20,
  measured at `cd9fdfd6b`. That row named the inline-at-the-row's-own-
  site case as its blind spot; this is what running it turned up.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner to
file it with, and the subject — one construction spelled more than once
— is S-DUP's charter. Any of the five may claim it by `git mv`.

