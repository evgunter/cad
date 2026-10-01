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

## 2026-10-01: both triggers fired, measured, and guarded

The plane × cone split lane (`plane-cone-elliptic-section-split-refusal`,
branch `reach/plane-cone-ellipse`) landed both doors at once: the split
admits cones and mints the tilted `Ellipse`, and the props lane computes
a cone face's flux and area in closed form, so a section-bounded cone
face is now a valid body at rest.

Measured before the guard: a frustum (radius 1 at `y = 0`, 1/2 at
`y = 1`, revolved about `y`) split by the plane through `(0, 0.5, 0)`
with normal `(0, cos 0.4, sin 0.4)` — a tilt about `x`, so the
section's height peaks inside its arcs rather than at the seam
vertices. On a 9×9×9 grid of points kept 0.02 clear of both surfaces,
`point_in_solid` answered **34 of 1358 queries wrongly** (every one
`In` for a point outside the half), the rest refusing. The same probe
on a cylinder: 0 wrong, the CONTACT-3 fix holding.

What landed with that lane is this item's own fallback arm, not its
fix: `cone_window_premise` (`solid_contain.rs`), called first in
`cone_chart_trim` and `cone_face_trim`, refuses `PartialConeFace` for
a cone face with any edge that is neither a rim (`Circle`) nor a
generator (`Line`). After it: 0 wrong, 1358 refused. The row is
`sweep/tests/reach_cone_split.rs`
`a_tilted_cone_cut_is_never_misread_by_containment` (red with the
guard removed). What remains here is the fix proper: the section's
side along each generator, the cylinder's `wall_outline` discipline,
so those faces answer instead of refusing.

### Narrowed in the same lane's fix pass

The guard refused every query on a body carrying such a face, the
point `(10, 10, 10)` included. The solid door now reads the face as
`FaceGeo::PartialCone` (`solid_contain.rs`): the face lies within a
ball about the apex (`partial_cone_reach`: vertex distances, and
`|centre − apex| + semi-major` for an ellipse edge), and a query
refuses only for `q` on the double cone inside that ball, or when every
schedule ray meets the cone ahead of `q` inside it. A ray that does
is set aside like a graze. The at-infinity side reads the face's
closed-form volume, which the cone's closed form now supplies for a
trimmed face too.

Measured (fix-pass probe, 11³ grid over `[−3, 3]³`, both halves, four
tilts each): narrowing frustum 10–22 refused of 2662 per tilt, widening
2, upright cone 0, and **0 wrong** throughout. `cone_face_trim`, the
face-scoped door, still refuses. The fix proper is unchanged: the
section's side along each generator.
