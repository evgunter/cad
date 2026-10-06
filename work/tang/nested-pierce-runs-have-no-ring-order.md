---
id: nested-pierce-runs-have-no-ring-order
kind: issue
title: A pierce whose Out runs nest about the pierced face's normal has no ring order, and ring_order's sort is unpinned
status: open
opened: 2026-10-06
priority: P1
cost: H
---


## What

Found in PR 4129's full review.

`boolean/vtxfac.rs` `classify_vertex_on_face` hangs a pierce's ring
struts in the order `ring_order` reads off the runs' start germs. Each
strut faces its germs from the next run's start germ. Both readings
assume the runs' Out wedges are disjoint about the pierced face's
outward normal, and nothing checks that. Runs of the piercing vertex on
one side of the plane are disjoint arcs of its link, but their
projections can nest, one run's germ interval inside another's.

- **The nested pose.** The reviewer reached it with an "arch" cone: three
  pyramid legs below the top and arches above it, apex at the vertex. It
  is also the only pose found where run order differs from angular order.
  It refuses downstream, not here.
- **The sort is unpinned.** In every pose that builds (the leaning wedges
  of `crates/topo/tests/holes_meeting_at_a_vertex.rs`, k = 2, 3, 4,
  reflex sectors and an L), run order is already angular order. The
  mutant `hang = 0..k` survives every row.

## The shape to give

A row where run order differs from angular order and the body builds,
with `topo::test_support::meeting::corners_disjoint` on it. Then either
give nested runs an order of their own, or refuse them typed at
`classify_vertex_on_face` before any strut is hung. Today the reviewer's
arch pose fails at a later step.
