---
id: f64-tier-3-reads-volume-uncomputable-in-band-on-hairline-revolves
kind: issue
title: At f64, tier 3 reads VolumeUncomputable in band on the 1.3 km tube at 1e-12 and the bool5r hairline revolves, so their doors refuse
status: open
opened: 2026-11-02
priority: P0
cost: M
refs: [6371]
---

Split from `the-sweep-doors-finish-refuses-what-the-box-studies-read`
(PR 6371). The box-scalar switch does not cover these: `f64` keeps the
door's full finish. They stay admitted and filed here.

**What refuses at `f64`.** The door now refuses what it used to return
unfinished. Each case is pinned where a test asserted a body:

- **ε = 1e-12:** `sweep/tests/verbs_tubewall_r1_probes.rs`
  (`closed_forms_at_varied_geometry`,
  `the_cavity_is_a_two_wall_torus_at_the_inner_bits`). The hollow tube
  (R 1300, ro 2.5, wall 0.001, full period) refuses at the revolve's
  finish: `VolumeUncomputable` at `props_rim_span_winding`, margin
  [0, 1.15e-12], in band. `hollow()` returns `None` there.
- **Default ε:** `sweep/tests/bool5r2_wedge_reach.rs` (the hairline
  revolve of a dimpled ring) and `bool5r1_probes.rs` (the dimple lune
  across the slit window). The revolve's finish refuses: tier 3's full
  walk, `VolumeUncomputable` / the knife-edge caps. Those probes read the
  door's construction state
  (`sweep::test_support::revolve_construction`).
- **Untested:** CARVE's measured witness (a tilted 1 × 1 profile with a
  200–400 m arc side, extruded 0.7, ε = 1e-12) refused tier 3 in 4 of 20
  poses, so the extrude refuses there.

**The test that says it is closed.** Each row above asserts its body
again (the probes read the door, not the construction twin), at the
same ε.
