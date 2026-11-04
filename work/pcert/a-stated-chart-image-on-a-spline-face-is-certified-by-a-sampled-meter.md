---
id: a-stated-chart-image-on-a-spline-face-is-certified-by-a-sampled-meter
kind: issue
title: A stated chart image on a spline face is certified by a sampled meter, so a carrier off its wall between the samples gets an edge certificate
status: open
opened: 2026-11-04
priority: P1
cost: M
---

Filed by `pcert/spline-certificate-door` (PR 6531, review fix GG2).

## What

The edge certificate's stated chart image (`geom-brep/src/certify.rs`,
`run_checks` → `sampled_checks`, the `Resolved::Chart` arm's
`"pcurve_map_residual"`) meters `|C(t) − S(P(t))|` at the `CERT_SAMPLES`
(9) schedule samples of the edge, `t = i/8` of the span. The carrier's
incidence read (`pcurve_cache::read_shown`, the spline chart's hull walk)
samples `INCIDENCE_SAMPLES + 1` (65) points, `t = i/64`. A carrier off its
wall only between both sets passes both.

The review's probe R2 (`review/6531-full` at `e5b91df9`,
`spline_door_rows.rs` `review_r2_between_samples`) builds it on the unit
ring wall: the column `u = ½` ruling as a degree-1 NURBS with a 1 mm
radial bulge wholly inside `t ∈ (0, 3/256)`, peaking at `t = 1/128`. At
`f64` and at `Interval`:

- `EdgeCurve::certify` of `EdgeDescriptionSpec::chart_image(column)`
  certifies: a kernel-issued edge certificate for a carrier 1 mm off its
  wall;
- `read_incidence` over the span answers `Ok`; `read_point_incidence` at
  the bulge reads it off;
- the pcurve certificate's envelope catches it: `certify_general` refuses
  `FittedLimb { HullValueChart }` (margin 1e-3), the image's own bound, and
  the iso lane refuses `IsoUnsupported` (the carrier is not the column's
  spline space). Neither ends it `CarrierOffChart`.

The residual meter is sampled on an analytic chart too. There the read
comes first (`pcurve_cache::incidence_ended`), and how much of the span
that read bounds is its class's (a line's reads the span, PR 6527; a
sampled class's reads its samples), so the same gap is open wherever the
read is sampled.

## Where it reaches

Traced at `pcert/spline-certificate-door` (the review's open question):

- Every production producer that states a chart image on a spline face
  closes with `topo::mint_pcurves`, whose pcurve certificate carries an
  envelope:
  - sweep: loft (`loft.rs` `assemble`), extrude, revolve, tube, blend;
  - topo: boolean (`ops.rs`), split, shell, offset;
  - step-import (`assemble.rs` `build_one_solid`), which runs tier 3 after it.
- Mass properties and STEP export need an `AtRestBody` (tier 3), whose
  `validate_pcurves` re-derives a rowless face through `PcurveCache`.
- The gap is topo's construction tier (`topo::construction::Construct`:
  `set_edge_curve`, `restate_stored_edge`, `describe_at_rest`), which by
  design leaves a spline face's rows to a later mint or tier 3
  (`attach.rs` `description_remints`).
  - A body built through it and never minted keeps the sampled certificate
    alone.
  - `transform_rigid`, `mirror_body`, `join_edges*` and `merge_faces`
    re-mint only a body that already stored rows.
  - `mesh::tessellate` and `topo::pcurve_of` act on such a body with no
    envelope check.
  - pncad does not re-export the construction tier (`pncad/src/topo.rs`),
    so no outside caller of pncad reaches it.

## Fix

Meter a stated image on a spline chart by a bound over the span, not at
samples: the edge certificate could certify the stated image through its
pcurve certificate (`PcurveCache::certify` of the stated image, whose
envelope is a sup), as the mint does, or the read could bound the carrier
between its samples. Then a carrier off its wall anywhere on the span ends
by the read or the envelope, and the probe is a row.
