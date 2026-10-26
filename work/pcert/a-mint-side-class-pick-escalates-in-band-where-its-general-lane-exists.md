---
id: a-mint-side-class-pick-escalates-in-band-where-its-general-lane-exists
kind: issue
title: The mint-side class picks (arc rim, iso row) escalate in band, and the arc rim's definite arm refuses IsoUnsupported, where the general lane exists (D3)
status: open
opened: 2026-10-20
---


Filed by PCERT unit E1 (`pcert/lane-choices`), from its sweep of lane
choices outside `pcurve_cache`. E1 rewrote the derivation's class
readings as D3 lane choices; these picks sit one layer up, on the
spline-chart mint, and still read the old way.

## What

A class pick chooses between a closed-form image and the general lane
on the same cells: a lane choice under D3 (`docs/DESIGN.md`,
`[verdicts.consumers]`). Three picks escalate in band instead:

- `crates/geom-brep/src/edge_nurbs.rs:823` `pick_arc_rim`
  (`pcurve_iso_arc_row_direction`, :833) and `first_within` (:880).
  Zero takes `IsoArc`, and definite takes the fitted image. An in-band
  reading, or an exact arc rim's certificate that escalates
  (`exact_arc_rim`, :1064), refuses `ArcRimEscalated` with a tolerance
  offer.
- `crates/topo/src/pcurves.rs:1268` and :1330 (`probe_pick` in
  `iso_schedule`: `pcurve_iso_wall_seam`, `_line_rim`, `_seam_column`).
  Zero takes `IsoLine`, and definite takes the measured rows and then
  `derive_general_image`. An in-band reading is deferred (:1389, "an
  escalated candidate outranks the image") and refuses
  `PcurveMintError::Escalated`.
- `crates/topo/src/pcurves.rs:1195` (the `arc_rim` closure, used on the
  circle's chart, scaffold and on-pair arms). An in-band reading
  escalates. A definite reading (`Ok(None)`) refuses `IsoUnsupported`,
  which is neither the general lane nor a lever-only "not yet". The
  intersection-circle path already falls through to the general image.
  Near :1344, an arc rim certificate that escalates keeps the closed
  form on an undecided reading.

## Fix

In band takes the lane definite takes: the fitted or general image,
with the pick recording nothing. Where the arc rim has no general lane,
refuse lever-only "not yet" on both arms. Each site needs a row with an
in-band pose, and a mutant that restores the escalation and reds it.
