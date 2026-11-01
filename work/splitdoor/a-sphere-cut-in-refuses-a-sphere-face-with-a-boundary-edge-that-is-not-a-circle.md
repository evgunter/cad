---
id: a-sphere-cut-in-refuses-a-sphere-face-with-a-boundary-edge-that-is-not-a-circle
kind: issue
title: ops::apply_cut_ins refuses a sphere face whose boundary holds an edge that is not a circle, so a sphere cut by a cylinder or cone refuses its cut-in
status: parked
opened: 2026-10-28
priority: P1
cost: M
blocked_on: [a-projected-boundary-on-an-analytic-chart-has-no-volume-lane]
---


Found on review of PR 5940 (`splitdoor/tilted-plug-rosters`), a
lines-and-circles roster its sweep missed: the sweep grepped `match`
arms on `Curve3::Line`/`Circle` and one-line `let … else` on them, and
this site is a `let geom::Curve3::Circle { .. } = … else` spread over
several lines in `crates/topo/src/boolean/ops.rs`.

## What

`ops::apply_cut_ins` cuts a sphere face along a meridian half-plane of
its chart, reading every boundary arc's crossings with the meridian's
plane through `circle_roots::first_harmonic_roots`
(`FirstHarmonic::circle_off_plane`). A boundary edge whose carrier is
not a circle refuses `FallbackExtentUnsupported { what: "a boundary
edge of the sphere face is not a circle" }`. A plane meets a sphere in
a circle, but a cylinder, cone or torus does not: a sphere face trimmed
by a bore (a ball with a hole drilled through it) holds a quartic edge,
and any cut-in on that face refuses. Not driven to a body here.

## Direction

The cut needs each boundary arc's crossings with one plane. For a
conic, that is the conic–plane meeting `splitting::plane_crossing_lane`
already reads (`ConicPlaneMeet`); for a spline or spiric, the crossing
layer's own plane rung. Build the fixture first (a sphere face bounded
by a cylinder section, taking a cut-in), then route the roots by kind.

## Measured (2026-10-29, at `26202d6f`)

The refusal is not reachable through the public doors today: no
at-rest body holds a sphere face with a non-circle edge, because the
result gate refuses the drilled ball before any cut-in is asked.

Fixture: the unit ball (`sweep::test_support::ball_poled_y`, seams in
`z = 0`) less a radius-0.15 bore, then a slab 0.985 from the centre
toward latitude 10° at azimuth π/2 (the banded-ball cut-in of
`snowman.rs`). The subtract that drills the ball refuses at ε 1e-6,
1e-9 and 1e-12 alike:

- bore along `z` through `(0.4, −0.4)`, its holes rings of the
  half-bands: `ResultInvalid { [VolumeUncomputable { source:
  RingOnCurvedFace { face } }] }`, raised by
  `topo::props::face_flux_about`'s ring gate;
- bore along `x` through `(y, z) = (−0.4, 0)`, across both seams:
  `ResultInvalid { [VolumeUncomputable { source: Face { source:
  QuadratureUnsupported { what: "curved-cut face half-edge carries a
  pcurve with no harmonic form on an analytic chart …" } } }] }`,
  raised by `topo::props::quad_lane` (the projected-image arm).

A conic on a sphere is a circle, so the conic route the direction
names adds nothing a sphere face can hold; every non-circle edge on a
sphere is a spline or spiric section whose pcurve is a projected or
fitted image, and `quad_lane` refuses a sphere chart outright ("conic
trim on a cone/sphere/torus chart"). An off-axis cone or torus bore is
the same class. The cut-in is therefore downstream of
`work/spiricarm/a-projected-boundary-on-an-analytic-chart-has-no-volume-lane.md`
(the sphere lane and its rings). `sphere_region::sphere_face_region` admits
circles only by the same roster (`None` on any other edge, read as
`unread()` in `apply_cut_ins`' piece choice), so a fix here needs that
reader too.

## Parked (2026-10-29)

Park this behind SPIRICARM's `a-projected-boundary-on-an-analytic-chart-has-no-volume-lane`, per the measurement above. Every non-circle edge a sphere face can hold carries a projected or fitted pcurve. The drilled ball that would take the cut-in refuses earlier, at `props`' volume gate, so a routing fix here has no fixture that can reach it at tier 3. When the blocker lands, this row owes both the roots routing in `apply_cut_ins` and widening `sphere_region::sphere_face_region` past its circle roster. (SPLITDOOR orchestrator)
