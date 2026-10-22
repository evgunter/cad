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

## Split phases done; the placement half is D′'s (2026-10-22)

The hygiene batch (`intent/hygiene-batch`) took the half that is not a
placement's: a split's `phase`. The item listed two of those rows, and
a sweep for `phase: …ang(0.0)` / `rad(0)` / `Formula.literal(0` over
`crates/` and `demos/` found four more. Their dispositions:

- `edit_step_segments.rs`
  (`a_carrier_loops_record_is_checked_at_the_same_doors_a_chains_is`),
  `m10_9_r1_probes_interval.rs` (`split_bore_disc`),
  `edit_set_program.rs`
  (`a_reshapings_values_strand_what_a_slot_edit_of_them_would_not`,
  `a_reshaping_reports_exactly_the_held_names_whose_referent_it_takes`)
  and `face_face_touch_carry.rs`: a phase with no turn, so it becomes
  `Formula::zero(Dimension::Angle)`. Nothing re-blessed.
- `edit_set_program.rs`
  (`a_circle_splits_phase_edit_moves_its_pieces_and_renames_none`):
  stays `ang(0.0)`. It is a written value that the test moves to `π/2`
  through `SetParam`.

Everything else on this row is left for D′. The mate and placement
values, the viewer `AddTransform` row (whose door is
`place_against`) and the Python direction components all fall there.
D′1 restates most of the listed fixture files, and D′2
(`a-mate-reads-only-geometry-and-a-value-charts-on-the-target-space`)
rewrites `Value { coord, by }`, `Coord` and the helpers (`place_at`,
`place_against`), with its spec listing this row among its re-reads.
So the placement half waits for D′2 to land, and then gets one pass
over whatever fixtures still write a zero.
