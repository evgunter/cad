---
id: an-ellipses-general-lane-is-written-on-no-chart
kind: issue
title: An ellipse has no general (projected) lane on any chart: an in-band ellipse, or one within the band of a chart that holds none, refuses not yet
status: closed
opened: 2026-11-03
priority: P1
cost: M
closed: 2026-11-04
pr: 6524
---



Filed by PCERT unit E4 (`pcert/circle-general-lane`), which gave a
circle its general lane on the cylinder, cone and torus. The unit's
brief left the ellipse out unless it fell out of the net path for free;
it does not, but it very nearly falls out of the circle's path, which
is why the cost is M rather than H.

## What

`projected::has_lane` admits no ellipse on any chart, so every ellipse
D3's lane choice sends to the general lane refuses lever-only "not yet"
(`PcurveCheck::ChartClass` in band, `NoGeneralLane { gap: Unwritten }`
definite). The ellipses that reach it today:

- an ellipse within the band of a **sphere** or a **torus**, which hold
  no ellipse: the sphere and torus arms of `derive_class`
  (`pcurve_cache.rs`, "A line or an ellipse the test did not show off
  the sphere", and the torus arm's same sentence) send it to the general
  lane, definite;
- a **cone** section whose orientation reads Zero or in band
  (`pcurve_chart_orientation` in the cone arm's `Curve3::Ellipse`
  case), or whose nappe reads in band (`pcurve_cone_chart_nappe`);
- a **cylinder** section whose orientation reads in band (the cylinder
  arm's moving case, `pcurve_chart_orientation`).

`crates/geom-brep/README.md`'s C4 page says every class of carrier a
chart can hold has a route into a certified row; the ellipse is the
class that still has none.

## Why it is close

The circle's lane, as E4 wrote it, reads only the carrier's harmonic
form `c + a·cos t + b·sin t` (`carrier_harmonic`), never `|a| = |b|`:

- `projected::FramedCarrier::Circle` stores the centre and the two
  coefficient vectors, and `ProjectedImage::frame_box` boxes them with
  `harmonic_span_box`;
- check 4's fidelity is `|Δc| + harmonic_sup(Δa, Δb)`;
- the incidence terms, `sphere_circle::off_sphere_coefficients` ("exactly,
  for any `a`, `b`") and `circle_chart::form_sup`, expand the chart's form
  for any `a`, `b`, and `circle_chart::rho_floor` likewise.

So the lane is probably `has_lane`'s ellipse arm, `projected::project`'s
`Curve3::Ellipse` arm into the same framed carrier (renamed for what it
is, a harmonic conic), check 1's carrier gate, and rows: an in-band cone
section, an ellipse within the band of a sphere, and the search in
`pcurve_cache::circle_lane_rows` widened to ellipses. Check the mesh
and topo readers of `FramedCarrier::Circle` (`mesh::chords`,
`mesh::memo`) when it is renamed.

## Closed

PR 6524. An ellipse takes the projected lane on the sphere, cylinder,
cone and torus (`projected::lane_of`'s `Lane::Conic`, the framed
carrier renamed `FramedCarrier::Conic`). The audit found no hidden
`|a| = |b|` in the circle lane's terms. It found that the charge read
`Conic::speed_hi` signed, so a conic stored with a negative semi-axis
would have been under-charged. Such a record breaks the
representation's invariant, so the edge certificate now refuses it,
typed (`CertifyError::SemiAxisNotPositive`, a defect), before any speed
reader. A cylinder conic whose orientation reads Zero now takes the
general lane, where it read a winding of `0` and ended as a measured
miss; the moving arm's winding is typed so a zero cannot be written. The cylinder's "in
band" orientation is unreachable by a certifiable carrier, and the
cone's is reachable only in a sliver; the PR states both arguments.

Rows and evidence: `pcurve_cache/ellipse_lane_rows.rs`, and a 50-digit
sweep (`scripts/oracles/ellipse_lane_50_digit.py`) with 0 under over
580 rows at both scalars.

Residue:
- `conic-speed-hi-reads-the-signed-larger-semi-axis`, claimed into this
  directory and closed here on the invariant;
- `work/tang/a-sphere-ring-bounded-by-an-ellipse-is-unread-by-the-chord-join.md`;
- `work/tesssize/an-ellipse-edge-on-a-sphere-or-torus-face-skips-the-lattice-step.md`.
