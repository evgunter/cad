---
id: a-torus-touch-is-read-by-its-near-band
kind: unit
title: A torus touch is read by its near band: every Zero tangency on a closed-form torus arm whose level function has only the forced critical points is a touch, saddles and parallels included
status: closed
opened: 2026-11-23
priority: P1
cost: H
refs: [a-torus-touch-along-a-whole-parallel-refuses-r-tan, a-torus-touch-at-a-definite-non-elliptic-point-refuses-r-tan, 4159, 3978]
closed: 2026-11-24
pr: 7611
branch: orbit/torus-touch-build
---


Carries both torus-touch rows, which are one question:
`a-torus-touch-along-a-whole-parallel-refuses-r-tan` (a touch along a
whole parallel) and
`a-torus-touch-at-a-definite-non-elliptic-point-refuses-r-tan` (an
isolated touch off the tube's elliptic half). Weighed by two designers,
who converged on the design below.

## What the touch argument needs

The section certificate's touch (`boolean::section_cert`, "A touch")
clears a tangency decided `Zero` on a pair with no event when one point
`at` of the contact places `Out` of either face. Its argument needs
three things and nothing about curvature, smallness or isolation:

- the pair is silent (no edge event);
- the **near band** `W` — the points of the torus whose level lies
  within the decided margin `m` of the partner's level — is connected
  and holds `at`;
- **S′**: the sweep records or refuses every edge that comes within
  about `2m` of the other carrier, over the other face.

Then the set of points of `W` in one face and over the other is open
and closed in `W`, so it is all of `W` or empty, and `at` placed `Out`
makes it empty.

The **level function** is the partner read on the torus: a plane's
height, the distance from a sphere's centre, the distance from a wall's
axis or a coaxial partner's core. Where it has only the critical points
every function on a torus must have (one minimum, one maximum and two
saddles; or a minimum circle and a maximum circle when the partner is
coaxial or square to the axis), the band about any critical level is
one piece — a disc at an extreme, an annulus about a parallel, a band
about a figure eight at a saddle — whatever the other critical values
do. The elliptic margin, the strict-extreme condition and the
next-critical-value gap are proxies for that, and go.

## The design

- **The rule.** On the closed-form torus arms — the plane, a sphere, a
  coaxial wall, sphere, torus or cone, and a wall parallel to the axis
  whose axis misses the tube — every tangency decided `Zero` whose
  level function the arm decides has only the forced critical points
  is a touch. The parallel-axis wall decides that as a margin (its
  axis misses the tube); the other arms have it by construction.
- **Section value (geom-brep).** A tangent component states its locus
  — a point, or the parallel `tangent_parallel` already builds for the
  tangent edge (C7), so one construction serves both — and whether its
  near band is one piece. A marched or general pose never claims it.
- **Certificate (topo).** `Touch` is admitted on that claim, not on the
  section being a point. `at` is one point of the locus, taken from the
  torus frame's own reference direction: not from the coordinate axes,
  and not by dividing by a tilt in band (PR 4159's MINOR-1).
- **Stays R-tan:** a margin left undecided; a marched or general pose;
  a pair with an event; a contact inside both faces (a genuine contact,
  C1's point, curve or crossing touch held by both faces: building it
  is the contact design's, not this certificate's); a parallel-axis
  wall whose axis pierces the tube.
- **Docs.** "A touch" is restated as L1 over the near band, with S′
  stated beside L1's S and the distance it needs, and the cases named
  in C1's terms. `Touch::at`'s "the carriers share `at` alone" goes.

## Build

- The plane arm reads geom-brep's own plane × torus classification
  (`plane_torus_section`, `plane_torus_tangency`, which has the
  axis-normal reading) rather than a second one in the certificate.
- **S′ rows**, one per root lane the touch leans on, each a rim that
  passes within half the zero band of the torus around an off-face
  touch without crossing it, which must refuse or record, never build.
  The line × wall "belly" branch in `boolean/reduce.rs` (a `Miss` read
  as no event after a clearance decided `Zero`) is exercised, and is
  sound only if that door's `Miss` is a decided-positive length.
- **Rows that move**: `a_touch_on_the_inner_equator_refuses_off_the_faces`
  (its ball pose and its brick pose build),
  `torus_tangencies_off_an_elliptic_extreme_are_not_touches`,
  `coaxial_partners_meet_the_torus_in_parallels`,
  `a_plane_tangent_to_the_tube_off_its_outer_half_refuses_as_a_tangency`,
  `torus_and_ball_every_class` (its comment calls the ball's contact in
  the hole a saddle: it is a strict minimum of the ball's distance).
  Off-face poses that build after: a hub flush with a ring's top, a
  sleeve standing above the ring's widest circle, a block in the hole
  with its face off the touch, a trimmed ball in the hole, a ball
  inside the tube against its inner half.

## Scope: strict extremes here, saddles after the line × torus door (2026-11-23)

The build lane audited every root door the touch leans on (ε = 1e-9,
K = 10). Each holds S′ except line × torus: its count-0 `Miss` is
decided on the quartic's discriminant, not on a clearance, so a line
passing 1e-11 above the torus is silent
(`work/germ/the-line-torus-door-misses-silently-within-the-band.md`).
At a strict extreme the edge that parts a section component from the
touch lies on the solid's side, where that door refuses within about
4ε; at a saddle the carriers cross along the lobes and the parting edge
can stand just outside, where it is silent. So this unit admits strict
extremes (points and parallels), with the elliptic margin and, where
the arm decides perfectness, the gap margin gone; saddle tangencies
keep R-tan. Admitting them is
`torus-saddle-touches-are-read-by-their-near-band`, parked on the door.

## Scope narrowed again: point extremes only (2026-11-23)

PR 7611's dual review found that a PARALLEL touch read off the exact pose
is unsound. The near band stays connected there, but `at` taken at the
torus's `u_ref` can stand on the gap side of a crossing, and the edges
parting the crossing from it are straight rulings outside the solid,
where line × torus is silent. The design's "on a parallel any point gives
the same verdict" holds only in the exact pose. What S′'s side argument
needs is `at` at the deepest point of the ACTUAL pose. So this unit admits
strict point extremes, with `at` read from the actual pose on every arm.
Parallels wait on the line × torus door with the saddles:
`torus-parallel-touches-wait-on-the-line-torus-door`.

## Ev's ruling (2026-11-24, PR 7587)

"if it's sound i'm happy with it": the near-band reading is approved in full, saddles and parallels included, wherever it is sound. Today it is sound at strict point extremes, which this unit builds (PR 7611). The saddle and parallel units stay parked on the line × torus door, because the door's silent answers are what stands between them and soundness.

## Closed (2026-11-24, PR 7611)

This unit builds strict point extremes:
- The elliptic margin is gone.
- `at` is read off the actual pose on every arm: exact for the plane and the sphere, first order in the tilt for the parallel wall.
- One `Section::touch` constructor.
- S′ is stated beside L1's S and pinned door by door, inside and outside. The line × torus silent rows name GERM's door row.

Off-face touches build: a ball in the hole, a ball in the tube, a plane at the top point, a wall in the hole on the inner equator, a rod tilted within the band. Off-exact poses with the dip on the far side refuse.

Review: dual, two reviewers. Round 1 was NOT-MERGEABLE: a parallel touch read off the exact pose built across a crossing, so parallels were withdrawn and are parked. Round 2 was APPROVE-WITH-FIXES from both, and a last pass followed on my read. Saddles and parallels wait on the door (`torus-saddle-touches-are-read-by-their-near-band`, `torus-parallel-touches-wait-on-the-line-torus-door`).
