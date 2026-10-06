---
id: shell-of-a-lofted-body-meets-the-oblique-corner-on-a-slanted-spline-seam
kind: issue
title: the per-chart offset door moves a cap rigidly, so on a loft whose seams slant the moved corner leaves the seam and shell refuses ReanchorOffCarrier before any wall fit
status: open
opened: 2026-10-06
priority: P1
cost: H
refs: [shell-refuses-every-lofted-body-at-a-wall-seam-carrier]
---


Filed by the `shell/lofted-wall-seam` lane, which gave
`replace_face::plan_reanchors` a NURBS re-anchor and measured where
the lofted bodies go next. This is the wall the twisted loft and the
vase now meet; it was hidden behind the carrier-lane refusal.

## Measured

`topo::shell(&body, 0.05, Tol::witness())` at the default ε, release
build:

| body | refusal (on a cap) | gap |
|---|---|---|
| twisted loft, 0.3 rad (`common::approx::twisted_loft`) | `ReanchorOffCarrier` on a wall–wall seam | 0.019466 m |
| circular vase (three circle sections r = 1, 1.3, 1, degree 2) | `ReanchorOffCarrier` on a wall–wall seam | 0.025370 m |
| straight square prism (`common::approx::prism`) | passes the re-anchor; see `work/iso/nurbs-iso-derive-line-rim-arm-refuses-an-interior-row.md` | — |

The twisted loft's gap is exactly `0.05 · sin(slant)`, where the slant
is the seam's chord against the cap normal (the seam drifts 0.4227 m
over its 1 m height): `crates/sweep/tests/encl_curved_loft_shell.rs`,
`shelling_the_curved_loft_refuses_at_the_oblique_cap_corner_before_any_fit`,
asserts that to 1e-12.

## Why

`replace_face_offset` moves ONE chart and keeps every neighbour
(`crates/topo/src/replace_face.rs` module docs, "What moves and what
does not"). A plane cap's boundary edges are transported rigidly by
`d·normal` (`transport_curve`), so the cap's corner lands `d` along
the cap normal. On a loft whose wall seams are parallel to that normal
(the prism) the corner stays on the seam and re-anchors; where the
seam slants it does not, and the gate `offset_reanchor_on_carrier`
refuses. The refusal is right: the cap's new boundary on an untouched
slanted wall is the moved plane ∩ the wall, not the old rim
translated, so the rigid transport would build the wrong body.

This is the #1081 oblique-junction class
(`ReanchorOffCarrier`'s docs), met on a plane × spline-wall corner.
`offset_planes_together` solves such corners only when every face is
a plane, and `offset_charts_together` only for coaxial charts.

## What a fix has to decide

A cap offset beside an untouched spline wall needs its rim re-derived
as `plane' ∩ wall` (the plane × NURBS lane already certifies that
class, `geom_brep::plane_nurbs_limbs`) and its corners at `plane' ∩
seam`, i.e. the corner slides ALONG the seam rather than along the cap
normal. Whether that is a new arm of the per-chart door (re-intersect
instead of transport, for a plane against a spline neighbour) or a
spline-wall member of the simultaneous doors is the design question.
Behind it the walls still meet the fit's budget at the default ε and
O4's `FittedBoundaryUnsupported`, measured in
`shell-refuses-every-lofted-body-at-a-wall-seam-carrier`.
