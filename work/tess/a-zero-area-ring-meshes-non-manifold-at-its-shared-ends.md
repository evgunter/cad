---
id: a-zero-area-ring-meshes-non-manifold-at-its-shared-ends
kind: issue
title: A face's zero-area ring meshes non-manifold: its two coincident edges share one index segment, or the floor's triangles wind them the same way
status: open
opened: 2026-10-19
priority: P2
cost: M
---


Found by `cleave/keel-slit-ring`, which builds the split's zero-area
ring (`work/cleave/a-keel-in-the-plane-wrapped-round-at-both-ends-refuses-a-degenerate-section.md`).

## What

`crates/topo/tests/a_keel_wrapped_at_both_ends_splits_as_a_ring.rs`,
row "closed at both ends": the box `[0, 4] × [0, 4] × [−1, 1]` less a
V-trench along x over `x ∈ [1.5, 2.5]`, split at `z = 0`. The side the
trench's walls are on carries the keel on its section face as a ring of
two coincident straight edges sharing both end vertices. It passes tiers
1, 2, 3 and 3′. Measured at the default ε:

- `mesh::tessellate` then `mesh::validate::check_mesh` refuses
  `NonManifoldEdge { count: 4 }` at the keel's vertex-index pair, at
  chordal 5e-3 and 0.1, either normal. `check_mesh` counts edges by
  vertex-index pair (`crates/mesh/src/validate.rs`, `check_mesh`), and
  both straight edges are the one segment between the keel's two
  vertices: wall a, wall b and the floor's triangles either side of the
  slit all use it. `signed_volume` is right (15.7).
- The curved twin (`crates/sweep/tests/a_curved_keel_in_the_plane.rs`,
  `a_curved_keel_closed_at_both_ends_splits_with_its_keel_a_ring`, its
  walls' side, chordal 5e-3) refuses `MismatchedWinding` with the V's tip
  on the plane and at `+0.6 ε`, and passes at `−0.6 ε`.
- The one-end slit (the same trench over `x ∈ [1.5, 4.5]`, and the
  curved one-end rows) meshes clean: its two edges end at two copies of
  the open end, so their segments differ.

## Owed

A mesh of a ring's two coincident edges that keeps them apart: the
tessellator keys a segment by B-rep edge (an interior index on each of
two edges that share both ends), or the census counts uses per B-rep
edge. Then the curved arm's winding, read once the straight one meshes.
