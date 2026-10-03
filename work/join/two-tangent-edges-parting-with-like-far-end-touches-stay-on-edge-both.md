---
id: two-tangent-edges-parting-with-like-far-end-touches-stay-on-edge-both
kind: issue
title: Two tangent edges that leave a vertex-pair site together and part, with their far ends recorded alike, both read OnEdge and the join cannot pair either
status: closed
opened: 2026-10-03
priority: P2
cost: H
design: true
refs: [a-germ-tangent-to-an-edge-reads-on-edge-where-its-arc-leaves-the-edge]
branch: join/2-zip-reads-segments
closed: 2026-10-03
pr: 3880
---


Found by JOIN-2 (PR 3880), fixing
`a-germ-tangent-to-an-edge-reads-on-edge-where-its-arc-leaves-the-edge`.

## What

`crates/topo/src/boolean/sectors.rs` `germ_loci`: where both sectors of
a germ at a vertex-pair site hold a bound read On, and the two edges
part (their far ends are not a recorded vertex pair), the segment runs
along at most one of them. `germ_loci` picks the one whose far end the
reduction recorded deeper in the partner (`Touch`: no contact, a vertex
pair, a vertex on a face). Two edges with the same `Touch` are left as
they read, both `OnEdge`. Then `join::partners` cannot pair either end,
and the op refuses `Join(UnpairedLooseEnds)`.

Two `Boundary` ends is the open shape: a line and an arc that leave a
site tangent, each ending on the other solid's boundary. The records
cannot tell which one lies inside the other's face. Which runs inside
is a trim fact, or second-order: the arc's curvature against the side
of the line its partner face lies on. Two `Apart` ends read `InFace` on
both sides, which is the truth: neither edge carries the segment.

No row in the suites or JOIN-1's batteries reaches the open shape
(measured on PR 3880's head). A fixture would be a rounded plate on a
rounded plate whose fillets are tangent at a shared vertex, with each
straight edge ending on the other's wall.

## Why it matters

It is the remaining case where a first-order reading names a germ's
cell. The fork is whether to decide it from the trim (a point of each
edge against the partner face) or from curvature. Neither is a record
the reduction already holds.

## Reached

PR 3880's review reached the shape, and on its first head it lost
unions main built through the zip's own enumeration:

- R1's `join2_r1_like_far_ends` (`ell a, r`): a plate with a convex
  fillet into a notch, under an L whose reflex corner sits on the
  fillet's far end. Main built the QA union SOUND at 3 radii; the head
  refused both orders `Join(UnpairedLooseEnds { count: 2 })`.
- R2's `join2_r2_like_far_end_touches`: a plate with a convex fillet
  then a notch open to the south, under a plate whose bottom edge is
  tangent at the fillet and re-meets the plate at the notch corner.
  Main built 24 of the 36 unions SOUND; the head refused all 36.

## Built (`join/2-zip-reads-segments`)

The trim decides it. `germ_loci`'s tie arm asks, for each edge, whether
its midpoint lies inside the partner's face the germ runs along: the
face across the partner's own edge from its sector's face
(`sectors.rs` `runs_in`; `bool_germ_tie_plane` decides the midpoint on
the face's plane, then `solid_contain::point_in_face` its trim). The
sweep splits an edge at every crossing of the partner, so the midpoint
speaks for the whole edge. Exactly one inside: the segment runs along
it, and the other germ lies in `tangent_face`. Anything else (both, or
neither, or undecided: a non-plane face, a graze, no single face across
the partner's edge) stays `OnEdge` both. A face, loop or trim the read
cannot resolve refuses rather than reading as undecided.

Every pose above builds SOUND in both orders: R1's 6 `ell` union lines,
R2's 36. Pinned by `join2_r2_probes`'s
`a_like_far_ends_tie_is_decided_by_the_partner_faces_trim` and
`join2_r1_probes`'s
`unions_through_ring_vertices_and_like_far_ends_build_sound`.

## Process note (JOIN orchestrator, 2026-10-03)

This row carries `design: true`, and it was decided without the
two-designer weighing `work/README.md` asks for: the orchestrator's
fix-pass brief chose the partner face's trim as the tie rule, and the
lane built it. The delta review flagged it (S7). What bounds the risk:
a wrong pick refuses at the join and never ships a body (the delta
review's mutant M2b: the trim pick reversed gives 42 SOUND→refusal and
0 BAD). The flag stays set so the record says what kind of question it
was.
