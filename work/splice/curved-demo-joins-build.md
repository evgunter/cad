---
id: curved-demo-joins-build
kind: issue
title: The curved demo joins build: klein walls 3 and 4, teapot walls 2 and 3, lily wall 8, each tracked at the stop it reaches today
status: parked
opened: 2026-11-03
priority: P0
cost: C
blocked_on: [a-curved-partners-arc-is-imaged-at-rest-no-finer-than-the-mints-finest-rung, a-loop-seam-meeting-the-neck-cylinder-at-its-edge-reads-in-band, a-torus-face-bounded-by-a-projected-spline-image-has-no-chord-bound, cone-and-sphere-faces-with-conic-or-spline-trims-have-no-trimmed-arm, point-containments-ray-test-has-no-spline-arm, the-chart-accounting-clears-a-thin-torus-only-in-sub-millimetre-cells]
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
update the row. The same set is in code: each join's probe is
`walls::join_wall`, which names its stop rows in
`walls::CURVED_JOIN_STOPS` (`demos/tour/src/walls.rs`), and
`walls::tests::the_umbrella_waits_on_every_join_stop` fails unless this
row's `blocked_on` is exactly the union of those rows. A probe re-pinned
onto a new stop therefore moves this row too, and lint's band rule then
holds the new stop to this row's P0.

Filed 2026-11-03, asked for in chat: the joins' rows were spread over SECT,
SECTARMS, SPLICE, LATHE and SPIRICARM, and the stops of three joins sat
at P1 or P2 under a P0 frontier (`work/README.md`, Priority: a blocker
inherits the band of what waits on it). Those rows were re-banded P0 in
the same change.

## The joins

| join | pin | stops today at | then, as far as is known |
|---|---|---|---|
| **klein wall 3**: the swept loop ∪ the revolved bulb | `demos/tour/src/klein.rs` `wall3_pinned` | `CurvedPierceEscalated { operand: B, face: 4v1 Cylinder, edge: 10v1, Crossing(OnEdge) }` at ε 1e-9 (measured 2026-11-22 on `lathe/skin-sub-arc-walls`): since the sweep cuts the loop's walls at their sub-arc joints, the boolean meets first the loop's seam at a profile vertex (in `y = 0`, a seam main already had), which ends exactly on the bulb's neck cylinder's edge and reads 3.2e-9 there, in band, `splice/a-loop-seam-meeting-the-neck-cylinder-at-its-edge-reads-in-band`. Its stop before, `GermFrameUnsupported { a_face: 6v1 Cone, b_face: 4v1 Nurbs }`, is no longer reached | the NURBS × cone section, `sectarms/a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm`, as far as is known |
| **klein wall 4**: bulb ∖ loop, the self-intersection cut | `demos/tour/src/klein.rs` `wall4_pinned` | wall 3's stop, the same refusal and payload as wall 3's union (`wall4_pinned` compares them), `splice/a-loop-seam-meeting-the-neck-cylinder-at-its-edge-reads-in-band` | everything wall 3 meets, under ∖ |
| **teapot wall 2**: handle (torus) ∪ vessel (sphere), drawn | `demos/tour/src/teapot.rs` `wall_probes` | the union BUILDS and validates, and its volume holds to the operands' less their overlap (`teapot::handle_overlap_volume`); the handle was re-cut so its roots are set into the belly wall (2026-11-25). The joined pot does not MESH: `UnsupportedCurve` from `mesh::chords::chart_image` on the section edge between the sphere and the torus, whose image on either face is a projected spline, `tesslane/cone-and-sphere-faces-with-conic-or-spline-trims-have-no-trimmed-arm` (the sphere's half) and `tesslane/a-torus-face-bounded-by-a-projected-spline-image-has-no-chord-bound` (the torus's). The montage draws pot and handle as two bodies | the sphere's projected-spline speed bound, eleven orders loose, `tesslane/a-projected-spline-images-speed-bound-on-a-sphere-is-eleven-orders-loose` |
| **teapot wall 3**: lofted spout ∪ vessel | `demos/tour/src/teapot.rs` `wall3_pinned` | past the join at ε 1e-6 and 1e-9: the section of the belly's sphere and the spout's outer wall is read and its chords certify, and the union stops at the inside/outside test, which cannot cross a spline face: `Containment(KindUnsupported { kind: Nurbs })` on the spout's outer wall, `inside/point-containments-ray-test-has-no-spline-arm`. At 1e-12 it stops in the join instead: a chord of the belly's sphere on a spout wall escalates at rest (`Escalated { check: PlaneNurbsHullValue }`, a 1.01e-12 m miss), `sectarms/a-curved-partners-arc-is-imaged-at-rest-no-finer-than-the-mints-finest-rung`. Its stops before, the NURBS × sphere germ frame (`GermFrameUnsupported { a_face: 6v1 Sphere, b_face: 3v3 Nurbs }`, measured 2026-11-04), the seam split (PR 6441) and the held pierce (PR 6435), are taken. This row named the germ frame until 2026-11-25, after the probe had moved past it | not measured past containment |
| **lily wall 8**: the lofted leaf grown out of the stem's tube | `demos/tour/src/lily.rs` `wall_probes` | `GermSectionUnbuilt { a_kind: Torus, b_kind: Nurbs, why: Refused(Unaccounted { … 200000-cell budget }) }`, the stem's tube against the leaf's ridge wall, measured at ε 1e-9 on 2026-11-24 (ORBIT's PR 7713, which retired `GermSectionSpiric`, the leaf's tilted cap's old stop; that pair is now the section value's carried section): the chart accounting runs out of cells against the thin torus, `ssiarith/the-chart-accounting-clears-a-thin-torus-only-in-sub-millimetre-cells` | the NURBS × torus section, `sectarms/a-nurbs-face-against-a-torus-has-no-section-arm`, and behind it `secttorus/c5-plane-torus-cone-cylinder-arms`, which that row is parked on |

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
  - cone × cone and cone × sphere in general pose,
    `sectcone/cone-sphere-and-cone-cone-in-general-pose-have-no-rung-three-door`
    (cone × torus off the axis has its door since PR 7395);
  - NURBS × cylinder, cone, sphere and torus, which is
    `sectarms/a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm`
    (X, staged) and its torus row;
  - plane × torus off the axis's planes (the general spiric), which is
    `secttorus/c5-plane-torus-cone-cylinder-arms` (X).

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
  table above, its entry in `walls::CURVED_JOIN_STOPS`, and
  `blocked_on` if the stop's row changed. The wall probe panics on any
  change of refusal, and its panic names the entry and this row; the
  tour's `the_umbrella_waits_on_every_join_stop` fails when the entry
  and `blocked_on` disagree.
- A PR that retires a wall (the join builds) deletes its table row and
  says so in its body. The row closes when the table is empty.
- A new curved demo join joins the table when its scene pins it, with
  `walls::join_wall` as its probe and an entry in
  `walls::CURVED_JOIN_STOPS`.

Moved from `work/issues/` to `work/splice/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.
