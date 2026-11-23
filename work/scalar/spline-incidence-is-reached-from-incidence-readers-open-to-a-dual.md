---
id: spline-incidence-is-reached-from-incidence-readers-open-to-a-dual
kind: issue
title: spline_incidence.rs is certification arithmetic reached from point_incident and incident_as at T: Decide, which a dual instantiates
status: open
opened: 2026-11-20
priority: P1
cost: M
parent: from-computed-claims-a-zero-tangent-at-a-dual
---


Found building `certification-interiors-are-closed-to-a-dual-by-a-marker-bound`, 2026-11-20. Measured by type-check: the bound was written and `cargo check -p geom-brep` read.

## What

`crates/geom-brep/src/pcurve_cache/spline_incidence.rs` is one algorithm with private types (`Net`, `Cell`, `Queued`, `Read`): a point's distance to a spline chart, by subdivision with `f64` rounding pads. Nothing in it is a body's data, so the whole file can take `Certifies` and be listed in `scripts/gates/certification-interiors.sh`.

Its two doors, `spline_incidence_as` and `spline_point_incidence_as`, are called from `pcurve_cache.rs`'s `incidence::incident_as` and `incidence::point_incident`, both `T: Decide`. Those are the incidence readers every pcurve door ends through, including the arm a scalar with no fitted lane takes (`certify_stated_image`'s `None` arm, `incidence_ended_unwitnessed`), so a dual with an edge on a spline chart instantiates the subdivision today.

## What it asks

The spline arm of the two readers split from the analytic arms: the analytic arms stay at `T: Decide`, and the spline arm is reached through a lane the scalar holds (`FittedLaneOf`, or a door of its own), so that a dual refuses there by type as it does at the fitted checks. What a dual's edge on a spline chart reads instead is the design question: the readers return a verdict the mint acts on.

Then `spline_incidence.rs` is bounded and listed.

## What is left, as of the fix pass on PR 7190 (2026-11-20)

Unchanged by the fix pass: `incidence::incident_as` and `incidence::point_incident` are `T: Decide` and call the subdivision with no lane, so this is the same shape as `the-certification-files-hold-body-data-beside-certification`'s (a): certification arithmetic a dual runs outside any lane. Unmeasured, as there: whether a dual test binary instantiates `spline_incidence_as` today (it needs a dual edge on a spline chart, and the dual corpus's bodies were not read for one).
