---
id: the-session-box-is-not-enforced-at-construction
kind: issue
title: The 1 km session box of [tolerance.units] is not enforced at construction: rows build 6 km bars, 1.3 km tori and 1e9 m placements
status: open
opened: 2026-10-26
priority: P2
cost: M
design: true
---



Filed by PR 5728's implementer lane, from fork row 203's fix.

**Finding.** DESIGN `[tolerance.units]` (`docs/DESIGN.md:663`) says the
session box is 1 km and that "geometry outside the range is rejected at
construction". `crates/geom-core/src/tolerance.rs:93` defines
`SESSION_BOX_EXTENT` and `:102` `EPS_MIN` from it, but nothing at
construction reads the box. Rows build outside it:

- `crates/topo/tests/long_edges_on_one_line.rs:85`
  `a_cube_on_a_six_kilometre_bar_builds_its_union`: a 6 km bar;
- `crates/sweep/tests/verbs_tubewall_r1_probes.rs` `CASES`: a torus of
  ring radius 1300 m;
- `crates/sweep/tests/sym11_far_placement_rows.rs:170` `PLACEMENTS`:
  bodies moved to 1e6, 3.7e7 and 1e9 m.

At ε = 1e-12 limb 2's certificate reaches the rounding of such geometry
(`AnalyticRung3Refusal::KernelLimit`, its bound within a few times the
carrier's own width), which the box would have kept out by
construction.

The bars no longer show it. A line against a plane is decided on its
two ends' measured distances (`edge_nurbs::measured_ends`), and the
6 km bars' ends measure within 2.3e-13 m, so they certify at 1e-12. The
tori and the far placements still reach the limit.

**Wanted.** Enforce the box at construction as the clause says, or
re-state the clause. That is Ev's call: it moves rows that build today.
