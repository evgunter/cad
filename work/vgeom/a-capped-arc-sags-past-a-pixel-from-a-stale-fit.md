---
id: a-capped-arc-sags-past-a-pixel-from-a-stale-fit
kind: issue
title: A capped arc sags past a pixel when the camera was fitted before the arc, or where the arc runs nearer the eye than the target
status: open
opened: 2026-11-25
priority: P3
cost: M
---



Found by the review of `vgeom/substitution-sweep` (PR 7764), which
raised `sketch.rs`'s `MAX_ARC_POINTS` to 1024 on a pixel argument made
at one seat.

**What.** The cap's rustdoc sizes it so a capped circle's sag,
`r·(1 − cos(π/N)) ≈ 4.7e-6·r`, is under a pixel on an 8K window at the
closest the camera dollies (`camera.rs` `band_floor`, `0.05·R`) to a
scene framed around the circle, at the target's depth. Two cases break
that argument:

- **A stale fit.** `scene_radius` R comes from the last fit, and
  a fit is owed only to the document's first body (`app.rs`, the
  `fit_camera` flag spent where the scene is rebuilt) and to Zoom to
  fit. The fit does take profiles in (`frame_camera` → `drawn_extent`),
  but only when one is owed. Frame a 1 m body, then sketch a 1 km
  circle: R ≈ 0.87 m, the band lets the eye sit about 1 km from the
  arc, and the capped sag of 4.7 m is about 24 px on 8K.
- **Points nearer the eye than the target.** An arc that passes
  between the eye and the target is magnified toward the near plane
  (`NEAR_FACTOR` = 1e-3 of the orbit distance), up to about 1000×. A
  δ-tessellated body has the same exposure at δ, but the capped arc's
  sag is `4.7e-6·r`, not δ.

**The question the fix answers first.** Whether the cap should defend
pixels at a seat at all, or scale with δ against the arc (a count
bounded by the profile's total drawn length over δ, priced like the
triangle budget prices bodies). The second makes the drawn sag ≤ δ
everywhere, as a body's is, and moves the bound onto cost instead of
onto one camera seat. Measure what a δ-scaled count costs per frame on
a profile with many large arcs before choosing.
