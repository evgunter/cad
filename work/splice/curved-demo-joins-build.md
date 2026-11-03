---
id: curved-demo-joins-build
kind: issue
title: The curved demo joins build: klein walls 3 and 4, teapot walls 2 and 3, lily walls 2 and 8, each tracked at the stop it reaches today
status: parked
opened: 2026-11-03
priority: P0
cost: C
blocked_on: [the-skin-cuts-an-arc-wall-at-each-sub-arc-joint, a-spline-face-has-no-subtract-or-intersect-lane, a-projected-boundary-on-an-analytic-chart-has-no-volume-lane, a-spline-wall-cannot-say-whether-it-holds-a-pierce-point, teapot-wall-3-at-1e-6-floors-the-meridian-against-a-spout-wall, c5-plane-torus-cone-cylinder-arms, cone-torus-rung-three-arm-waits-on-the-cone]
---


**What "curved booleans work" means, measured.** The tour's curved demo
joins are booleans a person authors on everyday curved parts: a swept
loop joined to a revolved bulb, a handle and a spout joined to a pot, a
flower welded onto a stem. Each is attempted on every tour run and
pinned by its typed refusal (`demos/tour/src/walls.rs`: a wall whose
refusal changes panics, so the pin cannot rot). This row keeps, in one
place, the stop each join reaches today and the rows known to stand
behind it. Its `blocked_on` is the set of current stops, so lint warns
when one closes. That warning is the cue to re-measure the join and
update the row.

Filed 2026-11-03, asked for in chat: the joins' rows were spread over SECT,
SECTARMS, SPLICE, LATHE and SPIRICARM, and the stops of three joins sat
at P1 or P2 under a P0 frontier (`work/README.md`, Priority: a blocker
inherits the band of what waits on it). Those rows were re-banded P0 in
the same change.

## The joins

| join | pin | stops today at | then, as far as is known |
|---|---|---|---|
| **klein wall 3**: the swept loop ∪ the revolved bulb | `demos/tour/src/klein.rs` `wall3_pinned` | `SplineCrossingUnsettled { why: Crease }`: the bulb's cone seam crosses the loop's half tube on a double knot, `lathe/the-skin-cuts-an-arc-wall-at-each-sub-arc-joint` | the NURBS × cone section, `sectarms/a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm` (its cone stage), behind `sect/the-cone-has-no-implicit-enclosure-or-metres-conversion` |
| **klein wall 4**: bulb ∖ loop, the self-intersection cut | `demos/tour/src/klein.rs` `wall4_pinned` | `CurvedPairUnsupported { op: Subtract, site: RevertRoster, kind: Nurbs }`, `splice/a-spline-face-has-no-subtract-or-intersect-lane` | everything wall 3 meets, under ∖ |
| **teapot wall 2**: handle (torus) ∪ vessel (sphere) | `demos/tour/src/teapot.rs` `wall_probes` | the union builds, and its result's volume refuses on the sphere face's fitted-chord trim, `spiricarm/a-projected-boundary-on-an-analytic-chart-has-no-volume-lane` | the handle's torus face, `spiricarm/a-torus-wall-trimmed-by-a-spline-rim-has-no-volume-lane`; then re-cut the handle's 11.2 mm overshoot (the probe's retire note) |
| **teapot wall 3**: lofted spout ∪ vessel | `demos/tour/src/teapot.rs` `wall_probes` | past its seam split since PR 6441 and past the held pierce since PR 6435 (2026-11-03). Default ε and 1e-12: `GermFrameUnsupported { a_kind: Sphere, b_kind: Nurbs }`, the belly's sphere against the spout's outer wall in the join, `sectarms/a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm`. 1e-6: `SplineCrossingUnsettled { why: Floor }`, the belly meridian against a spout bore wall, `splice/teapot-wall-3-at-1e-6-floors-the-meridian-against-a-spout-wall` | the NURBS × sphere section (the SECTARMS row above) |
| **lily wall 8**: the lofted leaf grown out of the stem's tube | `demos/tour/src/lily.rs` `wall_probes` | `GermSectionSpiric { RoutesToGeneralRung { "plane×torus" } }`: the tube's torus against the leaf's tilted cap, `sect/c5-plane-torus-cone-cylinder-arms` | the NURBS × torus section, `sectarms/a-nurbs-face-against-a-torus-has-no-section-arm` |
| **lily wall 2**: the lantern (cone neck) welded onto the arch (torus) | `demos/tour/src/lily.rs` `wall_probes`, `demos/tour/tests/lily.rs` `the_weld_refuses_at_the_cone_torus_pair` | `CurvedPairUnsupported { site: InteriorLoopGuard, Cone, Torus }`: no certificate arm for a cone against a torus off its axis, `sect/cone-torus-rung-three-arm-waits-on-the-cone` | itself parked on the cone row; past it, unmeasured |

Not joins, so not on this row: the tour's other walls are other verbs
(lily 3 and 4, an oblique extrude and a non-uniform scale), meshes
(lily 7, torusvessel 1) or a fillet (klein 2).

"Then" is what the rows' own measurements say comes next, and it has
been wrong before: lily wall 8's stop moved at least five times
between 2026-10-26 and 2026-11-01 (`work/splice/log.md`). Read it as a lower
bound.

## The ground behind them

The joins are witnesses. What they fail on is a few tables of
kind-pair arms, and most of the remaining curved-boolean work is
filling those tables. Read the rows here before estimating how far the
frontier is.

- **C5's section arms** (`crates/geom-brep/src/intersect.rs` `route`).
  Unimplemented today:
  - cone × cone, cone × sphere and cone × torus, all behind
    `sect/the-cone-has-no-implicit-enclosure-or-metres-conversion`
    (cone × sphere in general pose has no row of its own);
  - NURBS × cylinder, cone, sphere and torus, which is
    `sectarms/a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm`
    (X, staged) and its torus row;
  - plane × torus off the axis's planes (the general spiric), which is
    `sect/c5-plane-torus-cone-cylinder-arms` (X).

  `Approx` is off every roster by decision (`reduce.rs`,
  `boolean_arm_exists`), not by gap.
- **The join frames past a section**: cone against an oblique cylinder
  or cone (`sectarms/cone-germ-pairs-against-a-curved-face-have-no-section-frame`),
  skew cylinders (`sectarms/skew-cylinder-germ-pair-has-no-section-frame`).
- **The ops' rosters**: ∖ and ∩ admit no spline face
  (`splice/a-spline-face-has-no-subtract-or-intersect-lane`).
- **The result gate's volume lanes**: a union that builds is still
  refused when a face's trim has no quadrature. Cylinder and sphere
  faces trimmed by a fitted chord have none, so every cylinder × sphere
  boolean that builds refuses. Torus faces trimmed by a spline rim have
  none either. Both are the two SPIRICARM rows above.
- **The census at the result**:
  `restread/census-cross-solid-curved-pairs-undecidable-on-shell-results`
  (X), with three parked P0 rows behind it (REACHHOLD's door census and
  FLUSH's two).

## Keeping this row true

- A PR that re-pins one of these walls updates the join's row in the
  table above, and `blocked_on` if the stop's row changed. The wall
  probe panics on any change of refusal, so such a PR cannot miss that
  it moved one.
- A PR that retires a wall (the join builds) deletes its table row and
  says so in its body. The row closes when the table is empty.
- A new curved demo join joins the table when its scene pins it.

Moved from `work/issues/` to `work/splice/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.
