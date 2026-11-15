---
id: the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box
kind: issue
title: The extrude mints its plane normals and a cylinder's u_ref through arithmetic over the box, so tier 3 reads a tilt and a length the geometry does not have
status: dispatched
opened: 2026-11-14
priority: P0
cost: H
refs: [6371]
branch: boxfinish/sweep-frames
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

## Weighed (fork 268), and re-scoped to the doors

Two designers weighed it and, after one round, agree. The extrude is
handed `profile.plane()`, a rigid frame, drops it in its first
statements, and rebuilds every direction from points placed with it. It
is one act and more than one door:

- `swept::cap_plane` fits a Newell plane to placed corners: the
  extrude's two caps, the loft's two, the partial revolve's two;
- `extrude::side_surface` fits one for each line wall and mints an arc
  wall's `u_ref` as `rim.normalize()`;
- `swept::placed_segment_spec` mints each rim circle's `u_ref` the same
  way and each line rim's director as `(q_to − q_from).normalize()`;
  `strut_carrier` normalizes `w`;
- the revolve's surfaces read one frame (`AxisFrame`), but its latitude
  circles (`revolve/surfaces.rs`, `full.rs`, `turn.rs`) mint
  `rim.normalize()`.

**The unit: the sweep doors keep the frames they are handed.** A cap is
its station's frame (normal ± the frame's normal, signed by the
profile's winding; origin the frame's origin moved by the sweep;
reference direction the frame's first axis). A wall, a rim and a strut
take the frame turning a 2-D direction decided once in the sketch, from
the segment as the profile stores it today. A depth extrusion sweeps
along ± the sketch normal; a vector extrusion's direction is decided
once from the vector. `newell_plane` and the cap-orientation decision
lose their sweep callers. Branch `boxfinish/sweep-frames`.

**What it closes, and what it does not.** The slab and both
derived-frame rows, with the aimed-ray row. Not the tour's stop: an arc
whose centre and endpoints carry the box still gets a quotient for its
`u_ref` until the profile records the direction
(`the-profile-drops-the-unit-directions-its-verbs-made`, waiting on PR
6936). A frame that truly turns over the box is
`check-1-reads-a-turning-frames-length-off-its-bracket`.

**No ruling is needed to build it:** it is `[scalars]`' "a claim … is
built at the lane scalar, from its own inputs" applied to stored
surfaces, and both designers were sure of it. The rule's text is PR
6936's.

**Letter H; review tier: dual (concurrent pair).** It changes what
every extrude, loft cap and partial-revolve cap stores at every scalar
(a cap's origin moves from Newell's centroid to the frame's, so cap
chart coordinates and their pcurve rows move), and it removes a
decision and two refusals.

## Measured (M3, 2026-11-14): a box on an in-plane dimension

Probe row `m3_wide_slab` on `boxfinish/probe-switch-off` (commit
`bba45062`), switch off. A slab of width 1 ± 0.05, depth exact:

- caps: direction exact (0, 0, ±1), length `[0.8182, 1.2222]`;
- the walls y = 0 and y = 1: direction exact, the same length;
- the wall x = W, which the box moves:
  `([0.9623, 1], [-0.2, 0.2], [-0.2, 0.2])`, tilted;
- the wall x = 0: exact.

The drive certifies in four leaves; the root and the halves refuse
`OrientationUndecided` by the volume (`[-1.684, 4.272]`), and the
quarters read by rays, at 8.5 to 8.9 s a leaf (why so slow is not
traced). With the offset boxed too, 0 of 32 leaves certify. So "the
side walls are exact in direction" above is the depth-boxed slab's
case only. The frame gives this unit the caps and the fixed-direction
walls; the moving wall's tilt comes from a line direction recomputed
from boxed ends, which is
`the-profile-drops-the-unit-directions-its-verbs-made`.

## Measured (2026-11-14, the aimed-ray unit's fix pass, PR 6958): the edges' lines too

The extrude door's slab over a depth of 1 ± 0.05, `Sym<Interval>`,
switch on (the leaf as the door hands it on), each line edge's stored
carrier:

- the four bottom edges: direction exact, interval `[0, 1]`;
- the four upright edges: direction `(0, 0, [0.9048, 1.1053])`,
  interval `[0, [0.95, 1.05]]`;
- the four top edges: direction `([0.995, 1], 0, [−0.1, 0.1])` (and its
  three turns), interval `[0, [1, 1.005]]`.

A top edge's true direction is the bottom edge's, exact at every
depth; the stored one carries the depth's box in `z`. The side
reader's edge pass now reads each edge off this line
(`census::line_passes`), and sets a ray aside where its gap from an
edge is undecided inside the edge's span, so this width is what stops
most of the slab's rays: at ± 0.025 only the bottom cap's ray reads.
The reader's residue is
`an-aimed-rays-other-readings-still-carry-a-moving-faces-box-twice`.
