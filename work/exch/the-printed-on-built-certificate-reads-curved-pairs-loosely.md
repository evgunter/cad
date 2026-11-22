---
id: the-printed-on-built-certificate-reads-curved-pairs-loosely
kind: issue
title: the printed-on-built certificate reads curved pairs loosely (the circle composite up to 28x on a sub-span trim, the conic frame bound on a near-round ellipse), so its tightness depends on the kind pair and a declined spline can refuse where its promotion certifies
status: open
opened: 2026-11-21
priority: P2
cost: M
---


Found by the dual review of EXCH unit D (PR 7304). Both reviewers
DEMONSTRATED it.

`build::description_gap` (`crates/step-import/src/build.rs:1358`) reads
`on_built::printed_on` (`crates/step-import/src/on_built.rs:93`) at the
file's band, so a bound looser than the true stand-off refuses in band
where the stand-off itself reads Zero. Three sources:

- **The circle composite** (`recognize_curve::circle_locus_within`,
  `crates/step-import/src/recognize_curve.rs:383`, over
  `composite_sup_within`, `:323`). It is the INV-C1 sphere sup divided by
  the radius, combined with the plane sup; the span hulls overbound
  `|f ∘ C|`.
  - rev2's probe measured 3.4× the dense true distance.
  - rev1's fuzz measured up to 28× on sub-span trims.
  - Pinned: `tests/fixtures/declined/cyl_declined_loose.step` (the
    declined rim nudged 8e-11 m). At kernel ε 1e-12 it reads 1.07e-10 m
    against a true stand-off of about 4e-11 m at ε_in 1e-10, and refuses
    in band. Main imports it (three samples read Zero). The pins are
    `a_declined_rim_read_loosely_refuses_at_the_fine_band` (in
    `tests/an_edge_is_read_on_its_description_whatever_recognition_says.rs`)
    and its `tier_gate` cells. The fix turns both.
- **The kind-pair tightness gap.** A promoted circle on a built circle
  is read by `arc_on_circle` (`on_built.rs:260`, a tight sinusoid bound).
  The spline it was promoted from is read by the looser composite. So a
  rim that promotes can certify where the same rim declined refuses.
- **The whole-conic frame bound** (`Conic::bound_from`, `on_built.rs:398`,
  reached at `:117`). It bounds `|c₁ − c₂| + |A₁ − A₂| + |B₁ − B₂|` over a
  pairing of semi-diameters, not the locus. rev1's
  `rev1_a_circle_on_a_near_round_ellipse` read 0.02 m against a true
  distance ≤ 1e-12: a circle and an ellipse with nearly equal semi-axes
  pair seam to seam, which a principal frame does not do.

Also noted (rev2 S1): three mechanisms now bound a curve's distance from
an arc. They are the recognizer's `arc_residual` (composites plus
turning), `arc_on_circle` (sinusoid extremes) and the conic frame bound.
A fix might fold them into one, e.g. a trim-aware composite tight enough
for the conic pairs too.
