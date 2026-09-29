---
id: cone-chart-trim-reads-a-tilted-section-as-its-vertex-window
kind: issue
title: cone_chart_trim folds the slant window over boundary vertices with no edge-class check, so a cone face bounded by a tilted planar section would be misread; no door builds one today
status: open
opened: 2026-09-26
---


Found by CONTACT-3's class sweep. That unit fixed the cylinder half of
the same shape (`wall_outline`, `crates/topo/src/boolean/solid_contain.rs`).

## The defect, by reading

`cone_chart_trim` (`solid_contain.rs`) resolves a cone face's slant window
through `cone_slant_window`, which folds `v` over the outer cycle's
VERTICES. Neither function asks which carrier each edge is on. The
window is the face's own region only when every edge is a rim (a
slant iso-line) or a generator. A plane that is not perpendicular to the
axis meets the cone in a conic whose slant varies along the edge, and
its extreme sits in the edge's interior. The window would then over-cover
the face on one side of the section and under-cover it on the other.
That is the cylinder defect exactly, and it answers from the certified
walk rather than refusing.

The sphere arm does not share it: `sphere_chart_trim` checks each edge's
class (rim or meridian great circle) and answers `None`, which becomes
`PartialSphereFace`, for a tilted circle.

## Why it is unmeasured

No door on this tree builds a tilted cone section:

- `topo::splitting::split` gates the kind (`CurvedBooleanUnsupported`,
  kind `Cone`).
- The boolean refuses the plane×cone pair (`CurvedPairUnsupported`).
- STEP import refuses the adopted body at the at-rest gate: `TierInvalid`
  / `VolumeUncomputable`, `QuadratureUnsupported` "conic trim on a
  cone/sphere/torus chart". Measured on a frustum (radius 1 at z = 0,
  0.5 at z = 1) cut through `(0, 0, 1)` at tilts 0.3 and 0.6 rad.

The tilted sphere cut is refused the same way at import
(`NotIsoRectangle`, `props_rim_axis_parallel`).

## What closes it

The cylinder's discipline carries over. Ask an edge-class predicate
before reading the window as a region. A plane meets each generator
segment of one nappe at most once, so a section on a cone is a graph
over azimuth, and membership is the window cut by each plane's side
along the generator. The same parity argument `wall_outline` states
applies. Outside that class, refuse `PartialConeFace` or confine a
refusal the way `WallOutline::Unsupported` does.

The trigger to do it is whichever lands first: the props lane gaining a
cone conic-trim flux (which opens import), or a split/boolean cone arm.
Each opens a door to this face.
