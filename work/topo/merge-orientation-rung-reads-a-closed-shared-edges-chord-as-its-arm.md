---
id: merge-orientation-rung-reads-a-closed-shared-edges-chord-as-its-arm
kind: issue
title: topo: the declared-pair orientation rung is levered at the shared edge's chord, which is 0 on a closed edge, so it refuses a legitimate merge (and F7 reads the same chord)
status: review
pr: 3992
branch: topo/merge-orientation-rung-levers-at-the-extent
opened: 2026-09-30
priority: P2
cost: M
---


(TOPO, the fix pass of PR 3506: the review's MINOR-1.)

## What

`Body::planes_declared_equal` (`crates/topo/src/merge_faces.rs`)
verifies a declared face pair through `plane_eq`'s declared rung at
`arm = self.edge_chord_len(edge)`, the distance between the shared
edge's two end vertices. The F7 gate does the same:
`boolean::reduce::gate_maximal_faces` levers `oriented_plane_eq` at
`reduce::edge_chord_len`, read the same way.

- The orientation rung (`bool_plane_orient`) decides `cos · arm`, and
  it is asked only once parallelism (`|sin| · arm`) has read within the
  zero band at that arm. So `|cos| ≈ 1`, and the rung refuses exactly
  when the arm itself is within the band, whatever the faces' angle.
- On a CLOSED shared edge (a circle whose ends are one vertex) the
  chord is exactly 0, so every declared pair meeting there refuses,
  however clearly the two faces face the same way. Two coplanar faces
  split by a circle (a peg's top inside a ring's hole, after a union
  whose merge stage declares them) are the shape.
- On an open edge shorter than `K·ε` the refusal is honest: its ends
  do not lie clearly apart, and PR 3506's merge lever says so.

## Probe

In-crate, uncommitted (`crates/topo/src/torusfix_probe.rs`, adapted
from the review's `torusr_probe.rs`): two identical planes compared by
the declared rung at the arm a closed edge gives, routed as the merge
routes them.

```rust
let plane = PlaneDesc { origin: Point3::new(0.0, 0.0, 1.0), normal: Vec3::new(0.0, 0.0, 1.0) };
let declared = PlaneIdentity { s1: None, s2: None, declared: true };
let got = declared_pair_verdict(oriented_plane_eq(&plane, &plane, declared, 0.0, band), f, f);
```

It renders "whether the two declared faces face the same way across the
edge they share is undecided: margin 0e0 lies within the zero band
(±1e-9). Recourse: turn one of the two faces so both clearly face the
same way, across a shared edge whose ends lie clearly apart", on faces
that already face the same way. At F7 (`PlaneDoor::Neighbours`) the
same arm renders "whether two neighbouring faces of one operand lie on
one plane is undecided: margin 0e0 …", where the pair is in fact
coplanar and should be `NonMaximalFaces`.

## Repair shape

Lever the rung at a length the edge actually spans: the edge's extent
(the arc's length, or its bounding box's diagonal), which a closed
edge has, not its chord. Then the merge's lever and size noun read the
extent, and the F7 gate's orientation arm stops refusing coplanar
neighbours across a circle.

## What moved under it (PR 3532)

`Body::planes_declared_equal` no longer takes the edge key: it takes
the shared edge's two halves as the adjacency scan resolved them, and
`edge_chord_len(a, b)` reads the chord between their start vertices,
announcing a vertex or point that does not resolve. The
`unwrap_or_else(T::one)` fallback is gone. An extent lever therefore
has to thread the edge key back in (the scan holds it) and look up the
edge's curve, announcing a failed lookup the same way
(`DanglingRef::Geometry(GeomRef::Curve(..))`) rather than falling back
to a length; nothing else in the merge's half of this fix moved.
