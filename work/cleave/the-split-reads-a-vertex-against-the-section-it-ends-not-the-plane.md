---
id: the-split-reads-a-vertex-against-the-section-it-ends-not-the-plane
kind: unit
title: The split decides a vertex's distance from the section it will end (delta / sin psi), not its offset from the plane
status: open
opened: 2026-10-12
priority: P2
cost: M
parent: a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join
---

Part 4 of the design on
`a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join.md`.
It is separable from parts 1–3.

## What

`split_vertex_side` decides δ, the vertex's offset from the plane. The
edge certificate reads the vertex's distance from the section it ends,
which is δ/sin ψ, where ψ is the dihedral between the plane and the
wall at the vertex. Measured in a replica: δ = 0.2ε at ψ = atan 0.1
reads 2.01ε (review m2 on PR 4810; the brief's "0.5ε" pose is likely
0.2ε). A vertex admitted Zero at 0.99ε with ψ = 0.05 misses its
section by 20ε: a definite miss ending "kernel defect" for an input
the kernel accepted. That is two decisions inferred from each other
(Q1).

## Owed

Decide the vertex against the section it will end (δ/sin ψ over its
curved incident faces) at the split. Do the same at the Boolean's
germ-plane classification. Pin the m2 family (normal (cos φ, sin φ,
0.1), φ ∈ {0.9, 2.2}) at every ε. Once this lands, a definite endpoint
miss at the join is a defect, and the certificate's
`EndpointStart → Defect` row is true.
