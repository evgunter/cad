---
id: test-fixtures-write-a-placements-zero-values-as-literals
kind: issue
title: test fixtures write a mate's or placement's zero values, and a split's zero phase, as literals that mint variables the design writes as the constant 0
status: open
opened: 2026-10-21
priority: P3
cost: M
refs: [the-tour-writes-a-mates-spin-as-a-literal-zero]
---

## What

Found by the sweep for `the-tour-writes-a-mates-spin-as-a-literal-zero`
(2026-10-21), which fixed the demos and helpers and left test fixtures
outside its fence. Under `[variables.literals]` a written `0 m` or
`0 rad` is a variable; the constant `0` is `Formula::zero` (Python
`Formula.zero`). `Placed::identity`, `place_at`'s `written` and the
tour's `world.rs` already write the constant.

Shape swept (Rust and Python, `crates/*/tests`, `crates/*/src` test
modules): a `Coord::…` value, a `phase:` or a `rotation_angle:` whose
formula is a helper call on a literal zero (`len(0.0)`, `ang(0.0)`,
`fixture::ang(0.0)`, …), and Python's `Formula.literal(0.0)`. About
100 hits in 49 files. The largest:

- `crates/viewer/tests/combine_ops.rs` (16)
- `crates/editor-core/tests/intent_s3_c_mates.rs` (9)
- `crates/pncad-py/tests/test_placed_union.py` (8)
- `crates/editor-core/tests/msolve9_from_face.rs`,
  `msolve6_part_extent.rs` (6 each)
- `crates/pncad-py/tests/test_north_star.py` (5)
- `crates/editor-core/tests/msolve4_mate_memo.rs`,
  `msolve8_levered_clash.rs`, `crates/pncad-py/tests/test_document.py`
  (4 each)

Mate-value rows of the same shape stand in `intent_s2_f_mates.rs`,
`part_depth_bound.rs`, `names_verbatim_edge_evaluator.rs`,
`mate6_gather_mints.rs`, `asm_r2b_assembly.rs`,
`intent_s3_d_placements.rs` (`SpinAbout`) and
`corpus/part_select.rs`. `edit_step_segments.rs` and
`m10_9_r1_probes_interval.rs` write a split's `phase` as
`ang(0.0)`. `crates/viewer/src/app.rs`'s
`a_held_face_on_a_copy_a_feature_re_points_stays_held` writes a
transform's unused translation components and its angle as literal
zeros.

The Python hits are mostly direction components,
`(Formula.literal(1.0), Formula.literal(0.0), Formula.literal(0.0))`.
Those are a scalar literal per component where the design's constants
are dimensionless rationals. That is a neighbouring question and may be
a separate row.

## Owed

Go through the hits one by one. A zero that means "nothing written" (a
seat's spin, an identity slide, a phase with no turn, an unused
component) becomes the constant. A zero that a test means as a written
value it later moves stays as it is. Several of these tests assert on
the minted-variable count or on content keys, so expect re-blesses,
each with its cause stated.
