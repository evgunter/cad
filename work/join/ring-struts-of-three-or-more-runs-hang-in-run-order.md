---
id: ring-struts-of-three-or-more-runs-hang-in-run-order
kind: issue
title: A pierce's ring struts hang round the ring vertex in run order, unmeasured for three or more runs
status: closed
opened: 2026-10-04
priority: P3
cost: M
closed: 2026-10-06
parent: three-wedges-meeting-at-a-point-on-a-face-refuse-in-every-member-order
branch: tang/holes-meeting-at-a-vertex
pr: 4129
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

Today the pierce refuses typed: `BooleanError::PierceRunsUnordered`
(`vtxfac::classify_vertex_on_face`, right after the runs are read),
since PR 4026's fix pass. Its doc says what would license it.

Build a fixture whose vertex has three Out runs against a face, for
example a vertex of valence four or more with two reflex face angles.
A prism's vertex cannot do it: its link meets a plane at most four
times, so at most two runs (PR 4026's review r2, N3). Then hang the
ring struts in the walk's order, pin it with a row, and retire the
refusal.

## Evidence (the six-crossing pairing)

This is the vertex-on-face sibling of the vertex-vertex six-crossing
row, which builds as of branch `join/six-crossing-pairing`. Three Out
runs are six crossings of the piercing vertex's link with the pierced
plane's great circle. The runs on one side of the plane are disjoint
arcs of one hemisphere, so their germ pairs nest round the circle and
cannot cross. The ring struts' cyclic order has to follow that nesting
(`insert::b_runs` reads the same structure in B's walk order).

Still unreached: 6 048 face-placement runs of the 343° notch, the L
prism and a 203° shallow reflex, every op and both orders over the
pierce sweep's grid, are all SOUND, and none mints a third run.

## Closed (2026-10-06, TANG, PR 4129)

Closed in place by TANG's PR 4129
(`three-wedges-meeting-at-a-point-on-a-face-refuse-in-every-member-order`),
whose leaning wedges reach three and four Out runs. A prism's vertex
cannot reach three, but the union of several prisms' vertices at one
point can.
`vtxfac::ring_order` hangs the ring struts clockwise about the pierced
face's outward normal, starting from run 0's wedge, and
`PierceRunsUnordered` is retired. The row that pins the order is
`holes_meeting_at_a_vertex`'s `corners_disjoint`. That check is
independent of the facing rule: in the mirror order, the top face's
corners at the vertex overlap in each order that folds three wedges
before the plate.
