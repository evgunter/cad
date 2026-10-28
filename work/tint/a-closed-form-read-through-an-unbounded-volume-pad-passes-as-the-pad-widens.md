---
id: a-closed-form-read-through-an-unbounded-volume-pad-passes-as-the-pad-widens
kind: issue
title: Sweep rows read a closed form within volume_pad plus a slack, with no ceiling on the pad, so the check only gets easier as the enclosure widens
status: open
opened: 2026-10-28
priority: P3
cost: M
---


Found by the SPLITDOOR review of PR 5940 (`splitdoor/tilted-plug-rosters`).

## What

`crates/sweep/tests/common/differential.rs` `judge` holds a built body
to its closed form as `|v − want| ≤ VOLUME_CLAIM + volume_pad` **and**
`volume_pad ≤ VOLUME_PAD_BUDGET`; `judged_within` takes a caller's own
ceiling where the shared budget is known not to hold (PR 5940). Many
rows spell the same reading by hand and drop the second half: they
assert `|v − want| ≤ volume_pad + slack` and never bound `volume_pad`,
so a quadrature that widens (a regression in the certified enclosure)
passes them more easily, not less. Named on review:
`a_pierce_held_in_a_curved_face.rs` (`(m.enclosure.volume − volume).abs()
< m.enclosure.volume_pad + 1e-9`); `an_edge_running_past_a_face_reads_its_crossings.rs`
reads tiers and the far brick by hand.

Shape grep on main at PR 5940's base (`volume_pad +` in a sweep test
file that does not import `differential`) lists 32 files, among them
`a_ruling_lying_on_a_wall.rs`, `operand_gate_pose.rs`,
`conic_edge_curved_face.rs`, `near_circular_sections.rs`,
`reach_split_gate_*.rs`, `verbs_germarms.rs`, `split_through_a_ruling.rs`,
`common/oracles.rs` and `common/torus_walls.rs`. Not every hit is a
closed-form gate (some compare two measured bodies); each needs reading.
The grep cannot see a pad read through another name (`m.pad`, a helper's
return) or a check in `crates/topo/tests`.

## Direction

Route each closed-form gate through `differential::judged` /
`judged_within`, or assert a stated pad ceiling with its measured
reason at the site, as `continuation_pose.rs`'s `PAD_CEILING` and
`crossing_equal_rods.rs`'s do. Sibling, the opposite shape:
`quadtail/sweep-tests-hold-quadrature-midpoints-to-fixed-tolerances`
(a fixed midpoint tolerance ignoring the pad).
