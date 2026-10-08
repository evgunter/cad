---
id: a-pinch-union-refuses-ring-homing-in-one-member-order
kind: issue
title: Two blocks meeting at a corner on a plate's top refuse RingHomingAmbiguous in one member order and publish in the others
status: closed
opened: 2026-09-30
priority: P1
cost: M
closed: 2026-10-02
---


## What

Two blocks whose footprints meet at one corner on a plate's top: the
plate `[0,3] × [0,2] × [0,1]`, `p1` = x∈(1.0,1.5), y∈(−1,1), z∈(0.5,2.0),
`p2` = x∈(1.5,2.0), y∈(1,3), z∈(0.47,1.7). The two footprints touch at
(1.5, 1.0) only, so the plate's top is two pieces that meet at a vertex
(a pinch). The union publishes in five member orders and refuses in
`[p2, p1, plate]`:

`Boolean(Join(RingHomingAmbiguous { .. }))`: "a hole loop sits on the
divided face's outer boundary, so which piece holds it cannot be
decided".

The order dependence is the defect: the other five orders build the
same body. Found by the obstacle-mechanism measurement (branch
`emit/borders-mechanism-probe`, `crates/editor-core/tests/borders_probe.rs`,
fixture `pinch`); the refusal is raised by the ring rehoming in
`crates/topo/src/chord_join.rs`
(`SplitJoinError::RingHomingAmbiguous`). The same order-set's
`[p1, p2, plate]` body trips the tessellator's watertight census
(`work/tess/a-pinch-union-body-trips-the-watertight-census-in-one-member-order.md`).

## Outcome (2026-10-02, branch `tang/pinch-union-order`)

**Cause.** `p1` and `p2` touch along the vertical line x = 1.5, y = 1, and
the union keeps that contact as two coincident edges. A fold that joins
the blocks first then meets the plate's top with two edges piercing it at
one point, and mints one ring vertex per pierce. A fold that reaches the
plate before the second block meets the point as a vertex already there
(a v-v event) and builds one valence-6 vertex.
- `[p1, p2, plate]` therefore kept the top as one face through two
  coincident vertices (F 18, V 33): tier 3 passed, and the tessellator's
  census tripped.
- `[p2, p1, plate]` refused earlier: ring homing tested only a ring's
  anchor vertex, which was the other pierce's twin, so it landed
  `OnBoundary`.

This is a planar defect, not the valence-4 vocabulary of
`pinch-carrying-machinery-valence-4.md`. The result's pinch vertex is an
ordinary manifold vertex whose umbrella is one cycle of six faces.

**Fix.**
- `rehome_rings` homes a ring by its first vertex off the dividing run.
- After the carve, `weld_pinches` joins two pierces that survive on one
  kept face with a zero-length edge and collapses it. Across one loop
  this divides the face; across two loops (holes meeting at a corner) it
  joins them.
- The zip reads a pinch's correspondent per seam and through earlier
  fusions.
- The namer names the fused vertex by its seam junction.

All six orders now build F 19, E 49, V 32, tier-3 valid, at volume
8.215, the closed form, and every result tessellates.

**Filed.**
- `three-wedges-meeting-at-a-point-on-a-face-refuse-in-every-member-order`
  (TANG)
- `a-pinch-vertexs-name-depends-on-the-unions-member-order` (WIRE)

**Fix pass (2026-10-02).** On review, the first weld regressed bodies
main built correctly. It took its site from the first face around the
pierce that held both copies, and that could be a section face. Its seam
map also assumed one correspondent per seam. With a slab `X` that holds
the contact against the plate, all six ops refused, including
`plate − X` and both unions, which main builds.
- The weld site now comes from lineage: the pierced face's chord-mef
  fragments, section faces aside.
- The zip matches each run of a seam by both of its ends, so a seam that
  meets a welded pinch twice, on either side, zips.
- The tessellator's census expects two uses per chord that carries a
  segment.
- Now `plate − X` and both unions build main's body, identical by
  geometry, and `X − plate` and both intersections build and tessellate.
  Main built those three but could not tessellate them.
- Blocks through the plate and a third block making a second pinch
  build one body in every member order. On main both refused in some
  orders, and in others built bodies that did not tessellate.

Holes touching at a corner still refuse when the blocks fold first: the
pinch's strut ring has every vertex on the run. Filed:
`a-pierce-strut-at-a-pinch-has-no-vertex-off-the-run` (TANG).

**Fix pass 2 (2026-10-02).**
- Both operands' weld fusions are now recorded where the contact remap
  and the naming read them (`BooleanNaming::weld_merges_b`, B-clone
  keys).
- Every merge chain is read by one fold, `zip::survivor`.
- **What stays divergent, filed:**
  - The pinch union's contact record and its 3′ verdict follow the
    member folded last. The two orders that fold the plate last publish
    no v-v record and fail 3′.
    - The cause is not the weld. A boolean drops its operands' own
      contact records.
    - Main does the same at topo level.
    - Main refused `[p2, p1, plate]` loudly; this branch publishes that
      order's body, which fails 3′ silently.
    - Filed as `work/wire/a-boolean-drops-its-operands-own-contact-records.md`.
  - X ∩ P and P ∩ X mesh a doubled edge as one segment of four
    triangles, which `check_mesh` refuses. Filed as
    `work/tess/two-coincident-edges-between-one-vertex-pair-mesh-non-manifold.md`.
- `union_pinch_member_order.rs` asserts both divergences, citing the two
  rows.

## Closed (2026-10-02, TANG, PR 3796)

Every member order of the unit's pinch union builds the same tier-3
body, 19 / 49 / 32 at 8.215, and so do the sweep's siblings: the
side-face pinch, blocks through the plate, two pinches in all 24
orders, and the slab holding the contact in all six ops. The pinch is
welded on the kept fragment of the pierced face, with the fragment read
from lineage; bystander rings are homed by their first vertex off the
run. Three review rounds, the last two delta reviews, all
APPROVE-WITH-FIXES. Left open elsewhere:
- `a-pierce-strut-at-a-pinch-has-no-vertex-off-the-run` (m1, with the
  3N-staircase witness);
- `three-wedges-meeting-at-a-point-on-a-face-refuse-in-every-member-order`;
- WIRE `a-boolean-drops-its-operands-own-contact-records` (the record
  and 3′ verdict follow the member folded last);
- WIRE `a-pinch-vertexs-name-depends-on-the-unions-member-order`;
- TESS `two-coincident-edges-between-one-vertex-pair-mesh-non-manifold`.
