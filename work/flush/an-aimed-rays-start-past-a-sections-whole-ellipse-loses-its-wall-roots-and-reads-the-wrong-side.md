---
id: an-aimed-rays-start-past-a-sections-whole-ellipse-loses-its-wall-roots-and-reads-the-wrong-side
kind: issue
title: An aimed ray starting past a section's whole ellipse loses its wall roots and reads the wrong side
status: open
opened: 2026-10-26
priority: P1
cost: M
---

An aimed ray's count is meant to read the side wherever its aim lands
(`crates/topo/src/side_reader.rs` `aim_at`: "an aim off the face only
steers it"), so any aim must read the right side or nothing. One does
not.

**The pose.** `sweep` `split_seam_vanishing_tilt`, φ 2.2, ψ 1e-7, L 0,
sign 1: a unit cylinder split by a plane through its seam vertex `v`,
`10⁻⁷` off the wall's tangent plane. The section is an ellipse with a
semi-major axis of about `10⁷`.

**What reads wrong.** Found on `flush/aims-ties`, by moving
`RuledFaceRegion::aim` (`ruled_region.rs`) from its edges' middles to
their golden-section points:
- The wall's aim lands on `v`'s ruling, at `(−0.589, 0.808, 0.528)`.
- Its ray starts about `4·10⁷` m out. `beyond` and `selection_reach`
  (`solid_contain.rs`) take each curved edge's ball from
  `splitting/containment.rs` `loop_hull`/`step_ball`, which for the
  section is the whole ellipse's.
- At that start, the wall's roots (`line_wall_roots`, a quadratic in
  `t ≈ 4·10⁷`) land about `0.1` m off, read as boundary (count 0,
  sideless).
- The cut plane's crossing then counts `+1` first. The ray reads `In`,
  and check 7 reports `NegativeVolume` on a correctly oriented side.

On main the middle aim lands elsewhere and reads `Out`, so nothing
ships wrong today. `flush/aims-ties` keeps the middle aim for that
reason.

**What a fix has to settle.** A reach that bounds the edge rather than
its carrier: the arc's own extent, not the ellipse's semi-axes. Or a
ray start, and root solve, whose rounding stays inside the band at the
model's extent (`[verdicts]`: "a formula whose error is amplified by a
vanishing quantity … is a defect"). Either way, a pin of the pose with
the wall aimed at `v`'s ruling, reading `Out` or nothing.
