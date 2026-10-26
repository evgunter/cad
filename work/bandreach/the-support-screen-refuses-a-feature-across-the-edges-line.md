---
id: the-support-screen-refuses-a-feature-across-the-edges-line
kind: issue
title: blend: the support screen refuses a feature across a requested edge's line, where the band never reaches
status: open
opened: 2026-10-12
priority: P3
cost: M
---


## Finding

Predicate 2's support screen (`battery::face_clearance`, reading
`gap − setback` between `CHAIN_SAMPLES` points per edge) measures a
straight-line gap. That is two-sided: a boundary feature of the
support within the setback of a sample refuses whichever side of the
requested edge's line it lies on. But the band only ever reaches one
side, the trimline's, where the strip lies.

**Witness.** The tabbed pocket's outline gains a peninsula of the
support rising from the pocket's far wall, its tip `0.05` across the
tab's tip line, at `x = 3.0625`, midway between two samples.
- The screen reads `√(0.0625² + 0.05²) = 0.0800 < D` and refuses
  `FaceClearanceUncertified`, both verbs, pocket and boss, on the
  ring and on the outer cycle.
- The band lies wholly on the other side of the tip, so the geometry
  is clear.
- At `0.09` across, the screen passes, and the reach's region and the
  strip meter, both one-sided, build it at the closed form
  (`band_planar_cut_off_meters::a_feature_across_a_requested_edges_line_is_clear_of_its_strip`,
  which also pins the `0.05` refusal by the screen's gap).

**The pairwise check, cross-chain.** The same two-sidedness holds
between two requested edges. The tabbed pocket's far wall gains a
plateau whose top edge, from `(2.2, 2.5 − gap)` to `(3.8, 2.5 − gap)`,
is requested with the tab's tip. Each band removes (or, on the boss,
adds) material on its own side of the slot between them, so they
never meet. Measured, pocket and boss, both verbs, the outline closed
and open:
- `gap = 0.3` builds at the two prism closed forms, `ΔV = ±2.6 ×
  section`.
- Every `gap < 2D` (0.12, 0.1, 0.09, 0.05, 0.02) refuses
  `FaceClearanceUncertified { cross_chain: true }` at margin
  `gap − 2D`.
- At `gap = 0.2` the margin is `+1.7e-16`, inside the zero band, and it
  refuses Uncertified too.

Conservative, not unsound: nothing wrong is built.

## What would close it

The screen reads the side of the edge's line a feature lies on, and
clears one across it. Change the `0.05` row in
`a_feature_across_a_requested_edges_line_is_clear_of_its_strip` to
build, and the cross-chain plateau at `gap = 0.2` and below. The gap
the screen reads is predicate 2's measurement, so this is a change to
what predicate 2 decides: check `fillet3_face_clearance`'s contract in
`crates/sweep/README.md` before moving it.
