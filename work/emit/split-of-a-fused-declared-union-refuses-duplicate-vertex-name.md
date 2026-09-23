---
id: split-of-a-fused-declared-union-refuses-duplicate-vertex-name
kind: issue
title: A split of a legal declared union refuses Naming(Duplicate): a CrossingVertex and an OnToolVertex mint one name
status: dispatched
opened: 2026-09-23
priority: P0
cost: D
branch: emit/split-duplicate-vertex
---


## What

Splitting a legal declared union with a legal plane refuses
`NamingError::Duplicate`. That is an emission-bug refusal, and it reads
as "the kernel is broken". Two different vertices on the `Below` side
are handed the same name:

- a `CrossingVertex { side: Below, edge: … }`, where the plane cuts
  `a`'s rim edge `RimEdge(End, 2)`, `Fragment(OrderAlong { rank: 0, of: 2 })`;
- an `OnToolVertex { side: Below, of: a.CapVertex(End, 2) }`, where the
  plane passes through `a`'s cap vertex.

Either the split mints both, or two of its arms reach one name
(`crates/editor-core/src/names/emit_topo.rs`, `name_split`'s vertex
pass). **The cause is unmeasured**, including whether the `Duplicate` is
honest (two arms naming one vertex) or a real collision between two
vertices.

## Evidence

Measured by PR 3120's review (scratch at
`~/.local/share/cad-work/emit-mc-review/p2.txt`; not committed).

The union: `a` = x∈(0,1), `b` = x∈(0.5,1.5), both y,z∈(0,1), declared
`flush_pairs((a, a), (b, b))`. The third member `g` = x∈(1.2,1.3),
y∈(−1,2), z from 0.5 to 3.0 (`dz` 2.5).

- **Refusing orders:** `[a, g, b]` and `[g, a, b]`. Both unions fuse
  once PR 3120 lands, and both refused
  `Emission("seam edge between two merged faces")` before it.
- **The split:** the plane through (0, 1, 1) with normal (0, 1, 1)/√2.
  It refuses `Duplicate` with the two names above.
- **The same shape elsewhere:** the review's U-prism `bridge-slab` and
  `bridge-slab2` documents, split at 45°. `bridge-slab` refuses in 2
  orders and `bridge-slab2` in 4, all at the same split.

## Why it matters

A normal verb (split) breaks on normal geometry (a union of blocks)
with a refusal that tells the author to file a kernel bug. It is P0 for
the reason `seam-edge-between-two-merged-faces-refusal-a-legal-declared-union-reaches`
was.

## Found by

PR 3120's review (MINOR-2), filed by that PR's fix pass.
