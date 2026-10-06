---
id: nurbs-iso-derive-line-rim-arm-refuses-an-interior-row
kind: issue
title: nurbs_iso_derive's LINE rim arm places a rim only on a boundary row of a spline wall's chart, so a moved cap's rim on an interior row refuses
status: open
opened: 2026-10-06
priority: P1
cost: M
refs: [iso-derivation-arms-assume-an-edge-spans-the-charts-whole-domain]
---


Filed by SHELL's `shell/lofted-wall-seam` lane. Measured, by
execution, on `crates/sweep/tests/encl_curved_loft_shell.rs`,
`the_prisms_inward_cap_offset_re_anchors_its_seams_and_meets_the_interior_row`.

## Measured

The straight square prism lofted through `sweep::loft_body`
(`common::approx::prism`: planar caps, four planar described-NURBS
walls on `[0, 1]` in `v`). `topo::replace_face_offset(top cap, −0.05)`
now re-anchors the four vertical wall seams and re-describes the cap,
then the whole-body pcurve mint refuses:

```
Pcurve { source: Certify { half_edge: <a wall's half of a cap rim>,
  error: IsoUnsupported { what: "the carrier's start point lies on
  neither chart boundary — not a boundary iso of this face's chart" } } }
```

`topo::shell(&prism, 0.05)` refuses the same way on its first cap.

## Why

`crates/topo/src/pcurves.rs`, `nurbs_iso_derive`, the arm for a
`Curve3::Line` rim on a spline wall: the closed form places the rim on
the chart's `v = cv0`/`v = cv1` boundary row, and when that does not
match it measures the rim's two endpoint chart feet
(`derive_chart_foot`) for the `u` map — but still picks the ROW from
`[cv0, cv1]` alone (`side_pick(&row(p0x, plx), &[cv0, cv1])`). The moved
cap's rim lies on the untouched wall at `v = 0.95`, an interior row,
so neither candidate matches and the arm refuses with `no_boundary`.

The seam-column arm above it already offers the foot's coordinate as a
candidate for an interior column; the row arm does not offer `f0.y`.
Same family as
`iso-derivation-arms-assume-an-edge-spans-the-charts-whole-domain`
(the arms assume boundary residency), different arm and different
trigger: that one is a split sub-edge, this one a whole rim moved into
the chart's interior by an offset.

## What it blocks

Every cap offset of a spline-walled body whose seams are parallel to
the cap normal (the straight prism, the M7-8-style extruded walls) —
the first lofted body that otherwise reaches past the re-anchor.
