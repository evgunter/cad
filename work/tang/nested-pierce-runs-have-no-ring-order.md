---
id: nested-pierce-runs-have-no-ring-order
kind: issue
title: A pierce whose Out runs nest about the pierced face's normal hangs its ring struts in run order, which crosses the face's loop
status: closed
opened: 2026-10-06
closed: 2026-10-08
priority: P1
cost: H
---


## What

Found in PR 4129's full review.

`boolean/vtxfac.rs` `classify_vertex_on_face` hangs a pierce's ring
struts in the order `ring_order` reads off the runs' start germs. Each
strut faces its germs from the next run's start germ. Both readings
assume the runs' Out wedges are disjoint about the pierced face's
outward normal; the detector below now checks the order they imply. Runs of the piercing vertex on
one side of the plane are disjoint arcs of its link, but their
projections can nest, one run's germ interval inside another's.

- **The nested pose.** The reviewer reached it with an "arch" cone: three
  pyramid legs below the top and arches above it, apex at the vertex. It
  is also the only pose found where run order differs from angular order.
  It refused downstream (`JoinDesync`) until the detector below.
- **The sort was unpinned.** In every pose that builds (the leaning wedges
  of `crates/topo/tests/holes_meeting_at_a_vertex.rs`, k = 2, 3, 4,
  reflex sectors and an L), run order is already angular order, and the
  mutant `hang = 0..k` survived every row. The hang is now run order
  itself, guarded by the detector below, and the arch row kills a mutant
  that drops the detector.

## The shape to give

A row where run order differs from angular order and the body builds,
with `topo::test_support::meeting::corners_disjoint` on it. Then either
give nested runs an order of their own, or refuse them typed at
`classify_vertex_on_face` before any strut is hung. Today the reviewer's
arch pose fails at a later step.

## The detector (PR 4129, review 5)

The sort is now a check. `classify_vertex_on_face` hangs the struts in
run order, the order along the vertex's link, and faces each from the
next run's start germ in that same order. Before any write, it reads
the runs' angular order from their start germs (`ring_order`), and
where that is not the identity it refuses `PierceRunsNested`. So "the
hang is the angular order" is guarded rather than measured.

Witnesses:
- the reviewer's arch cone (three pyramid legs below the top, arches
  above it, apex at the vertex), which gave `[0, 2, 1]` in 90 calls,
  each ending in `JoinDesync` downstream before the detector;
- `topo::test_support::meeting::arch`: three wedges leant across one
  another (0–30° leaning 75°, 120–150° leaning 15°, 240–270° on its
  bisector). Its prisms' union refuses `PierceRunsNested` with three
  runs in every op against the plate, both orders
  (`holes_whose_runs_nest_at_their_vertex_refuse_typed_in_every_op`).

Review 5's N6: before the detector, on the head the reviewer read
(158db6a9), the arch fixture (the crossing pair and one wedge, in every
pose) refused `JoinDesync` for P − U, U − P, P ∩ U and P ∪ U, where
main had refused typed at vtxfac (`PierceRunsUnordered`). With the
detector, `meeting::arch` refuses `PierceRunsNested` in all six ops
(P − U, U − P, P ∪ U, U ∪ P, P ∩ U, U ∩ P), which the row asserts.

## The strut facing at k ≥ 3 is unpinned

With the runs' Out wedges disjoint and in clockwise order, the walk
from the next run's start germ meets this run's start before its end,
so every strut faces `Below`. Measured over the five holes rows: 6,498
k ≥ 3 readings, all `Below` (1,727 at k = 3, 4,771 at k = 4). The
mutant "always `Below` at k ≥ 3" survives every row. An `Above` at
k ≥ 3 needs a run whose start and end germs lie along one direction,
such as a reflex sector's bisector alone (the k = 2 unit row
`vtxfac::tests::a_bisector_run_after_the_other_run_keeps_its_corner`),
beside two more runs. No row reaches that pose.

Done when nested runs get an order of their own, the detector
retires, and a k ≥ 3 row with a bisector run pins the facing.


## Closed

`classify_vertex_on_face` sorts all the runs' germs clockwise about the
pierced face's normal (`germ_order`, the existing `bool_strut_side` and
`bool_strut_order` decides) and hangs the ring's struts as the tree that
sort gives (`crate::null::ring_tree`, no predicate of its own). The
order detector is retired. Runs nested under one hang as a star in an
order other than run order, every strut facing `Above` at k = 3
(`meeting::comb`); a run between others, and deeper nests, hang as a
path or a tree (`meeting::arch`, `meeting::arch_cone`,
`meeting::branching_cone`), and all build in every op and pose. The
enclosing case, filed as
`a-pierce-whose-run-chords-enclose-one-another-needs-a-tree-ring`, is
closed in the same PR. Facings alternate strictly by depth, so `Below`
and `Above` mix at one point only across depths.
