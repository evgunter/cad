---
id: the-side-reader-cannot-read-a-solids-orientation-over-a-box
kind: issue
title: The side reader cannot read a solid's orientation over a macroscopic box: every aimed ray's boundary margin blows up
status: open
opened: 2026-11-02
priority: P0
cost: H
refs: [6371]
---

Split from `the-sweep-doors-finish-refuses-what-the-box-studies-read`
(PR 6371). This is one of the rows that retire the box-scalar switch at
`topo::AtRestPolicy::leaf` (`crates/topo/src/props.rs`, the switch). Ev on PR 6371 (2026-11-02): "we should prioritize making it
actually work for all these cases, with the common cases as P0".

**What fails.** With the switch off, the sweep doors finish their own
result at `Interval`/`Sym`, and check 7 asks the side reader
(`crates/topo/src/side_reader.rs`) which side of the solid's faces holds
its material. Over the slab box of
`editor-core/tests/m10_3_driver_interval.rs::a_macroscopic_box_certifies_at_the_shipped_depth_budget`
(±0.05 on a 1.0 slab, one leaf) all 6 aimed rays are set aside on the same
test, whether a crossing lies inside the face's boundary:

- the aim points are box-wide enclosures (z ∈ [0.86, 1.14] on the top
  face);
- the boundary margins come back `[-0.91, 2.99]` and `[-5.22, 3.13]`
  against the (1e-9, 1e-8) band, and one ray's comes back NaN/refused
  ("6 of the 6 rays aimed at its faces read nothing",
  `side_reader.rs:192`);
- E12's symbolic zero does not discharge them, since they are not
  identically zero;
- bisection spends the 32-leaf budget, and the box certifies 0.

**Who it holds.** The slab box above; the derived-frame boss's plain and
A0-alone rungs at 5e-2
(`m10_derived_frame_interval.rs::m10_the_derived_frames_refusal_is_not_a_freeze`,
and the shipped rules in
`m10_the_derived_frame_extrude_agrees_with_its_authored_twin_below_that_width`);
and the tour's certified stop (`demos/tour/src/tolerance.rs`,
`eps_regression::certified_cells_run_green_at_default_eps`), which
reported 0.0000% certified, 99.4607% refused (budget).

**The test that says it is closed.** Delete the switch's box arm (or
make `leaf` call `finish` at `Interval`/`Sym`) and run those three rows:
each must pass with its main-era assertion (one leaf certifies; the
boss's rungs certify; the tour certifies its share).

**An untried idea.** Aim the side reader's rays at points that are exact
in the parameters (a face's own exact point, not an enclosure derived
across the box), so a crossing margin carries the face's own width and
not the box's. Unmeasured.
