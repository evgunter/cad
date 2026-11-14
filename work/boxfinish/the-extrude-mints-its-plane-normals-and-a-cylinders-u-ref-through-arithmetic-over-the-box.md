---
id: the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box
kind: issue
title: The extrude mints its plane normals and a cylinder's u_ref through arithmetic over the box, so tier 3 reads a tilt and a length the geometry does not have
status: open
opened: 2026-11-14
priority: P0
cost: H
design: true
refs: [6371]
---


Found by BOXFINISH's M1 measurement (2026-11-14, main `b363f5fa`, the
switch's box arm turned off on the probe branch
`boxfinish/probe-switch-off`; the full report is `probe/M1-REPORT.md`
there). Split from
`the-side-reader-cannot-read-a-solids-orientation-over-a-box`, which
named the side reader for all of it. This row retires the box-scalar
switch (`topo::AtRestPolicy::leaf`) with the others.

## Measured, with the switch off

**The slab** (`m10_3_driver_interval::a_macroscopic_box_certifies_at_the_shipped_depth_budget`,
depth 1 ± 0.05, at `Sym<Interval>`). The extrude's finish refuses
`OrientationUndecided { reading: Volume { margin: Enclosure { -1.617, 4.129 },
predicate: "positive_volume_exact" }, aimed: 6, set_aside: 6 }`, and the
drive certifies in four leaves, by the volume, where it certified in one.

- The top cap's stored normal is `([-0.2, 0.2], [-0.2, 0.2], [0.962, 1])`.
  Its true normal is (0, 0, 1) at every depth in the box. It is minted
  by `geom_brep::newell_plane`, which `sweep::extrude` calls for the
  caps (`cap_plane`) and for the line walls: a centroid Newell sum over
  corners that carry the box reads `z − z` as `[-0.1, 0.1]`.
- The four side walls' stored normals are exact in direction and
  `[0.818, 1.222]` in length.
- Every aimed ray is then set aside on `side_at_infinity_denom`
  (`[-1.808, 5.978]` and `[-12.29, 7.28]` against a (1e-9, 1e-8) band).
- **Substituting a midpoint normal by hand, together with a ray that is
  constant across the box,** the root reads by its rays: one leaf,
  836/969 decisions, the test passes in 0.15 s. Either substitution
  alone does not get there (the ray's half is the side-reader row). The
  substitution measures where the width enters; a midpoint normal is
  not sound where the true normal varies.

**The derived-frame rows** (`m10_derived_frame_interval`, both, at 5e-2
only) refuse at the widened CUBE, node 17, with the same payload shape
(volume `[-5.163, 15.214]`, cap normal ±0.1), not at the boss and not
at the placement. The boss finishes every time. With both substitutions
the twin row passes.

**The tour's certified stop** (`demo-tour certified`: 0.0000% certified,
99.4607% refused, all at the budget) is not held by check 7. The plate
finishes in every replay and its side reads by rays. The refusal is
check 1 on the two hole extrudes:
`SurfaceDatumUndecided { kind: Cylinder, datum: URef, measure: Length,
end: Upper, margin: Enclosure { -2.971e-4, 2.412e-4 }, band: (1e-9, 1e-8) }`,
raised by `topo::validate::surface_datum_errors` over
`Surface::representability_margins`. The datum is minted by
`sweep::extrude`'s arc arm as `u_ref: rim.normalize()`. Inferred: a
vector divided by its own length over a box is not recognisably unit,
and the width scales with the box, so 512 leaves never reach 1e-9.

## What it is evidence of

Three data a door knows by construction (a cap's normal is its sketch
plane's; a wall's is fixed by its edge and the sweep direction; a
cylinder's `u_ref` is unit) are re-derived from corners or by a division
that carries the box, and tier 3 then reads the box's width as tilt or
as length. `check-7s-quadrature-refuses-interval-nurbs-walls-with-inexact-pcurve-ends`
is a fourth of the kind: a chart end that is an exact 0 or 1 goes
through `(1/span)·t1`.

## The test that says it is closed

With the switch's box arm off, the slab row certifies in one leaf, both
derived-frame rows pass at 5e-2, and the tour certifies its share, each
with its main-era assertion.
