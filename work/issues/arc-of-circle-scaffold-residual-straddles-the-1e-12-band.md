---
id: arc-of-circle-scaffold-residual-straddles-the-1e-12-band
kind: issue
title: EdgeCurveSpec::arc_of_circle's scaffold residual encloses wider than the 1e-12 band on a computed arc
status: open
opened: 2026-09-29
priority: P3
cost: M
refs: [kevs-fan-merge-needs-a-re-describing-kill-door]
---

## What

`EdgeCurveSpec::arc_of_circle` (`crates/geom-brep/src/certify.rs`)
describes a circular arc by the scaffold
`MappedCurve::RevolvedPoint`: the start point rotated about the
carrier's axis by the swept angle `t1 - t0`. Certification compares
the circle carrier with that rotation (`CertCheck::MappedSource`).
Where the arc is COMPUTED — its centre an expression, its sweep an
`atan2` over directions read off that centre — the certified scalar
encloses the rotation much wider than the carrier, and the residual's
enclosure straddles the zero band at `CAD_TOLERANCE_EPS=1e-12`: the
certification escalates `Indeterminate` where f64 answers ulps.

## Measured

Found by `kevs-fan-merge-needs-a-re-describing-kill-door` (PR 3161).
The blend's two closure kills (`crates/sweep/src/blend/surgery.rs`,
`"rim closure kev"` and `"annulus closure kev"`) first handed
`Body::kev_describing` the meridian arc under this scaffold:

- the interval 1e-12 rows went red with `RebasedCarrier { error:
  Escalated { check: MappedSource, … Indeterminate { margin: [0,
  1.1e-12 … 2.0e-12], band.zero: 1e-12 } } }` — five sweep rows
  (`fillet_h4_concave_rim_interval`, both `fillet_h5_hostless_rim_interval`
  rows, `m6_surgery_interval::…composed_die…`, `rim_of_rows_interval`)
  and four `editor-core` interval rows over `die_composed`'s fillet;
- instrumented on the waist fixture: the centre's x enclosure is about
  1e-14 wide and the sweep angle's about 1.0e-12, so the carrier
  encloses the arc's midpoint about 1e-13 wide and the rotation about
  5e-12 wide, and their distance encloses at [0, 3.9e-12];
- the PR's review restored the arc and re-measured: the same five
  sweep rows red, `Escalated MappedSource`, the residual enclosing at
  about 1.3–2.0e-12.

The closure kills now hand the chord (`EdgeCurveSpec::line_between`),
whose carrier and scaffold are one affine expression each and enclose
at [0, 1.07e-14]; the description pass states the arc afterwards
through `attach_contact`, as the band's seam, and the 1e-12 rows are
green with it.

## Why it is a row

Any door that describes a computed arc under this scaffold at 1e-12
meets it: the spec is the conventional description for an
under-determined circular locus (its own docs), so the next caller to
reach for it on a computed circle will escalate on the tight row and
pass on every other. The residual's width comes from the sweep angle's
enclosure multiplying through the rotation, not from the geometry.

## Shapes

- State the rotation so its enclosure does not scale with the angle's
  (the sweep as the rotation carrying the start direction onto the end
  direction, rather than an angle re-fed through `cos`/`sin`).
- Or document the spec as unfit for a computed arc at the tightest
  row, with the chord-then-describe workaround the blend now uses.
