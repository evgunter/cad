---
id: a-plane-poses-in-plane-origin-enters-the-mate-coset
kind: issue
title: A Plane (or Axis) pose's representative origin, which the kind forgets, enters the mate solve's frame and coset
status: open
opened: 2026-10-19
priority: P1
cost: M
design: true
---


Found by a FORK-REST designer (INTENT stage 3 D, 2026-10-19), from code reading; not measured.

A `Plane` pose forgets in-plane motion and an `Axis` pose a slide along itself (D10 `[variables.poses]`), yet the representative frame the solve reads carries the face's own point as its origin: `mate::solve::frame_of` and `mate_coset` (`crates/editor-core/src/mate/solve.rs`) feed that origin into the map, where exact arithmetic would cancel it again. In f64 it does not quite cancel, so a copy stood on its own start cap against the world's xy solves to the identity only to rounding. On a box lane a parameter that moves the in-plane origin (and not the plane) widens the map, though the plane, and so the placement, does not move. That widening is the "known gap" D's handoff records against `mate::anchor::Anchors::fixed`.

The suggested direction: read each pose kind through a canonical representative before the coset (a plane by its normal and the foot of the perpendicular from its space's origin; an axis by its direction and the foot from the origin), so a kind's forgotten freedoms cannot reach the map. `[coincidence.structural]` already reads poses modulo their kind's symmetry. To measure: a stood copy's map at f64, Interval, Dual and Sym under such a representative, and the m10 box maps' widths.
