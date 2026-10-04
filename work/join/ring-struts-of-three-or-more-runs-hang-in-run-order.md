---
id: ring-struts-of-three-or-more-runs-hang-in-run-order
kind: issue
title: A pierce's ring struts hang round the ring vertex in run order, unmeasured for three or more runs
status: open
opened: 2026-10-04
priority: P3
cost: M
---


## What

Found while fixing `a-pierce-with-two-out-runs-at-one-vertex-refuses-every-op`.

`boolean/vtxfac.rs` `classify_vertex_on_face`, step 3, hangs one ring
null-edge strut per piercing-side run at the ring vertex. The first
strut grows from the empty ring (`MevSite::Lone`). Every later strut
is spliced just before the first strut's `he_plus`, so the struts lie
round the ring vertex in run order, which is the piercing vertex's
orbit order.

- With one or two runs, the order is free: two struts have one cyclic
  order.
- Each strut's facing is read by the walk about the pierced face's
  outward normal (`insert::strut_order`), for any number of runs.
- With three or more runs, the struts' cyclic order must also match
  the angular order of their runs' Out wedges about that normal.

Run order need not be that order. A vertex's link can cross the
pierced plane out of angular order, since its arcs on each side only
have to nest. Nothing reaches three runs today. The L-prism's reflex
corner has at most two in `join_pierce_runs_sweep.rs`, and no battery
was seen to mint a third.

## The shape to give

Build a fixture whose vertex has three Out runs against a face, for
example a vertex of valence four or more with two reflex face angles.
Then hang the ring struts in the walk's order, or refuse typed until
a row pins the order.
