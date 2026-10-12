---
id: the-edge-plane-lane-copies-no-ellipse-edge
kind: issue
title: The edge-plane lane copies no ellipse edge: a germ along an ellipse rim inside a curved face refuses as a section invariant
status: open
opened: 2026-10-12
priority: P2
cost: M
refs: [crossing-equal-cylinders-take-the-parallel-radical-plane-and-desync]
---


Found by `cleave/crossing-cylinders-join` once the join's kind arm
stopped reading every cylinder pair as parallel.

## What

The fixture is `crates/sweep/tests/crossing_equal_rods.rs`: two
radius-1 rods on crossing axes, `A` mitred by their bisector plane
`z = x` so that its ellipse rim lies on `B`'s wall. The germ runs along
`A`'s ellipse edge inside `B`'s wall face. `join::kind_arm` now reads
the axes apart (`face_pair_axes_parallel`), so the pair has no kind arm
and the edge-plane lane takes it (`GermLane::EdgePlane`, its plane the
ellipse's own). On the edge's side that lane's chord copies the edge
(`RingClosure::AlongEdge`, `JoinLane::AlongEdge`), and
`chord_join::along_edge_spec` copies only a line or a circle. Every op
in both orders, at spins `(0.4, 0.3)`, `(1.0, −0.7)` and `(0, 0)` and
with `B` from `x = 0.2` and `0.5`, at ε = 1e-9, 1e-6 and 1e-12, refuses

`Join(SectionInvariant { what: "a section segment along an edge whose
carrier has no chord lane (only lines and circles are copied)" })`.

That is a kernel-invariant type on valid input for a lane that is not
built, so the refusal does not tell its cause. `join`'s edge-plane
lane admits a circle or an ellipse edge (`edge_lane` in
`bool_connect`), but only the circle has a copy. The adjacency skip
can leave the edge's side with no chord to mint, and then the copy is
never asked, so refusing every ellipse at admission could refuse poses
that build today. Where the copy is asked is `chord_join`'s plan
(`JoinPlan::first` and `second`, private to that module).

## Measured past it

On the branch, uncommitted: `along_edge_spec` given an ellipse arm (the
circle arm's end-matching, the carrier reversed where the chord runs
against it, described as an image in the divided face's chart
`EdgeDescriptionSpec::chart`) moves all 24 runs to
`CurvedPairUnsupported { site: InteriorLoopGuard, operand: A, kind:
Cylinder }`. So the copy alone does not build the scene. The interior
loop guard is the next door.

## Direction

- Type the frontier where the copy is refused, so the refusal names an
  unbuilt lane and not a broken invariant. Its home is
  `chord_join::along_edge_spec`, a file other lanes are working in.
- Give an ellipse edge a copy. `EdgeCurveSpec::arc_of_circle`'s
  scaffold pushforward has no ellipse counterpart. The question is
  which description an ellipse copy carries: the chart of the face it
  divides, or the source edge's own.
- Then take the interior-loop guard measured above. The fixture flips
  to building at its closed forms (`vol A = 3π`, `vol B = (3 − x0)π`,
  and the shared volume in `shared`). Its `NO_ELLIPSE_COPY` acceptance
  comes out at the same time.
