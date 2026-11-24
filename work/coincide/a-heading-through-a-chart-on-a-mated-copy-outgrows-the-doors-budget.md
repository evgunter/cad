---
id: a-heading-through-a-chart-on-a-mated-copy-outgrows-the-doors-budget
kind: issue
title: A traced boss's heading through a chart on a copy mated onto a placed cylinder's cap outgrows the door's budget
status: open
opened: 2026-11-22
priority: P2
cost: M
---


Found by `doctail/the-tangency-prover-does-not-see-through-a-placement`'s fix, which sets a chart's spin from its frames at `Sym` (`SolveScalar for Sym<T>::spins_by_frames`, `crates/editor-core/src/eval/mod.rs`). That fix proved the heading through one placement by values and through a frame read off a box's corner, but not through this.

## What

`intent_s6_e_traced_on_a_face::a_boss_on_a_box_mated_flat_onto_the_cap_is_tangent_for_every_tilt`:
1. A cylinder is placed by values into a box's space, spun about `x` by the free `a` at `0.2` (its `y` and `z` spins written zeros, each a variable).
2. A thin box is placed flat on its top cap by the plane mate `sketch_on_face` writes (spin written zero, a variable).
3. A boss is drawn on that box's top cap (its own face chart) tracing the cylinder's wall.

`swept_along` (`coincide::replay`) reads the wall's axis in the sketch's coordinates: the box top's chart, through the mate's chart, onto the cylinder's placement. Its component across the sketch's `z` is a form in four symbols (589 DAG nodes) that is `OverTheBudget` at the door's 256 terms (`coincide::replay::BUDGET`). It discharges at 512, and at 4096.

The row is pinned red at every ε against this one (`test_utils::pinned_red::row`).

## What does not fix it

- **Raising `BUDGET` to 512.** That moves `coincidence_door::two_copies_on_one_axis_glue_their_walls_on_one_axis`'s goldened rungs (not analysed), and it makes a failing replay cost more: `coincidence_door::a_filleted_box_records_one_unproven_turn_per_corner` went from 4.7 s to 6.3 s in debug.
- **Reading the heading in the body's coordinates** (`sketch_to_body · z × axis`) instead of the sketch's. The `x` and `y` components still do not discharge at 256.

## Lead

Two least turns are nested: the box top's chart turns `z` onto a normal that the mate's least turn itself computed from the cap's. Each least turn's identity `S·z = d` holds only through `|d|² = 1`, and here `d` carries the inner turn's normalizing square roots. A heading read that does not compose the two charts, such as the sketch's `z` read as the chart's mated direction, which it is by construction, would keep the form to one turn's. But it states a fact about the chart rather than reading the replay's expression, so it needs an argument that the two agree as real functions.
