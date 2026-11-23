---
id: f64-tier-3-reads-volume-uncomputable-in-band-on-hairline-revolves
kind: issue
title: At f64, tier 3 reads VolumeUncomputable in band on the 1.3 km tube at 1e-12 and the bool5r hairline revolves, so their doors refuse
status: closed
opened: 2026-11-02
priority: P0
cost: M
refs: [6371]
closed: 2026-11-14
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

## Slate triage (2026-11-08, ONEHOME split)

- Moved to BOXFINISH (`work/boxfinish/`) by `git mv`, id and body unchanged. ONEHOME measured 78 budget points against 35 after PRs 6371 and 6380 landed, and split along its seams (`work/README.md` Track size): the rows that let the box scalars finish their sweep leaves at tier 3 and so retire Ev's switch (PR 6371) went to BOXFINISH.

## Closed (2026-11-14): measured, and split by what it found

Measured on main `b363f5fa` (BOXFINISH's M2 lane). The row is three
refusals under one title, and none is check 7 falling back on a volume:
on every refusing body the aimed rays read `Side(Out)`.
`VolumeUncomputable` is the certificate walk's refusal.

- **The 1.3 km tube is not a case.** As the door stands it refuses
  `OutsideSessionBox` (margin −302.5) before tier 3, and
  `verbs_tubewall_r1_probes.rs`'s `CASES` no longer holds it, so the
  pins this row cites are unreachable. With the box bypassed it refuses
  `props_rim_span_winding`, an `Interval` enclosure one ulp of τ wide at
  the rim's radius: `work/spiricarm/a-whole-turn-rims-headroom-is-read-through-a-one-ulp-tau.md`.
- **CARVE's witness, and a tube in a generic pose from about 300 m,**
  refuse `props_chart_loop_whole` at ε = 1e-12 in the `Interval`
  re-derivation, an honest enclosure several hundred times loose:
  `cylinder-and-torus-edge-misses-are-read-back-about-the-axis`, claimed
  from NEARPOINT, with the measurements in its body.
- **The bool5r hairline revolves refuse honestly.** At the default ε the
  reader's margin equals the true value (5.0e-10 and 6.0e-10 m against a
  1e-9 band): the geometry is in the band, so "asserts its body again"
  was the wrong test. What is left is a question about the door's
  admission:
  `work/lathe/a-hairline-slit-the-revolve-admits-refuses-at-its-finish-on-the-faces-own-lever.md`.
